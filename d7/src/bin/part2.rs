use d7::equation::Operations;
use d7::parse_input;

pub fn main() {
    let mut equations = parse_input(vec![
        Operations::Add,
        Operations::Multiply,
        Operations::Concat,
    ]);

    let sum: u64 = equations
        .iter_mut()
        .map(|eq| {
            eq.generate_operation_combinations();
            eq.compute_potential_results();

            if let Some(x) = eq.potential_results.iter().find(|v| **v == eq.target) {
                *x
            } else {
                0
            }
        })
        .sum();

    println!("sum of valid values: {}", sum);
}
