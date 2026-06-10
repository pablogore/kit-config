use serde_json::Value;
use crate::errors::ConfigError;

pub trait ConfigurationSource: Send + Sync {
    fn name(&self) -> &'static str;

    fn priority(&self) -> u8;

    fn is_optional(&self) -> bool;

    fn load(&self) -> Result<Value, ConfigError>;
}
