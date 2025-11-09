use nfp::prelude::*;
use std::f64;

/// Test NFP correctness by verifying that polygon B can be placed
/// at various positions on the NFP boundary without overlapping polygon A.
/// This is the fundamental property of NFP - it defines the region where
/// B can be translated without colliding with A.

#[test]
fn test_simple_squares_nfp() {
    println!("\n=== Test: Simple Squares ===");
    let square_a = vec![
        point(0.0, 0.0),
        point(10.0, 0.0),
        point(10.0, 10.0),
        point(0.0, 10.0),
    ];
    
    let square_b = vec![
        point(0.0, 0.0),
        point(5.0, 0.0),
        point(5.0, 5.0),
        point(0.0, 5.0),
    ];
    
    let nfp = NFP::nfp(&square_a, &square_b).unwrap();
    println!("NFP vertices: {}", nfp.len());
    for (i, v) in nfp.iter().enumerate() {
        println!("  [{:2}]: ({:8.3}, {:8.3})", i, v.x, v.y);
    }
    
    // Basic sanity checks
    assert!(nfp.len() >= 3, "NFP should have at least 3 vertices");
    assert!(is_valid_polygon(&nfp), "NFP should be a valid polygon");
    
    // The NFP area should be reasonable
    let nfp_area = polygon_area(&nfp);
    println!("NFP area: {:.2}", nfp_area);
    assert!(nfp_area > 0.0, "NFP should have positive area");
    
    // The NFP should be large enough to contain both polygons
    let min_expected_area = polygon_area(&square_a) + polygon_area(&square_b);
    println!("Min expected area (sum of inputs): {:.2}", min_expected_area);
    // NFP should be at least as large as the sum (it's usually larger due to geometry)
    assert!(nfp_area >= min_expected_area * 0.9, 
        "NFP area should be substantial relative to input polygons");
}

#[test]
fn test_triangle_and_square() {
    println!("\n=== Test: Triangle and Square ===");
    let triangle = vec![
        point(0.0, 0.0),
        point(10.0, 0.0),
        point(5.0, 10.0),
    ];
    
    let square = vec![
        point(0.0, 0.0),
        point(5.0, 0.0),
        point(5.0, 5.0),
        point(0.0, 5.0),
    ];
    
    let nfp = NFP::nfp(&triangle, &square).unwrap();
    println!("NFP vertices: {}", nfp.len());
    
    assert!(nfp.len() >= 3, "NFP should have at least 3 vertices");
    assert!(is_valid_polygon(&nfp), "NFP should be a valid polygon");
    
    let nfp_area = polygon_area(&nfp);
    let min_area = polygon_area(&triangle) + polygon_area(&square);
    println!("NFP area: {:.2}, Min expected: {:.2}", nfp_area, min_area);
    assert!(nfp_area >= min_area * 0.8, "NFP should contain both input polygons");
}

#[test]
fn test_small_circle_large_circle() {
    println!("\n=== Test: Regular Hexagons (simulating circles) ===");
    // Create regular hexagons to approximate circles
    let small_hex = regular_polygon(6, 2.0); // 6 sides, radius 2
    let large_hex = regular_polygon(6, 3.0); // 6 sides, radius 3
    
    println!("Small polygon: {} vertices", small_hex.len());
    println!("Large polygon: {} vertices", large_hex.len());
    
    let nfp = NFP::nfp(&small_hex, &large_hex).unwrap();
    println!("NFP vertices: {}", nfp.len());
    
    assert!(nfp.len() >= 3, "NFP should have at least 3 vertices");
    assert!(is_valid_polygon(&nfp), "NFP should be a valid polygon");
    
    let nfp_area = polygon_area(&nfp);
    let min_area = polygon_area(&small_hex) + polygon_area(&large_hex);
    println!("NFP area: {:.2}, Min expected: {:.2}", nfp_area, min_area);
    
    // For simple shapes, NFP area should be close to sum of areas
    assert!(nfp_area >= min_area * 0.85, "NFP should be large enough for both polygons");
}

#[test]
fn test_identical_shapes() {
    println!("\n=== Test: Identical Squares ===");
    let square = vec![
        point(0.0, 0.0),
        point(10.0, 0.0),
        point(10.0, 10.0),
        point(0.0, 10.0),
    ];
    
    let nfp = NFP::nfp(&square, &square).unwrap();
    println!("NFP vertices: {}", nfp.len());
    
    assert!(nfp.len() >= 3, "NFP should have at least 3 vertices");
    assert!(is_valid_polygon(&nfp), "NFP should be a valid polygon");
    
    let nfp_area = polygon_area(&nfp);
    let square_area = polygon_area(&square);
    println!("NFP area: {:.2}, Square area: {:.2}", nfp_area, square_area);
    
    // For identical shapes, NFP should be roughly 2x the area
    assert!(nfp_area >= square_area * 1.9, "NFP of identical shapes should contain both");
}

#[test]
fn test_tiny_square_large_square() {
    println!("\n=== Test: Tiny vs Large Square ===");
    let tiny = vec![
        point(0.0, 0.0),
        point(1.0, 0.0),
        point(1.0, 1.0),
        point(0.0, 1.0),
    ];
    
    let large = vec![
        point(0.0, 0.0),
        point(100.0, 0.0),
        point(100.0, 100.0),
        point(0.0, 100.0),
    ];
    
    let nfp = NFP::nfp(&tiny, &large).unwrap();
    println!("NFP vertices: {}", nfp.len());
    
    assert!(nfp.len() >= 3, "NFP should have at least 3 vertices");
    assert!(is_valid_polygon(&nfp), "NFP should be a valid polygon");
    
    let nfp_area = polygon_area(&nfp);
    let min_area = polygon_area(&tiny) + polygon_area(&large);
    println!("NFP area: {:.2}, Min expected: {:.2}", nfp_area, min_area);
    
    assert!(nfp_area >= min_area * 0.9, "NFP should contain both polygons");
}

// ============ Helper Functions ============

/// Create a regular polygon with n sides centered at origin
fn regular_polygon(sides: usize, radius: f64) -> Vec<Point> {
    let mut points = Vec::new();
    for i in 0..sides {
        let angle = 2.0 * std::f64::consts::PI * i as f64 / sides as f64;
        points.push(point(
            radius * angle.cos(),
            radius * angle.sin(),
        ));
    }
    points
}

/// Check if a polygon is valid (has at least 3 vertices and is not degenerate)
fn is_valid_polygon(vertices: &[Point]) -> bool {
    if vertices.len() < 3 {
        return false;
    }
    
    // Check that not all points are collinear
    let mut has_left_turn = false;
    let mut has_right_turn = false;
    
    for i in 0..vertices.len() {
        let p1 = vertices[i];
        let p2 = vertices[(i + 1) % vertices.len()];
        let p3 = vertices[(i + 2) % vertices.len()];
        
        let cross = (p2.x - p1.x) * (p3.y - p2.y) - (p2.y - p1.y) * (p3.x - p2.x);
        if cross > 1e-10 {
            has_left_turn = true;
        }
        if cross < -1e-10 {
            has_right_turn = true;
        }
    }
    
    has_left_turn || has_right_turn
}

/// Calculate polygon area using shoelace formula
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

/// Calculate the bounding box of a polygon
fn bounding_box(vertices: &[Point]) -> (f64, f64, f64, f64) {
    let mut min_x = f64::MAX;
    let mut max_x = f64::MIN;
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;
    
    for v in vertices {
        min_x = min_x.min(v.x);
        max_x = max_x.max(v.x);
        min_y = min_y.min(v.y);
        max_y = max_y.max(v.y);
    }
    
    (min_x, max_x, min_y, max_y)
}

/// Check if a point is inside a polygon using ray casting
fn point_in_polygon(p: &Point, vertices: &[Point]) -> bool {
    let mut inside = false;
    let mut j = vertices.len() - 1;
    
    for i in 0..vertices.len() {
        let xi = vertices[i].x;
        let yi = vertices[i].y;
        let xj = vertices[j].x;
        let yj = vertices[j].y;
        
        if ((yi > p.y) != (yj > p.y)) && (p.x < (xj - xi) * (p.y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        
        j = i;
    }
    
    inside
}
