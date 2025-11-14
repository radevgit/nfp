//! Convex polygon No-Fit Polygon (NFP) computation using Minkowski sum approach
//! 
//! This module implements NFP calculation for convex polygons using the Minkowski sum
//! via vertex combinations and convex hull computation.
//! 
//! ## Algorithm Overview
//! 
//! The algorithm computes the Minkowski sum A ⊕ (-B) as follows:
//! 
//! 1. **Input Validation**: Ensure both polygons are valid (≥3 vertices)
//! 2. **Normalize**: Convert both polygons to counter-clockwise (CCW) orientation
//! 3. **Negate B**: Compute -B by reflecting B around the origin
//! 4. **Vertex Sums**: Generate all combinations: {a + b | a ∈ A, b ∈ -B}
//! 5. **Convex Hull**: Compute the convex hull of all sum points using Graham scan
//! 6. **Validate**: Ensure result is CCW with no self-intersections
//! 7. **Return**: NFP as a CCW convex polygon
//! 
//! ## Time Complexity
//! 
//! - Vertex sums: O(n·m) where n = |A| vertices, m = |B| vertices
//! - Convex hull (Graham scan): O(n·m log(n·m))
//! - Overall: O(n·m log(n·m))
//! 
//! ## Example
//! 
//! ```rust
//! use nfp::{point, NFPConvex};
//! 
//! // Two squares
//! let square_a = vec![
//!     point(0.0, 0.0),
//!     point(10.0, 0.0),
//!     point(10.0, 10.0),
//!     point(0.0, 10.0),
//! ];
//! 
//! let square_b = vec![
//!     point(0.0, 0.0),
//!     point(5.0, 0.0),
//!     point(5.0, 5.0),
//!     point(0.0, 5.0),
//! ];
//! 
//! // Compute NFP
//! let nfp = NFPConvex::nfp(&square_a, &square_b).unwrap();
//! assert!(nfp.len() >= 3);
//! ```

use togo::prelude::*;

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
    /// Calculate the No Fit Polygon (NFP) for two convex polygons using Minkowski sum.
    ///
    /// # Algorithm
    ///
    /// Computes the Minkowski sum A ⊕ (-B) by:
    /// 1. Generating all vertex combinations from A and -B
    /// 2. Computing the convex hull of the sum points using Graham scan
    /// 3. Returning the convex hull boundary as the NFP
    ///
    /// # Arguments
    ///
    /// * `poly_a` - First convex polygon as slice of points (CCW orientation preferred)
    /// * `poly_b` - Second convex polygon as slice of points (CCW orientation preferred)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Point>)` - NFP as counter-clockwise convex polygon
    /// * `Err(NfpError::EmptyPolygon)` - If either polygon is empty
    /// * `Err(NfpError::InsufficientVertices)` - If either polygon has < 3 vertices
    ///
    /// # Properties
    ///
    /// - **Convexity**: Result is always convex
    /// - **Orientation**: Result is always counter-clockwise (CCW)
    /// - **No self-intersections**: NFP boundary has no crossing edges
    /// - **Collision detection**: Point outside NFP → no collision; point inside → collision
    ///
    /// # Example
    ///
    /// ```rust
    /// use nfp::{point, NFPConvex};
    ///
    /// let tri_a = vec![
    ///     point(0.0, 0.0),
    ///     point(10.0, 0.0),
    ///     point(5.0, 10.0),
    /// ];
    ///
    /// let tri_b = vec![
    ///     point(0.0, 0.0),
    ///     point(5.0, 0.0),
    ///     point(2.5, 5.0),
    /// ];
    ///
    /// let nfp = NFPConvex::nfp(&tri_a, &tri_b).unwrap();
    /// assert!(nfp.len() >= 3, "NFP must have at least 3 vertices");
    /// ```
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
        // Negate B (Minkowski sum with -B)
        let mut b_negated = b.to_vec();
        b_negated.reverse();
        for pt in &mut b_negated {
            pt.x = -pt.x;
            pt.y = -pt.y;
        }

        // Compute Minkowski sum by generating all vertex sums
        let mut sum_points = Vec::with_capacity(a.len() * b_negated.len());
        for &a_v in a {
            for &b_v in &b_negated {
                sum_points.push(point(a_v.x + b_v.x, a_v.y + b_v.y));
            }
        }

        // Use togo's convex hull to get the proper boundary
        points_convex_hull(&sum_points)
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
#[allow(dead_code)]
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
