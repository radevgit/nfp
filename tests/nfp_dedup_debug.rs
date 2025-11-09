use nfp::prelude::*;

#[test]
fn debug_nfp_20_polygon_deduplication() {
    // Generate same 20-edge polygons as nfp_20 example
    let poly_a = generate_20_edge_polygon(42);
    let poly_b = generate_20_edge_polygon(43);
    
    println!("\nPolygon A ({} vertices):", poly_a.len());
    for (i, p) in poly_a.iter().enumerate().take(3) {
        println!("  A[{}]: ({:.6}, {:.6})", i, p.x, p.y);
    }
    
    println!("Polygon B ({} vertices):", poly_b.len());
    for (i, p) in poly_b.iter().enumerate().take(3) {
        println!("  B[{}]: ({:.6}, {:.6})", i, p.x, p.y);
    }
    
    let result = NFP::nfp(&poly_a, &poly_b).unwrap();
    println!("\nNFP result: {} vertices (expected < 50, got 20*20=400)", result.len());
    
    // Print first 10 NFP vertices to see if they're actually different
    println!("\nFirst 10 NFP vertices:");
    for (i, p) in result.iter().enumerate().take(10) {
        println!("  NFP[{}]: ({:.10}, {:.10})", i, p.x, p.y);
    }
    
    // Check for near-duplicates manually
    let tolerance_1e10 = 1e-10;
    let tolerance_1e6 = 1e-6;
    let tolerance_1e4 = 1e-4;
    let tolerance_1e2 = 1e-2;
    
    let mut count_dups_1e10 = 0;
    let mut count_dups_1e6 = 0;
    let mut count_dups_1e4 = 0;
    let mut count_dups_1e2 = 0;
    
    for i in 0..result.len() {
        for j in (i+1)..result.len() {
            let dx = (result[i].x - result[j].x).abs();
            let dy = (result[i].y - result[j].y).abs();
            
            if dx < tolerance_1e10 && dy < tolerance_1e10 {
                count_dups_1e10 += 1;
            }
            if dx < tolerance_1e6 && dy < tolerance_1e6 {
                count_dups_1e6 += 1;
            }
            if dx < tolerance_1e4 && dy < tolerance_1e4 {
                count_dups_1e4 += 1;
            }
            if dx < tolerance_1e2 && dy < tolerance_1e2 {
                count_dups_1e2 += 1;
            }
        }
    }
    
    println!("\nDuplicate pairs (after angle sort):");
    println!("  Tolerance 1e-10: {} pairs", count_dups_1e10);
    println!("  Tolerance 1e-6:  {} pairs", count_dups_1e6);
    println!("  Tolerance 1e-4:  {} pairs", count_dups_1e4);
    println!("  Tolerance 1e-2:  {} pairs", count_dups_1e2);
    
    println!("\nExpected if properly deduplicated: ~20-40 vertices, got {}", result.len());
}

fn generate_20_edge_polygon(seed: u64) -> Vec<Point> {
    use std::f64::consts::PI;

    let mut points = Vec::new();
    let mut rng_state = seed;

    let lcg_next = |state: &mut u64| {
        *state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        (*state >> 32) as f32 as f64 / (u32::MAX as f64)
    };

    for _ in 0..20 {
        let angle = lcg_next(&mut rng_state) * 2.0 * PI;
        let radius = 20.0 * (0.5 + lcg_next(&mut rng_state) * 1.5);
        points.push(point(radius * angle.cos(), radius * angle.sin()));
    }

    // Simple angle sort - CCW
    let centroid = compute_centroid(&points);
    points.sort_unstable_by(|a, b| {
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
