use worker::*;
use serde_json::{json, Value};

use crate::utils;


pub async fn get<T>(req: Request, ctx: RouteContext<T>) -> Result<Response> {
    if !utils::auth(&req, &ctx) {
        return Response::error("Unauthenticated. Pass the API_KEY header! If you have passed it, check if it's right.", 401);
    }

    let kv = ctx.kv("rolenames")?;
    let keys_res = kv.list().execute().await?;  // limit of 100 keys; will this be a problem?
    let keys = keys_res.keys.iter().map(|key| key.name.as_str()).collect::<Vec<&str>>();

    let res_json = json!({"rolenames": keys});
    return Response::from_json(&res_json);
}

pub async fn post<T>(mut req: Request, ctx: RouteContext<T>) -> Result<Response> {
    if !utils::auth(&req, &ctx) {
        return Response::error("Unauthenticated. Pass the API_KEY header! If you have passed it, check if it's right.", 401);
    }
    
    let kv = ctx.kv("rolenames")?;
    let body: Value = req.json().await?;

    let rolename = match body.get("rolename") {
        Some(rn) => rn,
        None => { return Response::error("rolename is a required parameter", 400); }
    };
    let user_id = match body.get("added_by") {
        Some(uid) => uid,
        None => { return Response::error("added_by is a required parameter", 400); }
    };

    let kv_value = format!("{{\"added_by\": {} }}", user_id);
    kv.put(rolename.as_str().unwrap(), kv_value);
    return Response::empty();
}


pub async fn delete<T>(mut req: Request, ctx: RouteContext<T>) -> Result<Response> {
    if !utils::auth(&req, &ctx) {
        return Response::error("Unauthenticated. Pass the API_KEY header! If you have passed it, check if it's right.", 401);
    }

    let kv = ctx.kv("rolenames")?;
    let body: Value = req.json().await?;

    let rolename = match body.get("rolename") {
        Some(rn) => rn,
        None => { return Response::error("rolename is a required parameter", 400); }
    };

    kv.delete(rolename.as_str().unwrap()).await?;
    return Response::empty();
}
