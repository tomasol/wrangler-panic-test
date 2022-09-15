use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use worker::*;

mod utils;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();
    let router = Router::new();
    router
        .get("/", |_, _| {
            let old = COUNTER.fetch_add(1, Ordering::SeqCst);
            if old == 0 {
                panic!("");
            }
            Response::ok(old.to_string())
        })
        .get("/crash", |_, _| panic!("crash"))
        .get_async("/wait/:delay", |_, ctx| async move {
            let delay: Delay = match ctx.param("delay").unwrap().parse() {
                Ok(delay) => Duration::from_millis(delay).into(),
                Err(_) => return Response::error("invalid delay", 400),
            };

            let old = COUNTER.fetch_add(1, Ordering::SeqCst);

            assert!(old == 0);
            // Wait for the delay to pass
            delay.await;

            let old = COUNTER.load(Ordering::SeqCst);
            Response::ok(old.to_string())
        })
        .run(req, env)
        .await
}
