use tera::{Context as TeraContext, Tera};
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

async fn render_template(template: &str, title: &str) -> Result<Response> {
    let tera = create_tera();
    let mut context = TeraContext::new();
    context.insert("title", title);

    match tera.render(template, &context) {
        Ok(html) => {
            let mut headers = Headers::new();
            headers.set("Content-Type", "text/html")?;
            Ok(Response::ok(html)?.with_headers(headers))
        }
        Err(e) => Response::error(e.to_string(), 500),
    }
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async(
            "/",
            |_, _| async move { render_template("home", "Home").await },
        )
        .get_async("/about", |_, _| async move {
            render_template("about", "About Us").await
        })
        .get_async("/gospel", |_, _| async move {
            render_template("gospel", "Gospel").await
        })
        .get_async("/healthcheck", |_, _| async move { Response::ok("OK") })
        .run(req, env)
        .await
}
