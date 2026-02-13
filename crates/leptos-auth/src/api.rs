use serde::{Deserialize, Serialize};

use crate::{AuthError, AuthSession, AuthUser};

const API_BASE: &str = "/api/auth";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignInResponse {
    #[serde(rename = "access_token")]
    pub token: String,
    pub user: AuthUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignUpResponse {
    pub user: AuthUser,
    #[serde(rename = "access_token")]
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

async fn fetch_json<T: for<'de> Deserialize<'de>>(
    url: &str,
    method: &str,
    body: Option<String>,
    token: Option<String>,
    tenant: Option<String>,
) -> Result<T, AuthError> {
    let client = reqwest::Client::new();
    
    let mut req = match method {
        "GET" => client.get(url),
        "POST" => {
            let mut r = client.post(url);
            if let Some(body_data) = body {
                r = r.header("Content-Type", "application/json").body(body_data);
            }
            r
        }
        _ => return Err(AuthError::Network),
    };
    
    if let Some(token_val) = token {
        req = req.header("Authorization", format!("Bearer {}", token_val));
    }
    
    if let Some(tenant_val) = tenant {
        req = req.header("X-Tenant-Slug", tenant_val);
    }
    
    let resp = req.send().await.map_err(|_| AuthError::Network)?;
    let status = resp.status().as_u16();
    
    if status == 401 {
        return Err(if method == "POST" && url.contains("/login") {
            AuthError::InvalidCredentials
        } else {
            AuthError::Unauthorized
        });
    }
    
    if !resp.status().is_success() {
        return Err(AuthError::Http(status));
    }
    
    resp.json().await.map_err(|_| AuthError::Network)
}

pub async fn sign_in(
    email: String,
    password: String,
    tenant: String,
) -> Result<(AuthUser, AuthSession), AuthError> {
    let body = SignInRequest { email, password };
    let body_str = serde_json::to_string(&body).map_err(|_| AuthError::Network)?;
    
    let resp: SignInResponse = fetch_json(
        &format!("{}/login", API_BASE),
        "POST",
        Some(body_str),
        None,
        Some(tenant.clone()),
    )
    .await?;
    
    let session = AuthSession {
        token: resp.token,
        tenant,
    };
    
    Ok((resp.user, session))
}

pub async fn sign_up(
    email: String,
    password: String,
    name: Option<String>,
    tenant: String,
) -> Result<(AuthUser, AuthSession), AuthError> {
    let body = SignUpRequest {
        email,
        password,
        name,
    };
    let body_str = serde_json::to_string(&body).map_err(|_| AuthError::Network)?;
    
    let resp: SignUpResponse = fetch_json(
        &format!("{}/register", API_BASE),
        "POST",
        Some(body_str),
        None,
        Some(tenant.clone()),
    )
    .await?;
    
    let session = AuthSession {
        token: resp.token,
        tenant,
    };
    
    Ok((resp.user, session))
}

pub async fn sign_out(token: &str) -> Result<(), AuthError> {
    let _: serde_json::Value = fetch_json(
        &format!("{}/logout", API_BASE),
        "POST",
        None,
        Some(token.to_string()),
        None,
    )
    .await?;
    
    Ok(())
}

pub async fn get_current_user(token: &str, tenant: &str) -> Result<AuthUser, AuthError> {
    fetch_json(
        &format!("{}/me", API_BASE),
        "GET",
        None,
        Some(token.to_string()),
        Some(tenant.to_string()),
    )
    .await
}

pub async fn forgot_password(email: String) -> Result<(), AuthError> {
    let body = ForgotPasswordRequest { email };
    let body_str = serde_json::to_string(&body).map_err(|_| AuthError::Network)?;
    
    let _: serde_json::Value = fetch_json(
        &format!("{}/forgot-password", API_BASE),
        "POST",
        Some(body_str),
        None,
        None,
    )
    .await?;
    
    Ok(())
}

pub async fn reset_password(token: String, new_password: String) -> Result<(), AuthError> {
    let body = ResetPasswordRequest {
        token,
        new_password,
    };
    let body_str = serde_json::to_string(&body).map_err(|_| AuthError::Network)?;
    
    let _: serde_json::Value = fetch_json(
        &format!("{}/reset-password", API_BASE),
        "POST",
        Some(body_str),
        None,
        None,
    )
    .await?;
    
    Ok(())
}

pub async fn refresh_token(token: &str, tenant: &str) -> Result<String, AuthError> {
    #[derive(Deserialize)]
    struct RefreshResponse {
        #[serde(rename = "access_token")]
        token: String,
    }
    
    let resp: RefreshResponse = fetch_json(
        &format!("{}/refresh", API_BASE),
        "POST",
        None,
        Some(token.to_string()),
        Some(tenant.to_string()),
    )
    .await?;
    
    Ok(resp.token)
}
