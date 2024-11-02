use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/", |_, _| async move {
            Response::ok("Jesus is Lord!")
        })
        .get_async("/about", |_, _| async move {
            Response::ok("Welcome to our Cloudflare Development Website!")
        })
        .get_async("/gospel", |_, _| async move {
            Response::ok("Gospel means good news! The bad news is we have all sinned and deserve the wrath to come. But Jesus the Messiah died for our sins, was buried, and then raised on the third day, according to the scriptures. He ascended into heaven and right now is seated at the Father's right hand. Jesus said, \"I am the way, and the truth, and the life. No one comes to the Father except through me. The time is fulfilled, and the kingdom of God is at hand; repent and believe in the gospel.\"")
        })
        .get_async("/healthcheck", |_, _| async move {
            Response::ok("OK")
        })
        .run(req, env)
        .await
}
