import init, { solve_optimization_problem } from './pkg/optimizer_wasm.js';

async function run() {
    await init();
    const prices = [
        [13000.0, 5000.0, 2000.0, 4000.0, 6000.0, 7500.0, 8000.0, 3000.0, 9000.0, 8000.0, 2500.0, 2000.0, 4000.0, 4000.0, 5000.0, 400.0],
        [12000.0, 6000.0, 3000.0, 5000.0, 7000.0, 6000.0, 9000.0, 1000.0, 2000.0, 8500.0, 1500.0, 4000.0, 3000.0, 4200.0, 5500.0, 500.0],
        [14000.0, 5500.0, 2500.0, 3000.0, 8000.0, 7000.0, 6000.0, 2000.0, 6000.0, 9000.0, 2000.0, 2500.0, 4500.0, 4400.0, 4000.0, 600.0]
    ];
    const stocks = [
        [1, 2, 1, 1, 2, 3, 4, 5, 6, 3, 2, 2, 4, 4, 5, 8],
        [0, 2, 1, 2, 2, 3, 4, 5, 6, 3, 2, 2, 4, 4, 5, 8],
        [2, 2, 1, 3, 2, 3, 4, 5, 6, 3, 2, 2, 4, 4, 5, 8],
    ];
    const needs = [1, 2, 3, 4, 5, 6, 5, 4, 8, 7, 3, 5, 7, 9, 3, 18];
    const stores = [
        { 'name': 'Shop A', 'free_shipping_num': 50, 'free_shipping_sum': 110000, 'base_shipping': 5000 },
        { 'name': 'Shop B', 'free_shipping_num': 70, 'free_shipping_sum': 100000, 'base_shipping': 3000 },
        { 'name': 'Shop C', 'free_shipping_num': 20, 'free_shipping_sum': 120000, 'base_shipping': 7000 }
    ];

    const inputDataParts = [
        `Prices:\n${prices.map((p, i) => `Shop ${String.fromCharCode(65 + i)}: ${JSON.stringify(p)}`).join('\n')}`,
        `Stocks:\n${stocks.map((s, i) => `Shop ${String.fromCharCode(65 + i)}: ${JSON.stringify(s)}`).join('\n')}`,
        `Needs: ${JSON.stringify(needs)}`,
        `Stores:\n${stores.map(store => `${store.name}: ${JSON.stringify(store)}`).join('\n')}`
    ];

    // 入力データを表示
    const inputData = inputDataParts.join('\n\n');
    console.log(inputData + '\n');

    // 最適化問題を解く
    const result = solve_optimization_problem(
        prices,
        stocks,
        needs,
        stores
    );
    const [best_distribution, total_cost, shipping_cost] = result;
    const bestDistFormatted = best_distribution.map((dist, i) => `Shop ${String.fromCharCode(65 + i)}: ${JSON.stringify(dist)}`).join('\n');
    const resultText = `Best Distribution:\n${bestDistFormatted}\n\nTotal    Cost: ${total_cost}\nShipping Cost: ${shipping_cost}`;

    // 結果を標準出力に出力
    console.log(resultText);
}

run();