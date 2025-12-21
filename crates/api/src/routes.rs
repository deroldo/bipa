use axum::Router;
use bipa_core::state::AppState;
use derust::envx::Environment;
use derust::httpx::AppContext;
use swagger_ui_dist::{ApiDefinition, OpenApiSource};

pub struct Routes;

impl Routes {
    pub fn routes(env: &Environment) -> Router<AppContext<AppState>> {
        let mut routes = Router::new();

        if !env.is_production() {
            routes = routes.merge(swagger());
        }

        routes
    }
}

fn swagger() -> Router<AppContext<AppState>> {
    let api_def = ApiDefinition {
        uri_prefix: "/swagger",
        api_definition: OpenApiSource::Inline(include_str!("openapi.yaml")),
        title: Some("Bipa API"),
    };

    swagger_ui_dist::generate_routes(api_def).with_state(())
}
