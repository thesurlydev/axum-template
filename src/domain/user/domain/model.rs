use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// A strongly-typed user identifier.
///
/// Using a newtype instead of raw String provides:
/// - Type safety: prevents accidentally mixing user IDs with other string IDs
/// - Self-documenting code: function signatures clearly show they expect a user ID
/// - Centralized validation: ID format validation can be added here if needed
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(String);

impl UserId {
    /// Creates a new UserId from a string.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the inner string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the UserId and returns the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for UserId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for UserId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for UserId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Domain model representing a user in the application.
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: Option<String>,
    pub created_by: Option<UserId>,
    pub created_at: Option<DateTime<Utc>>,
    pub modified_by: Option<UserId>,
    pub modified_at: Option<DateTime<Utc>>,
}
