//! # NFP Library
//! 
//! A Rust library crate that implements No Fit Polygon (NFP) algorithms.
//! 
//! No Fit Polygon is a computational geometry technique used in nesting and packing
//! problems, particularly for 2D shape optimization. This library provides tools
//! for calculating NFPs between polygons (represented as `Vec<Point>`) to determine 
//! optimal placement strategies.
//! 
//! ## Usage
//! 
//! ```rust
//! use nfp::{point, NFPConvex};
//! 
//! // Create two triangles represented as Vec<Point>
//! let triangle_a = vec![
//!     point(0.0, 0.0),
//!     point(1.0, 0.0),
//!     point(0.5, 1.0),
//! ];
//! 
//! let triangle_b = vec![
//!     point(2.0, 2.0),
//!     point(3.0, 2.0),
//!     point(2.5, 3.0),
//! ];
//! 
//! // Calculate NFP between two convex polygons
//! let nfp = NFPConvex::nfp(&triangle_a, &triangle_b).unwrap();
//! ```

pub mod utils;
pub mod nfp_convex;
pub mod nfp_validation;
pub mod prelude;

mod nfp_tests;

// Re-export main types and error
pub use togo::prelude::Point;
pub use togo::prelude::point;
pub use nfp_convex::{NFPConvex, NfpError};
pub use nfp_convex::to_arcline;


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