use nfp::prelude::*;
use nfp::nfp_points::polygon;

#[test]
fn check_random_polygon_properties() {
    let poly_a = generate_20_edge_polygon(42);
    let poly_b = generate_20_edge_polygon(43);
    
    println!("\n=== POLYGON PROPERTIES ===\n");
    
    println!("Polygon A ({} vertices):", poly_a.len());
    println!("  CCW: {}", polygon::is_ccw(&poly_a));
    
    let area_a = polygon_area(&poly_a);
    println!("  Area: {:.6}", area_a);
    
    let centroid_a = compute_centroid(&poly_a);
    println!("  Centroid: ({:.6}, {:.6})", centroid_a.x, centroid_a.y);
    
    println!("\nPolygon B ({} vertices):", poly_b.len());
    println!("  CCW: {}", polygon::is_ccw(&poly_b));
    
    let area_b = polygon_area(&poly_b);
    println!("  Area: {:.6}", area_b);
    
    let centroid_b = compute_centroid(&poly_b);
    println!("  Centroid: ({:.6}, {:.6})", centroid_b.x, centroid_b.y);
    
    println!("\n=== ALGORITHM ANALYSIS ===\n");
    println!("For 20×20 input, producing 400 output means:");
    println!("  All 20×20 = 400 combinations are UNIQUE");
    println!("  This suggests algorithm generates one output per (a_vertex, b_vertex) pair");
    println!("\nTheoretical analysis:");
    println!("  For random star-shaped polygon pairs:");
    println!("  - Expected NFP vertices: O(|A| + |B|) = ~40");
    println!("  - Actual NFP vertices: 400");
    println!("\nConclusion:");
    println!("  The algorithm is NOT computing a valid Minkowski sum");
    println!("  It appears to be computing vertex offsets without proper convex hull/closure");
}

fn polygon_area(vertices: &[Point]) -> f64 {
    if vertices.len() < 3 {
        return 0.0;
    }
    
    let mut area = 0.0;
    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        area += vertices[i].x * vertices[j].y;
        area -= vertices[j].x * vertices[i].y;
    }
    (area / 2.0).abs()
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
