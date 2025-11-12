use nfp::prelude::*;
use std::time::Instant;

/// Number of iterations to run the NFP calculation
const ITERATIONS: usize = 20;

fn main() {
    println!("NFP Benchmark");
    println!("=============");
    println!("Iterations: {}\n", ITERATIONS);

    // Generate two different 500-edge polygons with fixed seeds
    println!("Generating polygons...");
    let poly_a = nfp::utils::generate_ellipse_polygon(500, 200.0, 50.0, 5.0, 46);
    let poly_b = nfp::utils::generate_ellipse_polygon(500, 200.0, 50.0, 5.0, 47);
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
cargo bench --bench nfp_convex500

Iterations: 20

Generating polygons...
Polygon A: 500 vertices
Polygon B: 500 vertices

Running benchmark (20 iterations)...

Results:
--------
Total time:     268.709642271s
Time per run:   13435482.114 μs
Runs per sec:   0
_______________________________________________
With new TOGO version
Total time:     5.782735538s
Time per run:   289136.777 μs
Runs per sec:   3
_______________________________________________
Using new TOGO version v0.6.11
Total time:     2.951319802s
Time per run:   147565.990 μs
Runs per sec:   7
_______________________________________________
*/