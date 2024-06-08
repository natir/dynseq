//! Benchmark creation of dynseq

/* std use */

/* 3rd party use */
use biotest::Format as _;

/* project use */

pub fn bench(c: &mut criterion::Criterion) {
    let mut g = c.benchmark_group("create");

    let mut rng = biotest::rand();
    let generator = biotest::Sequence::builder()
        .sequence_len(2usize.pow(14))
        .build()
        .unwrap();
    let mut seq = vec![];
    generator.record(&mut seq, &mut rng).unwrap();

    for node_len in (2..29).step_by(2) {
        g.bench_with_input(
            criterion::BenchmarkId::new("node_len", node_len),
            &seq,
            |b, seq| {
                b.iter(|| criterion::black_box(dynseq::DynSeq::new(seq, node_len).unwrap()));
            },
        );
    }

    for exp in 1..15 {
        let seq_len: usize = 2usize.pow(exp);
        let sub_seq = &seq[..seq_len];
        g.bench_with_input(
            criterion::BenchmarkId::new("seq_len/dynseq", seq_len),
            &sub_seq,
            |b, sub_seq| {
                b.iter(|| criterion::black_box(dynseq::DynSeq::new(sub_seq, 28).unwrap()));
            },
        );
        g.bench_with_input(
            criterion::BenchmarkId::new("seq_len/vec", seq_len),
            &sub_seq,
            |b, sub_seq| {
                b.iter(|| criterion::black_box(sub_seq.to_vec()));
            },
        );
    }
}
