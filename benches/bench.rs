use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use group_sort::{group_sort};

/// Create new vector - size * 20 (shuffled)
///
pub fn make_data(size: usize) -> Vec<usize>{
    let mut ff = Vec::new();
    for x in 0..size {
        for _ in 0..20{
            ff.push(x)
        }
    }
    ff.shuffle(&mut thread_rng());
    return ff
}

/// Group_sort (+ make new data)
fn new_sort(size: usize){
    let mut intervals = make_data(size);
    //sort_vector(&mut intervals);
    let mut network = group_sort(intervals);
}

/// Default sorting (+ make data)
fn old_sort(size: usize){
    let mut intervals = make_data(size);
    //sort_vector(&mut intervals);
    let mut network = intervals.sort();
}

/// Make data (as comparison)
fn data(size: usize){
    let mut intervals = make_data(size);
    //sort_vector(&mut intervals);
}


/// Benchmark with criterion
fn criterion_benchmark(c: &mut Criterion) {
    let f = 10000;
    println!("Benches run with vector of size 200000");
    c.bench_function("New implementation", |b| b.iter(|| new_sort(f)));
    c.bench_function("Default implementation", |b| b.iter(|| old_sort(f)));
    c.bench_function("Data generation solo", |b| b.iter(|| data(f)));

}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);