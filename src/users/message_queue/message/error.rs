use std::fmt::Display;
use std::fmt;
use std::error::Error;

use metrics_client::Error as MetricsClientError;

#[derive(Debug)]
pub enum HandleError {
    MetricsClientError(MetricsClientError)
}

impl Error for HandleError { }

impl Display for HandleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error handling message: ")?;

        match self {
            Self::MetricsClientError(e) => write!(f, "MetricsClientError({})", e),
        }
    }
}

impl From<MetricsClientError> for HandleError {
    fn from(value: MetricsClientError) -> Self {
        HandleError::MetricsClientError(value)
    }
}