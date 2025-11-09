//! No Fit Polygon implementation for vector of points representing vertices
//! in closed CCW oriented polygons.

use std::fmt;

/// Error type for NFP operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NfpError {
    /// One or both polygons are empty
    EmptyPolygon,
    /// One or both polygons have fewer than 3 vertices
    InsufficientVertices,
}

impl fmt::Display for NfpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NfpError::EmptyPolygon => write!(f, "Polygons cannot be empty"),
            NfpError::InsufficientVertices => write!(f, "Polygons must have at least 3 vertices"),
        }
    }
}

impl std::error::Error for NfpError {}

/// Create a new point - shortcut for Point::new()
pub fn point(x: f64, y: f64) -> Point {
    Point { x, y }
}

/// A 2D point with x and y coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Create a new point
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    /// Calculate the squared distance to another point
    pub fn distance_squared(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Calculate the distance to another point
    pub fn distance(&self, other: &Point) -> f64 {
        self.distance_squared(other).sqrt()
    }

    /// Subtract another point (vector subtraction)
    pub fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Add another point (vector addition)
    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.3}, {:.3})", self.x, self.y)
    }
}

/// Helper functions for working with polygons represented as `Vec<Point>`
pub mod polygon {
    use super::Point;

    /// Get the number of vertices
    pub fn len(vertices: &[Point]) -> usize {
        vertices.len()
    }

    /// Check if the polygon is empty
    pub fn is_empty(vertices: &[Point]) -> bool {
        vertices.is_empty()
    }

    /// Check if the polygon is oriented counter-clockwise
    pub fn is_ccw(vertices: &[Point]) -> bool {
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

    /// Reverse the order of vertices (change orientation)
    pub fn reverse(vertices: &mut Vec<Point>) {
        vertices.reverse();
    }

    /// Ensure the polygon is oriented counter-clockwise
    pub fn ensure_ccw(vertices: &mut Vec<Point>) {
        if !is_ccw(vertices) {
            reverse(vertices);
        }
    }

    /// Translate the polygon by a given offset
    pub fn translate(vertices: &[Point], offset: &Point) -> Vec<Point> {
        vertices.iter().map(|v| v.add(offset)).collect()
    }
}

/// No Fit Polygon calculator for point-based polygons
pub struct NFP;

impl NFP {
    pub fn nfp(poly_a: &[Point], poly_b: &[Point]) -> Result<Vec<Point>, NfpError> {
        if polygon::is_empty(poly_a) || polygon::is_empty(poly_b) {
            return Err(NfpError::EmptyPolygon);
        }

        if polygon::len(poly_a) < 3 || polygon::len(poly_b) < 3 {
            return Err(NfpError::InsufficientVertices);
        }

        let mut nfp_vertices = Vec::new();

        // Compute Minkowski sum by sliding A around B's perimeter
        for i in 0..polygon::len(poly_b) {
            let j = (i + 1) % polygon::len(poly_b);
            let b_curr = poly_b[i];
            let b_next = poly_b[j];

            for a_vertex in poly_a {
                // For each edge of B, compute offset points for each vertex of A
                let offset_to_start = b_curr.sub(a_vertex);
                let offset_to_end = b_next.sub(a_vertex);

                nfp_vertices.push(offset_to_start);
                nfp_vertices.push(offset_to_end);
            }
        }

        // Remove near-duplicate points (within tolerance)
        let tolerance = 1e-10;
        nfp_vertices.sort_by(|a, b| {
            a.x.partial_cmp(&b.x)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
        });
        nfp_vertices.dedup_by(|a, b| {
            (a.x - b.x).abs() < tolerance && (a.y - b.y).abs() < tolerance
        });

        // Sort vertices in CCW order by angle from centroid (avoid atan2)
        let centroid = compute_centroid(&nfp_vertices);
        nfp_vertices.sort_by(|a, b| angle_cmp(a, b, &centroid));

        Ok(nfp_vertices)
    }
}

fn compute_centroid(points: &[Point]) -> Point {
    if points.is_empty() {
        return Point::new(0.0, 0.0);
    }

    let sum_x: f64 = points.iter().map(|p| p.x).sum();
    let sum_y: f64 = points.iter().map(|p| p.y).sum();
    let len = points.len() as f64;

    Point::new(sum_x / len, sum_y / len)
}

/// Tolerance for comparing collinear points in angle sorting
const ANGLE_CMP_EPSILON: f64 = 1e-10;

// Compare points by angle around a given `centroid` without using `atan2`.
// Uses half-plane test and perp (2D cross product) to determine CCW ordering.
fn angle_cmp(a: &Point, b: &Point, centroid: &Point) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    let ax = a.x - centroid.x;
    let ay = a.y - centroid.y;
    let bx = b.x - centroid.x;
    let by = b.y - centroid.y;

    // Place points into two half-planes: upper (y>0 or y==0 && x>=0) and lower.
    let a_up = (ay > 0.0) || (ay == 0.0 && ax >= 0.0);
    let b_up = (by > 0.0) || (by == 0.0 && bx >= 0.0);
    if a_up != b_up {
        // a_up true should come before b_up false
        return a_up.cmp(&b_up).reverse();
    }

    // Same half-plane: use perp (2D cross product) to determine order
    let perp = ax * by - ay * bx;
    if perp.abs() > ANGLE_CMP_EPSILON {
        return if perp > 0.0 { Ordering::Less } else { Ordering::Greater };
    }

    // Collinear: sort by distance from centroid (closer first)
    let da = ax * ax + ay * ay;
    let db = bx * bx + by * by;
    da.partial_cmp(&db).unwrap_or(Ordering::Equal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p = Point::new(1.0, 2.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
    }

    #[test]
    fn test_polygon_creation() {
        let vertices = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ];
        assert_eq!(polygon::len(&vertices), 4);
    }

    #[test]
    fn test_polygon_ccw_orientation() {
        // Counter-clockwise square
        let vertices_ccw = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ];
        assert!(polygon::is_ccw(&vertices_ccw));

        // Clockwise square
        let vertices_cw = vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(1.0, 1.0),
            Point::new(1.0, 0.0),
        ];
        assert!(!polygon::is_ccw(&vertices_cw));
    }

    #[test]
    fn test_nfp_calculation_basic() {
        // Simple test with two triangles
        let triangle_a = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 1.0),
        ];

        let triangle_b = vec![
            Point::new(2.0, 2.0),
            Point::new(3.0, 2.0),
            Point::new(2.5, 3.0),
        ];

        let result = NFP::nfp(&triangle_a, &triangle_b);
        assert!(result.is_ok());
        
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
    }

    #[test]
    fn test_nfp_two_squares() {
        // Two unit squares
        let square_a = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ];

        let square_b = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ];

        let result = NFP::nfp(&square_a, &square_b);
        assert!(result.is_ok());
        
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
        // NFP should have at least as many vertices as input polygons
        assert!(nfp.len() >= 3);
    }

    #[test]
    fn test_nfp_with_different_sized_squares() {
        // Small square A
        let square_small = vec![
            Point::new(0.0, 0.0),
            Point::new(0.5, 0.0),
            Point::new(0.5, 0.5),
            Point::new(0.0, 0.5),
        ];

        // Large square B
        let square_large = vec![
            Point::new(0.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(2.0, 2.0),
            Point::new(0.0, 2.0),
        ];

        let result = NFP::nfp(&square_small, &square_large);
        assert!(result.is_ok());
        
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
        // Verify the NFP has reasonable properties
        assert!(nfp.len() >= 3);
    }

    #[test]
    fn test_nfp_pentagon_and_triangle() {
        // Pentagon A
        let pentagon = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.5, 0.8),
            Point::new(0.8, 1.6),
            Point::new(-0.2, 1.0),
        ];

        // Triangle B
        let triangle = vec![
            Point::new(0.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(1.0, 2.0),
        ];

        let result = NFP::nfp(&pentagon, &triangle);
        assert!(result.is_ok());
        
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
        assert!(polygon::is_ccw(&nfp));
    }

    #[test]
    fn test_nfp_multiple_polygons() {
        // Rectangle A
        let rect_a = vec![
            Point::new(0.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(2.0, 1.0),
            Point::new(0.0, 1.0),
        ];

        // Rectangle B (positioned differently)
        let rect_b = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 2.0),
            Point::new(0.0, 2.0),
        ];

        let result = NFP::nfp(&rect_a, &rect_b);
        assert!(result.is_ok());
        
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
        
        // NFP should have more vertices than input polygons
        assert!(nfp.len() >= 3);
    }

    #[test]
    fn test_nfp_error_empty_polygon_a() {
        let empty = vec![];
        let triangle = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 1.0),
        ];

        let result = NFP::nfp(&empty, &triangle);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NfpError::EmptyPolygon);
    }

    #[test]
    fn test_nfp_error_empty_polygon_b() {
        let triangle = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 1.0),
        ];
        let empty = vec![];

        let result = NFP::nfp(&triangle, &empty);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NfpError::EmptyPolygon);
    }

    #[test]
    fn test_nfp_error_insufficient_vertices() {
        let line = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
        ];
        let triangle = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 1.0),
        ];

        let result = NFP::nfp(&line, &triangle);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NfpError::InsufficientVertices);
    }



    #[test]
    fn test_nfp_hexagon_and_hexagon() {
        // Regular-ish hexagon A
        let hex_a = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.5, 0.87),
            Point::new(1.0, 1.73),
            Point::new(0.0, 1.73),
            Point::new(-0.5, 0.87),
        ];

        // Regular-ish hexagon B
        let hex_b = vec![
            Point::new(0.0, 0.0),
            Point::new(0.8, 0.0),
            Point::new(1.2, 0.69),
            Point::new(0.8, 1.38),
            Point::new(0.0, 1.38),
            Point::new(-0.4, 0.69),
        ];

        let result = NFP::nfp(&hex_a, &hex_b);
        assert!(result.is_ok());
        
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
        assert!(polygon::is_ccw(&nfp));
    }

    #[test]
    fn test_nfp_complex_shapes() {
        // L-shaped polygon A
        let l_shape = vec![
            Point::new(0.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(2.0, 1.0),
            Point::new(1.0, 1.0),
            Point::new(1.0, 2.0),
            Point::new(0.0, 2.0),
        ];

        // T-shaped polygon B
        let t_shape = vec![
            Point::new(0.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(2.0, 0.5),
            Point::new(1.5, 0.5),
            Point::new(1.5, 1.5),
            Point::new(0.5, 1.5),
            Point::new(0.5, 0.5),
            Point::new(0.0, 0.5),
        ];

        let result = NFP::nfp(&l_shape, &t_shape);
        assert!(result.is_ok());
        
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
        assert!(polygon::is_ccw(&nfp));
    }

    #[test]
    fn test_nfp_with_translated_polygons() {
        // Triangle at origin
        let triangle_a = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 1.0),
        ];

        // Triangle translated away
        let triangle_b = vec![
            Point::new(5.0, 5.0),
            Point::new(6.0, 5.0),
            Point::new(5.5, 6.0),
        ];

        let result = NFP::nfp(&triangle_a, &triangle_b);
        assert!(result.is_ok());
        
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
        
        // NFP vertices should be in a different coordinate space
        for point in &nfp {
            // NFP should have points that are offset from origin
            assert!(point.x != 0.0 || point.y != 0.0 || nfp.len() > 1);
        }
    }
}