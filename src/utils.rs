//! Common utility functions for NFP algorithms
//! 
//! Shared functionality for polygon manipulation and geometric operations.

use togo::prelude::Point;
use std::f64::consts::PI;

const EPS: f64 = 1e-9;

/// Compute Euclidean distance between two points
pub fn distance(p1: Point, p2: Point) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    (dx * dx + dy * dy).sqrt()
}

/// Check if polygon is counter-clockwise and optionally reverse if needed
pub fn is_ccw(vertices: &[Point]) -> bool {
    if vertices.len() < 3 {
        return false;
    }
    
    let mut area = 0.0;
    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        area += (vertices[j].x - vertices[i].x) * (vertices[j].y + vertices[i].y);
    }
    
    // Negative area = CCW
    area < 0.0
}

/// Ensure polygon is counter-clockwise, reverse if needed
pub fn ensure_ccw(vertices: &mut Vec<Point>) {
    if vertices.len() < 3 {
        return;
    }
    
    let mut area = 0.0;
    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        area += (vertices[j].x - vertices[i].x) * (vertices[j].y + vertices[i].y);
    }
    
    // Negative area = CCW, positive = CW
    if area > 0.0 {
        vertices.reverse();
    }
}

/// Normalize angle to [0, 2π)
pub fn normalize_angle(angle: f64) -> f64 {
    let mut a = angle % (2.0 * PI);
    if a < 0.0 {
        a += 2.0 * PI;
    }
    a
}

/// Find minimum vertex (by y, then by x)
pub fn find_min_vertex(vertices: &[Point]) -> Option<Point> {
    vertices.iter().min_by(|p1, p2| {
        match p1.y.partial_cmp(&p2.y).unwrap_or(std::cmp::Ordering::Equal) {
            std::cmp::Ordering::Equal => p1.x.partial_cmp(&p2.x).unwrap_or(std::cmp::Ordering::Equal),
            other => other,
        }
    }).copied()
}

/// Get precision epsilon for floating point comparisons
pub fn epsilon() -> f64 {
    EPS
}
