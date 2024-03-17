use axum::{extract::Query, Json};
use serde::Deserialize;
use std::convert::Infallible;

#[derive(Deserialize)]
pub struct Parallelogram {
    w: i32,
    h: i32,
}

//localhost:PORT/area/parallelogram?w=4&h=4
pub async fn parallelogram(Query(req): Query<Parallelogram>) -> Result<Json<i32>, Infallible> {
    let res = req.w * req.h;
    Ok(Json(res))
}
