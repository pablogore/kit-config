use crate::sources::ConfigurationSource;
use crate::validation::Validation;

/// Base configuration module trait
/// 
/// This trait defines the interface for all configuration modules in the framework.
/// Each module must implement how to provide default configuration values.
/// 
/// # Examples
/// 
/// Implementing a configuration module:
/// ```
/// use kit_config::modules::ConfigModule;
/// 
/// #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
/// struct MyModule {
///     pub value: String,
/// }
/// 
/// impl ConfigModule for MyModule {
///     fn defaults() -> Self {
///         Self {
///             value: "default".to_string(),
///         }
///     }
/// }
/// ```
pub trait ConfigModule {
    /// Get default configuration values
    /// 
    /// # Returns
    /// * `Self` - A new instance with default values
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::modules::ConfigModule;
    /// 
    /// // Assuming you have a module that implements ConfigModule
    /// // let defaults = MyModule::defaults();
    /// ```
    fn defaults() -> Self
    where
        Self: Sized;
}

/// Extension trait for configuration modules
/// 
/// This trait provides extension capabilities for configuration modules,
/// allowing registration of custom sources and validators.
/// 
/// # Examples
/// 
/// Using extension capabilities:
/// ```
/// use kit_config::modules::Extension;
/// use kit_config::sources::ConfigurationSource;
/// use kit_config::validation::Validation;
/// 
/// // Assuming you have a module that implements Extension
/// // let mut module = MyModule::defaults();
/// // module.register_source(Box::new(MyCustomSource::new()));
/// // module.register_validator(Box::new(MyCustomValidator::new()));
/// ```
pub trait Extension {
    /// Register custom source
    /// 
    /// # Arguments
    /// * `source` - A boxed configuration source trait object
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::modules::Extension;
    /// use kit_config::sources::ConfigurationSource;
    /// 
    /// // Assuming you have a module that implements Extension
    /// // let mut module = MyModule::defaults();
    /// // module.register_source(Box::new(MyCustomSource::new()));
    /// ```
    fn register_source(&mut self, source: Box<dyn ConfigurationSource>);
    
    /// Register custom validator
    /// 
    /// # Arguments
    /// * `validator` - A boxed validation trait object
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::modules::Extension;
    /// use kit_config::validation::Validation;
    /// 
    /// // Assuming you have a module that implements Extension
    /// // let mut module = MyModule::defaults();
    /// // module.register_validator(Box::new(MyCustomValidator::new()));
    /// ```
    fn register_validator(&mut self, validator: Box<dyn Validation>);
}

pub mod infra;
pub mod logging;