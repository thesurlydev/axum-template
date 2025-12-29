use crate::{
    common::{
        app_state::AppState,
        dto::RestApiResponse,
        error::AppError,
        jwt::Claims,
        pagination::{PageRequest, PageResponse},
        validated_json::ValidatedJson,
    },
    domain::user::{
        CreateUserDto, PagedUserDto, SearchUserDto, UpdateUserDto, UserDto, UserId,
        UserServiceTrait,
    },
};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension,
};

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses((status = 200, description = "Get user by ID", body = UserDto)),
    tag = "Users"
)]
pub async fn get_user_by_id(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = UserId::from(id);
    let user = state.user_service.get_user_by_id(&user_id).await?;
    Ok(RestApiResponse::success(UserDto::from(user)))
}

#[utoipa::path(
    get,
    path = "/users",
    params(
        ("id" = Option<String>, Query, description = "Filter by user ID"),
        ("username" = Option<String>, Query, description = "Filter by username"),
        ("email" = Option<String>, Query, description = "Filter by email"),
        PageRequest,
    ),
    responses((status = 200, description = "List users with optional filters", body = PagedUserDto)),
    tag = "Users"
)]
pub async fn get_user_list(
    State(state): State<AppState>,
    Query(params): Query<SearchUserDto>,
    Query(page_request): Query<PageRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (users, total) = state.user_service.get_user_list(params, &page_request).await?;
    let user_dtos: Vec<UserDto> = users.into_iter().map(UserDto::from).collect();
    let response: PagedUserDto = PageResponse::new(user_dtos, total, &page_request).into();
    Ok(RestApiResponse::success(response))
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserDto,
    responses((status = 201, description = "User created successfully", body = UserDto)),
    tag = "Users"
)]
pub async fn create_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(mut payload): ValidatedJson<CreateUserDto>,
) -> Result<impl IntoResponse, AppError> {
    payload.modified_by = claims.sub.clone();

    let user = state.user_service.create_user(payload).await?;

    Ok(RestApiResponse::created(UserDto::from(user)))
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    request_body = UpdateUserDto,
    responses((status = 200, description = "Update user", body = UserDto)),
    tag = "Users"
)]
pub async fn update_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<String>,
    ValidatedJson(mut payload): ValidatedJson<UpdateUserDto>,
) -> Result<impl IntoResponse, AppError> {
    // Set the modified_by field to the current user's ID.
    payload.modified_by = claims.sub.clone();

    let user_id = UserId::from(id);
    let user = state.user_service.update_user(&user_id, payload).await?;
    Ok(RestApiResponse::success(UserDto::from(user)))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    responses((status = 204, description = "User deleted")),
    tag = "Users"
)]
pub async fn delete_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<StatusCode, AppError> {
    let user_id = UserId::from(id);
    state.user_service.delete_user(&user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
