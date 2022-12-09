use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use group_sort::{group_sort};

/// Size * 20 vector (shuffled)
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

fn new_sort(size: usize){
    let mut intervals = make_data(size);
    //sort_vector(&mut intervals);
    let mut network = group_sort(intervals);
}

fn old_sort(size: usize){
    let mut intervals = make_data(size);
    //sort_vector(&mut intervals);
    let mut network = intervals.sort();
}

fn data(size: usize){
    let mut intervals = make_data(size);
    //sort_vector(&mut intervals);
}

fn criterion_benchmark(c: &mut Criterion) {
    let f = 10000;
    c.bench_function("New implementation", |b| b.iter(|| new_sort(f)));
    c.bench_function("Default implementation", |b| b.iter(|| old_sort(f)));
    c.bench_function("Data generation solo", |b| b.iter(|| data(f)));

}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);