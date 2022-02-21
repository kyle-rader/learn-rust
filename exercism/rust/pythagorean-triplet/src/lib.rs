use std::collections::HashSet;

type PyTriple = [u32; 3];

pub fn find(sum: u32) -> HashSet<PyTriple> {
    // For a valid [a, b, c] the following must hold true:
    // a < b < c < sum
    // a^2 + b^2 = c^2
    let mut results: HashSet<PyTriple> = HashSet::new();

    for a in 0..sum/3 {
        for b in a+1..(sum+1-a) {
            let triple = [a, b, (sum-a-b)];

            if is_py_triplet(triple) {
                results.insert(triple);
            }
        }
    }
    results
}

fn is_py_triplet(triplet: PyTriple) -> bool {
    let squared: Vec<f64> = triplet
        .iter()
        .map(|x| (*x as f64).powf(2.0))
        .collect();
    squared[0] + squared[1] == squared[2]
}
