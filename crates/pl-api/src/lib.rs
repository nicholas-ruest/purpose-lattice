use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use pl_auth::{authenticate, require, Principal};
use pl_config::RuntimeConfig;
use pl_errors::LatticeError;
use pl_orchestrator::{LatticeService, RegisterCovenant};
use pl_plan::ProcessingPlan;
use pl_repository::Repository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
#[derive(Clone)]
pub struct AppState {
    service: LatticeService,
    auth_mode: String,
}
#[derive(Serialize)]
struct Health {
    status: &'static str,
    service: &'static str,
}
#[derive(Deserialize)]
struct Session {
    role: String,
}
fn correlation(h: &HeaderMap) -> String {
    pl_middleware::validate_correlation(h.get("x-correlation-id").and_then(|v| v.to_str().ok()))
        .unwrap_or_else(|_| "invalid".into())
}
fn failure(e: LatticeError, id: String) -> Response {
    let p = e.problem(id);
    (
        StatusCode::from_u16(p.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        Json(p),
    )
        .into_response()
}
async fn session(Json(s): Json<Session>) -> Response {
    if !["steward", "analyst"].contains(&s.role.as_str()) {
        return failure(LatticeError::Unauthorized, "session".into());
    }
    let cookie = format!(
        "pl_session={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=3600",
        s.role
    );
    let mut r = (StatusCode::NO_CONTENT, ()).into_response();
    r.headers_mut()
        .insert(header::SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());
    r
}
async fn auth(
    State(s): State<Arc<AppState>>,
    h: HeaderMap,
    mut req: axum::extract::Request,
    next: Next,
) -> Response {
    match authenticate(
        h.get(header::COOKIE).and_then(|v| v.to_str().ok()),
        &s.auth_mode,
    ) {
        Ok(p) => {
            req.extensions_mut().insert(p);
            next.run(req).await
        }
        Err(e) => failure(e, correlation(&h)),
    }
}
async fn health() -> Json<Health> {
    Json(Health {
        status: "ok",
        service: "purpose-lattice",
    })
}
async fn create(
    State(s): State<Arc<AppState>>,
    h: HeaderMap,
    Extension(p): Extension<Principal>,
    Json(c): Json<RegisterCovenant>,
) -> Response {
    if let Err(e) = require(&p, "steward").and_then(|_| {
        pl_middleware::validate_idempotency(h.get("idempotency-key").and_then(|v| v.to_str().ok()))
            .map(|_| ())
    }) {
        return failure(e, correlation(&h));
    }
    match s.service.register(c, &correlation(&h)) {
        Ok(r) => (StatusCode::CREATED, Json(r)).into_response(),
        Err(e) => failure(e, correlation(&h)),
    }
}
async fn list(State(s): State<Arc<AppState>>, h: HeaderMap) -> Response {
    match s.service.list() {
        Ok(r) => Json(r).into_response(),
        Err(e) => failure(e, correlation(&h)),
    }
}
async fn evaluate(
    State(s): State<Arc<AppState>>,
    h: HeaderMap,
    Json(p): Json<ProcessingPlan>,
) -> Response {
    if let Err(e) =
        pl_middleware::validate_idempotency(h.get("idempotency-key").and_then(|v| v.to_str().ok()))
    {
        return failure(e, correlation(&h));
    }
    match s.service.evaluate(p, &correlation(&h)) {
        Ok(r) => (StatusCode::CREATED, Json(r)).into_response(),
        Err(e) => failure(e, correlation(&h)),
    }
}
async fn audit(State(s): State<Arc<AppState>>, h: HeaderMap, Path(id): Path<String>) -> Response {
    match s.service.audit(&id) {
        Ok(r) => Json(r).into_response(),
        Err(e) => failure(e, correlation(&h)),
    }
}
pub fn router(c: &RuntimeConfig) -> Result<Router, LatticeError> {
    let repo = Repository::open(&c.namespace_dir())?;
    pl_seed::seed(repo.clone())?;
    let state = Arc::new(AppState {
        service: LatticeService::new(repo)?,
        auth_mode: c.auth_mode.clone(),
    });
    let protected = Router::new()
        .route("/covenants", post(create).get(list))
        .route("/covenants/{id}/evaluate", post(evaluate))
        .route("/covenants/{id}/audit", get(audit))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth));
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact(HeaderValue::from_static(
            "http://127.0.0.1:14273",
        )))
        .allow_credentials(true)
        .allow_headers([
            header::CONTENT_TYPE,
            header::HeaderName::from_static("idempotency-key"),
            header::HeaderName::from_static("x-correlation-id"),
        ])
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST]);
    Ok(Router::new()
        .route("/healthz", get(health))
        .route("/readyz", get(health))
        .route("/api/v1/session", post(session))
        .nest("/api/v1", protected)
        .with_state(state)
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(cors))
}
pub async fn serve(c: RuntimeConfig) -> Result<(), LatticeError> {
    let l = tokio::net::TcpListener::bind(c.bind)
        .await
        .map_err(|e| LatticeError::Dependency(e.to_string()))?;
    axum::serve(l, router(&c)?)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .map_err(|e| LatticeError::Dependency(e.to_string()))
}
