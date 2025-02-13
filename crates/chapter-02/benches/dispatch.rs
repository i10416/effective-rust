use chapter_02::dynamic_dispatch;
use chapter_02::static_dispatch;
use chapter_02::Rect;
use chapter_02::RectInlinable;
use criterion::{criterion_group, criterion_main, Criterion};

const ITERATION: i32 = 100;

fn bench_dyn_area(c: &mut Criterion) {
    c.bench_function("bench_dyn_area", |b| {
        b.iter(|| {
            std::hint::black_box(for _ in 1..=ITERATION {
                dynamic_dispatch(&Rect {
                    width: 10,
                    height: 10,
                });
            });
        });
    });
}

fn bench_dyn_area_inlineable(c: &mut Criterion) {
    c.bench_function("bench_dyn_area_inlineable", |b| {
        b.iter(|| {
            std::hint::black_box(for _ in 1..=ITERATION {
                dynamic_dispatch(&RectInlinable {
                    width: 10,
                    height: 10,
                });
            });
        });
    });
}



fn bench_static_area(c: &mut Criterion) {
    c.bench_function("bench_static_area", |b| {
        b.iter(|| {
            std::hint::black_box(for _ in 1..=ITERATION {
                static_dispatch(&Rect {
                    width: 10,
                    height: 10,
                });
            });
        });
    });
}

fn bench_static_area_inlineable(c: &mut Criterion) {
    c.bench_function("bench_static_area_inlineable", |b| {
        b.iter(|| {
            std::hint::black_box(for _ in 1..=ITERATION {
                static_dispatch(&RectInlinable {
                    width: 10,
                    height: 10,
                });
            });
        });
    });
}



criterion_group!(
    benches,
    bench_dyn_area,
    bench_dyn_area_inlineable,
    bench_static_area,
    bench_static_area_inlineable
);
criterion_main!(benches);
