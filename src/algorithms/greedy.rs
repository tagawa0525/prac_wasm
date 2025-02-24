use crate::Store;

pub fn optimize(
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

    total_cost += super::utils::calculate_shipping_cost(&best_dist, prices, stores);

    (best_dist, total_cost)
}
