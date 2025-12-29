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
    pub mod user_dto;
}

mod infra {
    pub mod postgres_repository;
    pub mod postgres_service;
}

// Re-export commonly used items for convenience
pub use api::routes::{user_routes, UserApiDoc};
pub use domain::model::{User, UserId};
pub use domain::repository::UserRepository;
pub use domain::service::UserServiceTrait;
pub use dto::user_dto::{CreateUserDto, PagedUserDto, SearchUserDto, UpdateUserDto, UserDto};
pub use infra::postgres_service::UserService as UserServiceImpl;
