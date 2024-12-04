use serde::Serialize;
use axum::http::StatusCode;
use axum::Json;
use projects_client::axum::extractors::ProjectsClient;
use projects_client::source_requests::SourceRequestsClient;
use axum::extract::Path;
use projects_client::source_requests::list;

#[derive(Serialize)]
pub struct ListSourceRequestsResponse {
    #[serde(rename = "sr")]
    pub source_requests: Vec<SourceRequest>,
}

#[derive(Serialize)]
pub struct SourceRequest {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "sr")]
    pub source_request: SourceRequestSummary,
}

impl From<list::SourceRequest> for SourceRequest {
    fn from(source_request: list::SourceRequest) -> Self {
        SourceRequest {
            id: source_request.id,
            source_request: source_request.source_request.into(),
        }
    }
}

#[derive(Serialize)]
pub enum SourceRequestSummary {
    #[serde(rename = "n")]
    New(NewSourceRequestSummary),
    #[serde(rename = "a")]
    Approved(ApprovedSourceRequestSummary),
    #[serde(rename = "c")]
    Completed(CompletedSourceRequestSummary),
}

impl From<list::Summary> for SourceRequestSummary {
    fn from(source_request: list::Summary) -> Self {
        match source_request {
            list::Summary::New(sr) => SourceRequestSummary::New(sr.into()),
            list::Summary::Approved(sr) => SourceRequestSummary::Approved(sr.into()),
            list::Summary::Completed(sr) => SourceRequestSummary::Completed(sr.into()),
        }
    }
}

#[derive(Serialize)]
pub struct NewSourceRequestSummary {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
}

impl From<list::New> for NewSourceRequestSummary {
    fn from(new: list::New) -> Self {
        NewSourceRequestSummary {
            project_id: new.project_id,
            user_id: new.user_id,
            title: new.title,
        }
    }
}

#[derive(Serialize)]
pub struct ApprovedSourceRequestSummary {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
}

impl From<list::Approved> for ApprovedSourceRequestSummary {
    fn from(approved: list::Approved) -> Self {
        ApprovedSourceRequestSummary {
            project_id: approved.project_id,
            user_id: approved.user_id,
            title: approved.title,
        }
    }
}

#[derive(Serialize)]
pub struct CompletedSourceRequestSummary {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
}

impl From<list::Completed> for CompletedSourceRequestSummary {
    fn from(completed: list::Completed) -> Self {
        CompletedSourceRequestSummary {
            project_id: completed.project_id,
            user_id: completed.user_id,
            title: completed.title,
        }
    }
}

pub async fn list_source_requests(
    projects_client: ProjectsClient,
    Path(project_id): Path<String>,
) ->  Result<Json<ListSourceRequestsResponse>, StatusCode> {
    let response = projects_client
        .list_source_requests(&project_id)
        .await;

    let source_requests = match response {
        Ok(source_requests) => source_requests,
        Err(projects_client::Error::Status(StatusCode::NOT_FOUND)) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let response = ListSourceRequestsResponse {
        source_requests: source_requests
            .source_requests
            .into_iter()
            .map(Into::into)
            .collect()

    };

    Ok(Json(response))
}