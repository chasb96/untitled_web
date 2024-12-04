use error::HandleError;

use super::ProjectViewed;

mod error;
pub mod project_viewed;

pub trait Queueable:  {
    async fn handle(self) -> Result<(), HandleError>;
}

pub enum Message {
    ProjectViewed(ProjectViewed),
}

impl Queueable for Message {
    async fn handle(self) -> Result<(), HandleError> {
        match self {
            Self::ProjectViewed(message) => message.handle().await,
        }
    }
}