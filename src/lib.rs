use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/", |_, _| async move {
            Response::ok("Jesus is Lord!")
        })
        .get_async("/about", |_, _| async move {
            Response::ok("Welcome to our Christian website!")
        })
        .get_async("/verse", |_, _| async move {
            Response::ok("For God so loved the world, that he gave his only Son, that whoever believes in him should not perish but have eternal life. - John 3:16")
        })
        .get_async("/healthcheck", |_, _| async move {
            Response::ok("OK")
        })
        .run(req, env)
        .await
}