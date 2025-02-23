use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

mod algorithms;

#[derive(Serialize, Deserialize, Debug)]
struct Store {
    name: String,
    free_shipping: FreeShipping,
    base_shipping: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct FreeShipping {
    num: u32,
    sum: f64,
}

#[wasm_bindgen]
pub fn solve_optimization_problem(
    prices: JsValue,
    stocks: JsValue,
    needs: JsValue,
    stores: JsValue,
) -> JsValue {
    let prices: Vec<Vec<f64>> = from_value(prices).unwrap();
    let stocks: Vec<Vec<u32>> = from_value(stocks).unwrap();
    let needs: Vec<u32> = from_value(needs).unwrap();
    let stores: Vec<Store> = from_value(stores).unwrap();

    let (best_dist, total_cost) =
        algorithms::greedy::optimize_by_greedy(&prices, &stocks, &needs, &stores);

    let result = to_value(&(best_dist, total_cost)).unwrap();
    result
}
