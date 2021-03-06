extern crate time;
extern crate rand;

use self::rand::Rng;
use print_utils;

pub fn solve(matrix: &mut Vec<Vec<i32>>,
             temperature: f32,
             annealing_velocity: f32,
             time_max: i64) -> (Vec<i32>, i32) {
    //println!("Przygotowywanie zmiennych…");

    let timer_start = time::PreciseTime::now();

    let mut _temperature: f32 = temperature;

    // Przygotowanie ścieżki początkowej
    // Generowanie "najtańszej lokalnie" ścieżki
    // let mut current_path: Vec<i32> = find_first_path(&matrix, rand::thread_rng().gen_range(0, (matrix.len() as i32) - 1));
    let mut current_path: Vec<i32> = Vec::new();
    for i in 0..(matrix.len() as i32) {
        current_path.push(i);
    }
    rand::thread_rng().shuffle(&mut current_path);

    let mut current_value = path_value(&matrix, &current_path);

    // Najlepsze znane rozwiązanie - to to jedno, właśnie stworzone.
    let mut best_path = current_path.clone();
    let mut best_value = current_value.clone();

    //println!("Początek algorytmu...");
    while _temperature > 1.0 {
        // Warunek sprawdzający, czy przekroczono czas.
        if timer_start.to(time::PreciseTime::now()).num_seconds() >= time_max {
            break;
        }

        let new_path: Vec<i32> = swap_random_cities(&current_path);
        let new_value = path_value(&matrix, &new_path);

        let p: f32 = probability(&current_value, &new_value, &temperature);
        let random: f32 = rand::thread_rng().gen_range(0.0f32, 1.0f32);

        if p >= random {
            current_value = new_value;
            current_path = new_path;

            if current_value < best_value {
                best_value = current_value;
                best_path = current_path.clone();

                println!("{} {}", best_value, timer_start.to(time::PreciseTime::now()).num_nanoseconds().unwrap());
            }
        }

        _temperature *= annealing_velocity;
    }

    let timer_stop = time::PreciseTime::now();

    print_utils::print_result(best_value,
                              best_path.clone(),
                              timer_start.to(timer_stop)
                                         .num_nanoseconds()
                                         .unwrap());

    // println!("{} {} {} {} {} {} {:?}", matrix.len(), temperature, _temperature, annealing_velocity, best_value, timer_start.to(timer_stop).num_nanoseconds().unwrap(), best_path.clone());

    return (best_path, best_value);
}

pub fn path_value(matrix: &Vec<Vec<i32>>, path: &Vec<i32>) -> i32 {
    let mut result: i32 = 0;

    for i in 0..(path.len() - 1) {
        result += matrix[path[i] as usize][path[i + 1] as usize];
    }

    result += matrix[path.last().unwrap().clone() as usize][path.first().unwrap().clone() as usize];

    return result;
}

fn swap_random_cities(path: &Vec<i32>) -> Vec<i32> {
    let a = rand::thread_rng().gen_range(0, path.len());
    let b = rand::thread_rng().gen_range(0, path.len());

    let mut new_path = path.clone();
    new_path[a] = path[b];
    new_path[b] = path[a];

    return new_path;
}

fn probability(current_value: &i32, new_value: &i32, temperature: &f32) -> f32 {
    let probability: f32;

    if current_value > new_value {
        probability = 1.0;
    } else {
        let exponent: f32 = ((current_value - new_value) as f32) / temperature.clone();
        probability = exponent.exp();
    }

    return probability;
}

fn find_first_path(matrix: &Vec<Vec<i32>>, start_city_index: i32) -> Vec<i32> {
    let mut path: Vec<i32> = vec![];

    assert_eq!(true, start_city_index < matrix.len() as i32);
    path.push(start_city_index);

    for i in 1..matrix.len() {
        let mut min_cost: i32 = <i32>::max_value();
        let mut index: i32 = 0;
        for j in 0..matrix.len() {
            if i == j {
                continue;
            }
            if matrix[path[i - 1] as usize][j] < min_cost {
                if !path.contains(&(j as i32)) {
                    min_cost = matrix[path[i - 1] as usize][j];
                    index = j as i32;
                }
            }
        }
        path.push(index);
    }

    return path;
}

#[cfg(test)]
mod sa_tests {
    use simulated_annealing;

    #[test]
    fn test_first_path() {
        let mut tested_matrix: Vec<Vec<i32>> = Vec::new();
        tested_matrix.push(vec![<i32>::max_value(), 20, 30, 10, 11]);
        tested_matrix.push(vec![15, <i32>::max_value(), 16, 4, 2]);
        tested_matrix.push(vec![3, 5, <i32>::max_value(), 2, 4]);
        tested_matrix.push(vec![19, 6, 18, <i32>::max_value(), 3]);
        tested_matrix.push(vec![16, 4, 7, 16, <i32>::max_value()]);

        let expected_result = vec![0, 3, 4, 1, 2];
        let result = simulated_annealing::find_first_path(&tested_matrix, 0);

        assert_eq!(true, result.len() == expected_result.len());

        let mut test = true;
        for i in 0..result.len() {
            if result[i] != expected_result[i] {
                test = false;
                break;
            }
        }

        eprintln!("{:?} vs {:?}", expected_result, result);
        assert_eq!(true, test);
    }

    /* Testuje obliczanie kosztu ścieżki */
    #[test]
    fn test_path_value() {
        let mut tested_matrix: Vec<Vec<i32>> = Vec::new();
        tested_matrix.push(vec![<i32>::max_value(), 20, 30, 10, 11]);
        tested_matrix.push(vec![15, <i32>::max_value(), 16, 4, 2]);
        tested_matrix.push(vec![3, 5, <i32>::max_value(), 2, 4]);
        tested_matrix.push(vec![19, 6, 18, <i32>::max_value(), 3]);
        tested_matrix.push(vec![16, 4, 7, 16, <i32>::max_value()]);

        let tested_path: Vec<i32> = vec![0, 1, 2, 3, 4];
        let expected_cost: i32 = 20 + 16 + 2 + 3 + 16;
        assert_eq!(expected_cost, simulated_annealing::path_value(&tested_matrix, &tested_path));

        let tested_path: Vec<i32> = vec![4, 3, 2, 1, 0];
        let expected_cost: i32 = 16 + 18 + 5 + 15 + 11;
        assert_eq!(expected_cost, simulated_annealing::path_value(&tested_matrix, &tested_path));

        let tested_path: Vec<i32> = vec![4, 1, 2, 3, 0];
        let expected_cost: i32 = 4 + 16 + 2 + 19 + 11;
        assert_eq!(expected_cost, simulated_annealing::path_value(&tested_matrix, &tested_path));
    }
}