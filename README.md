## Group sort 

This is a sorting algorithm for random, sequential (0..X) data with no missing values. If data is maximum unsorted, group_sort performs 2x faster than default implemented sorting logarithm (quick sort). 

- **THIS IS NOT INPLACE SORTING**
- **Should be O(n) time and space** 
- **Only sorted by one value**  

### Algorithm
- Check for something


### Comparison 
- Shuffled, sorted
- Size

### Load with
```Rust
[dependencies]
group_sort = {git = "https://github.com/MoinSebi/group_sort"}
```

### Test and benchmark
```text
git clone https://github.com/MoinSebi/group_sort
cd group_sort

# Test 
cargo test 
cargo test -- --nocapture #Check output
# Bench
cargo bench
```

# TODO
- Do it for multidimensional vectors
- Multithreading 
- Add more documentation 
  - Algo. 
  - Comp. 