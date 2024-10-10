use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rusty_vector::{naive::NaiveIndex, point::VecPoint, Index};

pub fn criterion_benchmark(c: &mut Criterion) {
    const INDEX_SIZES: &[usize] = &[1000];
    const VECTOR_SIZES: &[usize] = &[5, 100];

    let mut g = c.benchmark_group("naive");
    for &index_size in INDEX_SIZES {
        for &vector_size in VECTOR_SIZES {
            g.bench_with_input(
                format!("index={}, vector={}", index_size, vector_size),
                &(index_size, vector_size),
                |b, &(index_size, vector_size)| {
                    b.iter(|| black_box(perform_index(index_size, vector_size, NaiveIndex::new())));
                },
            );
        }
    }
    g.finish();
}

fn perform_index(index_size: usize, vector_size: usize, mut index: impl Index<usize, VecPoint>) {
    for i in 0..index_size {
        let mut vector = VecPoint::new();
        vector.extend((0..vector_size).map(|j| (j as f64).sin() * (i as f64).cos()));
        index.insert(i, vector);
    }

    for i in 0..100 {
        let mut vector = VecPoint::new();
        vector.extend((0..vector_size).map(|j| (i + j) as f64));
        assert_eq!(index.find_keys(&vector, 5).len(), 5);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
