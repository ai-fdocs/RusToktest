pub mod auth;
pub mod error;
pub mod id;
pub mod module;
pub mod permissions;
pub mod types;
pub use error::{Error, Result};
pub use id::generate_id;
pub use module::RusToKModule;
pub use permissions::{get_role_permissions, Module, Permission, PermissionKey};
pub use types::{UserRole, UserStatus};

pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::id::generate_id;
    pub use crate::permissions::{get_role_permissions, Module, Permission, PermissionKey};
    pub use crate::types::{UserRole, UserStatus};
    pub use uuid::Uuid;
}
