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
//! use nfp::{point, NFP};
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
//! // Calculate NFP between two polygons
//! let nfp = NFP::nfp(&triangle_a, &triangle_b).unwrap();
//! ```

pub mod nfp_points;

// Re-export the main types for easier use
pub use nfp_points::{point, Point, NFP};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let point = Point::new(1.0, 2.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
    }
}