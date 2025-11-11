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
    let poly_a = generate_200_edge_polygon(44);
    let poly_b = generate_200_edge_polygon(45);
    println!("Polygon A: {} vertices", poly_a.len());
    println!("Polygon B: {} vertices", poly_b.len());

    // Warm up
    let _ = NFPConvex::nfp(&poly_a, &poly_b);

    // Run benchmark
    println!("\nRunning benchmark ({} iterations)...", ITERATIONS);
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

/// Generate a convex polygon with ~200 vertices using an elongated parametric shape
fn generate_200_edge_polygon(seed: u64) -> Vec<Point> {
    let mut points = Vec::new();
    let steps = 200;
    
    // Generate points along an elongated ellipse with perturbation for variety
    for i in 0..steps {
        let t = 2.0 * std::f64::consts::PI * (i as f64) / (steps as f64);
        
        // Elongated ellipse: a=100 (major axis), b=30 (minor axis)
        let a = 100.0;
        let b = 30.0;
        
        // Add slight perturbation based on seed for variety
        let perturbation = ((seed.wrapping_mul(i as u64).wrapping_add(54321) % 1000) as f64 / 1000.0) * 3.0;
        let radius = (a * a * (t.sin() * t.sin()) + b * b * (t.cos() * t.cos())).sqrt() + perturbation;
        
        let x = radius * t.cos();
        let y = radius * t.sin();
        points.push(point(x, y));
    }
    
    points
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
*/
