use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role_enum")]
pub enum RoleEnum {
    #[sea_orm(string_value = "user")]
    User,
    #[sea_orm(string_value = "vip")]
    Vip,
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "super_admin")]
    SuperAdmin,
}

#[derive(Debug, Clone)]
pub enum Identity {
    User,
    Vip,
    Admin,
    SuperAdmin,
}
impl std::fmt::Display for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

impl Identity {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Identity::User => "user",
            Identity::Vip => "vip",
            Identity::Admin => "admin",
            Identity::SuperAdmin => "super_admin",
        }
    }
    pub(crate) fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "vip" => Identity::Vip,
            "admin" => Identity::Admin,
            "super_admin" => Identity::SuperAdmin,
            _ => Identity::User,
        }
    }
    #[allow(unused)]
    pub fn role_to_identity(role: RoleEnum) -> Self {
        match role {
            RoleEnum::User => Identity::User,
            RoleEnum::Vip => Identity::Vip,
            RoleEnum::Admin => Identity::Admin,
            RoleEnum::SuperAdmin => Identity::SuperAdmin,
        }
    }
}

/// 实现自定义的序列化 trait
impl serde::Serialize for Identity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// 实现自定义的反序列化 trait
impl<'de> serde::Deserialize<'de> for Identity {
    fn deserialize<D>(deserializer: D) -> anyhow::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Identity::from_str(&s))
    }
}
