//! This module defines the `UserRepository` trait, which abstracts
//! the database operations related to user entities.

use std::future::Future;

use crate::{
    common::pagination::PageRequest,
    domain::user::{CreateUserDto, SearchUserDto, UpdateUserDto},
};

use super::model::{User, UserId};

use sqlx::{PgPool, Postgres, Transaction};

/// Trait representing repository-level operations for user entities.
/// Provides methods for creating, retrieving, updating, and deleting users in the database.
pub trait UserRepository: Send + Sync {
    /// Finds a user by their unique identifier.
    fn find_by_id(
        &self,
        pool: &PgPool,
        id: &UserId,
    ) -> impl Future<Output = Result<Option<User>, sqlx::Error>> + Send;

    /// Finds user list by condition with pagination.
    /// Returns a tuple of (users, total_count).
    fn find_list(
        &self,
        pool: &PgPool,
        search_user_dto: SearchUserDto,
        page_request: &PageRequest,
    ) -> impl Future<Output = Result<(Vec<User>, u64), sqlx::Error>> + Send;

    /// Creates a new user record using the provided data within an active transaction.
    fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user: CreateUserDto,
    ) -> impl Future<Output = Result<UserId, sqlx::Error>> + Send;

    /// Updates an existing user record using the provided data.
    fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: &UserId,
        user: UpdateUserDto,
    ) -> impl Future<Output = Result<Option<User>, sqlx::Error>> + Send;

    /// Deletes a user by their unique identifier within an active transaction.
    fn delete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: &UserId,
    ) -> impl Future<Output = Result<bool, sqlx::Error>> + Send;
}
