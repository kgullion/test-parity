use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rand::{Rng, SeedableRng, rngs::StdRng};
use test_parity::full_parity as fp;
use test_parity::swap_parity as sp;

const SAMPLE_N: usize = 1000;
const RNG_SEED: u64 = 0x_5EED_C0DE;

macro_rules! add_full_bench {
    ($group:ident, $func:expr, $input:expr) => {
        $group.bench_function(stringify!($func), |bencher| {
            bencher.iter_with_large_drop(|| {
                $input
                    .0
                    .iter()
                    .flat_map(|lhs| {
                        $input
                            .1
                            .iter()
                            .map(|rhs| $func(*lhs, *rhs, $input.2, $input.3, $input.4))
                    })
                    .collect::<Vec<_>>()
            })
        });
    };
}
fn bench_full_parity(c: &mut Criterion, p: u32, q: u32) {
    let dims = p + q;
    let (pmask, qmask) = test_parity::clifford_masks(p, q);
    let metric_mask = pmask | qmask;

    let mut rng = StdRng::seed_from_u64(RNG_SEED);
    let lhs: Vec<_> = (0..SAMPLE_N)
        .map(|_| rng.gen_range(0..=metric_mask))
        .collect();
    let rhs: Vec<_> = (0..SAMPLE_N)
        .map(|_| rng.gen_range(0..=metric_mask))
        .collect();
    let input = black_box((lhs, rhs, pmask, qmask, dims));

    let mut group = c.benchmark_group(format!("Cl({},{})", p, q));

    // add_full_bench!(group, fp::naive_full, input);
    // add_full_bench!(group, fp::aap_full, input);
    // add_full_bench!(group, fp::fun_aap_full, input);
    add_full_bench!(group, fp::gerenuk_full, input);
    add_full_bench!(group, fp::gerenuk_late_a_rsh_full, input);
    add_full_bench!(group, fp::gerenuk_no_a_rsh_full, input);
    // add_full_bench!(group, fp::starfighter_full, input);
    add_full_bench!(group, fp::pixel_full, input);
    add_full_bench!(group, fp::pppt2_full, input);

    group.finish();
}

macro_rules! add_swap_bench {
    ($group:ident, $func:expr, $input:expr) => {
        $group.bench_function(stringify!($func), |bencher| {
            bencher.iter_with_large_drop(|| {
                $input
                    .0
                    .iter()
                    .flat_map(|lhs| {
                        $input
                            .1
                            .iter()
                            .map(|rhs| $func(*lhs, *rhs))
                    })
                    .collect::<Vec<_>>()
            })
        });
    };
}
fn bench_swap_parity(c: &mut Criterion) {
    let max = test_parity::Mask::MAX >> 1; // drop a bit since some algos don't work with full width
    let mut rng = StdRng::seed_from_u64(RNG_SEED);
    let lhs: Vec<_> = (0..SAMPLE_N)
        .map(|_| rng.gen_range(0..=max))
        .collect();
    let rhs: Vec<_> = (0..SAMPLE_N)
        .map(|_| rng.gen_range(0..=max))
        .collect();
    let input = black_box((lhs, rhs));

    let mut group = c.benchmark_group("SwapParity");

    // add_swap_bench!(group, sp::naive_swap, input);
    // add_swap_bench!(group, sp::aap_swap, input);
    // add_swap_bench!(group, sp::fun_aap_swap, input);
    add_swap_bench!(group, sp::gerenuk_swap, input);
    add_swap_bench!(group, sp::gerenuk_late_a_rsh_swap, input);
    add_swap_bench!(group, sp::gerenuk_no_a_rsh_swap, input);
    // add_swap_bench!(group, sp::starfighter_swap, input);
    add_swap_bench!(group, sp::pixel_swap, input);
    add_swap_bench!(group, sp::ppp2_swap, input);

    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_swap_parity(c);
    bench_full_parity(c, 3, 0);
    bench_full_parity(c, 4, 1);
    bench_full_parity(c, 16, 16);
    bench_full_parity(c, 32, 31);
    bench_full_parity(c, 0, 63);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
