// GraphQL API для аутентификации (leptos-auth)
// Использует leptos-graphql для всех запросов

use serde::{Deserialize, Serialize};

use crate::{AuthError, AuthSession, AuthUser};

// ============================================================================
// GraphQL Queries & Mutations
// ============================================================================

const SIGN_IN_MUTATION: &str = r#"
mutation SignIn($email: String!, $password: String!) {
    signIn(email: $email, password: $password) {
        token
        user {
            id
            email
            name
        }
    }
}
"#;

const SIGN_UP_MUTATION: &str = r#"
mutation SignUp($email: String!, $password: String!, $name: String) {
    signUp(email: $email, password: $password, name: $name) {
        token
        user {
            id
            email
            name
        }
    }
}
"#;

const SIGN_OUT_MUTATION: &str = r#"
mutation SignOut {
    signOut
}
"#;

const CURRENT_USER_QUERY: &str = r#"
query CurrentUser {
    currentUser {
        id
        email
        name
    }
}
"#;

const FORGOT_PASSWORD_MUTATION: &str = r#"
mutation ForgotPassword($email: String!) {
    forgotPassword(email: $email)
}
"#;

const RESET_PASSWORD_MUTATION: &str = r#"
mutation ResetPassword($token: String!, $newPassword: String!) {
    resetPassword(token: $token, newPassword: $newPassword)
}
"#;

const REFRESH_TOKEN_MUTATION: &str = r#"
mutation RefreshToken {
    refreshToken {
        token
    }
}
"#;

// ============================================================================
// Response Types (GraphQL)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SignInData {
    #[serde(rename = "signIn")]
    sign_in: SignInPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SignInPayload {
    token: String,
    user: AuthUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SignUpData {
    #[serde(rename = "signUp")]
    sign_up: SignUpPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SignUpPayload {
    token: String,
    user: AuthUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CurrentUserData {
    #[serde(rename = "currentUser")]
    current_user: AuthUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RefreshTokenData {
    #[serde(rename = "refreshToken")]
    refresh_token: RefreshTokenPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RefreshTokenPayload {
    token: String,
}

// ============================================================================
// Helper function: Execute GraphQL request
// ============================================================================

async fn execute_graphql<V, T>(
    query: &str,
    variables: Option<V>,
    token: Option<String>,
    tenant: String,
) -> Result<T, AuthError>
where
    V: Serialize,
    T: for<'de> Deserialize<'de>,
{
    // Import leptos-graphql types
    use serde_json::Value;
    
    // Build GraphQL request
    #[derive(Serialize)]
    struct GraphQLRequest<V> {
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        variables: Option<V>,
    }
    
    let request = GraphQLRequest {
        query: query.to_string(),
        variables,
    };
    
    // Send HTTP request
    let client = reqwest::Client::new();
    let mut req = client
        .post("http://localhost:5150/api/graphql")
        .json(&request)
        .header("X-Tenant-Slug", tenant);
    
    if let Some(t) = token {
        req = req.header("Authorization", format!("Bearer {}", t));
    }
    
    let response = req.send().await.map_err(|_| AuthError::Network)?;
    
    if response.status() == 401 {
        return Err(AuthError::Unauthorized);
    }
    
    if !response.status().is_success() {
        return Err(AuthError::Http(response.status().as_u16()));
    }
    
    // Parse GraphQL response
    #[derive(Deserialize)]
    struct GraphQLResponse<T> {
        data: Option<T>,
        errors: Option<Vec<GraphQLError>>,
    }
    
    #[derive(Deserialize)]
    struct GraphQLError {
        message: String,
    }
    
    let graphql_response: GraphQLResponse<T> = response
        .json()
        .await
        .map_err(|_| AuthError::Network)?;
    
    if let Some(errors) = graphql_response.errors {
        if let Some(err) = errors.first() {
            // Check for specific error types
            if err.message.contains("Invalid credentials") 
                || err.message.contains("Invalid email or password") {
                return Err(AuthError::InvalidCredentials);
            }
            if err.message.contains("Unauthorized") {
                return Err(AuthError::Unauthorized);
            }
            return Err(AuthError::Network);
        }
    }
    
    graphql_response.data.ok_or(AuthError::Network)
}

// ============================================================================
// Public API Functions
// ============================================================================

pub async fn sign_in(
    email: String,
    password: String,
    tenant: String,
) -> Result<(AuthUser, AuthSession), AuthError> {
    let variables = serde_json::json!({
        "email": email,
        "password": password,
    });
    
    let response: SignInData = execute_graphql(
        SIGN_IN_MUTATION,
        Some(variables),
        None, // no token yet
        tenant.clone(),
    )
    .await?;
    
    let session = AuthSession {
        token: response.sign_in.token,
        tenant,
    };
    
    Ok((response.sign_in.user, session))
}

pub async fn sign_up(
    email: String,
    password: String,
    name: Option<String>,
    tenant: String,
) -> Result<(AuthUser, AuthSession), AuthError> {
    let variables = serde_json::json!({
        "email": email,
        "password": password,
        "name": name,
    });
    
    let response: SignUpData = execute_graphql(
        SIGN_UP_MUTATION,
        Some(variables),
        None, // no token yet
        tenant.clone(),
    )
    .await?;
    
    let session = AuthSession {
        token: response.sign_up.token,
        tenant,
    };
    
    Ok((response.sign_up.user, session))
}

pub async fn sign_out(token: &str, tenant: &str) -> Result<(), AuthError> {
    let _: serde_json::Value = execute_graphql(
        SIGN_OUT_MUTATION,
        None::<serde_json::Value>,
        Some(token.to_string()),
        tenant.to_string(),
    )
    .await?;
    
    Ok(())
}

pub async fn get_current_user(token: &str, tenant: &str) -> Result<AuthUser, AuthError> {
    let response: CurrentUserData = execute_graphql(
        CURRENT_USER_QUERY,
        None::<serde_json::Value>,
        Some(token.to_string()),
        tenant.to_string(),
    )
    .await?;
    
    Ok(response.current_user)
}

pub async fn forgot_password(email: String, tenant: String) -> Result<(), AuthError> {
    let variables = serde_json::json!({
        "email": email,
    });
    
    let _: serde_json::Value = execute_graphql(
        FORGOT_PASSWORD_MUTATION,
        Some(variables),
        None,
        tenant,
    )
    .await?;
    
    Ok(())
}

pub async fn reset_password(
    token: String,
    new_password: String,
    tenant: String,
) -> Result<(), AuthError> {
    let variables = serde_json::json!({
        "token": token,
        "newPassword": new_password,
    });
    
    let _: serde_json::Value = execute_graphql(
        RESET_PASSWORD_MUTATION,
        Some(variables),
        None,
        tenant,
    )
    .await?;
    
    Ok(())
}

pub async fn refresh_token(token: &str, tenant: &str) -> Result<String, AuthError> {
    let response: RefreshTokenData = execute_graphql(
        REFRESH_TOKEN_MUTATION,
        None::<serde_json::Value>,
        Some(token.to_string()),
        tenant.to_string(),
    )
    .await?;
    
    Ok(response.refresh_token.token)
}
