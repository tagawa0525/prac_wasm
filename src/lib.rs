use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Store {
    name: String,
    free_shipping: FreeShipping,
    base_shipping: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct FreeShipping {
    num: i32,
    sum: f64,
}

#[wasm_bindgen]
pub fn solve_optimization_problem(
    prices: JsValue,
    stocks: JsValue,
    needs: JsValue,
    stores: JsValue,
) -> JsValue {
    // ここに最適化問題を解くロジックを実装します
    let prices: Vec<Vec<f64>> = from_value(prices).unwrap();
    let stocks: Vec<Vec<i32>> = from_value(stocks).unwrap();
    let needs: Vec<i32> = from_value(needs).unwrap();
    let stores: Vec<Store> = from_value(stores).unwrap();

    // 例として、単純な計算を行う
    let result = format!(
        "Prices: {:?}, Stocks: {:?}, Needs: {:?}, Stores: {:?}",
        prices, stocks, needs, stores
    );
    JsValue::from_str(&result)
}
