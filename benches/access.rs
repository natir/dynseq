//! Benchmark access of dynseq

/* std use */

/* 3rd party use */
use biotest::Format as _;

/* project use */

pub fn bench(c: &mut criterion::Criterion) {
    let mut g = c.benchmark_group("get");

    let mut rng = biotest::rand();
    let generator = biotest::Sequence::builder()
        .sequence_len(100)
        .build()
        .unwrap();
    let mut seq = vec![];
    generator.record(&mut seq, &mut rng).unwrap();

    let dynseq = dynseq::DynSeq::new(&seq, 28).unwrap();

    for index in (0..100).step_by(10) {
        g.bench_with_input(
            criterion::BenchmarkId::new("dynseq_index", index),
            &index,
            |b, index| {
                b.iter(|| criterion::black_box(dynseq.get(*index)));
            },
        );

        g.bench_with_input(
            criterion::BenchmarkId::new("vec_index", index),
            &index,
            |b, index| {
                b.iter(|| criterion::black_box(seq.get(*index)));
            },
        );
    }
}
