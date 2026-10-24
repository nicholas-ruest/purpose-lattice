const BASE = "http://127.0.0.1:18180/api/v1";
let sequence = 0;
async function call<T>(path: string, init: RequestInit = {}): Promise<T> {
  sequence++;
  const r = await fetch(BASE + path, {
    ...init,
    credentials: "include",
    headers: {
      "content-type": "application/json",
      "x-correlation-id": "browser-" + sequence,
      "idempotency-key": "purpose-lattice-" + Date.now() + "-" + sequence,
      ...init.headers,
    },
  });
  if (!r.ok) {
    const p = await r.json().catch(() => ({ detail: r.statusText }));
    throw new Error(p.detail || p.title);
  }
  return r.status === 204 ? (undefined as T) : r.json();
}
export type Covenant = {
  dataset_id: string;
  name: string;
  owner: string;
  allowed_purposes: string[];
  sensitive_fields: string[];
  allowed_regions: string[];
  retention_days: number;
};
export type Evaluation = {
  dataset_id: string;
  decision: "allow" | "deny" | "review";
  risk_score: number;
  drift_upper: number;
  minimization_ratio: number;
  selected_columns: number;
  sensitive_columns: number;
  reasons: string[];
  analogues: unknown[];
  receipt: string;
  correlation_id: string;
};
export const api = {
  session: (role: "steward" | "analyst") =>
    call<void>("/session", { method: "POST", body: JSON.stringify({ role }) }),
  list: () => call<Covenant[]>("/covenants"),
  create: (x: unknown) =>
    call<Covenant>("/covenants", { method: "POST", body: JSON.stringify(x) }),
  evaluate: (id: string, x: unknown) =>
    call<Evaluation>("/covenants/" + id + "/evaluate", {
      method: "POST",
      body: JSON.stringify(x),
    }),
  audit: (id: string) => call<unknown[]>("/covenants/" + id + "/audit"),
};
