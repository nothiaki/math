use axum::{extract::Query, Json};
use serde::Deserialize;
use std::convert::Infallible;

#[derive(Deserialize)]
pub struct Body {
    nums: String,
}

//localhost:PORT/sum?nums=12,15,20
pub async fn sum(Query(req): Query<Body>) -> Result<Json<i32>, Infallible> {
    let list: Vec<i32> = string_to_int_vec(req);
    let res = list.iter().sum();

    Ok(Json(res))
}

//localhost:PORT/sub?nums=12,15,20
pub async fn sub(Query(req): Query<Body>) -> Result<Json<i32>, Infallible> {
    let list: Vec<i32> = string_to_int_vec(req);
    let mut res: i32 = list[0];

    for i in 1..list.len() {
        res -= list[i];
    }

    Ok(Json(res))
}

pub fn string_to_int_vec(list: Body) -> Vec<i32> {
    list.nums
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
