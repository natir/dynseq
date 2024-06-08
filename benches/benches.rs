//! Aggregate benchmark

/* std use */

/* 3rd party use */
use criterion;

/* mod declaration */
mod access;
mod create;
mod insert;

/* project use */

fn setup(c: &mut criterion::Criterion) {
    create::bench(c);
    access::bench(c);
    insert::bench(c);
}

criterion::criterion_group!(benches, setup);

criterion::criterion_main!(benches);
