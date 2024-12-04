use metrics_client::ViewUserRequest;
use metrics_client::MetricsClient;

use super::Message;
use super::error::HandleError;

pub struct UserViewed {
    pub id: String,
}

impl UserViewed {
    pub async fn handle(self) -> Result<(), HandleError> {
        MetricsClient::default()
            .view_user(ViewUserRequest {
                user_id: self.id,
            })
            .await
            .map_err(HandleError::from)
    }
}

impl Into<Message> for UserViewed {
    fn into(self) -> Message {
        Message::UserViewed(self)
    }
}