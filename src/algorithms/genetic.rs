use rand::Rng;

use super::utils::calculate_total_cost;
use crate::Store;

#[derive(Clone)]
struct Individual {
    genes: Vec<Vec<u32>>,
    fitness: f64,
}

pub fn optimize(
    prices: &Vec<Vec<f64>>,
    stocks: &Vec<Vec<u32>>,
    needs: &Vec<u32>,
    stores: &Vec<Store>,
) -> (Vec<Vec<u32>>, f64) {
    let mut rng = rand::rng();
    let population_size = 50;
    let mut population: Vec<Individual> = (0..population_size)
        .map(|_| {
            let dist = generate_initial_distribution(&stocks, &needs, &mut rng);
            let fitness = calculate_total_cost(&dist, &prices, &stores);
            Individual {
                genes: dist,
                fitness,
            }
        })
        .collect();

    for _ in 0..1000 {
        // 世代数
        population.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
        let mut new_population = vec![population[0].clone()]; // エリート保存

        while new_population.len() < population_size {
            let parent1 = &population[rng.random_range(0..population_size / 2)]; // 上位50%から選択
            let parent2 = &population[rng.random_range(0..population_size / 2)];
            let mut child_dist = if rng.random::<f64>() < 0.7 {
                crossover(&parent1.genes, &parent2.genes, &stocks, &needs, &mut rng)
            } else {
                parent1.genes.clone()
            };
            if rng.random::<f64>() < 0.1 {
                child_dist = mutate(&child_dist, &stocks, &needs, &mut rng);
            }
            let fitness = calculate_total_cost(&child_dist, &prices, &stores);
            new_population.push(Individual {
                genes: child_dist,
                fitness,
            });
        }
        population = new_population;
    }

    population.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
    (population[0].genes.clone(), population[0].fitness)
}

// 初期化
fn generate_initial_distribution(
    stocks: &Vec<Vec<u32>>,
    needs: &Vec<u32>,
    rng: &mut impl Rng,
) -> Vec<Vec<u32>> {
    let num_shops = stocks.len();
    let num_items = needs.len();
    let mut genes = vec![vec![0; num_items]; num_shops];
    let mut remaining_needs = needs.clone();

    for item in 0..num_items {
        let mut need = needs[item];
        while need > 0 {
            let shop = rng.random_range(0..num_shops);
            let available = stocks[shop][item] - genes[shop][item];
            if available > 0 {
                let take = rng.random_range(1..=available.min(need));
                genes[shop][item] += take;
                need -= take;
                remaining_needs[item] -= take;
            }
        }
    }
    genes
}

// 交叉
fn crossover(
    parent1: &Vec<Vec<u32>>,
    parent2: &Vec<Vec<u32>>,
    stocks: &Vec<Vec<u32>>,
    needs: &Vec<u32>,
    rng: &mut impl Rng,
) -> Vec<Vec<u32>> {
    let num_shops = parent1.len();
    // let num_items = parent1[0].len();
    let mut child = parent1.clone();
    let point1 = rng.random_range(0..num_shops);
    let point2 = rng.random_range(point1..num_shops);

    for shop in point1..=point2 {
        child[shop] = parent2[shop].clone();
    }
    adjust_distribution(&mut child, stocks, needs); // 在庫と必要数を調整
    child
}

// 突然変異
fn mutate(
    genes: &Vec<Vec<u32>>,
    stocks: &Vec<Vec<u32>>,
    needs: &Vec<u32>,
    rng: &mut impl Rng,
) -> Vec<Vec<u32>> {
    let mut new_dist = genes.clone();
    let num_shops = stocks.len();
    let num_items = needs.len();

    let item = rng.random_range(0..num_items);
    let from_shop = rng.random_range(0..num_shops);
    let to_shop = rng.random_range(0..num_shops);

    if from_shop != to_shop && new_dist[from_shop][item] > 0 {
        let max_move =
            new_dist[from_shop][item].min(stocks[to_shop][item] - new_dist[to_shop][item]);
        if max_move > 0 {
            let move_amount = rng.random_range(1..=max_move);
            new_dist[from_shop][item] -= move_amount;
            new_dist[to_shop][item] += move_amount;
        }
    }
    new_dist
}

fn adjust_distribution(genes: &mut Vec<Vec<u32>>, stocks: &Vec<Vec<u32>>, needs: &Vec<u32>) {
    let num_shops = stocks.len();
    let num_items = needs.len();

    for item in 0..num_items {
        let mut total = genes.iter().map(|shop| shop[item]).sum::<u32>();
        while total != needs[item] {
            for shop in 0..num_shops {
                if total < needs[item] && genes[shop][item] < stocks[shop][item] {
                    genes[shop][item] += 1;
                    total += 1;
                } else if total > needs[item] && genes[shop][item] > 0 {
                    genes[shop][item] -= 1;
                    total -= 1;
                }
                if total == needs[item] {
                    break;
                }
            }
        }
    }
}
