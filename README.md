# NFP - No Fit Polygon

A Rust library that computes the No Fit Polygon (Minkowski sum) of two closed counter-clockwise oriented polygons.

## Overview

NFP implements the Minkowski sum algorithm for polygon nesting and packing problems. Given two CCW-oriented polygons, it computes the boundary region where one polygon can be placed relative to another without overlapping.

## Usage

```rust
use nfp::{point, NFP};

// Define two CCW-oriented polygons using the point() shortcut
let poly_a = vec![
    point(0.0, 0.0),
    point(1.0, 0.0),
    point(0.5, 1.0),
];

let poly_b = vec![
    point(0.0, 0.0),
    point(2.0, 0.0),
    point(2.0, 2.0),
    point(0.0, 2.0),
];

// Calculate the No Fit Polygon
match NFP::nfp(&poly_a, &poly_b) {
    Ok(nfp) => println!("NFP has {} vertices", nfp.len()),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Requirements

- Both polygons must be closed (first and last points form an edge)
- Both polygons must be oriented counter-clockwise (CCW)
- Both polygons must have at least 3 vertices

## API

### `point(x, y)`
Shortcut function to create a new point.

### `Point`
2D point with x, y coordinates.

**Methods:**
- `new(x, y)` - Create a new point
- `distance(other)` - Calculate distance to another point
- `distance_squared(other)` - Calculate squared distance
- `add(other)` - Vector addition
- `sub(other)` - Vector subtraction

### `NFP`
Main calculator for Minkowski sums.

**Methods:**
- `nfp(poly_a, poly_b)` - Compute NFP of two polygons

### `polygon` module
Utility functions for polygon operations:
- `len(vertices)` - Get vertex count
- `is_empty(vertices)` - Check if empty
- `is_ccw(vertices)` - Check counter-clockwise orientation
- `ensure_ccw(vertices)` - Ensure CCW orientation (mutates)
- `translate(vertices, offset)` - Translate polygon by offset
