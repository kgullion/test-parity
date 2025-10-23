use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rand::{Rng, SeedableRng, rngs::StdRng};
use test_parity::FrameMetric;

fn bench_parity(metric: &FrameMetric, c: &mut Criterion) {
    let mask = metric.supremum;
    // If the space is small, iterate exhaustively. Otherwise sample randomly.
    let max_exhaust = 1u64 << 16; // up to 65536 total pairs exhaustively
    let space_size = (mask as u64 + 1) * (mask as u64 + 1);

    let (lhs_values, rhs_values) = if false && space_size <= max_exhaust {
        (
            (0..=mask).collect::<Vec<u32>>(),
            (0..=mask).collect::<Vec<u32>>(),
        )
    } else {
        // sample N random pairs (lhs, rhs) using a seeded RNG for reproducibility
        let sample_n = 100;
        let mut rng = StdRng::seed_from_u64(0x_5EED_C0DE);
        let mut lhs = Vec::with_capacity(sample_n);
        let mut rhs = Vec::with_capacity(sample_n);
        for _ in 0..sample_n {
            lhs.push(rng.gen_range(0..=mask));
            rhs.push(rng.gen_range(0..=mask));
        }
        (lhs, rhs)
    };

    let mut group = c.benchmark_group(format!("parity/{}+{}", metric.positive, metric.negative));

    group.bench_function(BenchmarkId::new("mul_parity", "orig"), |b| {
        b.iter(|| {
            for &lhs in &lhs_values {
                for &rhs in &rhs_values {
                    black_box(metric.mul_parity(lhs, rhs));
                }
            }
        });
    });

    group.bench_function(BenchmarkId::new("aap_parity", "aap"), |b| {
        b.iter(|| {
            for &lhs in &lhs_values {
                for &rhs in &rhs_values {
                    black_box(metric.aap_parity(lhs, rhs));
                }
            }
        });
    });

    group.bench_function(BenchmarkId::new("fun_aap_parity", "fun"), |b| {
        b.iter(|| {
            for &lhs in &lhs_values {
                for &rhs in &rhs_values {
                    black_box(metric.fun_aap_parity(lhs, rhs));
                }
            }
        });
    });

    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    let metrics = [
        FrameMetric::new(3, 0),
        FrameMetric::new(4, 1),
        FrameMetric::new(16, 16),
        FrameMetric::new(32, 0),
        FrameMetric::new(0, 32),
    ];

    for metric in &metrics {
        bench_parity(metric, c);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
