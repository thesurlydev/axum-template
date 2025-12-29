use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

/// Default page size when not specified
pub const DEFAULT_PAGE_SIZE: u32 = 20;

/// Maximum allowed page size
pub const MAX_PAGE_SIZE: u32 = 100;

/// Pagination request parameters for list endpoints.
#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct PageRequest {
    /// Page number (1-indexed). Defaults to 1.
    #[param(minimum = 1, default = 1)]
    #[serde(default = "default_page")]
    pub page: u32,

    /// Number of items per page. Defaults to 20, max 100.
    #[param(minimum = 1, maximum = 100, default = 20)]
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    DEFAULT_PAGE_SIZE
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

impl PageRequest {
    /// Returns the page size, clamped to MAX_PAGE_SIZE.
    pub fn page_size(&self) -> u32 {
        self.page_size.min(MAX_PAGE_SIZE).max(1)
    }

    /// Returns the page number, ensuring it's at least 1.
    pub fn page(&self) -> u32 {
        self.page.max(1)
    }

    /// Calculates the offset for SQL queries.
    pub fn offset(&self) -> i64 {
        ((self.page().saturating_sub(1)) * self.page_size()) as i64
    }

    /// Calculates the limit for SQL queries.
    pub fn limit(&self) -> i64 {
        self.page_size() as i64
    }
}

/// Paginated response wrapper containing items and pagination metadata.
#[derive(Debug, Clone, Serialize)]
pub struct PageResponse<T> {
    /// The items on the current page
    pub items: Vec<T>,
    /// Total number of items across all pages
    pub total: u64,
    /// Current page number (1-indexed)
    pub page: u32,
    /// Number of items per page
    pub page_size: u32,
    /// Total number of pages
    pub total_pages: u32,
}

impl<T> PageResponse<T> {
    /// Creates a new paginated response.
    pub fn new(items: Vec<T>, total: u64, page_request: &PageRequest) -> Self {
        let page_size = page_request.page_size();
        let total_pages = ((total as f64) / (page_size as f64)).ceil() as u32;

        Self {
            items,
            total,
            page: page_request.page(),
            page_size,
            total_pages,
        }
    }
}
