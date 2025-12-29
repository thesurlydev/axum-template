use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{common::pagination::PageResponse, domain::user::User};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub created_by: Option<String>,
    #[serde(with = "crate::common::ts_format::option")]
    pub created_at: Option<DateTime<Utc>>,
    pub modified_by: Option<String>,
    #[serde(with = "crate::common::ts_format::option")]
    pub modified_at: Option<DateTime<Utc>>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id.into_inner(),
            username: user.username,
            email: user.email,
            created_by: user.created_by.map(|id| id.into_inner()),
            created_at: user.created_at,
            modified_by: user.modified_by.map(|id| id.into_inner()),
            modified_at: user.modified_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SearchUserDto {
    pub id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateUserDto {
    #[validate(length(max = 64, message = "Username cannot exceed 64 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[serde(skip_deserializing)]
    pub modified_by: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateUserDto {
    #[validate(length(max = 64, message = "Username cannot exceed 64 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub modified_by: String,
}

/// Paginated response containing a list of users.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PagedUserDto {
    /// The users on the current page
    pub items: Vec<UserDto>,
    /// Total number of users across all pages
    pub total: u64,
    /// Current page number (1-indexed)
    pub page: u32,
    /// Number of items per page
    pub page_size: u32,
    /// Total number of pages
    pub total_pages: u32,
}

impl From<PageResponse<UserDto>> for PagedUserDto {
    fn from(page: PageResponse<UserDto>) -> Self {
        Self {
            items: page.items,
            total: page.total,
            page: page.page,
            page_size: page.page_size,
            total_pages: page.total_pages,
        }
    }
}
