use error::HandleError;

use super::UserViewed;

mod error;
pub mod user_viewed;

pub trait Queueable:  {
    async fn handle(self) -> Result<(), HandleError>;
}

pub enum Message {
    UserViewed(UserViewed),
}

impl Queueable for Message {
    async fn handle(self) -> Result<(), HandleError> {
        match self {
            Self::UserViewed(message) => message.handle().await,
        }
    }
}