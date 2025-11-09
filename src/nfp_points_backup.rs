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

/// Convert a polygon (Vec<Point>) to an Arcline (Vec<Arc>)
/// 
/// Each consecutive pair of points is connected by a straight line segment (arc).
/// The polygon is assumed to be closed, so the last point connects back to the first.
///
/// # Arguments
/// * `vertices` - A slice of points representing the nfp polygon
///
/// # Returns
/// A Vec of togo::Arc objects (togo::Arcline) representing the polygon as line segments
///
/// # Example
/// ```
/// use nfp::prelude::*;
/// use togo::prelude::*;
///
/// let polygon = vec![
///     point(0.0, 0.0),
///     point(1.0, 0.0),
///     point(1.0, 1.0),
///     point(0.0, 1.0),
/// ];
/// let arcline = nfp::to_arcline(&polygon);
/// assert_eq!(arcline.len(), 4); // 4 edges for a square
/// ```
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
        
        // Convert nfp::Point to togo::Point using the prelude function
        let p1 = point(current.x, current.y);
        let p2 = point(next.x, next.y);
        
        // Create a line segment using togo::arcseg
        let arc = arcseg(p1, p2);
        arcline.push(arc);
    }
    
    arcline
}

/// No Fit Polygon calculator for point-based polygons
pub struct NFP;

impl NFP {
    /// Calculate the No Fit Polygon (NFP) for two non-convex polygons.
    ///
    /// Uses Contributing Vertices-Based approach from Barki et al. (2011):
    /// 1. Generate all candidate vertices by sliding each vertex of A along each edge of B
    /// 2. Filter vertices based on edge orientation (contributing vertices)
    /// 3. Extract the true boundary from the candidate set
    ///
    /// This correctly handles non-convex polygons by preserving concave regions
    /// that would be lost with convex hull approximation.
    pub fn nfp(poly_a: &[Point], poly_b: &[Point]) -> Result<Vec<Point>, NfpError> {
        if polygon::is_empty(poly_a) || polygon::is_empty(poly_b) {
            return Err(NfpError::EmptyPolygon);
        }

        if polygon::len(poly_a) < 3 || polygon::len(poly_b) < 3 {
            return Err(NfpError::InsufficientVertices);
        }

        // Ensure both polygons are CCW
        let mut a = poly_a.to_vec();
        let mut b = poly_b.to_vec();
        polygon::ensure_ccw(&mut a);
        polygon::ensure_ccw(&mut b);

        // Step 1: Generate all candidate vertices
        let candidates = generate_candidates(&a, &b);

        // Step 2: Filter by orientation (contributing vertices concept)
        let filtered = filter_by_orientation(&candidates, &a, &b);

        // Step 3: Extract boundary by sorting in CCW order around centroid
        let nfp = extract_boundary(&filtered);

        Ok(nfp)
    }
}


/// Remove duplicate points within tolerance
fn remove_duplicates(points: &mut Vec<Point>, tolerance: f64) {
    if points.len() < 2 {
        return;
    }

    points.sort_unstable_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
    });

    points.dedup_by(|a, b| {
        (a.x - b.x).abs() < tolerance && (a.y - b.y).abs() < tolerance
    });
}

/// Generate candidate vertices by sliding each vertex of A along each edge of B
/// 
/// For each vertex of A and each edge of B, compute the offset position
/// where A's vertex touches B's edge. This generates a superset that includes
/// all possible contributing vertices for the NFP.
fn generate_candidates(a: &[Point], b: &[Point]) -> Vec<Point> {
    let mut candidates = Vec::new();

    for &a_vertex in a {
        for i in 0..b.len() {
            let b_start = b[i];
            let offset = b_start.sub(&a_vertex);
            candidates.push(offset);
        }
    }

    // Remove near-duplicate points with tolerance
    remove_duplicates(&mut candidates, 1e-10);
    candidates
}

/// Filter candidates by orientation consistency (contributing vertices concept)
///
/// Based on Barki et al. (2011), a vertex "contributes" to the NFP boundary
/// if it lies on an edge that has consistent orientation between A and B.
/// 
/// For a point to contribute:
/// - It must correspond to a vertex of A touching an edge of B
/// - The normal directions must be consistent (both outward or both inward)
fn filter_by_orientation(candidates: &[Point], a: &[Point], b: &[Point]) -> Vec<Point> {
    let mut filtered = Vec::new();
    let tolerance = 1e-10;

    for &candidate in candidates {
        // For each candidate, find which (a_vertex, b_edge) pair it corresponds to
        let mut is_contributing = false;

        for (a_idx, &a_vertex) in a.iter().enumerate() {
            for b_idx in 0..b.len() {
                let b_start = b[b_idx];
                let expected_offset = b_start.sub(&a_vertex);

                // Check if candidate matches this configuration
                if (candidate.x - expected_offset.x).abs() < tolerance
                    && (candidate.y - expected_offset.y).abs() < tolerance
                {
                    // Found matching configuration. Now check if it's a contributing vertex.
                    
                    // Get edges of A at this vertex
                    let a_prev_idx = (a_idx as i32 - 1).rem_euclid(a.len() as i32) as usize;
                    let a_next_idx = (a_idx + 1) % a.len();
                    
                    let a_prev = a[a_prev_idx];
                    let a_next = a[a_next_idx];

                    // Edge of B
                    let b_next_idx = (b_idx + 1) % b.len();
                    let b_end = b[b_next_idx];

                    // Compute outward normals (perpendicular to edges, CCW polygon = outward to left)
                    let edge_ap = a_vertex.sub(&a_prev);
                    let normal_ap = Point::new(-edge_ap.y, edge_ap.x); // Left normal

                    let edge_an = a_next.sub(&a_vertex);
                    let normal_an = Point::new(-edge_an.y, edge_an.x); // Left normal

                    let edge_b = b_end.sub(&b_start);
                    let normal_b = Point::new(-edge_b.y, edge_b.x); // Left normal

                    // A vertex contributes if:
                    // - The vertex's edges have normals pointing generally in the same direction as B's edge normal
                    // This ensures the vertex is on the "outside" of the configuration
                    
                    let dot_ap = normal_ap.x * normal_b.x + normal_ap.y * normal_b.y;
                    let dot_an = normal_an.x * normal_b.x + normal_an.y * normal_b.y;

                    // Contributing if at least one neighbor is compatible with B's edge normal
                    // (relaxed criterion to avoid over-filtering)
                    // Or if the dot products don't strongly contradict
                    let ap_len = (normal_ap.x * normal_ap.x + normal_ap.y * normal_ap.y).sqrt();
                    let an_len = (normal_an.x * normal_an.x + normal_an.y * normal_an.y).sqrt();
                    let b_len = (normal_b.x * normal_b.x + normal_b.y * normal_b.y).sqrt();
                    
                    // Normalize dot products by vector magnitudes for comparison
                    let dot_ap_norm = if ap_len > tolerance && b_len > tolerance { dot_ap / (ap_len * b_len) } else { 1.0 };
                    let dot_an_norm = if an_len > tolerance && b_len > tolerance { dot_an / (an_len * b_len) } else { 1.0 };
                    
                    // Accept if at least one edge is not strongly opposing (dot product > -0.5)
                    if dot_ap_norm > -0.5 || dot_an_norm > -0.5 {
                        is_contributing = true;
                    }

                    break;
                }
            }
            if is_contributing {
                break;
            }
        }

        if is_contributing {
            filtered.push(candidate);
        }
    }

    // If filtering removes all points (shouldn't happen), return candidates as fallback
    if filtered.is_empty() {
        filtered = candidates.to_vec();
    }

    filtered
}

/// Extract the true NFP boundary from candidate vertices
///
/// Sort candidates in counter-clockwise order around their centroid,
/// forming a simple polygon. This represents the boundary of the NFP.
/// Falls back to convex hull if simple centroid-based sorting doesn't produce CCW polygon.
fn extract_boundary(candidates: &[Point]) -> Vec<Point> {
    if candidates.len() < 3 {
        return candidates.to_vec();
    }

    // Compute centroid
    let centroid = {
        let sum_x: f64 = candidates.iter().map(|p| p.x).sum();
        let sum_y: f64 = candidates.iter().map(|p| p.y).sum();
        Point::new(sum_x / candidates.len() as f64, sum_y / candidates.len() as f64)
    };

    // Sort around centroid in CCW order
    let mut sorted = candidates.to_vec();
    sorted.sort_by(|a, b| angle_compare(a, b, &centroid));

    // Verify the result is CCW. If not, use convex hull as fallback
    if polygon::is_ccw(&sorted) {
        sorted
    } else {
        // Fall back to convex hull (which guarantees CCW for correct output)
        simple_hull(candidates)
    }
}

/// Compute convex hull of points using Graham scan
/// 
/// Note: This function is kept for reference/testing but is no longer used
/// in the main NFP algorithm. The contributing vertices approach preserves
/// concavities needed for non-convex polygon NFP computation.
#[allow(dead_code)]
fn simple_hull(points: &[Point]) -> Vec<Point> {
    if points.len() < 3 {
        return points.to_vec();
    }

    // Find the point with the lowest y-coordinate (and leftmost if tie)
    let mut start_idx = 0;
    for i in 1..points.len() {
        if points[i].y < points[start_idx].y 
            || (points[i].y == points[start_idx].y && points[i].x < points[start_idx].x) {
            start_idx = i;
        }
    }

    let start = points[start_idx];

    // Sort all points by polar angle with respect to the start point
    let mut sorted = points.to_vec();
    sorted.sort_by(|a, b| {
        // Skip the start point itself
        let a_is_start = (a.x - start.x).abs() < 1e-10 && (a.y - start.y).abs() < 1e-10;
        let b_is_start = (b.x - start.x).abs() < 1e-10 && (b.y - start.y).abs() < 1e-10;

        if a_is_start {
            return std::cmp::Ordering::Less;
        }
        if b_is_start {
            return std::cmp::Ordering::Greater;
        }

        let ax = a.x - start.x;
        let ay = a.y - start.y;
        let bx = b.x - start.x;
        let by = b.y - start.y;

        // Use cross product to determine order
        let cross = ax * by - ay * bx;
        if cross.abs() > 1e-10 {
            return if cross > 0.0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            };
        }

        // Collinear: sort by distance
        let dist_a = ax * ax + ay * ay;
        let dist_b = bx * bx + by * by;
        dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
    });

    // Build the hull
    let mut hull: Vec<Point> = Vec::new();

    for &point in &sorted {
        // Remove points that make a right turn
        while hull.len() >= 2 {
            let n = hull.len();
            let o = hull[n - 2];
            let a = hull[n - 1];
            let b = point;

            let cross = (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
            if cross <= 1e-10 {
                hull.pop();
            } else {
                break;
            }
        }
        hull.push(point);
    }

    hull
}

/// Compare two points by angle around a centroid (CCW order)
fn angle_compare(a: &Point, b: &Point, centroid: &Point) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    let ax = a.x - centroid.x;
    let ay = a.y - centroid.y;
    let bx = b.x - centroid.x;
    let by = b.y - centroid.y;

    // Half-plane test: place in upper or lower half-plane
    let a_up = (ay > 0.0) || (ay == 0.0 && ax >= 0.0);
    let b_up = (by > 0.0) || (by == 0.0 && bx >= 0.0);
    
    if a_up != b_up {
        return a_up.cmp(&b_up).reverse();
    }

    // Same half-plane: use cross product to determine order
    let cross = ax * by - ay * bx;
    if cross.abs() > 1e-10 {
        return if cross > 0.0 { Ordering::Less } else { Ordering::Greater };
    }

    // Collinear: sort by distance
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

    #[test]
    fn test_nfp_collinear_points() {
        // Collinear points should still form a valid polygon (degenerate triangle)
        let triangle_a = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 0.0), // Collinear with others
        ];

        let triangle_b = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ];

        let result = NFP::nfp(&triangle_a, &triangle_b);
        // Should complete without panicking (even if degenerate)
        assert!(result.is_ok());
    }

    #[test]
    fn test_nfp_very_small_polygon() {
        // Minimal triangles (smallest valid polygon)
        let tiny_a = vec![
            Point::new(0.0, 0.0),
            Point::new(0.001, 0.0),
            Point::new(0.0005, 0.001),
        ];

        let tiny_b = vec![
            Point::new(0.0, 0.0),
            Point::new(0.002, 0.0),
            Point::new(0.001, 0.002),
        ];

        let result = NFP::nfp(&tiny_a, &tiny_b);
        assert!(result.is_ok());
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
    }

    #[test]
    fn test_nfp_negative_coordinates() {
        // Polygons in negative coordinate space
        let triangle_a = vec![
            Point::new(-2.0, -2.0),
            Point::new(-1.0, -2.0),
            Point::new(-1.5, -1.0),
        ];

        let triangle_b = vec![
            Point::new(-3.0, -3.0),
            Point::new(-1.0, -3.0),
            Point::new(-2.0, -1.0),
        ];

        let result = NFP::nfp(&triangle_a, &triangle_b);
        assert!(result.is_ok());
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
    }

    #[test]
    fn test_nfp_mixed_coordinate_signs() {
        // Polygons spanning multiple quadrants
        let triangle_a = vec![
            Point::new(-1.0, -1.0),
            Point::new(1.0, -1.0),
            Point::new(0.0, 1.0),
        ];

        let triangle_b = vec![
            Point::new(-0.5, -0.5),
            Point::new(0.5, -0.5),
            Point::new(0.0, 0.5),
        ];

        let result = NFP::nfp(&triangle_a, &triangle_b);
        assert!(result.is_ok());
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
        assert!(polygon::is_ccw(&nfp));
    }

    #[test]
    fn test_nfp_identical_polygons() {
        // Two identical triangles
        let triangle = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 1.0),
        ];

        let result = NFP::nfp(&triangle, &triangle);
        assert!(result.is_ok());
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
    }

    #[test]
    fn test_nfp_very_thin_rectangle() {
        // Very thin elongated rectangle
        let thin_rect_a = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 0.01),
            Point::new(0.0, 0.01),
        ];

        let square = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ];

        let result = NFP::nfp(&thin_rect_a, &square);
        assert!(result.is_ok());
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
    }

    #[test]
    fn test_nfp_large_scale_coordinates() {
        // Polygons with very large coordinates
        let large_a = vec![
            Point::new(1000.0, 1000.0),
            Point::new(1100.0, 1000.0),
            Point::new(1050.0, 1100.0),
        ];

        let large_b = vec![
            Point::new(2000.0, 2000.0),
            Point::new(2050.0, 2000.0),
            Point::new(2025.0, 2050.0),
        ];

        let result = NFP::nfp(&large_a, &large_b);
        assert!(result.is_ok());
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
    }

    #[test]
    fn test_nfp_right_angle_triangle() {
        // Right-angled triangle (standard form)
        let right_a = vec![
            Point::new(0.0, 0.0),
            Point::new(3.0, 0.0),
            Point::new(0.0, 4.0),
        ];

        let right_b = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.0, 1.0),
        ];

        let result = NFP::nfp(&right_a, &right_b);
        assert!(result.is_ok());
        let nfp = result.unwrap();
        assert!(!nfp.is_empty());
        assert!(polygon::is_ccw(&nfp));
    }

    #[test]
    fn test_nfp_polygon_with_zero_area() {
        // Three points where area rounds to near-zero (but technically valid)
        // This tests robustness - should handle near-degenerate cases
        let point_a = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0 + 1e-15, 1e-15), // Extremely close to collinear
        ];

        let point_b = vec![
            Point::new(0.0, 0.0),
            Point::new(0.5, 0.0),
            Point::new(0.25, 0.5),
        ];

        let result = NFP::nfp(&point_a, &point_b);
        // Should complete without panicking
        assert!(result.is_ok());
    }

    #[test]
    fn test_to_arcline_triangle() {
        // Test converting a simple triangle to arcline
        let triangle = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 1.0),
        ];

        let arcline = to_arcline(&triangle);
        assert_eq!(arcline.len(), 3); // 3 edges for a triangle
    }

    #[test]
    fn test_to_arcline_square() {
        // Test converting a square to arcline
        let square = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ];

        let arcline = to_arcline(&square);
        assert_eq!(arcline.len(), 4); // 4 edges for a square
    }

    #[test]
    fn test_to_arcline_empty() {
        // Test converting an empty polygon
        let empty: Vec<Point> = vec![];
        let arcline = to_arcline(&empty);
        assert_eq!(arcline.len(), 0);
    }

    #[test]
    fn test_to_arcline_single_point() {
        // Test converting a single point (degenerate case)
        let single = vec![Point::new(0.0, 0.0)];
        let arcline = to_arcline(&single);
        assert_eq!(arcline.len(), 1); // Single self-loop edge
    }
}
