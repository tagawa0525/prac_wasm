use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

pub mod algorithms;

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    pub name: String,
    pub free_shipping_num: u32,
    pub free_shipping_sum: f64,
    pub base_shipping: f64,
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

    let (best_dist, total_cost, shipping_cost) =
        algorithms::genetic::optimize(&prices, &stocks, &needs, &stores);

    let result = to_value(&(best_dist, total_cost, shipping_cost)).unwrap();
    result
}
