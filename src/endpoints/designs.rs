//! Designs API endpoint implementation
//!
//! The Designs API allows you to:
//!
//! | Method | HTTP | Endpoint | OAuth Scope | Description |
//! |--------|------|----------|-------------|-------------|
//! | [`list`](DesignsApi::list) | `GET` | `/v1/designs` | `design:meta:read` | List user's designs |
//! | [`get`](DesignsApi::get) | `GET` | `/v1/designs/{designId}` | `design:meta:read` | Get design metadata |
//! | [`create`](DesignsApi::create) | `POST` | `/v1/designs` | `design:content:write` | Create new design |
//!
//! ## Usage
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken};
//! use canva_connect::models::{CreateDesignRequest, DesignTypeInput, PresetDesignTypeInput, PresetDesignTypeName};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new(AccessToken::new("your-access-token"));
//! let designs_api = client.designs();
//!
//! // List designs
//! let designs = designs_api.list(None, None, None, None).await?;
//! println!("Found {} designs", designs.items.len());
//!
//! // Create a new presentation
//! let create_request = CreateDesignRequest {
//!     design_type: Some(DesignTypeInput::Preset(PresetDesignTypeInput {
//!         design_type: PresetDesignTypeName::Presentation,
//!     })),
//!     title: Some("My Presentation".to_string()),
//!     asset_id: None,
//! };
//! let new_design = designs_api.create(create_request).await?;
//! println!("Created design: {}", new_design.design.id);
//!
//! // Get design details
//! let design = designs_api.get(&new_design.design.id).await?;
//! println!("Design title: {:?}", design.design.title);
//! # Ok(())
//! # }
//! ```

use crate::{
    client::Client,
    models::{
        CreateDesignRequest, CreateDesignResponse, GetDesignResponse, GetListDesignResponse,
        OwnershipType, SortByType,
    },
    Result,
};

/// Designs API client
#[derive(Debug, Clone)]
pub struct DesignsApi {
    client: Client,
}

impl DesignsApi {
    /// Create a new Designs API client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// List designs in the user's projects
    ///
    /// **Required OAuth scope:** `design:meta:read`
    ///
    /// # Parameters
    ///
    /// - `query`: Search term to filter designs
    /// - `continuation`: Continuation token for pagination
    /// - `ownership`: Filter by ownership (owned, shared, or any)
    /// - `sort_by`: Sort order for results
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use canva_connect::{Client, auth::AccessToken};
    /// use canva_connect::models::{OwnershipType, SortByType};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new(AccessToken::new("token"));
    /// let designs = client.designs();
    ///
    /// // List all designs
    /// let all_designs = designs.list(None, None, None, None).await?;
    ///
    /// // Search for designs
    /// let search_results = designs.list(
    ///     Some("presentation".to_string()),
    ///     None,
    ///     Some(OwnershipType::Owned),
    ///     Some(SortByType::ModifiedDescending)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        query: Option<String>,
        continuation: Option<String>,
        ownership: Option<OwnershipType>,
        sort_by: Option<SortByType>,
    ) -> Result<GetListDesignResponse> {
        let mut params = Vec::new();

        if let Some(q) = query {
            params.push(("query", q));
        }
        if let Some(cont) = continuation {
            params.push(("continuation", cont));
        }
        if let Some(own) = ownership {
            let ownership_str = match own {
                OwnershipType::Any => "any",
                OwnershipType::Owned => "owned",
                OwnershipType::Shared => "shared",
            };
            params.push(("ownership", ownership_str.to_string()));
        }
        if let Some(sort) = sort_by {
            let sort_str = match sort {
                SortByType::Relevance => "relevance",
                SortByType::ModifiedDescending => "modified_descending",
                SortByType::ModifiedAscending => "modified_ascending",
                SortByType::TitleDescending => "title_descending",
                SortByType::TitleAscending => "title_ascending",
            };
            params.push(("sort_by", sort_str.to_string()));
        }

        let path = if params.is_empty() {
            "/v1/designs".to_string()
        } else {
            let query_string = params
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            format!("/v1/designs?{query_string}")
        };

        self.client.get_json(&path).await
    }

    /// Get design metadata by ID
    ///
    /// **Required OAuth scope:** `design:meta:read`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use canva_connect::{Client, auth::AccessToken};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new(AccessToken::new("token"));
    /// let designs = client.designs();
    ///
    /// let design = designs.get("DAFVztcvd9z").await?;
    /// println!("Design: {:?}", design.design.title);
    /// println!("Edit URL: {}", design.design.urls.edit_url);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, design_id: &str) -> Result<GetDesignResponse> {
        let path = format!("/v1/designs/{}", urlencoding::encode(design_id));
        self.client.get_json(&path).await
    }

    /// Create a new design
    ///
    /// **Required OAuth scope:** `design:content:write`
    ///
    /// You can create a design using either:
    /// - A preset design type (e.g., presentation, Instagram post)
    /// - Custom dimensions (width and height in pixels)
    /// - An existing asset ID to insert into the design
    ///
    /// **Note:** Blank designs created with this API are automatically deleted
    /// if they're not edited within 7 days.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use canva_connect::{Client, auth::AccessToken};
    /// use canva_connect::models::{
    ///     CreateDesignRequest, DesignTypeInput, PresetDesignTypeInput,
    ///     PresetDesignTypeName, CustomDesignTypeInput
    /// };
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new(AccessToken::new("token"));
    /// let designs = client.designs();
    ///
    /// // Create a presentation
    /// let request = CreateDesignRequest {
    ///     design_type: Some(DesignTypeInput::Preset(PresetDesignTypeInput {
    ///         design_type: PresetDesignTypeName::Presentation,
    ///     })),
    ///     title: Some("My Presentation".to_string()),
    ///     asset_id: None,
    /// };
    /// let design = designs.create(request).await?;
    ///
    /// // Create a custom-sized design
    /// let custom_request = CreateDesignRequest {
    ///     design_type: Some(DesignTypeInput::Custom(CustomDesignTypeInput {
    ///         width: 800,
    ///         height: 600,
    ///     })),
    ///     title: Some("Custom Design".to_string()),
    ///     asset_id: None,
    /// };
    /// let custom_design = designs.create(custom_request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, request: CreateDesignRequest) -> Result<CreateDesignResponse> {
        self.client.post_json("/v1/designs", &request).await
    }
}
