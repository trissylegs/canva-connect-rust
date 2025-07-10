//! API endpoint implementations

pub mod assets;

pub use assets::AssetsApi;

// Stub implementations for other endpoints
use crate::client::Client;

macro_rules! stub_api {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            #[allow(dead_code)]
            client: Client,
        }

        impl $name {
            pub fn new(client: Client) -> Self {
                Self { client }
            }
        }
    };
}

stub_api!(AutofillApi);
stub_api!(BrandTemplatesApi);
stub_api!(CommentsApi);
stub_api!(DesignsApi);
stub_api!(ExportsApi);
stub_api!(FoldersApi);
stub_api!(UserApi);
