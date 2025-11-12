use nfp::prelude::*;
use std::time::Instant;

/// Number of iterations to run the NFP calculation
const ITERATIONS: usize = 100;

fn main() {
    println!("NFP Benchmark");
    println!("=============");
    println!("Iterations: {}\n", ITERATIONS);

    // Generate two different 200-edge polygons with fixed seeds
    println!("Generating polygons...");
    let poly_a = nfp::utils::generate_ellipse_polygon(200, 100.0, 30.0, 3.0, 44);
    let poly_b = nfp::utils::generate_ellipse_polygon(200, 100.0, 30.0, 3.0, 45);
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
cargo bench --bench nfp_convex200

Iterations: 100

Generating polygons...
Polygon A: 200 vertices
Polygon B: 200 vertices

Running benchmark (100 iterations)...

Results:
--------
Total time:     31.409163293s
Time per run:   314091.633 μs
Runs per sec:   3
_______________________________________________
With new TOGO version
Total time:     3.033751272s
Time per run:   30337.513 μs
Runs per sec:   33
_______________________________________________
Using new TOGO version v0.6.11
Total time:     1.618352554s
Time per run:   16183.526 μs
Runs per sec:   62
_______________________________________________
*/
