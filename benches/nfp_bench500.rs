use nfp::prelude::*;
use std::time::Instant;

/// Number of iterations to run the NFP calculation
const ITERATIONS: usize = 100;

fn main() {
    println!("NFP Benchmark");
    println!("=============");
    println!("Iterations: {}\n", ITERATIONS);

    // Generate two different 500-edge polygons with fixed seeds
    println!("Generating polygons...");
    let poly_a = generate_500_edge_polygon(46);
    let poly_b = generate_500_edge_polygon(47);
    println!("Polygon A: {} vertices", poly_a.len());
    println!("Polygon B: {} vertices", poly_b.len());

    // Warm up
    let _ = NFP::nfp(&poly_a, &poly_b);

    // Run benchmark
    println!("\nRunning benchmark ({} iterations)...", ITERATIONS);
    let start = Instant::now();

    for _ in 0..ITERATIONS {
        let _ = NFP::nfp(&poly_a, &poly_b);
    }

    let elapsed = start.elapsed();

    // Print results
    println!("\nResults:");
    println!("--------");
    println!("Total time:     {:?}", elapsed);
    println!("Time per run:   {:.3} μs", elapsed.as_secs_f64() * 1_000_000.0 / ITERATIONS as f64);
    println!("Runs per sec:   {:.0}", ITERATIONS as f64 / elapsed.as_secs_f64());
}

/// Generate a 500-edge polygon using a fixed seed via bit manipulation
/// to avoid runtime dependency on DataGen
fn generate_500_edge_polygon(seed: u64) -> Vec<Point> {
    use std::f64::consts::PI;

    let mut points = Vec::new();
    let mut rng_state = seed;

    // Simple LCG (Linear Congruential Generator) for reproducible randomness
    let lcg_next = |state: &mut u64| {
        *state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        (*state >> 32) as f32 as f64 / (u32::MAX as f64)
    };

    // Generate 500 points in polar coordinates
    for _ in 0..500 {
        let angle = lcg_next(&mut rng_state) * 2.0 * PI;
        let radius = 0.5 + lcg_next(&mut rng_state) * 1.5;
        points.push(point(radius * angle.cos(), radius * angle.sin()));
    }

    // Sort by angle from centroid to ensure CCW ordering
    let centroid = compute_centroid(&points);
    points.sort_by(|a, b| {
        let angle_a = (a.y - centroid.y).atan2(a.x - centroid.x);
        let angle_b = (b.y - centroid.y).atan2(b.x - centroid.x);
        angle_a.partial_cmp(&angle_b).unwrap_or(std::cmp::Ordering::Equal)
    });

    points
}

fn compute_centroid(points: &[Point]) -> Point {
    if points.is_empty() {
        return point(0.0, 0.0);
    }

    let sum_x: f64 = points.iter().map(|p| p.x).sum();
    let sum_y: f64 = points.iter().map(|p| p.y).sum();
    let len = points.len() as f64;

    point(sum_x / len, sum_y / len)
}

/*
cargo bench --bench nfp_bench500

Iterations: 100

Generating polygons...
Polygon A: 500 vertices
Polygon B: 500 vertices

Running benchmark (100 iterations)...

Results:
--------
Total time:     13.578851406s
Time per run:   135.789 ms
Runs per sec:   7
_______________________________________________
Opt 1 ditch atan2

Total time:     6.140549308s
Time per run:   61405.493 μs
Runs per sec:   16
_______________________________________________
Opt2 unstable sorting

Total time:     4.848619896s
Time per run:   48486.199 μs
Runs per sec:   21
_______________________________________________
*/