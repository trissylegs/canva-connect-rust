use canva_connect::endpoints::user::{Capability, TeamUserSummary, UserProfile};

#[test]
fn test_team_user_summary_creation() {
    let user = TeamUserSummary {
        user_id: "auDAbliZ2rQNNOsUl5OLu".to_string(),
        team_id: "Oi2RJILTrKk0KRhRUZozX".to_string(),
    };

    assert_eq!(user.user_id, "auDAbliZ2rQNNOsUl5OLu");
    assert_eq!(user.team_id, "Oi2RJILTrKk0KRhRUZozX");
}

#[test]
fn test_user_profile_creation() {
    let profile = UserProfile {
        display_name: "Jane Doe".to_string(),
    };

    assert_eq!(profile.display_name, "Jane Doe");
}

#[test]
fn test_capability_serialization() {
    let autofill = Capability::Autofill;
    let brand_template = Capability::BrandTemplate;
    let resize = Capability::Resize;

    assert_eq!(serde_json::to_string(&autofill).unwrap(), "\"autofill\"");
    assert_eq!(
        serde_json::to_string(&brand_template).unwrap(),
        "\"brand_template\""
    );
    assert_eq!(serde_json::to_string(&resize).unwrap(), "\"resize\"");
}

#[test]
fn test_capability_deserialization() {
    let autofill: Capability = serde_json::from_str("\"autofill\"").unwrap();
    let brand_template: Capability = serde_json::from_str("\"brand_template\"").unwrap();
    let resize: Capability = serde_json::from_str("\"resize\"").unwrap();

    assert!(matches!(autofill, Capability::Autofill));
    assert!(matches!(brand_template, Capability::BrandTemplate));
    assert!(matches!(resize, Capability::Resize));
}

#[test]
fn test_capability_display() {
    assert_eq!(format!("{}", Capability::Autofill), "autofill");
    assert_eq!(format!("{}", Capability::BrandTemplate), "brand_template");
    assert_eq!(format!("{}", Capability::Resize), "resize");
}

#[test]
fn test_capability_debug() {
    let capability = Capability::Autofill;
    let debug_str = format!("{capability:?}");
    assert!(debug_str.contains("Autofill"));
}

#[test]
fn test_team_user_summary_serialization() {
    let user = TeamUserSummary {
        user_id: "test_user".to_string(),
        team_id: "test_team".to_string(),
    };

    let json = serde_json::to_string(&user).unwrap();
    let parsed: TeamUserSummary = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed.user_id, user.user_id);
    assert_eq!(parsed.team_id, user.team_id);
}

#[test]
fn test_user_profile_serialization() {
    let profile = UserProfile {
        display_name: "Test User".to_string(),
    };

    let json = serde_json::to_string(&profile).unwrap();
    let parsed: UserProfile = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed.display_name, profile.display_name);
}

#[test]
fn test_capabilities_list() {
    let capabilities = vec![
        Capability::Autofill,
        Capability::BrandTemplate,
        Capability::Resize,
    ];

    let json = serde_json::to_string(&capabilities).unwrap();
    let parsed: Vec<Capability> = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed.len(), 3);
    assert!(matches!(parsed[0], Capability::Autofill));
    assert!(matches!(parsed[1], Capability::BrandTemplate));
    assert!(matches!(parsed[2], Capability::Resize));
}
