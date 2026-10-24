use pl_errors::{LatticeError, Result};
#[derive(Clone, Debug)]
pub struct Principal {
    pub role: String,
}
pub fn authenticate(cookie: Option<&str>, mode: &str) -> Result<Principal> {
    if mode != "dev-session" {
        return Err(LatticeError::Unauthorized);
    }
    let role = cookie
        .and_then(|h| {
            h.split(';')
                .map(str::trim)
                .find_map(|p| p.strip_prefix("pl_session="))
        })
        .ok_or(LatticeError::Unauthorized)?;
    if !["steward", "analyst"].contains(&role) {
        return Err(LatticeError::Unauthorized);
    }
    Ok(Principal { role: role.into() })
}
pub fn require(principal: &Principal, role: &str) -> Result<()> {
    if principal.role == role {
        Ok(())
    } else {
        Err(LatticeError::Forbidden(format!("{role} role required")))
    }
}
