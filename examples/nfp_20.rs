use nfp::prelude::*;
use togo::prelude::{self as togo_prelude, *};

fn main() {
    println!("NFP Example - 20 edge polygons");
    println!("==============================\n");

    // Generate two different 20-edge polygons with fixed seeds
    println!("Generating polygons...");
    let poly_a = generate_20_edge_polygon(42);
    let poly_b = generate_20_edge_polygon(43);
    
    println!("Polygon A: {} vertices", poly_a.len());
    println!("Polygon B: {} vertices\n", poly_b.len());

    println!("Computing NFP...");
    
    // Debug: Check polygon orientations
    println!("Polygon A is CCW: {}", is_ccw(&poly_a));
    println!("Polygon B is CCW: {}", is_ccw(&poly_b));
    
    // Print first few vertices
    println!("First 3 vertices of Polygon A:");
    for (i, p) in poly_a.iter().take(3).enumerate() {
        println!("  A[{}]: ({:.2}, {:.2})", i, p.x, p.y);
    }
    println!("First 3 vertices of Polygon B:");
    for (i, p) in poly_b.iter().take(3).enumerate() {
        println!("  B[{}]: ({:.2}, {:.2})", i, p.x, p.y);
    }
    
    let result = NFP::nfp(&poly_a, &poly_b).unwrap();
    println!("NFP result: {} vertices", result.len());
    println!("NFP result is CCW: {}", is_ccw(&result));
    println!();

    println!("Converting to arclines...");
    let arcline_a = to_arcline(&poly_a);
    let arcline_b = to_arcline(&poly_b);
    let nfpresult = to_arcline(&result);
    
    println!("Arcline A: {} arcs", arcline_a.len());
    println!("Arcline B: {} arcs", arcline_b.len());
    println!("Arcline NFP: {} arcs", nfpresult.len());
    
    // Check for self-intersections
    let has_self_int_a = arcline_has_self_intersection(&arcline_a);
    let has_self_int_b = arcline_has_self_intersection(&arcline_b);
    let has_self_int_nfp = arcline_has_self_intersection(&nfpresult);
    
    println!("Arcline A has self-intersection: {}", has_self_int_a);
    println!("Arcline B has self-intersection: {}", has_self_int_b);
    println!("NFP result has self-intersection: {}", has_self_int_nfp);
    println!();
    
    let mut svg = SVG::new(300.0, 300.0, Some("/tmp/nfp.svg"));
    let arcline_a = arcline_translate(&arcline_a, togo_prelude::point(100.0, 100.0));
    let arcline_b = arcline_translate(&arcline_b, togo_prelude::point(100.0, 100.0));
    let nfpresult = arcline_translate(&nfpresult, togo_prelude::point(100.0, 100.0));
    svg.arcline(&arcline_a, "red");
    svg.arcline(&arcline_b, "blue");
    svg.arcline(&nfpresult, "black");
    svg.write_stroke_width(0.1);
}

fn generate_20_edge_polygon(seed: u64) -> Vec<nfp::Point> {
    use std::f64::consts::PI;

    let mut points = Vec::new();
    let mut rng_state = seed;

    // Simple LCG (Linear Congruential Generator) for reproducible randomness
    let lcg_next = |state: &mut u64| {
        *state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        (*state >> 32) as f32 as f64 / (u32::MAX as f64)
    };

    for _ in 0..20 {
        let angle = lcg_next(&mut rng_state) * 2.0 * PI;
        let radius = 20.0 * (0.5 + lcg_next(&mut rng_state) * 1.5);
        points.push(nfp::point(radius * angle.cos(), radius * angle.sin()));
    }

    // Sort by angle from centroid to ensure CCW ordering (using nfp's optimized comparator)
    let centroid = compute_centroid(&points);
    points.sort_unstable_by(|a, b| {
        angle_cmp(a, b, &centroid)
    });

    points
}

fn compute_centroid(points: &[nfp::Point]) -> nfp::Point {
    if points.is_empty() {
        return nfp::point(0.0, 0.0);
    }

    let sum_x: f64 = points.iter().map(|p| p.x).sum();
    let sum_y: f64 = points.iter().map(|p| p.y).sum();
    let len = points.len() as f64;

    nfp::point(sum_x / len, sum_y / len)
}

// Check if polygon is CCW using shoelace formula
fn is_ccw(vertices: &[nfp::Point]) -> bool {
    if vertices.len() < 3 {
        return false;
    }

    let mut sum = 0.0;
    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        let vi = &vertices[i];
        let vj = &vertices[j];
        sum += (vj.x - vi.x) * (vj.y + vi.y);
    }
    sum < 0.0
}

// Compare points by angle around a given centroid without using atan2
// This is the same algorithm used internally by NFP
fn angle_cmp(a: &nfp::Point, b: &nfp::Point, centroid: &nfp::Point) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    let ax = a.x - centroid.x;
    let ay = a.y - centroid.y;
    let bx = b.x - centroid.x;
    let by = b.y - centroid.y;

    // Place points into two half-planes: upper (y>0 or y==0 && x>=0) and lower
    let a_up = (ay > 0.0) || (ay == 0.0 && ax >= 0.0);
    let b_up = (by > 0.0) || (by == 0.0 && bx >= 0.0);
    if a_up != b_up {
        return a_up.cmp(&b_up).reverse();
    }

    // Same half-plane: use perp (2D cross product) to determine order
    let perp = ax * by - ay * bx;
    if perp.abs() > 1e-10 {
        return if perp > 0.0 { Ordering::Less } else { Ordering::Greater };
    }

    // Collinear: sort by distance from centroid
    let da = ax * ax + ay * ay;
    let db = bx * bx + by * by;
    da.partial_cmp(&db).unwrap_or(Ordering::Equal)
}

// Check if arcline has self-intersection using togo
fn arcline_has_self_intersection(arcline: &[togo_prelude::Arc]) -> bool {
    togo_prelude::arcline_has_self_intersection(&arcline.to_vec())
}
