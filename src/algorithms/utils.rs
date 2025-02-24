use crate::Store;

pub fn calculate_shipping_cost(
    best_dist: &Vec<Vec<u32>>,
    prices: &Vec<Vec<f64>>,
    stores: &Vec<Store>,
) -> f64 {
    // 送料の計算
    let mut shipping_cost = 0.0;
    for (store_idx, store) in stores.iter().enumerate() {
        let total_items: u32 = best_dist[store_idx].iter().sum();
        let total_price: f64 = best_dist[store_idx]
            .iter()
            .enumerate()
            .map(|(item_idx, &quantity)| quantity as f64 * prices[store_idx][item_idx])
            .sum();

        if total_items < store.free_shipping_num && total_price < store.free_shipping_sum {
            shipping_cost += store.base_shipping;
        }
    }
    return shipping_cost;
}
