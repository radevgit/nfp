//! Validation tests for NFP correctness
//! 
//! These tests verify that the NFP actually represents valid no-fit positions
//! by checking the fundamental property: a point in the NFP means B cannot 
//! be placed there without overlapping A.

use crate::nfp_points::Point;

/// Check if point P is strictly inside polygon (not on boundary)
fn point_strictly_inside_polygon(p: &Point, poly: &[Point]) -> bool {
    if poly.len() < 3 {
        return false;
    }

    let tolerance = 1e-6;
    
    // First check if point is on the boundary - if so, it's not strictly inside
    for i in 0..poly.len() {
        let p1 = poly[i];
        let p2 = poly[(i + 1) % poly.len()];
        
        // Vector from p1 to p2
        let edge_x = p2.x - p1.x;
        let edge_y = p2.y - p1.y;
        let edge_len_sq = edge_x * edge_x + edge_y * edge_y;
        
        if edge_len_sq > tolerance * tolerance {
            // Vector from p1 to p
            let to_p_x = p.x - p1.x;
            let to_p_y = p.y - p1.y;
            
            // Project p onto the edge
            let t = (to_p_x * edge_x + to_p_y * edge_y) / edge_len_sq;
            
            if t >= -tolerance && t <= 1.0 + tolerance {
                // p's projection is within the segment - check perpendicular distance
                let closest_x = p1.x + t * edge_x;
                let closest_y = p1.y + t * edge_y;
                let dist_sq = (p.x - closest_x) * (p.x - closest_x) + (p.y - closest_y) * (p.y - closest_y);
                
                if dist_sq < tolerance * tolerance {
                    return false; // Point is on boundary
                }
            }
        }
    }

    // Use ray casting to check if strictly inside
    let mut inside = false;
    let mut p1 = poly[poly.len() - 1];

    for &p2 in poly {
        if (p2.y > p.y) != (p1.y > p.y)
            && p.x < (p1.x - p2.x) * (p.y - p2.y) / (p1.y - p2.y) + p2.x
        {
            inside = !inside;
        }
        p1 = p2;
    }

    inside
}

/// Check if two polygons overlap (have common interior points)
fn polygons_overlap(a: &[Point], b: &[Point]) -> bool {
    let tolerance = 1e-9;

    // Check if any vertex of B is inside A
    for &b_v in b {
        if point_strictly_inside_polygon(&b_v, a) {
            return true;
        }
    }

    // Check if any vertex of A is inside B
    for &a_v in a {
        if point_strictly_inside_polygon(&a_v, b) {
            return true;
        }
    }

    // Check if any edges intersect
    for i in 0..a.len() {
        let a1 = a[i];
        let a2 = a[(i + 1) % a.len()];

        for j in 0..b.len() {
            let b1 = b[j];
            let b2 = b[(j + 1) % b.len()];

            // Check if segments [a1,a2] and [b1,b2] intersect
            if segments_intersect(&a1, &a2, &b1, &b2, tolerance) {
                return true;
            }
        }
    }

    false
}

/// Check if two line segments intersect (proper intersection, not just touching)
fn segments_intersect(p1: &Point, p2: &Point, p3: &Point, p4: &Point, tolerance: f64) -> bool {
    let d1 = ccw(p3, p1, p2);
    let d2 = ccw(p4, p1, p2);
    let d3 = ccw(p1, p3, p4);
    let d4 = ccw(p2, p3, p4);

    let has_opposite_sign = |a: f64, b: f64| -> bool { a * b < -tolerance };

    has_opposite_sign(d1, d2) && has_opposite_sign(d3, d4)
}

/// CCW test: return signed area * 2
fn ccw(a: &Point, b: &Point, c: &Point) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

/// Translate polygon by offset
fn translate_polygon(poly: &[Point], offset: &Point) -> Vec<Point> {
    poly.iter().map(|p| p.add(offset)).collect()
}

/// Validate that an NFP point truly represents a no-fit position
/// 
/// For a point to be valid in the NFP:
/// 1. If we place B at this position, B should NOT overlap A
/// 2. The point should be on the boundary of the no-fit region
pub fn validate_nfp_point(nfp_point: &Point, a: &[Point], b: &[Point]) -> bool {
    // Translate B by the NFP point offset
    let b_translated = translate_polygon(b, nfp_point);

    // B should NOT overlap with A at this position
    !polygons_overlap(a, &b_translated)
}

/// Validate entire NFP: all points should represent valid no-fit positions
pub fn validate_nfp(nfp: &[Point], a: &[Point], b: &[Point]) -> (bool, String) {
    for (idx, &nfp_point) in nfp.iter().enumerate() {
        if !validate_nfp_point(&nfp_point, a, b) {
            return (
                false,
                format!("NFP point {} at ({:.3}, {:.3}) is invalid - B overlaps A at this position",
                    idx, nfp_point.x, nfp_point.y)
            );
        }
    }

    (true, "NFP is valid".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nfp_points::point;

    #[test]
    fn test_nfp_validation_simple_rectangles() {
        let rect_a = vec![
            point(0.0, 0.0),
            point(2.0, 0.0),
            point(2.0, 1.0),
            point(0.0, 1.0),
        ];

        let rect_b = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 0.5),
            point(0.0, 0.5),
        ];

        // Use the TRUE Luo & Rao algorithm
        let nfp_result = crate::nfp_luo_rao::compute_nfp_luo_rao(&rect_a, &rect_b);
        let (valid, _) = validate_nfp(&nfp_result, &rect_a, &rect_b);
        
        assert!(valid, "NFP should be valid for simple rectangles");
    }

    #[test]
    fn test_nfp_validation_triangles() {
        let tri_a = vec![
            point(0.0, 0.0),
            point(3.0, 0.0),
            point(1.5, 2.0),
        ];

        let tri_b = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(0.5, 1.0),
        ];

        let nfp_result = crate::nfp_luo_rao::compute_nfp_luo_rao(&tri_a, &tri_b);
        println!("Triangle NFP vertices:");
        for (i, &p) in nfp_result.iter().enumerate() {
            println!("  {}: ({:.3}, {:.3})", i, p.x, p.y);
        }
        
        let (valid, _) = validate_nfp(&nfp_result, &tri_a, &tri_b);
        
        assert!(valid, "NFP should be valid for triangles");
    }

    #[test]
    fn test_nfp_validation_l_shapes() {
        let l_shape = vec![
            point(0.0, 0.0),
            point(2.0, 0.0),
            point(2.0, 1.0),
            point(1.0, 1.0),
            point(1.0, 2.0),
            point(0.0, 2.0),
        ];

        let small_rect = vec![
            point(0.0, 0.0),
            point(0.5, 0.0),
            point(0.5, 0.5),
            point(0.0, 0.5),
        ];

        let nfp_result = crate::nfp_luo_rao::compute_nfp_luo_rao(&l_shape, &small_rect);
        let (valid, _) = validate_nfp(&nfp_result, &l_shape, &small_rect);
        
        assert!(valid, "NFP should be valid for L-shapes");
    }
}
