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


pub fn auth(req: &Request, ,ctx: &RequestContext) -> bool {
    const API_KEY = ctx.var("API-KEY")?.to_string();
    return API_KEY == req.headers().get("API-KEY").unwrap_or("never gonna give you up".to_string());
}
