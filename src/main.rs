const MAX_TEST: u64 = 10000;

fn main() {
    let test_limit: u64 = 100;
    let optimal_limit = find_optimal(test_limit);
    let mut success_sum: u64 = 0;
    for number in 1..=MAX_TEST {
        if is_prime(number, optimal_limit) == is_prime_slow(number) {
            success_sum += 1
        }
    }
    println!(
        "success percentage for up to {MAX_TEST} is: {}%",
        success_sum as f64 / MAX_TEST as f64 * 100.0
    );
}

fn is_prime(number: u64, limit: u64) -> bool {
    //spoiler: optimal limit is 19 but no way ill hardcode that
    if (1..=limit).contains(&number) {
        number % 2 == 1
    } else {
        false
    }
}

fn is_prime_slow(number: u64) -> bool {
    let mut result: bool = true;
    for i in 2..number - 1 {
        if number % i == 0 {
            result = false;
            break;
        }
    }
    return result;
}

fn find_optimal(test_limit: u64) -> u64 {
    let mut optimal = 1;
    let mut optimal_sum = 0;
    for optimal_case in 1..=test_limit {
        //println!("testing optimal for: {optimal_case}");
        let mut sum = 0;
        for number in 1..=MAX_TEST {
            if is_prime(number, optimal_case) == is_prime_slow(number) {
                sum += 1;
            }
        }
        if optimal_sum < sum {
            println!("found new optimal at: {optimal_case}");
            optimal = optimal_case;
            optimal_sum = sum;
        }
    }
    println!("optimal for case up to {test_limit} is: {optimal}");
    optimal
}
