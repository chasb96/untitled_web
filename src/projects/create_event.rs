use std::collections::HashMap;

use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Path, http::StatusCode, Json};
use projects_client::axum::extractors::ProjectsClient;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum EventRequest {
    #[serde(rename = "a")]
    AddFiles(AddFilesRequest),
    #[serde(rename = "rm")]
    RemoveFiles(RemoveFilesRequest),
    #[serde(rename = "mv")]
    RenameFiles(RenameFilesRequest),
}

impl Into<projects_client::create_event::EventRequest> for EventRequest {
    fn into(self) -> projects_client::create_event::EventRequest {
        match self {
            EventRequest::AddFiles(request) => projects_client::create_event::EventRequest::AddFiles(request.into()),
            EventRequest::RemoveFiles(request) => projects_client::create_event::EventRequest::RemoveFiles(request.into()),
            EventRequest::RenameFiles(request) => projects_client::create_event::EventRequest::RenameFiles(request.into()),
        }
    }
}

#[derive(Deserialize)]
pub struct AddFilesRequest {
    #[serde(rename = "pe")]
    pub previous_event_id: String,
    #[serde(rename = "f")]
    pub files: Vec<AddFileRequest>,
}

impl Into<projects_client::create_event::AddFilesRequest> for AddFilesRequest {
    fn into(self) -> projects_client::create_event::AddFilesRequest {
        projects_client::create_event::AddFilesRequest {
            previous_event_id: self.previous_event_id,
            files: self.files
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct AddFileRequest {
    #[serde(rename = "p")]
    pub path: String,
    #[serde(rename = "f")]
    pub file_id: String,
}

impl Into<projects_client::create_event::AddFileRequest> for AddFileRequest {
    fn into(self) -> projects_client::create_event::AddFileRequest {
        projects_client::create_event::AddFileRequest {
            path: self.path,
            file_id: self.file_id,
        }
    }
}

#[derive(Deserialize)]
pub struct RemoveFilesRequest {
    #[serde(rename = "pe")]
    pub previous_event_id: String,
    #[serde(rename = "p")]
    pub paths: Vec<String>,
}

impl Into<projects_client::create_event::RemoveFilesRequest> for RemoveFilesRequest {
    fn into(self) -> projects_client::create_event::RemoveFilesRequest {
        projects_client::create_event::RemoveFilesRequest {
            previous_event_id: self.previous_event_id,
            paths: self.paths,
        }
    }
}

#[derive(Deserialize)]
pub struct RenameFilesRequest {
    #[serde(rename = "pe")]
    pub previous_event_id: String,
    #[serde(rename = "p")]
    pub paths: HashMap<String, String>,
}

impl Into<projects_client::create_event::RenameFilesRequest> for RenameFilesRequest {
    fn into(self) -> projects_client::create_event::RenameFilesRequest {
        projects_client::create_event::RenameFilesRequest {
            previous_event_id: self.previous_event_id,
            paths: self.paths,
        }
    }
}

pub async fn create_event(
    Authenticate(user): Authenticate<ClaimsUser>,
    projects_client: ProjectsClient,
    Path(project_id): Path<String>,
    Json(request): Json<EventRequest>,
) -> Result<StatusCode, StatusCode> {
    let project = projects_client
        .get_project_by_id(&project_id)
        .await;

    let project = match project {
        Ok(project) => project,
        Err(projects_client::Error::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if project.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    let result = projects_client
        .create_event(&project_id, request.into())
        .await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(projects_client::Error::Status(status_code)) => Err(status_code),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}