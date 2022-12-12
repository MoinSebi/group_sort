/// Sorting with groups
///
/// Original implementation
pub fn group_sort(input: Vec<usize>) -> Vec<usize>{
    let max_val = input.iter().max().unwrap();
    let mut steps = vec![0; *max_val+1];
    input.iter().for_each(|y| { steps[*y] += 1});
    let cstep_index = cum_step_index(steps);
    let mut update = vec![0; *max_val+1];
    let mut result = vec![0; input.len()];

    input.into_iter().for_each(|x | { result[cstep_index[x] + update[x]] = x; update[x] += 1});
    //input.into_iter().map(|x| { result[cstep_index[x] + update[x]] = x; update[x] += 1});
    return result
}

/// Sorting with groups2
///
/// Directly sort it with groups
pub fn group_sort2(input: Vec<usize>) -> Vec<usize>{
    let max_val = input.iter().max().unwrap();
    let mut steps = vec![vec![]; *max_val+1];
    input.into_iter().for_each(|x| {steps[x].push(x.clone())});
    let steps: Vec<usize> = steps.into_iter().flat_map(|array| array.into_iter()).collect();
    return steps
}


/// Sorting with groups3
///
/// Like original implementation but with out update
pub fn group_sort3(input: Vec<usize>) -> Vec<usize>{
    let max_val = input.iter().max().unwrap();
    let mut steps = vec![0; *max_val+1];
    let mut ss = vec![];
    input.iter().for_each(|y| {ss.push(steps[*y]);  steps[*y] += 1});
    let cstep_index = cum_step_index(steps);

    let mut result = vec![0; input.len()];
    input.into_iter().enumerate().for_each(|(i, x) | { result[cstep_index[x] + ss[i]] = x});
    //input.into_iter().map(|x| { result[cstep_index[x] + update[x]] = x; update[x] += 1});
    return result


}



/// Cumulative step index
///
/// Step index -> cum step index
/// Biggest number -> Length of the sequential data (group_sort input)
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

    /// Create new vector - size * 20 (shuffled)
    ///
    pub fn make_data(size: usize) -> Vec<usize>{
        let mut ff = Vec::new();
        for y in 0..40 {
            for x in 0..size{
                ff.push(x)
            }
        }
        //ff.shuffle(&mut thread_rng());
        return ff
    }


    #[test]
    fn test_true() {
        let result = vec![0,1,2,3,3,3,4,1,1,5,5,6,7,6,2,3];


        let i = group_sort3(result);
        let mut result = vec![0,1,2,3,3,3,4,1,1,5,5,6,7,6,2,3];
        result.sort();
        assert_eq!(i, result);
    }

    #[test]
    fn testing_big() {
        use std::time::Instant;
        let result = make_data(10000);

        println!("Running tests: size 100000*20\n");
        println!("Group sorting");
        let start = Instant::now();

        group_sort2(result);
        let elapsed = start.elapsed();
        println!("Millis: {} ms", elapsed.as_millis());
        println!();
        println!("Group sorting");
        let result = make_data(10000);

        let start = Instant::now();
        group_sort3(result);
        let elapsed = start.elapsed();
        println!("Millis: {} ms", elapsed.as_millis());
        println!();


        let mut result = make_data(10000);
        println!("Default sorting ");
        let start = Instant::now();
        result.sort();
        let elapsed = start.elapsed();
        println!("Millis: {} ms", elapsed.as_millis());
        println!("DONE");
        println!();
    }
}
