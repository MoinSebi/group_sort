/// Sorting with groups
///
/// Only sequential data
pub fn group_sort(input: Vec<usize>) -> Vec<usize>{
    let max_val = input.iter().max().unwrap();
    let mut steps = vec![0; *max_val+1];
    input.iter().map(|y| { steps[*y] += 1});

    let cstep_index = cum_step_index(steps);

    let mut update = vec![0; *max_val+1];
    let mut result = vec![0; input.len()];
    input.into_iter().map(|x| { result[cstep_index[x] + update[x]] = x; update[x] += 1});
    return result
}

/// Step index
///
/// Step index -> cum step index
pub fn cum_step_index(v: Vec<usize>) -> Vec<usize>{
    let mut last_index = 0;
    let mut step_ind: Vec<usize> = v.iter().map(|x|{
        last_index += x;
        last_index
    }).collect();
    // Add 0 at the beginning
    step_ind.insert(0, 0);
    // Remove last element
    step_ind.pop();

    return step_ind

}



#[cfg(test)]
mod tests {
    use rand::thread_rng;
    use rand::seq::SliceRandom;
    use super::*;

    /// Shuffled
    pub fn make_data(u: usize) -> Vec<usize>{
        let mut ff = Vec::new();
        for x in 0..u{
            for _ in 0..20{
                ff.push(x)
            }
        }
        ff.shuffle(&mut thread_rng());
        return ff
    }



    #[test]
    fn testing_big() {
        use std::time::Instant;
        let result = make_data(10000);
        println!("New");
        let start = Instant::now();

        group_sort(result);
        let elapsed = start.elapsed();
        println!("Millis: {} ms", elapsed.as_millis());
        let mut result = make_data(10000);
        println!("Old");
        let start = Instant::now();

        result.sort();
        let elapsed = start.elapsed();
        println!("Millis: {} ms", elapsed.as_millis());

    }
}
