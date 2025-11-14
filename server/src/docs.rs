use utoipa::{
    Modify,
    OpenApi,
    openapi::{
        ComponentsBuilder,
        security::{ApiKey, ApiKeyValue, SecurityScheme},
    },
};

#[derive(OpenApi)]
#[openapi(info(
   title = "Justice API",
   version = env!("CARGO_PKG_VERSION"),
   description = "Documentation for the Justice API."
), modifiers(&SecurityAddon))]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let mut components = openapi
            .components
            .take()
            .unwrap_or_else(|| ComponentsBuilder::new().build());

        components.add_security_scheme(
            "auth",
            SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("session"))),
        );

        openapi.components = Some(components);
    }
}