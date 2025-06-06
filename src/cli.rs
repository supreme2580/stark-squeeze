// benches/compression_bench.rs
// Benchmark suite for StarkSqueeze compression algorithms using Criterion.
//
// Run with:
//     cargo bench --bench compression_bench
//
// Add the following to the workspace root `Cargo.toml` under [dev-dependencies]:
// criterion = "^0.5"
// rand = "^0.8"
//
// Make sure each compression algorithm exposes `compress` and `decompress` functions
// that take `&[u8]` and return a `Vec<u8>`.
//
// ----------------------------
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{rngs::StdRng, Rng, SeedableRng};

use stark_squeeze::algorithms::{
    brotli::{compress as brotli_compress, decompress as brotli_decompress},
    lz4::{compress as lz4_compress, decompress as lz4_decompress},
    snappy::{compress as snappy_compress, decompress as snappy_decompress},
    // ⚠️  Extend this list with additional algorithms you add to the library.
};

/// Convenience wrapper to treat an algorithm pair uniformly in the benches
struct Algo<'a> {
    name: &'static str,
    compress: &'a dyn Fn(&[u8]) -> Vec<u8>,
    decompress: &'a dyn Fn(&[u8]) -> Vec<u8>,
}

/// Generates deterministic pseudo‑random data of `size` bytes so that every
/// benchmark run is reproducible. The fixed seed ensures identical input across
/// CI runners and local machines.
fn generate_data(size: usize) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(42);
    (0..size).map(|_| rng.gen()).collect()
}

fn bench_compression(c: &mut Criterion) {
    // Register each algorithm here. Add new ones as you implement them.
    let algorithms = vec![
        Algo {
            name: "lz4",
            compress: &lz4_compress,
            decompress: &lz4_decompress,
        },
        Algo {
            name: "snappy",
            compress: &snappy_compress,
            decompress: &snappy_decompress,
        },
        Algo {
            name: "brotli",
            compress: &brotli_compress,
            decompress: &brotli_decompress,
        },
    ];

    // Data sizes that approximate common StarkNet storage payloads.
    let sizes = [
        ("1KB", 1 * 1024usize),
        ("100KB", 100 * 1024),
        ("1MB", 1 * 1024 * 1024),
    ];

    // ── Benchmark loop ────────────────────────────────────────────────────────
    for (label, size) in sizes.iter() {
        let data = generate_data(*size);

        for algo in algorithms.iter() {
            let mut group = c.benchmark_group(format!("{}-{}", algo.name, label));
            group.throughput(Throughput::Bytes(*size as u64));

            // Compress benchmark
            group.bench_function(BenchmarkId::new("compress", algo.name), |b| {
                b.iter(|| {
                    (algo.compress)(black_box(&data));
                })
            });

            // Prepare a compressed buffer once for the decompression bench so
            // that we only measure the decompression stage.
            let compressed = (algo.compress)(&data);

            // Decompress benchmark
            group.bench_function(BenchmarkId::new("decompress", algo.name), |b| {
                b.iter(|| {
                    (algo.decompress)(black_box(&compressed));
                })
            });

            // Optional: compute and print compression ratio once per group
            let ratio = (compressed.len() as f64 / *size as f64) * 100.0;
            group.bench_with_input(BenchmarkId::new("ratio", algo.name), &ratio, |b, &r| {
                b.iter(|| black_box(r))
            });

            group.finish();
        }
    }
}

criterion_group!(compression_benches, bench_compression);
criterion_main!(compression_benches);

// ----------------------------
// 📄  README snippet ──────────────────────────────────────────────────────────
// After adding this file you can run the benches locally:
//   $ cargo bench --bench compression_bench
//
// CI integration example (GitHub Actions):
//
// ```yaml
// name: benches
// on: [push, pull_request]
// jobs:
//   criterion:
//     runs-on: ubuntu-latest
//     steps:
//       - uses: actions/checkout@v4
//       - name: Install stable toolchain
//         uses: dtolnay/rust-toolchain@stable
//       - name: Run benches
//         run: cargo bench --bench compression_bench -- --output-format bencher | tee bench.txt
//       - name: Upload artifact
//         uses: actions/upload-artifact@v4
//         with:
//           name: bench-results
//           path: bench.txt
// ```
//
// The `--output-format bencher` flag lets you plug the results into tools like
// criterion‑plot or compare‑bench, so you can surface regressions early.
// ---------------------------------------------------------------------------
