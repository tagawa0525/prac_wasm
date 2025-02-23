use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

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

    let (best_dist, total_cost) = optimize_by_greedy(&prices, &stocks, &needs, &stores);

    let result = to_value(&(best_dist, total_cost)).unwrap();
    result
}

// Greedy Algorithmを使用して最適化問題を解く
fn optimize_by_greedy(
    prices: &Vec<Vec<f64>>,
    stocks: &Vec<Vec<u32>>,
    needs: &Vec<u32>,
    stores: &Vec<Store>,
) -> (Vec<Vec<u32>>, f64) {
    let mut best_dist = vec![vec![0; needs.len()]; stores.len()];
    let mut total_cost = 0.0;

    for (item_idx, &need) in needs.iter().enumerate() {
        let mut remaining_need = need;
        for (store_idx, _store) in stores.iter().enumerate() {
            if remaining_need == 0 {
                break;
            }
            let available_stock = stocks[store_idx][item_idx];
            let purchase_quantity = remaining_need.min(available_stock);
            best_dist[store_idx][item_idx] = purchase_quantity;
            remaining_need -= purchase_quantity;
            total_cost += purchase_quantity as f64 * prices[store_idx][item_idx];
        }
    }

    // 送料の計算
    for (store_idx, store) in stores.iter().enumerate() {
        let total_items: u32 = best_dist[store_idx].iter().sum();
        let total_price: f64 = best_dist[store_idx]
            .iter()
            .enumerate()
            .map(|(item_idx, &quantity)| quantity as f64 * prices[store_idx][item_idx])
            .sum();

        if total_items < store.free_shipping.num && total_price < store.free_shipping.sum {
            total_cost += store.base_shipping;
        }
    }

    (best_dist, total_cost)
}
