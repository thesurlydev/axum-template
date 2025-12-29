use std::sync::Arc;

use crate::domain::{auth::AuthService, user::UserServiceImpl};

use super::config::Config;

/// AppState is a struct that holds the application-wide shared state.
/// It is passed to request handlers via Axum's extension mechanism.
#[derive(Clone)]
pub struct AppState {
    /// Global application configuration.
    pub config: Config,
    /// Service handling authentication-related logic.
    pub auth_service: Arc<AuthService>,
    /// Service handling user-related logic.
    pub user_service: Arc<UserServiceImpl>,
}

impl AppState {
    /// Creates a new instance of AppState with the provided dependencies.
    pub fn new(
        config: Config,
        auth_service: Arc<AuthService>,
        user_service: Arc<UserServiceImpl>,
    ) -> Self {
        Self {
            config,
            auth_service,
            user_service,
        }
    }
}
