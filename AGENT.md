# AGENT.md - Canva Connect API Workspace

## Project Structure
- `public-api.yml` - OpenAPI 3.0 specification for Canva Connect API
- This is an API specification workspace focused on the Canva Connect REST API

## API Overview
- **Base URL**: `https://api.canva.com/rest`
- **Version**: OpenAPI 3.0.0
- **Authentication**: OAuth 2.0 with authorization code flow
- **Rate Limiting**: Per-client-user limits specified per endpoint

## API Categories
- **Assets**: Upload, manage, and retrieve user content library assets
- **Autofill**: Enterprise feature for populating brand templates with data
- **Brand Templates**: Enterprise templates for consistent branding
- **Comments**: Design collaboration and feedback system
- **Designs**: Create, manage, and manipulate Canva designs
- **Exports**: Export designs to various formats
- **Folders**: Organize and manage design collections
- **OAuth**: Authentication and authorization flows
- **Users**: User profile and account information

## Key Patterns
- Asynchronous jobs for long-running operations (uploads, exports, autofill)
- Pagination with continuation tokens for large result sets
- Rate limiting with per-client-user quotas
- Enterprise-only features require Canva Enterprise organization membership
- Preview APIs marked with warnings about breaking changes

## Security & Compliance
- OAuth scopes control access to different API capabilities
- Rate limiting enforced per client and user combination
- Enterprise features restricted to organization members
