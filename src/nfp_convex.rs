//! Convex polygon No-Fit Polygon (NFP) computation using Minkowski sum approach
//! 
//! This module implements NFP calculation for convex polygons using edge merging.
//! The algorithm combines edges from both polygons, sorts them by angle, and traces
//! the boundary to produce the NFP.

use togo::prelude::Point;
use crate::utils;

/// Error type for NFP operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NfpError {
    /// One or both polygons are empty
    EmptyPolygon,
    /// One or both polygons have fewer than 3 vertices
    InsufficientVertices,
}

impl std::fmt::Display for NfpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NfpError::EmptyPolygon => write!(f, "Polygons cannot be empty"),
            NfpError::InsufficientVertices => write!(f, "Polygons must have at least 3 vertices"),
        }
    }
}

impl std::error::Error for NfpError {}

/// NFP calculator for convex polygons using Minkowski sum (edge merging method)
pub struct NFPConvex;

impl NFPConvex {
    /// Calculate the No Fit Polygon (NFP) for two convex polygons
    /// using Minkowski Sum (edge merging method).
    ///
    /// This implementation works for convex polygons by combining edges from both
    /// polygons, sorting them by angle, and tracing the boundary to produce the NFP.
    ///
    /// The algorithm:
    /// 1. Reflects polygon B around the origin
    /// 2. Collects all edges from A and reflected B
    /// 3. Sorts edges by angle
    /// 4. Traces the boundary starting from the sum of minimum points
    /// 5. Returns the NFP boundary as a CCW polygon
    ///
    /// # Arguments
    /// * `poly_a` - First convex polygon (as slice of points)
    /// * `poly_b` - Second convex polygon (as slice of points)
    ///
    /// # Returns
    /// * `Ok(Vec<Point>)` - NFP as a counter-clockwise polygon
    /// * `Err(NfpError)` - If polygons are empty or have insufficient vertices
    pub fn nfp(poly_a: &[Point], poly_b: &[Point]) -> Result<Vec<Point>, NfpError> {
        if poly_a.is_empty() || poly_b.is_empty() {
            return Err(NfpError::EmptyPolygon);
        }

        if poly_a.len() < 3 || poly_b.len() < 3 {
            return Err(NfpError::InsufficientVertices);
        }

        Ok(Self::compute(poly_a, poly_b))
    }

    /// Internal computation function
    fn compute(a: &[Point], b: &[Point]) -> Vec<Point> {
        // Ensure both polygons are in CCW order
        let mut a_ccw = a.to_vec();
        let mut b_ccw = b.to_vec();
        utils::ensure_ccw(&mut a_ccw);
        utils::ensure_ccw(&mut b_ccw);

        // Negate B (Minkowski sum with -B)
        let mut b_negated = b_ccw.clone();
        b_negated.reverse();
        for pt in &mut b_negated {
            pt.x = -pt.x;
            pt.y = -pt.y;
        }

        // Compute Minkowski sum by generating all vertex sums
        let mut sum_points = Vec::new();
        for &a_v in &a_ccw {
            for &b_v in &b_negated {
                sum_points.push(togo::prelude::point(a_v.x + b_v.x, a_v.y + b_v.y));
            }
        }

        // Use togo's convex hull to get the proper boundary
        use togo::algo::pointline_convex_hull;
        let sum_points_togo: Vec<togo::prelude::Point> = sum_points;
        let hull_togo = pointline_convex_hull(&sum_points_togo);

        // Convert back to our Point type
        let mut hull: Vec<Point> = hull_togo
            .iter()
            .map(|p| Point { x: p.x, y: p.y })
            .collect();

        // Ensure CCW
        utils::ensure_ccw(&mut hull);

        hull
    }
}

/// Convert a polygon (Vec<Point>) to an Arcline (Vec<Arc>)
/// 
/// Each consecutive pair of points is connected by a straight line segment (arc).
/// The polygon is assumed to be closed, so the last point connects back to the first.
///
/// # Arguments
/// * `vertices` - A slice of points representing the NFP polygon
///
/// # Returns
/// A Vec of togo::Arc objects representing the polygon as line segments
pub fn to_arcline(vertices: &[Point]) -> Vec<togo::prelude::Arc> {
    use togo::prelude::*;
    
    let mut arcline: Vec<Arc> = Vec::new();
    
    if vertices.is_empty() {
        return arcline;
    }
    
    // Connect each point to the next, wrapping around to close the polygon
    for i in 0..vertices.len() {
        let current = vertices[i];
        let next = vertices[(i + 1) % vertices.len()];
        
        let p1 = point(current.x, current.y);
        let p2 = point(next.x, next.y);
        
        let arc = arcseg(p1, p2);
        arcline.push(arc);
    }
    
    arcline
}
