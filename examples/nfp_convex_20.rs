use nfp::prelude::*;
use togo::prelude::{self as togo_prelude, *};

fn main() {
    println!("NFP Example - 20 edge polygons");
    println!("==============================\n");

    // Generate two different 20-edge polygons with fixed seeds
    println!("Generating polygons...");
    let poly_a = nfp::utils::generate_ellipse_polygon(20, 20.0, 8.0, 1.0, 42);
    let poly_b = nfp::utils::generate_ellipse_polygon(20, 20.0, 8.0, 1.0, 43);
    
    println!("Polygon A: {} vertices", poly_a.len());
    println!("Polygon B: {} vertices\n", poly_b.len());

    println!("Computing NFP...");
    
    // Debug: Check polygon orientations
    println!("Polygon A is CCW: {}", nfp::utils::is_ccw(&poly_a));
    println!("Polygon B is CCW: {}", nfp::utils::is_ccw(&poly_b));
    
    // Print all A and B vertices
    println!("All vertices of Polygon A:");
    for (i, p) in poly_a.iter().enumerate() {
        println!("  A[{}]: ({:.2}, {:.2})", i, p.x, p.y);
    }
    println!("All vertices of Polygon B:");
    for (i, p) in poly_b.iter().enumerate() {
        println!("  B[{}]: ({:.2}, {:.2})", i, p.x, p.y);
    }
    println!();
    
    let result = NFPConvex::nfp(&poly_a, &poly_b).unwrap();
    println!("NFP result: {} vertices", result.len());
    println!("NFP result is CCW: {}", nfp::utils::is_ccw(&result));
    
    // Check if NFP is actually convex
    let is_convex_nfp = check_convexity(&result);
    println!("NFP is convex: {}", is_convex_nfp);
    println!();

    println!("Converting to arclines...");
    let arcline_a = nfp::nfp_convex::to_arcline(&poly_a);
    let arcline_b = nfp::nfp_convex::to_arcline(&poly_b);
    let nfpresult = nfp::nfp_convex::to_arcline(&result);
    
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
    
    // Debug: Print NFP vertices
    println!("NFP vertices:");
    for (i, p) in result.iter().enumerate() {
        println!("  NFP[{}]: ({:.2}, {:.2})", i, p.x, p.y);
    }
    
    // Debug: Print negated B polygon
    println!("\nNegated B polygon (what algo uses internally):");
    for (i, p) in poly_b.iter().enumerate() {
        let neg_p = nfp::point(-p.x, -p.y);
        println!("  -B[{}]: ({:.2}, {:.2})", i, neg_p.x, neg_p.y);
    }
    
    let trans = point(100.0, 100.0);
    let ttt = point(-16.0, -16.0);
    
    // Test: Check if B origin is inside NFP
    let b_origin = trans + ttt;
    let is_inside_nfp = point_in_polygon(b_origin, &result);
    println!("\nB origin position: ({:.2}, {:.2})", b_origin.x, b_origin.y);
    println!("Is B origin inside NFP? {}\n", is_inside_nfp);
    
    // DEBUG: Check actual collision at this position
    println!("=== COLLISION DEBUG ===");
    println!("A bounds: min=({:.2}, {:.2}), max=({:.2}, {:.2})", 
        poly_a.iter().map(|p| p.x).fold(f64::INFINITY, f64::min),
        poly_a.iter().map(|p| p.y).fold(f64::INFINITY, f64::min),
        poly_a.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max),
        poly_a.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max)
    );
    println!("B bounds (at origin): min=({:.2}, {:.2}), max=({:.2}, {:.2})",
        poly_b.iter().map(|p| p.x).fold(f64::INFINITY, f64::min),
        poly_b.iter().map(|p| p.y).fold(f64::INFINITY, f64::min),
        poly_b.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max),
        poly_b.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max)
    );
    
    let b_at_origin = b_origin;
    println!("B bounds (at {:?}): min=({:.2}, {:.2}), max=({:.2}, {:.2})",
        b_at_origin,
        b_at_origin.x + poly_b.iter().map(|p| p.x).fold(f64::INFINITY, f64::min),
        b_at_origin.y + poly_b.iter().map(|p| p.y).fold(f64::INFINITY, f64::min),
        b_at_origin.x + poly_b.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max),
        b_at_origin.y + poly_b.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max)
    );
    
    // Check if bounding boxes overlap (necessary but not sufficient for collision)
    let a_min_x = poly_a.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
    let a_max_x = poly_a.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
    let a_min_y = poly_a.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
    let a_max_y = poly_a.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
    
    let b_min_x = b_at_origin.x + poly_b.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
    let b_max_x = b_at_origin.x + poly_b.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
    let b_min_y = b_at_origin.y + poly_b.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
    let b_max_y = b_at_origin.y + poly_b.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
    
    let bbox_overlap_x = a_min_x <= b_max_x && a_max_x >= b_min_x;
    let bbox_overlap_y = a_min_y <= b_max_y && a_max_y >= b_min_y;
    let bbox_overlap = bbox_overlap_x && bbox_overlap_y;
    
    println!("Bounding boxes overlap? {}", bbox_overlap);
    println!("  X overlap: A[{:.2}, {:.2}] ∩ B[{:.2}, {:.2}] = {}", a_min_x, a_max_x, b_min_x, b_max_x, bbox_overlap_x);
    println!("  Y overlap: A[{:.2}, {:.2}] ∩ B[{:.2}, {:.2}] = {}", a_min_y, a_max_y, b_min_y, b_max_y, bbox_overlap_y);
    
    let mut svg = SVG::new(300.0, 300.0, Some("/tmp/nfp.svg"));
    let arcline_a = arcline_translate(&arcline_a, trans);
    let arcline_b = arcline_translate(&arcline_b, trans + ttt);
    let nfpresult = arcline_translate(&nfpresult, trans);
    
    // AFTER translation - check bounding boxes of translated polygons
    println!("\n=== AFTER TRANSLATION (in SVG coordinates) ===");
    let a_min_x_t = arcline_a.iter().map(|arc| arc.a.x.min(arc.b.x)).fold(f64::INFINITY, f64::min);
    let a_max_x_t = arcline_a.iter().map(|arc| arc.a.x.max(arc.b.x)).fold(f64::NEG_INFINITY, f64::max);
    let a_min_y_t = arcline_a.iter().map(|arc| arc.a.y.min(arc.b.y)).fold(f64::INFINITY, f64::min);
    let a_max_y_t = arcline_a.iter().map(|arc| arc.a.y.max(arc.b.y)).fold(f64::NEG_INFINITY, f64::max);
    
    let b_min_x_t = arcline_b.iter().map(|arc| arc.a.x.min(arc.b.x)).fold(f64::INFINITY, f64::min);
    let b_max_x_t = arcline_b.iter().map(|arc| arc.a.x.max(arc.b.x)).fold(f64::NEG_INFINITY, f64::max);
    let b_min_y_t = arcline_b.iter().map(|arc| arc.a.y.min(arc.b.y)).fold(f64::INFINITY, f64::min);
    let b_max_y_t = arcline_b.iter().map(|arc| arc.a.y.max(arc.b.y)).fold(f64::NEG_INFINITY, f64::max);
    
    println!("A bounds (translated): x[{:.2}, {:.2}], y[{:.2}, {:.2}]", a_min_x_t, a_max_x_t, a_min_y_t, a_max_y_t);
    println!("B bounds (translated): x[{:.2}, {:.2}], y[{:.2}, {:.2}]", b_min_x_t, b_max_x_t, b_min_y_t, b_max_y_t);
    
    let bbox_overlap_x_t = a_min_x_t <= b_max_x_t && a_max_x_t >= b_min_x_t;
    let bbox_overlap_y_t = a_min_y_t <= b_max_y_t && a_max_y_t >= b_min_y_t;
    println!("Bounding boxes overlap (translated)? X:{} Y:{} = {}", bbox_overlap_x_t, bbox_overlap_y_t, bbox_overlap_x_t && bbox_overlap_y_t);
    println!();


    svg.circle(&circle(trans, 1.0), "green");
    svg.circle(&circle(trans+ttt, 0.3), "violet");
    svg.arcline(&arcline_a, "red");
    svg.arcline(&arcline_b, "blue");
    svg.arcline(&nfpresult, "black");
    let a = arcline_a[0];
    let b = arcline_b[0];
    let n = nfpresult[0];
    svg.circle(&circle(a.a, 1.0), "red");
    svg.circle(&circle(b.a, 1.0), "blue");
    svg.circle(&circle(n.a, 1.0), "black");
    svg.write_stroke_width(0.1);
}

// Check if arcline has self-intersection using togo
fn arcline_has_self_intersection(arcline: &[togo_prelude::Arc]) -> bool {
    togo_prelude::arcline_has_self_intersection(&arcline.to_vec())
}

// Point-in-polygon test using ray casting algorithm
fn point_in_polygon(point: nfp::Point, polygon: &[nfp::Point]) -> bool {
    let mut inside = false;
    let px = point.x;
    let py = point.y;

    for i in 0..polygon.len() {
        let j = (i + 1) % polygon.len();
        let xi = polygon[i].x;
        let yi = polygon[i].y;
        let xj = polygon[j].x;
        let yj = polygon[j].y;

        let intersect = ((yi > py) != (yj > py))
            && (px < (xj - xi) * (py - yi) / (yj - yi) + xi);
        if intersect {
            inside = !inside;
        }
    }

    inside
}

// Check if polygon is convex using cross product
fn check_convexity(vertices: &[nfp::Point]) -> bool {
    if vertices.len() < 3 {
        return false;
    }

    let mut all_positive = true;
    let mut all_negative = true;

    for i in 0..vertices.len() {
        let v0 = &vertices[i];
        let v1 = &vertices[(i + 1) % vertices.len()];
        let v2 = &vertices[(i + 2) % vertices.len()];

        let dx1 = v1.x - v0.x;
        let dy1 = v1.y - v0.y;
        let dx2 = v2.x - v1.x;
        let dy2 = v2.y - v1.y;

        let cross = dx1 * dy2 - dy1 * dx2;

        if cross > 1e-10 {
            all_negative = false;
        }
        if cross < -1e-10 {
            all_positive = false;
        }
    }

    all_positive || all_negative
}
