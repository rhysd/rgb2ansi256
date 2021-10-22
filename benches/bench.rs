use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn rgb2ansi256_vs_ansi_colours(c: &mut Criterion) {
    let mut data = vec![];
    for r in (0..=255).step_by(10) {
        for g in (3..=255).step_by(10) {
            for b in (6..=255).step_by(10) {
                data.push((r, g, b));
            }
        }
    }

    c.bench_function("rgb2ansi256", |b| {
        b.iter(|| {
            for rgb in data.iter().copied() {
                black_box(rgb2ansi256::rgb_to_ansi256(rgb.0, rgb.1, rgb.2));
            }
        })
    });
    c.bench_function("ansi_colours", |b| {
        b.iter(|| {
            for rgb in data.iter().copied() {
                black_box(ansi_colours::ansi256_from_rgb(rgb));
            }
        })
    });
}

criterion_group!(bench, rgb2ansi256_vs_ansi_colours);
criterion_main!(bench);
