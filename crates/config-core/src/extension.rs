use crate::source::ConfigurationSource;
use crate::validation::Validation;

pub trait Extension {
    fn register_source(&mut self, source: Box<dyn ConfigurationSource>);

    fn register_validator(&mut self, validator: Box<dyn Validation>);
}
