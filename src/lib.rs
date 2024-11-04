use axum::response::IntoResponse;
use axum::{response::Html, routing::get, Router};
use tera::{Context as TeraContext, Tera};
use tower_service::Service;
use worker::*;

fn create_tera() -> Tera {
    let mut tera = Tera::default();
    tera.add_raw_template("base", include_str!("./html/base.html"))
        .unwrap();
    tera.add_raw_template("header", include_str!("./html/header.html"))
        .unwrap();
    tera.add_raw_template("footer", include_str!("./html/footer.html"))
        .unwrap();
    tera.add_raw_template("sidebar", include_str!("./html/sidebar.html"))
        .unwrap();
    tera.add_raw_template("home", include_str!("./html/home.html"))
        .unwrap();
    tera.add_raw_template("about", include_str!("./html/about.html"))
        .unwrap();
    tera.add_raw_template("gospel", include_str!("./html/gospel.html"))
        .unwrap();
    tera
}

async fn render_template(template: &str, title: &str) -> axum::response::Response {
    let tera = create_tera();
    let mut context = TeraContext::new();
    context.insert("title", title);

    match tera.render(template, &context) {
        Ok(html) => Html(html).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template error: {}", e),
        )
            .into_response(),
    }
}

fn router() -> Router {
    Router::new()
        .route("/", get(|| async { render_template("home", "Home").await }))
        .route(
            "/about",
            get(|| async { render_template("about", "About Us").await }),
        )
        .route(
            "/gospel",
            get(|| async { render_template("gospel", "Gospel").await }),
        )
        .route("/healthcheck", get(|| async { "OK" }))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    // let headers = req.headers();
    // console_log!(
    //     "Worker request: {} {:?}, CF-Ray: {:?}, CF-IPCountry: {:?}",
    //     req.method().to_string(),
    //     req,
    //     _env,
    //     _ctx
    // );

    // Handle the request using the Axum router
    Ok(router().call(req).await?)
}
