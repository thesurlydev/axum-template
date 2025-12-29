//! This module defines the `UserServiceTrait` responsible for user-related business logic.
//! It abstracts operations such as user creation, retrieval, update, and deletion.

use std::future::Future;

use crate::{
    common::{error::AppError, pagination::PageRequest},
    domain::user::{CreateUserDto, SearchUserDto, UpdateUserDto, User, UserId},
};

/// Trait defining business operations for user management.
/// Provides methods for interacting with users in a domain-agnostic way.
/// Returns domain User objects - handlers are responsible for converting to DTOs.
pub trait UserServiceTrait: Send + Sync {
    /// Retrieves a user by their unique identifier.
    fn get_user_by_id(&self, id: &UserId) -> impl Future<Output = Result<User, AppError>> + Send;

    /// Retrieves users with optional filters and pagination.
    /// Returns a tuple of (users, total_count).
    fn get_user_list(
        &self,
        search_user_dto: SearchUserDto,
        page_request: &PageRequest,
    ) -> impl Future<Output = Result<(Vec<User>, u64), AppError>> + Send;

    /// Creates a new user.
    fn create_user(
        &self,
        create_user: CreateUserDto,
    ) -> impl Future<Output = Result<User, AppError>> + Send;

    /// Updates an existing user with the given payload.
    fn update_user(
        &self,
        id: &UserId,
        payload: UpdateUserDto,
    ) -> impl Future<Output = Result<User, AppError>> + Send;

    /// Deletes a user by their unique identifier.
    fn delete_user(&self, id: &UserId) -> impl Future<Output = Result<String, AppError>> + Send;
}
