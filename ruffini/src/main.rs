fn ruffini_rule(polynomial: Vec<i32>, possible_root: i32) -> Vec<i32> {
    let mut remainder = polynomial[0];
    let mut result = vec![remainder];

    for &coeff in polynomial.iter().skip(1) {
        let term = remainder * possible_root + coeff;
        remainder = term;
        result.push(term);
    }

    result
}

fn find_rational_root(polynomial: Vec<i32>) -> Option<i32> {
    let constant_factor = polynomial.last().unwrap().abs();

    for i in 1..=constant_factor {
        if constant_factor % i == 0 {
            let positive_test_root = i;
            let negative_test_root = -i;

            if ruffini_rule(polynomial.clone(), positive_test_root).last().unwrap() == &0 {
                return Some(positive_test_root);
            }
            if ruffini_rule(polynomial.clone(), negative_test_root).last().unwrap() == &0 {
                return Some(negative_test_root);
            }
        }
    }

    None
}

fn main() {
    let polynomial = vec![1, -1, 4, -12]; // Represents x^3 - x^2 + 4x - 12

    match find_rational_root(polynomial) {
        Some(root) => println!("A rational root is: {}", root),
        None => println!("No rational roots were found using the Ruffini rule."),
    }
}
