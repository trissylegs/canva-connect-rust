//! OAuth scopes documentation for the Canva Connect API.
//!
//! This module documents all available OAuth scopes that can be requested
//! when authenticating with the Canva Connect API.

/// All available OAuth scopes for the Canva Connect API
///
/// These scopes determine what permissions your application has when accessing
/// the Canva Connect API on behalf of a user.
///
/// ## Asset Scopes
///
/// - **`asset:read`** - Read access to user's assets (images, videos, audio)
///   - Get asset metadata
///   - Check upload job status
///   - List assets within designs
///
/// - **`asset:write`** - Write access to user's assets
///   - Upload new assets
///   - Update asset properties (name, tags)
///   - Delete assets
///
/// ## Design Scopes
///
/// - **`design:meta:read`** - Read access to design metadata
///   - Get design information (title, thumbnail, etc.)
///   - List user's designs
///   - Access design properties
///
/// - **`design:content:read`** - Read access to design content
///   - Export designs to various formats
///   - Access design elements and structure
///   - Read design data for processing
///
/// - **`design:content:write`** - Write access to design content
///   - Create new designs
///   - Modify existing designs
///   - Import designs from external sources
///   - Use autofill functionality
///   - Resize designs
///
/// ## Brand Template Scopes
///
/// - **`brandtemplate:meta:read`** - Read access to brand template metadata
///   - Get brand template information
///   - List available brand templates
///   - Access template properties
///
/// - **`brandtemplate:content:read`** - Read access to brand template content
///   - Access template structure and elements
///   - Export brand templates
///   - Use templates as basis for designs
///
/// ## Folder Scopes
///
/// - **`folder:read`** - Read access to user's folders
///   - List folders and their contents
///   - Get folder metadata
///   - Navigate folder hierarchy
///
/// - **`folder:write`** - Write access to user's folders
///   - Create new folders
///   - Update folder properties
///   - Delete folders
///   - Move items between folders
///
/// ## Comment Scopes
///
/// - **`comment:read`** - Read access to comments
///   - List comments on designs
///   - Get comment content and metadata
///   - Access comment threads
///
/// - **`comment:write`** - Write access to comments
///   - Create new comments
///   - Reply to existing comments
///   - Update comment content
///   - Delete comments
///
/// ## Profile Scopes
///
/// - **`profile:read`** - Read access to user profile information
///   - Get user's basic profile data
///   - Access user preferences
///   - Read account information
///
/// ## Scope Usage Guidelines
///
/// ### Principle of Least Privilege
///
/// Always request the minimum scopes necessary for your application's functionality.
/// This improves user trust and reduces security risks.
///
/// ### Scope Combinations
///
/// You can request multiple scopes in a single OAuth authorization request:
///
/// ```text
/// scope=asset:read asset:write design:meta:read
/// ```
///
/// ### Read vs Write Permissions
///
/// - **Read scopes** allow your application to access data without modification
/// - **Write scopes** typically include read permissions for the same resource type
/// - Always prefer read-only access when write access is not needed
///
/// ## Common Scope Patterns
///
/// ### Asset Management Application
/// ```text
/// scope=asset:read asset:write folder:read folder:write
/// ```
///
/// ### Design Export Service
/// ```text
/// scope=design:meta:read design:content:read
/// ```
///
/// ### Design Creation Tool
/// ```text
/// scope=asset:read design:content:write brandtemplate:meta:read
/// ```
///
/// ### Comment Management System
/// ```text
/// scope=comment:read comment:write design:meta:read
/// ```
pub mod oauth_scopes {
    /// Asset-related scopes
    pub const ASSET_READ: &str = "asset:read";
    pub const ASSET_WRITE: &str = "asset:write";

    /// Design-related scopes
    pub const DESIGN_META_READ: &str = "design:meta:read";
    pub const DESIGN_CONTENT_READ: &str = "design:content:read";
    pub const DESIGN_CONTENT_WRITE: &str = "design:content:write";

    /// Brand template scopes
    pub const BRAND_TEMPLATE_META_READ: &str = "brandtemplate:meta:read";
    pub const BRAND_TEMPLATE_CONTENT_READ: &str = "brandtemplate:content:read";

    /// Folder-related scopes
    pub const FOLDER_READ: &str = "folder:read";
    pub const FOLDER_WRITE: &str = "folder:write";

    /// Comment-related scopes
    pub const COMMENT_READ: &str = "comment:read";
    pub const COMMENT_WRITE: &str = "comment:write";

    /// Profile-related scopes
    pub const PROFILE_READ: &str = "profile:read";
}
