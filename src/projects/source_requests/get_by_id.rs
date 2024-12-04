use serde::Serialize;
use axum::extract::Path;
use projects_client::source_requests::SourceRequestsClient;
use projects_client::axum::extractors::ProjectsClient;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::Json;
use std::collections::HashSet;
use projects_client::source_requests::comments::CommentsClient;
use std::collections::HashMap;
use tokio::join;
use users_client::axum::extractors::UsersClient;
use projects_client::get_project_by_id;
use projects_client::source_requests::get_by_id;

#[derive(Serialize)]
pub struct SourceRequestWrapper {
    #[serde(flatten)]
    pub source_request: SourceRequest,
    #[serde(rename = "p")]
    pub project: Project,
    #[serde(rename = "co")]
    pub comments: Vec<Comment>,
    #[serde(rename = "d")]
    pub diff: HashMap<String, DiffItem>
}

#[derive(Serialize)]
pub enum SourceRequest {
    #[serde(rename = "n")]
    New(New),
    #[serde(rename = "a")]
    Approved(Approved),
    #[serde(rename = "c")]
    Completed(Completed),
}

impl From<get_by_id::GetByIdResponse> for SourceRequest {
    fn from(source_request: get_by_id::GetByIdResponse) -> Self {
        match source_request {
            get_by_id::GetByIdResponse::New(source_request) => SourceRequest::New(source_request.into()),
            get_by_id::GetByIdResponse::Approved(source_request) => SourceRequest::Approved(source_request.into()),
            get_by_id::GetByIdResponse::Completed(source_request) => SourceRequest::Completed(source_request.into()),
        }
    }
}

#[derive(Serialize)]
pub struct FileMap {
    #[serde(rename = "p")]
    pub path: String,
    #[serde(rename = "f")]
    pub file_id: String,
}

#[derive(Serialize)]
pub struct New {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "d")]
    description: String,
    #[serde(rename = "f")]
    pub files: Vec<FileMap>,
}

impl From<get_by_id::New> for New {
    fn from(source_request: get_by_id::New) -> Self {
        Self {
            project_id: source_request.project_id,
            user_id: source_request.user_id,
            title: source_request.title,
            description: source_request.description,
            files: source_request
                .files
                .into_iter()
                .map(|file| FileMap {
                    path: file.path,
                    file_id: file.file_id,
                })
                .collect(),
        }
    }
}

#[derive(Serialize)]
pub struct Approved {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "d")]
    description: String,
    #[serde(rename = "a")]
    pub approvers: HashSet<String>,
    #[serde(rename = "f")]
    pub files: Vec<FileMap>,
}

impl From<get_by_id::Approved> for Approved {
    fn from(source_request: get_by_id::Approved) -> Self {
        Self {
            project_id: source_request.project_id,
            user_id: source_request.user_id,
            title: source_request.title,
            description: source_request.description,
            approvers: source_request.approvers,
            files: source_request
                .files
                .into_iter()
                .map(|file| FileMap {
                    path: file.path,
                    file_id: file.file_id,
                })
                .collect(),
        }
    }
}

#[derive(Serialize)]
pub struct Completed {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "d")]
    description: String,
    #[serde(rename = "a")]
    pub approvers: HashSet<String>,
    #[serde(rename = "f")]
    pub files: Vec<FileMap>,
}

impl From<get_by_id::Completed> for Completed {
    fn from(source_request: get_by_id::Completed) -> Self {
        Self {
            project_id: source_request.project_id,
            user_id: source_request.user_id,
            title: source_request.title,
            description: source_request.description,
            approvers: source_request.approvers,
            files: source_request
                .files
                .into_iter()
                .map(|file| FileMap {
                    file_id: file.file_id,
                    path: file.path,
                })
                .collect(),
        }
    }
}

#[derive(Serialize)]
pub struct Project {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "o")]
    pub owner: String,
}

impl From<get_project_by_id::ProjectResponse> for Project {
    fn from(project: get_project_by_id::ProjectResponse) -> Self {
        Self {
            id: project.id,
            owner: project.user_id,
        }
    }
}

#[derive(Serialize)]
pub struct Comment {
    #[serde(rename = "o")]
    pub owner: Option<CommentOwner>,
    #[serde(rename = "c")]
    content: String,
}

#[derive(Serialize)]
pub struct CommentOwner {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "u")]
    pub username: String,
}

#[derive(PartialEq, Serialize)]
pub struct DiffItem {
    #[serde(rename = "f")]
    pub from: String,
    #[serde(rename = "t")]
    pub to: String,
}

pub async fn get_by_id(
    projects_client: ProjectsClient,
    users_client: UsersClient,
    Path((project_id, source_request_id)): Path<(String, String)>
) -> impl IntoResponse {
    let source_request = projects_client
        .get_source_request(&project_id, &source_request_id)
        .await;

    let source_request = match source_request {
        Ok(source_request) => source_request,
        Err(projects_client::Error::Status(StatusCode::NOT_FOUND)) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let (comments, diff, project) = join!(
        projects_client.list_comments(&project_id, &source_request_id),
        projects_client.get_source_request_diff(&project_id, &source_request_id),
        projects_client.get_project_by_id(&project_id),
    );

    let (comments, diff, project) = match (comments, diff, project) {
        (Ok(comments), Ok(diff), Ok(project)) => (comments, diff, project),
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let user_ids = comments
        .comments
        .iter()
        .map(|comment| comment.user_id.clone())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();

    let map_users_response = users_client
        .map_users(user_ids)
        .await;

    let users = match map_users_response {
        Ok(map_users_response) => map_users_response.users,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let response = SourceRequestWrapper {
        source_request: source_request.into(),
        project: project.into(),
        comments: comments
            .comments
            .into_iter()
            .map(|comment| Comment {
                content: comment.content,
                owner: users
                    .get(&comment.user_id)
                    .map(|user| CommentOwner {
                        id: comment.user_id,
                        username: user.username.clone(),
                    })
            })
            .collect(),
        diff: diff
            .diff_items
            .into_iter()
            .map(|(key, value)| (key, DiffItem {
                from: value.from,
                to: value.to,
            }))
            .collect(),
    };

    Ok(Json(response))
}