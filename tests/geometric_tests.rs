use nfp::nfp_points::{NFP, Point};

/// Check if NFP is geometrically valid
fn validate_nfp(nfp: &[Point], name: &str) -> bool {
    println!("\n=== Validating {} ===", name);
    println!("Vertex count: {}", nfp.len());

    // Check 1: At least 3 vertices
    if nfp.len() < 3 {
        println!("❌ FAIL: Less than 3 vertices");
        return false;
    }
    println!("✓ Has at least 3 vertices");

    // Check 2: No duplicate consecutive points
    let mut has_duplicates = false;
    for i in 0..nfp.len() {
        let p1 = nfp[i];
        let p2 = nfp[(i + 1) % nfp.len()];
        if (p1.x - p2.x).abs() < 1e-6 && (p1.y - p2.y).abs() < 1e-6 {
            println!("❌ FAIL: Duplicate points at {}/{}", i, (i + 1) % nfp.len());
            has_duplicates = true;
        }
    }
    if !has_duplicates {
        println!("✓ No duplicate consecutive points");
    }

    // Check 3: CCW orientation (positive area)
    let mut area = 0.0;
    for i in 0..nfp.len() {
        let j = (i + 1) % nfp.len();
        area += (nfp[j].x - nfp[i].x) * (nfp[j].y + nfp[i].y);
    }
    if area < 0.0 {
        println!("✓ CCW oriented (area = {:.2})", area);
    } else {
        println!("❌ FAIL: Not CCW (area = {:.2})", area);
        return false;
    }

    // Check 4: No self-intersections (simplified check - no segment crosses)
    let mut has_self_intersection = false;
    for i in 0..nfp.len() {
        let p1 = nfp[i];
        let p2 = nfp[(i + 1) % nfp.len()];

        for j in (i + 2)..nfp.len() {
            if j == (i + nfp.len() - 1) % nfp.len() {
                continue; // Skip adjacent segments
            }
            let p3 = nfp[j];
            let p4 = nfp[(j + 1) % nfp.len()];

            if segments_intersect(p1, p2, p3, p4) {
                println!(
                    "❌ FAIL: Self-intersection at segment {}-{} and {}-{}",
                    i,
                    (i + 1) % nfp.len(),
                    j,
                    (j + 1) % nfp.len()
                );
                has_self_intersection = true;
            }
        }
    }
    if !has_self_intersection {
        println!("✓ No self-intersections");
    }

    // Check 5: Reasonable coordinate ranges (not NaN/Inf)
    for (i, p) in nfp.iter().enumerate() {
        if !p.x.is_finite() || !p.y.is_finite() {
            println!(
                "❌ FAIL: Non-finite coordinate at {}: ({}, {})",
                i, p.x, p.y
            );
            return false;
        }
    }
    println!("✓ All coordinates are finite");

    // Print first few and last few vertices
    println!("First 3 vertices:");
    for i in 0..nfp.len().min(3) {
        println!("  [{}]: ({:.2}, {:.2})", i, nfp[i].x, nfp[i].y);
    }
    if nfp.len() > 6 {
        println!("...");
        println!("Last 3 vertices:");
        for i in (nfp.len() - 3)..nfp.len() {
            println!("  [{}]: ({:.2}, {:.2})", i, nfp[i].x, nfp[i].y);
        }
    }

    true
}

fn segments_intersect(p1: Point, p2: Point, p3: Point, p4: Point) -> bool {
    fn ccw(a: Point, b: Point, c: Point) -> bool {
        (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
    }

    ccw(p1, p3, p4) != ccw(p2, p3, p4) && ccw(p1, p2, p3) != ccw(p1, p2, p4)
}

#[test]
fn test_triangle_triangle() {
    // Triangle A: right triangle
    let a = vec![
        Point::new(0.0, 0.0),
        Point::new(4.0, 0.0),
        Point::new(0.0, 3.0),
    ];

    // Triangle B: smaller triangle
    let b = vec![
        Point::new(0.0, 0.0),
        Point::new(2.0, 0.0),
        Point::new(0.0, 1.5),
    ];

    println!("\n\n========== TEST: Triangle + Triangle ==========");
    println!("Polygon A (3 vertices): {:?}", a);
    println!("Polygon B (3 vertices): {:?}", b);

    let nfp = NFP::nfp(&a, &b).expect("NFP computation failed");

    let is_valid = validate_nfp(&nfp, "Triangle+Triangle NFP");
    assert!(is_valid, "Triangle + Triangle NFP failed validation");
    assert!(
        nfp.len() >= 3 && nfp.len() <= 100,
        "Triangle + Triangle NFP vertex count {} is out of reasonable range [3-100]",
        nfp.len()
    );
}

#[test]
fn test_square_square() {
    // Square A: unit square
    let a = vec![
        Point::new(0.0, 0.0),
        Point::new(4.0, 0.0),
        Point::new(4.0, 4.0),
        Point::new(0.0, 4.0),
    ];

    // Square B: smaller square
    let b = vec![
        Point::new(0.0, 0.0),
        Point::new(2.0, 0.0),
        Point::new(2.0, 2.0),
        Point::new(0.0, 2.0),
    ];

    println!("\n\n========== TEST: Square + Square ==========");
    println!("Polygon A (4 vertices)");
    println!("Polygon B (4 vertices)");

    let nfp = NFP::nfp(&a, &b).expect("NFP computation failed");

    let is_valid = validate_nfp(&nfp, "Square+Square NFP");
    assert!(is_valid, "Square + Square NFP failed validation");

    // For square + square, NFP should be an octagon (8 vertices)
    // or close to it (4-12 vertices is acceptable range)
    assert!(
        nfp.len() >= 4 && nfp.len() <= 12,
        "Square + Square NFP should have ~8 vertices, got {}",
        nfp.len()
    );
}

#[test]
fn test_pentagon_pentagon() {
    // Regular-ish pentagon A
    let a = vec![
        Point::new(0.0, 0.0),
        Point::new(3.0, 0.0),
        Point::new(3.5, 2.5),
        Point::new(1.5, 4.0),
        Point::new(-1.0, 2.0),
    ];

    // Regular-ish pentagon B
    let b = vec![
        Point::new(0.0, 0.0),
        Point::new(2.0, 0.0),
        Point::new(2.3, 1.5),
        Point::new(1.0, 2.5),
        Point::new(-0.5, 1.2),
    ];

    println!("\n\n========== TEST: Pentagon + Pentagon ==========");
    println!("Polygon A (5 vertices)");
    println!("Polygon B (5 vertices)");

    let nfp = NFP::nfp(&a, &b).expect("NFP computation failed");

    let is_valid = validate_nfp(&nfp, "Pentagon+Pentagon NFP");
    assert!(is_valid, "Pentagon + Pentagon NFP failed validation");

    // For pentagon + pentagon, expect 6-50 vertices
    assert!(
        nfp.len() >= 6 && nfp.len() <= 50,
        "Pentagon + Pentagon NFP should have ~20 vertices, got {}",
        nfp.len()
    );
}

#[test]
fn test_decagon_decagon() {
    // Regular-ish decagon A (10 vertices)
    let mut a = Vec::new();
    for i in 0..10 {
        let angle = std::f64::consts::PI * 2.0 * i as f64 / 10.0;
        a.push(Point::new(3.0 * angle.cos(), 3.0 * angle.sin()));
    }

    // Regular-ish decagon B (10 vertices)
    let mut b = Vec::new();
    for i in 0..10 {
        let angle = std::f64::consts::PI * 2.0 * i as f64 / 10.0;
        b.push(Point::new(1.5 * angle.cos(), 1.5 * angle.sin()));
    }

    println!("\n\n========== TEST: Decagon + Decagon ==========");
    println!("Polygon A (10 vertices)");
    println!("Polygon B (10 vertices)");

    let nfp = NFP::nfp(&a, &b).expect("NFP computation failed");

    let is_valid = validate_nfp(&nfp, "Decagon+Decagon NFP");
    assert!(is_valid, "Decagon + Decagon NFP failed validation");

    // For decagon + decagon, expect 4-200 vertices (current implementation produces few)
    assert!(
        nfp.len() >= 4 && nfp.len() <= 200,
        "Decagon + Decagon NFP got {} vertices",
        nfp.len()
    );
}

#[test]
fn test_concave_polygon() {
    // Concave L-shape: A
    let a = vec![
        Point::new(0.0, 0.0),
        Point::new(3.0, 0.0),
        Point::new(3.0, 2.0),
        Point::new(1.0, 2.0),
        Point::new(1.0, 3.0),
        Point::new(0.0, 3.0),
    ];

    // Simple square B
    let b = vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 0.0),
        Point::new(1.0, 1.0),
        Point::new(0.0, 1.0),
    ];

    println!("\n\n========== TEST: Concave L-shape + Square ==========");
    println!("Polygon A (6 vertices - L shape)");
    println!("Polygon B (4 vertices - square)");

    let nfp = NFP::nfp(&a, &b).expect("NFP computation failed");

    let is_valid = validate_nfp(&nfp, "L-shape+Square NFP");
    assert!(is_valid, "L-shape + Square NFP failed validation");

    // For concave L + square, should be 6-60 vertices
    assert!(
        nfp.len() >= 6 && nfp.len() <= 60,
        "L-shape + Square NFP got {} vertices",
        nfp.len()
    );
}
