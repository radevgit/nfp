//! # NFP Library - No Fit Polygon Computation
//! 
//! A Rust library for calculating No Fit Polygon (NFP) for shape nesting and packing problems.
//! 
//! ## What is NFP?
//! 
//! The No Fit Polygon is a computational geometry tool used in 2D nesting and packing optimization.
//! It defines the forbidden region where one polygon cannot be placed relative to another without
//! collision. When the origin of polygon B is outside the NFP, the two polygons do not overlap.
//! 
//! ## Current Implementation
//! 
//! This library provides **NFP for convex polygons** using the Minkowski sum approach:
//! 
//! 1. **Algorithm**: Minkowski Sum via vertex combinations and convex hull
//! 2. **Input**: Two convex polygons represented as `Vec<Point>` (counter-clockwise orientation)
//! 3. **Output**: NFP boundary as a convex polygon (counter-clockwise)
//! 4. **Complexity**: O(n² log n) where n is the total number of vertices
//! 
//! ### How it Works
//! 
//! The algorithm computes the Minkowski sum A ⊕ (-B):
//! - For each vertex in A and each vertex in -B, compute their sum
//! - Compute the convex hull of all resulting points
//! - The convex hull boundary is the NFP
//! 
//! ## Examples
//! 
//! ### Basic Usage with Simple Polygons
//! 
//! ```rust
//! use nfp::{point, NFPConvex};
//! 
//! // Triangle A at origin
//! let triangle_a = vec![
//!     point(0.0, 0.0),
//!     point(10.0, 0.0),
//!     point(5.0, 10.0),
//! ];
//! 
//! // Triangle B at origin
//! let triangle_b = vec![
//!     point(0.0, 0.0),
//!     point(5.0, 0.0),
//!     point(2.5, 5.0),
//! ];
//! 
//! // Calculate NFP
//! let nfp = NFPConvex::nfp(&triangle_a, &triangle_b).unwrap();
//! println!("NFP has {} vertices", nfp.len());
//! ```
//! 
//! ### Collision Detection Using NFP
//! 
//! ```rust
//! use nfp::{point, NFPConvex};
//! 
//! // Define two squares
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
//! // Calculate NFP
//! let nfp = NFPConvex::nfp(&square_a, &square_b).unwrap();
//! 
//! // To check if placing B at position (px, py) collides with A:
//! // If B's origin at (px, py) is OUTSIDE the NFP, there is NO collision
//! // If B's origin at (px, py) is INSIDE the NFP, there IS a collision
//! let test_position = point(12.0, 12.0);
//! println!("Position valid for placement: {}", 
//!          !point_in_polygon(test_position, &nfp));
//! 
//! fn point_in_polygon(pt: nfp::prelude::Point, polygon: &[nfp::prelude::Point]) -> bool {
//!     let mut inside = false;
//!     let mut p1 = polygon[polygon.len() - 1];
//!     for p2 in polygon {
//!         if ((p2.y > pt.y) != (p1.y > pt.y))
//!             && (pt.x < (p1.x - p2.x) * (pt.y - p2.y) / (p1.y - p2.y) + p2.x)
//!         {
//!             inside = !inside;
//!         }
//!         p1 = *p2;
//!     }
//!     inside
//! }
//! ```
//! 
//! ## Key Properties
//! 
//! - **Convexity**: NFP of two convex polygons is convex
//! - **Orientation**: NFP is always counter-clockwise (CCW)
//! - **No Self-Intersections**: NFP boundary has no crossing edges
//! - **Minkowski Sum**: NFP(A, B) = A ⊕ (-B)
//! 
//! ## Requirements
//! 
//! - Both input polygons must be **convex**
//! - Both input polygons must have at least **3 vertices**
//! - Polygons should be in **counter-clockwise (CCW) orientation**
//!   (the library will auto-correct if needed)
//! 
//! ## Performance
//! 
//! For typical use cases:
//! - **10-20 vertices**: < 1 μs
//! - **100 vertices**: ~25 μs  
//! - **200 vertices**: ~90 μs
//! - **500 vertices**: ~280 μs

pub mod utils;
pub mod nfp_convex;
pub mod nfp_validation;
pub mod prelude;

mod nfp_tests;

// Re-export main public API
pub use togo::prelude::Point;
pub use togo::prelude::point;
pub use nfp_convex::{NFPConvex, NfpError};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let point = Point { x: 1.0, y: 2.0 };
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
    }
}