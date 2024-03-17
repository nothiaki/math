use axum::{extract::Query, routing::get, Json, Router};
use serde::Deserialize;
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/sum", get(sum))
        .route("/sub", get(sub))
        .route("/area/parallelogram", get(parallelogram_area));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

#[derive(Deserialize)]
struct Body {
    nums: String,
}

//localhost:3000/sum?nums=12,15,20
async fn sum(Query(req): Query<Body>) -> Result<Json<i32>, Infallible> {
    let list: Vec<i32> = string_to_int_vec(req);
    let res = list.iter().sum();

    Ok(Json(res))
}

//localhost:3000/sub?nums=12,15,20
async fn sub(Query(req): Query<Body>) -> Result<Json<i32>, Infallible> {
    let list: Vec<i32> = string_to_int_vec(req);
    let mut res: i32 = list[0];

    for i in 1..list.len() {
        res -= list[i];
    }

    Ok(Json(res))
}

fn string_to_int_vec(list: Body) -> Vec<i32> {
    list.nums
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

#[derive(Deserialize)]
struct Parallelogram {
    w: i32,
    h: i32,
}

//localhost:3000/area/parallelogram?w=4&h=4
async fn parallelogram_area(Query(req): Query<Parallelogram>) -> Result<Json<i32>, Infallible> {
    let res = req.w * req.h;
    Ok(Json(res))
}
