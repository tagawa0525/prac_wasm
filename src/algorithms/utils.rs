use crate::Store;

// 送料の計算
pub fn calculate_shipping_cost(
    dist: &Vec<Vec<u32>>, // dist[店舗][商品] = 商品の個数 // どの店舗からどの商品を何個買うか
    prices: &Vec<Vec<f64>>, // prices[店舗][商品] = 商品の価格 // 各店舗の当該商品の価格
    stores: &Vec<Store>,  //store[店舗] = 店舗の情報 // 各店舗の送料の情報
) -> f64 {
    let mut shipping_cost = 0.0;
    for (store_idx, store) in stores.iter().enumerate() {
        let (store_sum, store_num) = calculate_store_cost(store_idx, dist, prices);

        // 無料条件を満たしていない場合、送料を加算
        if store_num < store.free_shipping_num && store_sum < store.free_shipping_sum {
            shipping_cost += store.base_shipping;
        }
    }
    return shipping_cost;
}

// 合計コストの計算
pub fn calculate_total_cost(
    dist: &Vec<Vec<u32>>, // dist[店舗][商品] = 商品の個数 // どの店舗からどの商品を何個買うか
    prices: &Vec<Vec<f64>>, // prices[店舗][商品] = 商品の価格 // 各店舗の当該商品の価格
    stores: &Vec<Store>,  // store[店舗] = 店舗の情報 // 各店舗の送料の情報
) -> f64 {
    let mut total_cost = 0.0;
    for (store_idx, store) in stores.iter().enumerate() {
        let (store_sum, store_num) = calculate_store_cost(store_idx, dist, prices);
        total_cost += store_sum;

        if store_num < store.free_shipping_num && store_sum < store.free_shipping_sum {
            total_cost += store.base_shipping
        }
    }
    total_cost
}

// 各店舗の商品の合計個数と合計金額を計算
fn calculate_store_cost(
    store_idx: usize,       // 店舗のインデックス
    dist: &Vec<Vec<u32>>,   // dist[店舗][商品] = 商品の個数 // どの店舗からどの商品を何個買うか
    prices: &Vec<Vec<f64>>, // prices[店舗][商品] = 商品の価格 // 各店舗の当該商品の価格
) -> (f64, u32) {
    let dist_store = &dist[store_idx];
    let price_store = &prices[store_idx];
    let mut store_sum = 0.0;
    let mut store_num = 0;
    for (item_idx, &count) in dist_store.iter().enumerate() {
        store_sum += price_store[item_idx] * count as f64;
        store_num += count;
    }
    (store_sum, store_num)
}
