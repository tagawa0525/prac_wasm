mod common;

use self::common::*;
use optimizer_wasm::algorithms::greedy::optimize_by_greedy;

#[test]
fn test_optimize_by_greedy() {
    let (prices, stocks, needs, stores) = get_test_data();

    let (best_dist, total_cost) = optimize_by_greedy(&prices, &stocks, &needs, &stores);

    // 期待される結果をここに記述
    let expected_best_dist = vec![
        vec![1, 2, 1, 1, 2, 3, 4, 4, 6, 3, 2, 2, 4, 4, 3, 8],
        vec![0, 0, 1, 2, 2, 3, 1, 0, 2, 3, 1, 2, 3, 4, 0, 8],
        vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 2],
    ];
    let expected_total_cost = 395400.0;

    assert_eq!(best_dist, expected_best_dist);
    assert_eq!(total_cost, expected_total_cost);
}
