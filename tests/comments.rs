use canva_connect::{auth::AccessToken, endpoints::comments::*, Client};

#[test]
fn test_comments_api_creation() {
    let access_token = AccessToken::new("test_token".to_string());
    #[allow(clippy::expect_used)]
    let client = Client::new(access_token).expect("Failed to create client");

    let _comments_api = client.comments();
}

#[test]
fn test_create_thread_request_creation() {
    let request = CreateThreadRequest {
        message_plaintext: "This is a test comment".to_string(),
        assignee_id: None,
    };

    assert_eq!(request.message_plaintext, "This is a test comment");
    assert!(request.assignee_id.is_none());
}

#[test]
fn test_create_thread_request_with_assignee() {
    let request = CreateThreadRequest {
        message_plaintext: "Assigned comment".to_string(),
        assignee_id: Some("user_123".to_string()),
    };

    assert_eq!(request.message_plaintext, "Assigned comment");
    assert_eq!(request.assignee_id, Some("user_123".to_string()));
}

#[test]
fn test_create_reply_request_creation() {
    let request = CreateReplyRequest {
        message_plaintext: "This is a reply".to_string(),
    };

    assert_eq!(request.message_plaintext, "This is a reply");
}

#[test]
fn test_comment_object_input_design() {
    let object = CommentObjectInput::Design {
        design_id: "design_123".to_string(),
    };

    match object {
        CommentObjectInput::Design { design_id } => {
            assert_eq!(design_id, "design_123");
        }
    }
}

#[test]
fn test_comment_object_input_serialization() {
    let object = CommentObjectInput::Design {
        design_id: "design_456".to_string(),
    };

    let serialized = serde_json::to_string(&object).expect("Failed to serialize");
    assert!(serialized.contains("\"type\":\"design\""));
    assert!(serialized.contains("\"design_id\":\"design_456\""));
}

#[test]
fn test_comment_object_input_deserialization() {
    let json = r#"{"type":"design","design_id":"design_789"}"#;
    let object: CommentObjectInput = serde_json::from_str(json).expect("Failed to deserialize");

    match object {
        CommentObjectInput::Design { design_id } => {
            assert_eq!(design_id, "design_789");
        }
    }
}

#[test]
fn test_create_thread_request_serialization() {
    let request = CreateThreadRequest {
        message_plaintext: "Test comment".to_string(),
        assignee_id: Some("user_456".to_string()),
    };

    let serialized = serde_json::to_string(&request).expect("Failed to serialize");
    assert!(serialized.contains("\"message_plaintext\":\"Test comment\""));
    assert!(serialized.contains("\"assignee_id\":\"user_456\""));
}

#[test]
fn test_create_thread_request_serialization_no_assignee() {
    let request = CreateThreadRequest {
        message_plaintext: "Test comment without assignee".to_string(),
        assignee_id: None,
    };

    let serialized = serde_json::to_string(&request).expect("Failed to serialize");
    assert!(serialized.contains("\"message_plaintext\":\"Test comment without assignee\""));
    // Should not include assignee_id field when None (skip_serializing_if)
    assert!(!serialized.contains("assignee_id"));
}

#[test]
fn test_create_reply_request_serialization() {
    let request = CreateReplyRequest {
        message_plaintext: "This is a reply message".to_string(),
    };

    let serialized = serde_json::to_string(&request).expect("Failed to serialize");
    assert!(serialized.contains("\"message_plaintext\":\"This is a reply message\""));
}

#[test]
fn test_comments_api_debug() {
    let access_token = AccessToken::new("debug_token".to_string());
    #[allow(clippy::expect_used)]
    let client = Client::new(access_token).expect("Failed to create client");
    let comments_api = client.comments();

    let debug_str = format!("{comments_api:?}");
    assert!(debug_str.contains("CommentsApi"));
}

#[test]
fn test_comments_api_clone() {
    let access_token = AccessToken::new("clone_token".to_string());
    #[allow(clippy::expect_used)]
    let client = Client::new(access_token).expect("Failed to create client");
    let comments_api = client.comments();

    let cloned_api = comments_api.clone();

    // Both should be separate instances but functionally equivalent
    let original_debug = format!("{comments_api:?}");
    let cloned_debug = format!("{cloned_api:?}");
    assert_eq!(original_debug, cloned_debug);
}

#[test]
fn test_create_thread_request_with_empty_message() {
    let request = CreateThreadRequest {
        message_plaintext: "".to_string(),
        assignee_id: None,
    };

    assert!(request.message_plaintext.is_empty());

    // Should still serialize properly
    let serialized = serde_json::to_string(&request).expect("Failed to serialize");
    assert!(serialized.contains("\"message_plaintext\":\"\""));
}

#[test]
fn test_create_reply_request_with_empty_message() {
    let request = CreateReplyRequest {
        message_plaintext: "".to_string(),
    };

    assert!(request.message_plaintext.is_empty());

    // Should still serialize properly
    let serialized = serde_json::to_string(&request).expect("Failed to serialize");
    assert!(serialized.contains("\"message_plaintext\":\"\""));
}

#[test]
fn test_create_thread_request_with_long_message() {
    let long_message = "a".repeat(1000);
    let request = CreateThreadRequest {
        message_plaintext: long_message.clone(),
        assignee_id: None,
    };

    assert_eq!(request.message_plaintext.len(), 1000);
    assert_eq!(request.message_plaintext, long_message);
}

#[test]
fn test_create_reply_request_with_special_characters() {
    let message_with_special_chars = "Test with special chars: àáâãäåæçèéêë 🎨🎭🎪";
    let request = CreateReplyRequest {
        message_plaintext: message_with_special_chars.to_string(),
    };

    assert_eq!(request.message_plaintext, message_with_special_chars);

    // Should serialize properly with special characters
    let serialized = serde_json::to_string(&request).expect("Failed to serialize");
    assert!(serialized.contains("🎨🎭🎪"));
}

#[test]
fn test_create_thread_request_debug_format() {
    let request = CreateThreadRequest {
        message_plaintext: "Debug test".to_string(),
        assignee_id: Some("debug_user".to_string()),
    };

    let debug_str = format!("{request:?}");
    assert!(debug_str.contains("CreateThreadRequest"));
    assert!(debug_str.contains("Debug test"));
    assert!(debug_str.contains("debug_user"));
}

#[test]
fn test_create_reply_request_debug_format() {
    let request = CreateReplyRequest {
        message_plaintext: "Reply debug test".to_string(),
    };

    let debug_str = format!("{request:?}");
    assert!(debug_str.contains("CreateReplyRequest"));
    assert!(debug_str.contains("Reply debug test"));
}
