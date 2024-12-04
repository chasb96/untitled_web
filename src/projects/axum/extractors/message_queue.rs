use std::ops::Deref;

use axum::{async_trait, extract::FromRequestParts};
use axum::http::StatusCode;
use axum::http::request::Parts;

use crate::projects::message_queue::MessageQueueProducer;

pub struct MessageQueueExtractor(MessageQueueProducer);

impl Deref for MessageQueueExtractor {
    type Target = MessageQueueProducer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for MessageQueueExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for MessageQueueExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(MessageQueueExtractor::default())
    }
}