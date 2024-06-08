//! Benchmark insert of dynseq

/* std use */

/* 3rd party use */
use biotest::Format as _;

/* project use */

pub fn bench(c: &mut criterion::Criterion) {
    let mut g = c.benchmark_group("insert");

    let mut rng = biotest::rand();
    let generator = biotest::Sequence::builder()
        .sequence_len(2usize.pow(14))
        .build()
        .unwrap();
    let mut seq = vec![];
    generator.record(&mut seq, &mut rng).unwrap();

    for exp in 1..15 {
        let index = 2usize.pow(exp);
        g.bench_with_input(
            criterion::BenchmarkId::new("vec_insert", index),
            &index,
            |b, index| {
                b.iter(|| seq.insert(*index, b'N'));
            },
        );
    }
}
