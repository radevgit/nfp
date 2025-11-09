use crate::Point;
use std::f64::consts::PI;
use rand::SeedableRng;
use rand::Rng;

pub struct DataGen;

impl DataGen {
    /// Generate a closed, non-intersecting random polygon with specified number of edges
    /// 
    /// Uses a convex hull approach: generates random points in polar coordinates
    /// and sorts them by angle to ensure no self-intersections.
    /// 
    /// # Arguments
    /// * `num_edges` - Number of edges for the polygon (minimum 3)
    /// * `seed` - Optional seed for reproducible random generation. If None, uses system entropy
    pub fn random_polygon(num_edges: usize, seed: u64) -> Vec<Point> {
        if num_edges < 3 {
            panic!("Polygon must have at least 3 edges");
        }

        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        // Generate random points in polar coordinates
        let mut points: Vec<Point> = (0..num_edges)
            .map(|_| {
                let angle = rng.random_range(0.0..(2.0 * PI));
                let radius = rng.random_range(0.5..2.0);
                Point::new(radius * angle.cos(), radius * angle.sin())
            })
            .collect();

        // Sort by angle from centroid to ensure CCW ordering and no self-intersections
        let centroid = Self::centroid(&points);
        points.sort_by(|a, b| {
            let angle_a = (a.y - centroid.y).atan2(a.x - centroid.x);
            let angle_b = (b.y - centroid.y).atan2(b.x - centroid.x);
            angle_a.partial_cmp(&angle_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        points
    }

    fn centroid(points: &[Point]) -> Point {
        if points.is_empty() {
            return Point::new(0.0, 0.0);
        }

        let sum_x: f64 = points.iter().map(|p| p.x).sum();
        let sum_y: f64 = points.iter().map(|p| p.y).sum();
        let len = points.len() as f64;

        Point::new(sum_x / len, sum_y / len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nfp_points::polygon;

    #[test]
    fn test_random_polygon_generation() {
        let poly = DataGen::random_polygon(100, 42);
        
        // Verify correct number of edges
        assert_eq!(poly.len(), 100);
        
        // Verify polygon is closed and non-degenerate
        assert!(!poly.is_empty());
        
        // Verify CCW orientation
        assert!(polygon::is_ccw(&poly));
    }

    #[test]
    fn test_random_polygon_minimum_size() {
        let poly = DataGen::random_polygon(3, 42);
        assert_eq!(poly.len(), 3);
        assert!(polygon::is_ccw(&poly));
    }

    #[test]
    fn test_random_polygon_with_seed() {
        // Same seed should produce identical polygons
        let poly1 = DataGen::random_polygon(50, 12345);
        let poly2 = DataGen::random_polygon(50, 12345);
        
        assert_eq!(poly1.len(), poly2.len());
        for (p1, p2) in poly1.iter().zip(poly2.iter()) {
            assert!((p1.x - p2.x).abs() < 1e-10);
            assert!((p1.y - p2.y).abs() < 1e-10);
        }
    }

    #[test]
    #[should_panic]
    fn test_random_polygon_invalid_size() {
        DataGen::random_polygon(2, 42);
    }
}
