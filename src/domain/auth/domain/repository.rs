//! This module defines the `UserAuthRepository` trait, which provides an abstraction
//! over database operations related to user authentication records.

use std::future::Future;

use super::model::UserAuth;

use sqlx::{PgPool, Postgres, Transaction};

/// Trait representing the repository contract for user authentication data.
/// Enables decoupling of business logic from direct database interaction.
pub trait UserAuthRepository: Send + Sync {
    /// Finds a user authentication record by the user's username.
    /// Returns `Ok(Some(UserAuth))` if found, or `Ok(None)` if not found.
    fn find_by_user_name(
        &self,
        pool: PgPool,
        user_name: String,
    ) -> impl Future<Output = Result<Option<UserAuth>, sqlx::Error>> + Send;

    /// Inserts a new user authentication record into the database using a transaction.
    fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_auth: UserAuth,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send;
}
