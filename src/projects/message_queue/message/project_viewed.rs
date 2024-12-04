use metrics_client::ViewProjectRequest;
use metrics_client::MetricsClient;

use super::error::HandleError;
use super::Message;

pub struct ProjectViewed {
    pub id: String,
}

impl ProjectViewed {
    pub async fn handle(self) -> Result<(), HandleError> {
        MetricsClient::default()
            .view_project(ViewProjectRequest {
                project_id: self.id,
            })
            .await
            .map_err(HandleError::from)
    }
}

impl Into<Message> for ProjectViewed {
    fn into(self) -> Message {
        Message::ProjectViewed(self)
    }
}