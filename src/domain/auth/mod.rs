mod api {
    mod handlers;
    pub mod routes;
}

mod domain {
    pub mod model;
    pub mod repository;
    pub mod service;
}

mod dto {
    pub mod auth_dto;
}

mod infra {
    pub mod postgres_repository;
    pub mod postgres_service;
}

// Re-export commonly used items for convenience
pub use api::routes::{user_auth_routes, UserAuthApiDoc};
pub use domain::model::UserAuth;
pub use domain::repository::UserAuthRepository;
pub use domain::service::AuthServiceTrait;
pub use dto::auth_dto::AuthUserDto;
pub use infra::postgres_service::PostgresAuthService as AuthService;
