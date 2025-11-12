use nfp::prelude::*;
use std::time::Instant;

/// Number of iterations to run the NFP calculation
const ITERATIONS: usize = 500;

fn main() {
    println!("NFP Example - 200 edge polygons");
    println!("================================");
    println!("Iterations: {}\n", ITERATIONS);

    // Generate two different 200-edge polygons with fixed seeds
    println!("Generating polygons...");
    let poly_a = nfp::utils::generate_ellipse_polygon(200, 100.0, 30.0, 3.0, 42);
    let poly_b = nfp::utils::generate_ellipse_polygon(200, 100.0, 30.0, 3.0, 43);
    println!("Polygon A: {} vertices", poly_a.len());
    println!("Polygon B: {} vertices", poly_b.len());


    // Run benchmark
    println!("\nRunning ({} iterations)...", ITERATIONS);
    let start = Instant::now();

    for _ in 0..ITERATIONS {
        let _ = NFPConvex::nfp(&poly_a, &poly_b);
    }

    let elapsed = start.elapsed();

    // Print results
    println!("\nResults:");
    println!("--------");
    println!("Total time:     {:?}", elapsed);
    println!("Time per run:   {:.3} ms", elapsed.as_secs_f64() * 1000.0 / ITERATIONS as f64);
    println!("Runs per sec:   {:.0}", ITERATIONS as f64 / elapsed.as_secs_f64());
}
