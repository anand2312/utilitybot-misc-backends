use cfg_if::cfg_if;
use worker::*;


cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}


pub fn auth<T>(req: &Request, ctx: &RouteContext<T>) -> bool {
    let API_KEY = ctx.var("API-KEY").unwrap().to_string();
    let user_key = match req.headers().get("API_KEY") {
        Ok(x) => {
            match x {
                Some(key) => key,
                None => return false
            }
        },
        _ => { return false }
    };
    return API_KEY == user_key;
}
