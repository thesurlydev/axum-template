use crate::{
    common::{error::AppError, pagination::PageRequest},
    domain::user::{
        domain::{
            model::{User, UserId},
            repository::UserRepository,
            service::UserServiceTrait,
        },
        dto::user_dto::{CreateUserDto, SearchUserDto, UpdateUserDto},
        infra::postgres_repository::UserRepo,
    },
};
use sqlx::PgPool;
use std::sync::Arc;

/// Service struct for handling user-related operations
/// such as creating, updating, deleting, and fetching users.
/// It uses a repository pattern to abstract the data access layer.
#[derive(Clone)]
pub struct UserService {
    pub pool: PgPool,
    pub repo: UserRepo,
}

impl UserService {
    /// constructor for the service.
    pub fn new(pool: PgPool) -> Arc<Self> {
        Arc::new(Self {
            pool,
            repo: UserRepo,
        })
    }
}

impl UserServiceTrait for UserService {
    /// Retrieves a user by their ID.
    async fn get_user_by_id(&self, id: &UserId) -> Result<User, AppError> {
        self.repo
            .find_by_id(&self.pool, id)
            .await
            .inspect_err(|e| tracing::error!("Error retrieving user: {e}"))?
            .ok_or_else(|| AppError::NotFound("User not found".into()))
    }

    /// Retrieves users with optional filters and pagination.
    async fn get_user_list(
        &self,
        search_user_dto: SearchUserDto,
        page_request: &PageRequest,
    ) -> Result<(Vec<User>, u64), AppError> {
        self.repo
            .find_list(&self.pool, search_user_dto, page_request)
            .await
            .inspect_err(|e| tracing::error!("Error fetching users: {e}"))
            .map_err(AppError::from)
    }

    /// Creates a new user.
    async fn create_user(&self, create_user: CreateUserDto) -> Result<User, AppError> {
        let mut tx = self.pool.begin().await?;

        let user_id = self
            .repo
            .create(&mut tx, create_user)
            .await
            .inspect_err(|e| tracing::error!("Error creating user: {e}"))?;

        tx.commit().await?;

        self.repo
            .find_by_id(&self.pool, &user_id)
            .await
            .inspect_err(|e| tracing::error!("Error retrieving user: {e}"))?
            .ok_or_else(|| AppError::NotFound("User not found".into()))
    }

    /// Updates an existing user.
    async fn update_user(&self, id: &UserId, payload: UpdateUserDto) -> Result<User, AppError> {
        let mut tx = self.pool.begin().await?;

        let user = self
            .repo
            .update(&mut tx, id, payload)
            .await
            .inspect_err(|e| tracing::error!("Error updating user: {e}"))?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        tx.commit().await?;
        Ok(user)
    }

    /// Deletes a user by their ID.
    async fn delete_user(&self, id: &UserId) -> Result<String, AppError> {
        let mut tx = self.pool.begin().await?;

        let deleted = self
            .repo
            .delete(&mut tx, id)
            .await
            .inspect_err(|e| tracing::error!("Error deleting user: {e}"))?;

        if !deleted {
            return Err(AppError::NotFound("User not found".into()));
        }

        tx.commit().await?;
        Ok("User deleted".into())
    }
}
