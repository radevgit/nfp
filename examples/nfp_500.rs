use nfp::prelude::*;
use std::time::Instant;

/// Number of iterations to run the NFP calculation
const ITERATIONS: usize = 100;

fn main() {
    println!("NFP Example - 500 edge polygons");
    println!("================================");
    println!("Iterations: {}\n", ITERATIONS);

    // Generate two different 500-edge polygons with fixed seeds
    println!("Generating polygons...");
    let poly_a = generate_500_edge_polygon(42);
    let poly_b = generate_500_edge_polygon(43);
    println!("Polygon A: {} vertices", poly_a.len());
    println!("Polygon B: {} vertices", poly_b.len());

    // Warm up
    let _ = NFP::nfp(&poly_a, &poly_b);

    // Run benchmark
    println!("\nRunning ({} iterations)...", ITERATIONS);
    let start = Instant::now();

    for _ in 0..ITERATIONS {
        let _ = NFP::nfp(&poly_a, &poly_b);
    }

    let elapsed = start.elapsed();

    // Print results
    println!("\nResults:");
    println!("--------");
    println!("Total time:     {:?}", elapsed);
    println!("Time per run:   {:.3} ms", elapsed.as_secs_f64() * 1000.0 / ITERATIONS as f64);
    println!("Runs per sec:   {:.0}", ITERATIONS as f64 / elapsed.as_secs_f64());
}

/// Generate a 500-edge polygon using a fixed seed via bit manipulation
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
