use worker::*;

mod utils;
mod rolename;


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
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get_async("/", |_, _| async move { Response::ok("utilitybot miscellaneous backends worker. why are you here?") })
        .get_async("/rolenames", rolename::get)
        .post_async("/rolenames", rolename::post)
        .delete_async("/rolenames", rolename::delete)
        .run(req, env)
        .await
}
