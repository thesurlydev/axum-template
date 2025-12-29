use crate::{
    common::pagination::PageRequest,
    domain::user::{
        domain::{
            model::{User, UserId},
            repository::UserRepository,
        },
        dto::user_dto::{CreateUserDto, SearchUserDto, UpdateUserDto},
    },
};

use sqlx::{PgPool, Postgres, QueryBuilder, Row, Transaction};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepo;

const FIND_USER_BY_ID_QUERY: &str = r#"
    SELECT id, username, email, created_by, created_at, modified_by, modified_at
    FROM users
    WHERE id = $1
    "#;

impl UserRepository for UserRepo {
    async fn find_list(
        &self,
        pool: &PgPool,
        search_user_dto: SearchUserDto,
        page_request: &PageRequest,
    ) -> Result<(Vec<User>, u64), sqlx::Error> {
        // Build WHERE clause for reuse
        let build_where_clause = |builder: &mut QueryBuilder<'_, Postgres>| {
            if let Some(s) = search_user_dto
                .id
                .as_deref()
                .filter(|s| !s.trim().is_empty())
            {
                builder.push(" AND id = ");
                builder.push_bind(s.to_string());
            }

            if let Some(s) = search_user_dto
                .username
                .as_deref()
                .filter(|s| !s.trim().is_empty())
            {
                builder.push(" AND username LIKE ");
                builder.push_bind(format!("%{s}%"));
            }
        };

        // Count query
        let mut count_builder =
            QueryBuilder::<Postgres>::new("SELECT COUNT(*) as count FROM users WHERE 1=1");
        build_where_clause(&mut count_builder);
        let count_row = count_builder.build().fetch_one(pool).await?;
        let total: i64 = count_row.get("count");

        // Data query with pagination
        let mut data_builder = QueryBuilder::<Postgres>::new(
            "SELECT id, username, email, created_by, created_at, modified_by, modified_at FROM users WHERE 1=1",
        );
        build_where_clause(&mut data_builder);
        data_builder.push(" ORDER BY created_at DESC LIMIT ");
        data_builder.push_bind(page_request.limit());
        data_builder.push(" OFFSET ");
        data_builder.push_bind(page_request.offset());

        let users = data_builder.build_query_as::<User>().fetch_all(pool).await?;

        Ok((users, total as u64))
    }

    async fn find_by_id(&self, pool: &PgPool, id: &UserId) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(FIND_USER_BY_ID_QUERY)
            .bind(id.as_str())
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user: CreateUserDto,
    ) -> Result<UserId, sqlx::Error> {
        let id = UserId::new(Uuid::new_v4().to_string());

        sqlx::query(
            r#"
                INSERT INTO users (id, username, email, created_by, modified_by)
                VALUES ($1, $2, $3, $4, $5)
                "#,
        )
        .bind(id.as_str())
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.modified_by)
        .bind(&user.modified_by)
        .execute(&mut **tx)
        .await?;

        Ok(id)
    }

    async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: &UserId,
        user: UpdateUserDto,
    ) -> Result<Option<User>, sqlx::Error> {
        let existing = sqlx::query_as::<_, User>(FIND_USER_BY_ID_QUERY)
            .bind(id.as_str())
            .fetch_optional(&mut **tx)
            .await?;

        if existing.is_some() {
            sqlx::query(
                r#"
                UPDATE users
                SET username = $1,
                    email = $2,
                    modified_by = $3,
                    modified_at = NOW()
                WHERE id = $4
                "#,
            )
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.modified_by)
            .bind(id.as_str())
            .execute(&mut **tx)
            .await?;

            let updated_user = sqlx::query_as::<_, User>(FIND_USER_BY_ID_QUERY)
                .bind(id.as_str())
                .fetch_one(&mut **tx)
                .await?;

            return Ok(Some(updated_user));
        }
        Ok(None)
    }

    async fn delete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: &UserId,
    ) -> Result<bool, sqlx::Error> {
        let res = sqlx::query(r#"DELETE FROM users WHERE id = $1"#)
            .bind(id.as_str())
            .execute(&mut **tx)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}
