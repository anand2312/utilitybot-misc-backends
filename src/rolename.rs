use worker::*;
use serde_json::json;

mod utils;


pub async fn get(req: Request, ctx: RouteContext) -> Result<Response> {
    if utils::auth(&req, &ctx) {
        return Response::error("Unauthenticated. Pass the API_KEY header!", 401);
    }

    let kv = ctx.kv("rolenames")?;
    let keys_res = kv.keys().list().execute().await?;  // limit of 100 keys; will this be a problem?
    let keys = keys_res.keys.map(|key| key.name).collect::<Vec<String>>();

    let res_json = json!({"rolenames": keys});
    return Response::from_json(res_json);
}

pub async fn post(req: Request, ctx: RequestContext) -> Result<Response> {
    if utils::auth(&req, &ctx) {
        return Response::error("Unauthenticated. Pass the API_KEY header!", 401);
    }
    
    let kv = ctx.kv("rolenames");
    let body = req.json().await?;

    let rolename = match body.get("rolename") {
        Some(rn) => rn,
        None => { return Response::error("rolename is a required parameter", 400); }
    };
    let user_id = match body.get("added_by") {
        Some(uid) => uid,
        None => { return Response::error("added_by is a required parameter", 400); }
    };

    let kv_value = format!("{{\"added_by\": {} }}", user_id);
    kv.put(rolename, kv_value).execute().await?;
    return Response::empty();
}


pub async fn delete(req: Request, ctx: RequestContext) -> Result<Response> {
    if utils::auth(&req, &ctx) {
        return Response::error("Unauthenticated. Pass the API_KEY header!", 401);
    }

    let kv = ctx.kv("rolenames");
    let body = req.json().await?;

    let rolename = match body.get("rolename") {
        Some(rn) => rn,
        None => { return Response::error("rolename is a required parameter", 400); }
    };

    kv.delete(rolename).execute().await?;
    return Response::empty();
}
