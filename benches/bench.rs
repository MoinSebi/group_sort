use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use group_sort::{group_sort, group_sort2, group_sort3};

/// Create new vector - size * 20 (shuffled)
///
pub fn make_data(size: usize) -> Vec<usize>{
    let mut ff = Vec::new();
    for x in 0..40 {
        for y in 0..size{
            ff.push(y)
        }
    }
    //ff.shuffle(&mut thread_rng());
    return ff
}

/// Group_sort (+ make new data)
fn new_sort(size: usize){
    let mut intervals = make_data(size);
    //sort_vector(&mut intervals);
    let mut network = group_sort(intervals);
}

/// Group_sort2 (+ make new data)
fn new_sort2(size: usize){
    let mut intervals = make_data(size);
    //sort_vector(&mut intervals);
    let mut network = group_sort2(intervals);
}

/// Group_sort3 (+ make new data)
fn new_sort3(size: usize){
    let mut intervals = make_data(size);
    //sort_vector(&mut intervals);
    let mut network = group_sort3(intervals);
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
    c.bench_function("New implementation1", |b| b.iter(|| new_sort(f)));
    c.bench_function("New implementation2", |b| b.iter(|| new_sort2(f)));
    c.bench_function("New implementation3", |b| b.iter(|| new_sort3(f)));
    c.bench_function("Default implementation", |b| b.iter(|| old_sort(f)));
    c.bench_function("Data generation solo", |b| b.iter(|| data(f)));

}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);