use optimizer_wasm::Store;

pub fn get_test_data() -> (Vec<Vec<f64>>, Vec<Vec<u32>>, Vec<u32>, Vec<Store>) {
    #[rustfmt::skip]
        let prices = vec![
            vec![ 13000.0, 5000.0, 2000.0, 4000.0, 6000.0, 7500.0, 8000.0, 3000.0, 9000.0, 8000.0, 2000.0, 2000.0, 4000.0, 4000.0, 5000.0, 400.0 ],
            vec![ 12000.0, 6000.0, 3000.0, 5000.0, 7000.0, 6000.0, 9000.0, 1000.0, 2000.0, 8500.0, 1000.0, 4000.0, 3000.0, 4000.0, 5500.0, 500.0 ],
            vec![ 14000.0, 5500.0, 2500.0, 3000.0, 8000.0, 7000.0, 6000.0, 2000.0, 6000.0, 9000.0, 2000.0, 2500.0, 4500.0, 4000.0, 4000.0, 600.0 ],
        ];
    let stocks = vec![
        vec![1, 2, 1, 1, 2, 3, 4, 5, 6, 3, 2, 2, 4, 4, 5, 8],
        vec![0, 2, 1, 2, 2, 3, 4, 5, 6, 3, 2, 2, 4, 4, 5, 8],
        vec![2, 2, 1, 3, 2, 3, 4, 5, 6, 3, 2, 2, 4, 4, 5, 8],
    ];
    let needs = vec![1, 2, 3, 4, 5, 6, 5, 4, 8, 7, 3, 5, 7, 9, 3, 18];
    let stores = vec![
        Store {
            name: "Shop A".to_string(),
            free_shipping_num: 5,
            free_shipping_sum: 110.0,
            base_shipping: 5.0,
        },
        Store {
            name: "Shop B".to_string(),
            free_shipping_num: 7,
            free_shipping_sum: 100.0,
            base_shipping: 3.0,
        },
        Store {
            name: "Shop C".to_string(),
            free_shipping_num: 2,
            free_shipping_sum: 120.0,
            base_shipping: 7.0,
        },
    ];
    (prices, stocks, needs, stores)
}
