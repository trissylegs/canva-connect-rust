use canva_connect::error::*;
use std::error::Error as StdError;

#[test]
fn test_error_display() {
    let auth_error = Error::Auth("Invalid token".to_string());
    assert_eq!(
        auth_error.to_string(),
        "Authentication error: Invalid token"
    );

    let generic_error = Error::Generic("Something went wrong".to_string());
    assert_eq!(generic_error.to_string(), "Something went wrong");

    let rate_limit_error = Error::RateLimit;
    assert_eq!(rate_limit_error.to_string(), "Rate limit exceeded");
}

#[test]
fn test_api_error_code_display() {
    assert_eq!(format!("{}", ApiErrorCode::NotFound), "NOT_FOUND");
    assert_eq!(format!("{}", ApiErrorCode::Unauthorized), "UNAUTHORIZED");
    assert_eq!(format!("{}", ApiErrorCode::Forbidden), "FORBIDDEN");
    assert_eq!(
        format!("{}", ApiErrorCode::TooManyRequests),
        "TOO_MANY_REQUESTS"
    );
    assert_eq!(
        format!("{}", ApiErrorCode::InternalServerError),
        "INTERNAL_SERVER_ERROR"
    );
    assert_eq!(
        format!("{}", ApiErrorCode::Unknown("CUSTOM".to_string())),
        "CUSTOM"
    );
}

#[test]
fn test_api_error_code_from_string() {
    assert_eq!(
        ApiErrorCode::from("NOT_FOUND".to_string()),
        ApiErrorCode::NotFound
    );
    assert_eq!(
        ApiErrorCode::from("UNAUTHORIZED".to_string()),
        ApiErrorCode::Unauthorized
    );
    assert_eq!(
        ApiErrorCode::from("FORBIDDEN".to_string()),
        ApiErrorCode::Forbidden
    );
    assert_eq!(
        ApiErrorCode::from("TOO_MANY_REQUESTS".to_string()),
        ApiErrorCode::TooManyRequests
    );
    assert_eq!(
        ApiErrorCode::from("INTERNAL_SERVER_ERROR".to_string()),
        ApiErrorCode::InternalServerError
    );
    assert_eq!(
        ApiErrorCode::from("UNKNOWN_CODE".to_string()),
        ApiErrorCode::Unknown("UNKNOWN_CODE".to_string())
    );
}

#[test]
fn test_api_error_code_equality() {
    assert_eq!(ApiErrorCode::NotFound, ApiErrorCode::NotFound);
    assert_ne!(ApiErrorCode::NotFound, ApiErrorCode::Unauthorized);

    let unknown1 = ApiErrorCode::Unknown("CUSTOM".to_string());
    let unknown2 = ApiErrorCode::Unknown("CUSTOM".to_string());
    let unknown3 = ApiErrorCode::Unknown("OTHER".to_string());

    assert_eq!(unknown1, unknown2);
    assert_ne!(unknown1, unknown3);
}

#[test]
fn test_api_error_creation() {
    let error = Error::Api {
        code: ApiErrorCode::NotFound,
        message: "Resource not found".to_string(),
    };

    let error_str = error.to_string();
    assert!(error_str.contains("NOT_FOUND"));
    assert!(error_str.contains("Resource not found"));
}

#[test]
fn test_api_error_from_api_error_struct() {
    let api_error = ApiError {
        code: "UNAUTHORIZED".to_string(),
        message: "Invalid credentials".to_string(),
    };

    let error = Error::from(api_error);
    match error {
        Error::Api { code, message } => {
            assert_eq!(code, ApiErrorCode::Unauthorized);
            assert_eq!(message, "Invalid credentials");
        }
        _ => panic!("Expected API error"),
    }
}

#[test]
fn test_error_debug() {
    let error = Error::Auth("Test auth error".to_string());
    let debug_str = format!("{error:?}");
    assert!(debug_str.contains("Auth"));
    assert!(debug_str.contains("Test auth error"));
}

#[test]
fn test_error_source() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let error = Error::Io(io_error);

    // Test that we can access the source
    assert!(error.source().is_some());
}

#[test]
fn test_json_error_conversion() {
    let invalid_json = r#"{"incomplete": json"#;
    let parse_result: std::result::Result<serde_json::Value, serde_json::Error> =
        serde_json::from_str(invalid_json);

    match parse_result {
        Err(json_error) => {
            let error = Error::from(json_error);
            match error {
                Error::Json(_) => {} // Expected
                _ => panic!("Expected JSON error"),
            }
        }
        Ok(_) => panic!("Expected JSON parsing to fail"),
    }
}

#[test]
fn test_error_variants() {
    let auth_error = Error::Auth("auth error".to_string());
    let generic_error = Error::Generic("generic error".to_string());
    let rate_limit_error = Error::RateLimit;

    // Test pattern matching
    match auth_error {
        Error::Auth(msg) => assert_eq!(msg, "auth error"),
        _ => panic!("Expected auth error"),
    }

    match generic_error {
        Error::Generic(msg) => assert_eq!(msg, "generic error"),
        _ => panic!("Expected generic error"),
    }

    match rate_limit_error {
        Error::RateLimit => {} // Expected
        _ => panic!("Expected rate limit error"),
    }
}

#[test]
fn test_api_error_deserialization() {
    let json = r#"{"code": "NOT_FOUND", "message": "Resource not found"}"#;
    let api_error: ApiError = serde_json::from_str(json).unwrap();

    assert_eq!(api_error.code, "NOT_FOUND");
    assert_eq!(api_error.message, "Resource not found");
}

#[test]
fn test_result_type_alias() {
    fn returns_result() -> Result<String> {
        Ok("success".to_string())
    }

    let result = returns_result();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}
