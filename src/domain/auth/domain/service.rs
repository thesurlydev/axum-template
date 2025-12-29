//! This module defines the authentication service trait used to abstract
//! user login and registration logic.

use std::future::Future;

use crate::{
    common::{
        error::AppError,
        jwt::{AuthBody, AuthPayload},
    },
    domain::auth::AuthUserDto,
};

/// Trait defining the contract for authentication-related operations.
/// Implementors are responsible for handling user creation and login logic.
pub trait AuthServiceTrait: Send + Sync {
    /// Registers a new user authentication entry.
    fn create_user_auth(
        &self,
        auth_user: AuthUserDto,
    ) -> impl Future<Output = Result<(), AppError>> + Send;

    /// Authenticates a user and returns a JWT token payload on success.
    fn login_user(
        &self,
        auth_payload: AuthPayload,
    ) -> impl Future<Output = Result<AuthBody, AppError>> + Send;
}
