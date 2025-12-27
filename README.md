# test_parity [(report)](https://kgullion.github.io/test-parity/report/)

Small Rust playground for testing and benchmarking different bit-level algorithms that compute the **sign/parity** arising in the geometric product of blade bitmasks.

The crate currently has two related modules:

- **Swap parity**: `swap_parity::*` computes the parity of swaps needed to merge two basis elements (bitmasks).
  - Signature: `fn(a: Mask, b: Mask) -> bool`.
- **Full parity**: `full_parity::*` computes swap parity plus the effect of negative basis vectors (Cl(p,q) metric sign).
  - Signature: `fn(a: Mask, b: Mask, pmask: Mask, qmask: Mask, dims: u32) -> bool`.

`Mask` is `u64` (see `src/lib.rs`).

## Running tests

Property tests (via `proptest`) check that each implementation matches the reference `naive_*` implementation.

- Run all tests:
  - `cargo test`

Note: Some implementations use x86_64 intrinsics (e.g. `pclmulqdq`) and are called unconditionally in wrappers like `pixel_*` / `ppp2_*`. On machines that **don’t** support the required CPU features, you may get an illegal-instruction crash (SIGILL).

## Running benchmarks

Benchmarks use Criterion (see `benches/parity_bench.rs`).

Run all benches: `cargo bench`

Open the HTML report: `target/criterion/report/index.html`

The benchmark groups include:
- `SwapParity` for the swap-only modules
- `Cl(p,q)` for several (p,q) signatures for the full modules

## Adding a new implementation

### 1) Add the function

Add your function to the appropriate module:

- Swap-only: `src/swap_parity.rs`
  - Must match: `pub fn my_algo_swap(a: Mask, b: Mask) -> bool`
- Full parity: `src/full_parity.rs`
  - Must match: `pub fn my_algo_full(a: Mask, b: Mask, pmask: Mask, qmask: Mask, dims: u32) -> bool`

Prefer keeping the signature exactly the same so it can be plugged into tests and the Criterion harness easily.

### 2) Add it to the property tests

In the module’s `#[cfg(test)]` section, compare your implementation to `naive_*`:

- Swap parity: the test uses `let np = naive_swap(a, b);` and then `prop_assert_eq!(np, my_algo_swap(a, b));`
- Full parity: the test computes `(pmask, qmask, dims)` from `p` and then compares to `naive_full(...)`

This is the main correctness gate.

### 3) Add it to the benchmarks

Wire it into `benches/parity_bench.rs`:

- Swap parity: add `add_swap_bench!(group, sp::my_algo_swap, input);`
- Full parity: add `add_full_bench!(group, fp::my_algo_full, input);`

The benchmark name comes from `stringify!($func)`, so keeping clear function names helps interpret the report.

### 4) If you use CPU-specific instructions

If you add an implementation that requires a CPU feature (e.g. PCLMULQDQ, AVX2, etc.), prefer a safe wrapper that checks at runtime before calling the `#[target_feature]` function.

Typical pattern on x86_64:

- `#[target_feature(enable = "...")] unsafe fn kernel(...) -> ...`
- `pub fn wrapper(...) -> ... { if is_x86_feature_detected!("...") { unsafe { kernel(...) } } else { fallback(...) } }`

This avoids SIGILL on unsupported machines.
