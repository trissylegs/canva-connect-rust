use canva_connect::rate_limit::*;

#[test]
fn test_api_rate_limiter_creation() {
    let rate_limiter = ApiRateLimiter::default();
    
    // Should create successfully
    assert!(format!("{:?}", rate_limiter).contains("ApiRateLimiter"));
}

#[test]
fn test_api_rate_limiter_debug() {
    let limiter1 = ApiRateLimiter::default();
    
    // Should be able to debug print
    assert!(format!("{:?}", limiter1).contains("ApiRateLimiter"));
}

#[test]
fn test_rate_limiter_basic_operation() {
    let rate_limiter = ApiRateLimiter::default();
    
    // Should be able to create rate limiter
    // This test mainly verifies the API is accessible
    assert!(format!("{:?}", rate_limiter).contains("ApiRateLimiter"));
}

#[test]
fn test_rate_limit_info_creation() {
    let info = RateLimitInfo {
        remaining: Some(100),
        reset_at: Some(chrono::Utc::now()),
        limit: Some(1000),
    };
    
    assert_eq!(info.remaining, Some(100));
    assert_eq!(info.limit, Some(1000));
    assert!(info.reset_at.is_some());
}

#[test]
fn test_rate_limit_info_with_zero_remaining() {
    let future_time = chrono::Utc::now() + chrono::Duration::minutes(1);
    let info = RateLimitInfo {
        remaining: Some(0),
        reset_at: Some(future_time),
        limit: Some(100),
    };
    
    assert_eq!(info.remaining, Some(0));
    assert!(info.reset_at.unwrap() > chrono::Utc::now());
}

#[test]
fn test_rate_limit_info_debug() {
    let info = RateLimitInfo {
        remaining: Some(50),
        reset_at: Some(chrono::Utc::now()),
        limit: Some(100),
    };
    
    let debug_str = format!("{:?}", info);
    assert!(debug_str.contains("remaining"));
}

#[test]
fn test_rate_limit_info_clone() {
    let now = chrono::Utc::now();
    let info1 = RateLimitInfo {
        remaining: Some(25),
        reset_at: Some(now),
        limit: Some(100),
    };
    
    let info2 = info1.clone();
    assert_eq!(info1.remaining, info2.remaining);
    assert_eq!(info1.reset_at, info2.reset_at);
    assert_eq!(info1.limit, info2.limit);
}

#[test]
fn test_rate_limit_info_with_past_reset_time() {
    let past_time = chrono::Utc::now() - chrono::Duration::minutes(1);
    let info = RateLimitInfo {
        remaining: Some(75),
        reset_at: Some(past_time),
        limit: Some(100),
    };
    
    assert_eq!(info.remaining, Some(75));
    assert!(info.reset_at.unwrap() < chrono::Utc::now());
}
