use crate::UserRole;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Стандартные модули системы
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Module {
    Core,
    Commerce,
    Blog,
    Media,
    Settings,
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Module::Core => "core",
            Module::Commerce => "commerce",
            Module::Blog => "blog",
            Module::Media => "media",
            Module::Settings => "settings",
        };
        write!(f, "{name}")
    }
}

impl FromStr for Module {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "core" => Ok(Module::Core),
            "commerce" => Ok(Module::Commerce),
            "blog" => Ok(Module::Blog),
            "media" => Ok(Module::Media),
            "settings" => Ok(Module::Settings),
            _ => Err(format!("Unknown module: {value}")),
        }
    }
}

/// Действия (permissions)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Permission {
    Create,
    Read,
    Update,
    Delete,
    Manage,
    Publish,
    Approve,
    Invite,
    Remove,
    Custom(String),
}

impl Permission {
    pub fn as_str(&self) -> &str {
        match self {
            Permission::Create => "create",
            Permission::Read => "read",
            Permission::Update => "update",
            Permission::Delete => "delete",
            Permission::Manage => "manage",
            Permission::Publish => "publish",
            Permission::Approve => "approve",
            Permission::Invite => "invite",
            Permission::Remove => "remove",
            Permission::Custom(value) => value.as_str(),
        }
    }
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Permission {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "create" => Ok(Permission::Create),
            "read" => Ok(Permission::Read),
            "update" => Ok(Permission::Update),
            "delete" => Ok(Permission::Delete),
            "manage" => Ok(Permission::Manage),
            "publish" => Ok(Permission::Publish),
            "approve" => Ok(Permission::Approve),
            "invite" => Ok(Permission::Invite),
            "remove" => Ok(Permission::Remove),
            custom => Ok(Permission::Custom(custom.to_string())),
        }
    }
}

/// Полное представление права: module.action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionKey {
    pub module: Module,
    pub action: Permission,
}

impl PermissionKey {
    pub fn new(module: Module, action: Permission) -> Self {
        Self { module, action }
    }

    pub fn as_string(&self) -> String {
        format!("{}.{}", self.module, self.action)
    }
}

/// Базовый мэппинг ролей к стандартным правам
pub fn get_role_permissions(role: &UserRole) -> Vec<PermissionKey> {
    match role {
        UserRole::SuperAdmin => vec![
            PermissionKey::new(Module::Core, Permission::Manage),
            PermissionKey::new(Module::Commerce, Permission::Manage),
            PermissionKey::new(Module::Blog, Permission::Manage),
            PermissionKey::new(Module::Media, Permission::Manage),
            PermissionKey::new(Module::Settings, Permission::Manage),
        ],
        UserRole::Admin => vec![
            PermissionKey::new(Module::Core, Permission::Manage),
            PermissionKey::new(Module::Commerce, Permission::Manage),
            PermissionKey::new(Module::Blog, Permission::Manage),
            PermissionKey::new(Module::Media, Permission::Manage),
        ],
        UserRole::Manager => vec![
            PermissionKey::new(Module::Core, Permission::Read),
            PermissionKey::new(Module::Commerce, Permission::Manage),
            PermissionKey::new(Module::Blog, Permission::Manage),
            PermissionKey::new(Module::Media, Permission::Manage),
        ],
        UserRole::Customer => vec![PermissionKey::new(Module::Core, Permission::Read)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_key_as_string() {
        let key = PermissionKey::new(Module::Core, Permission::Read);
        assert_eq!(key.as_string(), "core.read");
    }

    #[test]
    fn parse_module_and_permission() {
        assert_eq!(Module::from_str("blog").unwrap(), Module::Blog);
        assert_eq!(Permission::from_str("manage").unwrap(), Permission::Manage);
    }
}
