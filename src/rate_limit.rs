//! Rate limiting utilities for the Canva Connect API

use governor::{Quota, RateLimiter};
use nonzero_ext::nonzero;
use std::num::NonZeroU32;
use std::time::Duration;

/// Rate limiter for API requests
#[derive(Debug)]
pub struct ApiRateLimiter {
    limiter: RateLimiter<governor::state::direct::NotKeyed, governor::state::InMemoryState, governor::clock::DefaultClock>,
}

impl ApiRateLimiter {
    /// Create a new rate limiter with the given rate limit per minute
    pub fn new(requests_per_minute: u32) -> Self {
        let quota = Quota::per_minute(NonZeroU32::new(requests_per_minute).unwrap_or(nonzero!(60u32)));
        let limiter = RateLimiter::direct(quota);
        
        Self { limiter }
    }

    /// Create a conservative rate limiter (30 requests per minute)
    pub fn conservative() -> Self {
        Self::new(30)
    }

    /// Create a permissive rate limiter (100 requests per minute)
    pub fn permissive() -> Self {
        Self::new(100)
    }

    /// Wait until a request can be made
    pub async fn wait_for_request(&self) {
        self.limiter.until_ready().await;
    }

    /// Check if a request can be made immediately
    pub fn can_make_request(&self) -> bool {
        self.limiter.check().is_ok()
    }
}

impl Default for ApiRateLimiter {
    fn default() -> Self {
        Self::conservative()
    }
}

/// Rate limit information from API response headers
#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    /// Number of requests remaining in the current window
    pub remaining: Option<u32>,
    /// Time when the rate limit window resets
    pub reset_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Total requests allowed in the current window
    pub limit: Option<u32>,
}

impl RateLimitInfo {
    /// Parse rate limit information from HTTP headers
    pub fn from_headers(headers: &reqwest::header::HeaderMap) -> Self {
        let remaining = headers
            .get("x-ratelimit-remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok());

        let reset_at = headers
            .get("x-ratelimit-reset")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<i64>().ok())
            .and_then(|timestamp| chrono::DateTime::from_timestamp(timestamp, 0));

        let limit = headers
            .get("x-ratelimit-limit")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok());

        Self {
            remaining,
            reset_at,
            limit,
        }
    }

    /// Check if the rate limit is close to being exceeded
    pub fn is_near_limit(&self) -> bool {
        match (self.remaining, self.limit) {
            (Some(remaining), Some(limit)) => {
                let usage_percentage = (limit - remaining) as f64 / limit as f64;
                usage_percentage > 0.8 // 80% usage
            }
            _ => false,
        }
    }

    /// Get the time until the rate limit resets
    pub fn time_until_reset(&self) -> Option<Duration> {
        self.reset_at.and_then(|reset_at| {
            let now = chrono::Utc::now();
            if reset_at > now {
                Some((reset_at - now).to_std().ok()?)
            } else {
                None
            }
        })
    }
}
