use nfp::prelude::*;
use nfp::utils;
use std::time::Instant;

/// Number of iterations to run the NFP calculation
const ITERATIONS: usize = 1000;

fn main() {
    println!("NFP Benchmark");
    println!("=============");
    println!("Iterations: {}\n", ITERATIONS);

    // Generate two different 100-edge polygons with fixed seeds
    println!("Generating polygons...");
    let poly_a = nfp::utils::generate_ellipse_polygon(100, 50.0, 20.0, 2.0, 42);
    let poly_b = nfp::utils::generate_ellipse_polygon(100, 50.0, 20.0, 2.0, 43);
    println!("Polygon A: {} vertices", poly_a.len());
    println!("Polygon B: {} vertices", poly_b.len());

    // Run benchmark
    println!("\nRunning benchmark ({} iterations)...", ITERATIONS);

    // Warm up
    let _ = NFPConvex::nfp(&poly_a, &poly_b);
    
    let start = Instant::now();

    for _ in 0..ITERATIONS {
        let _ = NFPConvex::nfp(&poly_a, &poly_b);
    }

    let elapsed = start.elapsed();

    // Print results
    println!("\nResults:");
    println!("--------");
    println!("Total time:     {:?}", elapsed);
    println!("Time per run:   {:.3} μs", elapsed.as_secs_f64() * 1_000_000.0 / ITERATIONS as f64);
    println!("Runs per sec:   {:.0}", ITERATIONS as f64 / elapsed.as_secs_f64());
}

/*
cargo bench --bench nfp_convex100

Iterations: 1000

Generating polygons...
Polygon A: 100 vertices
Polygon B: 100 vertices

Running benchmark (1000 iterations)...

Results:
--------
Total time:     22.464362652s
Time per run:   22464.363 μs
Runs per sec:   45
_______________________________________________
Using new TOGO version v0.6.10
Total time:     4.618061378s
Time per run:   4618.061 μs
Runs per sec:   217
_______________________________________________
Using new TOGO version v0.6.11
Total time:     2.492149655s
Time per run:   2492.150 μs
Runs per sec:   401
_______________________________________________
Data changed so now the time increase:
Total time:     3.848276513s
Time per run:   3848.277 μs
Runs per sec:   260
_______________________________________________
New Convex Hull in TOGO
Total time:     1.71532328s
Time per run:   1715.323 μs
Runs per sec:   583
_______________________________________________
*/
