use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error("{0}")]
pub struct InvalidOperation(pub &'static str);

#[derive(Debug, Clone, Error)]
pub struct InvalidValue {
    pub expected: Option<String>,
    pub found: Option<String>,
    pub additional_msg: Option<String>
}

impl Display for InvalidValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut e = String::from("invalid value encountered, ");

        if self.expected.is_some() {
            e.push_str(format!("expected: '{}', ", self.expected.as_ref().unwrap()).as_str());
        } else {
            e.push_str("expected value not provided, ");
        };

        if self.found.is_some() {
            e.push_str(format!("found: '{}'", self.found.as_ref().unwrap()).as_str());
        } else {
            e.push_str("found value not provided");
        };

        if self.additional_msg.is_some() {
            e.push_str(format!(", {}", self.additional_msg.as_ref().unwrap()).as_str());
        };

        write!(f, "{}", e)
    }
}

impl InvalidValue {
    pub fn new() -> InvalidValue {
        Self {
            expected: None,
            found: None,
            additional_msg: None
        }
    }

    pub fn with_expected<T: Into<String>>(mut self, expected: T) -> InvalidValue {
        self.expected = Some(expected.into());
        self
    }

    pub fn with_found<T: Into<String>>(mut self, found: T) -> InvalidValue {
        self.found = Some(found.into());
        self
    }

    pub fn with_additional_msg<T: Into<String>>(mut self, msg: T) -> InvalidValue {
        self.additional_msg = Some(msg.into());
        self
    }
}

impl Default for InvalidValue {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Error)]
#[error("{0}")]
pub struct InvalidState(pub &'static str);
