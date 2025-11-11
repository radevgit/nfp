# NFP - No Fit Polygon

[![Crates.io](https://img.shields.io/crates/v/nfp.svg?color=blue)](https://crates.io/crates/nfp)
[![Documentation](https://docs.rs/nfp/badge.svg)](https://docs.rs/nfp)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A Rust library for computing the No Fit Polygon (Minkowski sum) of two convex polygons for nesting and packing optimization.

## What is NFP?

The No Fit Polygon defines the forbidden region where one polygon cannot be placed relative to another without collision. When polygon B's origin is **outside** the NFP, the polygons do **not** overlap. When it's **inside**, they **do** collide.

## Features

- ✅ **Convex polygon support** with Minkowski sum via vertex combinations and convex hull
- ✅ **Fast computation** using Graham scan algorithm
- ✅ **Validated output** - CCW orientation, convex, no self-intersections
- ✅ **Automatic orientation correction** - handles CCW/CW input
- ✅ **Zero-copy collision detection** - use point-in-polygon test on NFP

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nfp = "0.3"
```

## Quick Start

```rust
use nfp::prelude::*;

// Two convex polygons at origin (CCW orientation)
let square_a = vec![
    point(0.0, 0.0),
    point(10.0, 0.0),
    point(10.0, 10.0),
    point(0.0, 10.0),
];

let square_b = vec![
    point(0.0, 0.0),
    point(5.0, 0.0),
    point(5.0, 5.0),
    point(0.0, 5.0),
];

// Compute NFP
match NFPConvex::nfp(&square_a, &square_b) {
    Ok(nfp) => {
        println!("NFP has {} vertices", nfp.len());
        // Now use NFP for collision detection:
        // point inside NFP → collision, point outside → no collision
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Collision Detection

Once you have the NFP, detect collisions by testing if B's origin is inside or outside:

```rust
use nfp::prelude::*;

fn point_in_polygon(pt: Point, polygon: &[Point]) -> bool {
    let mut inside = false;
    let mut p1 = polygon[polygon.len() - 1];
    for p2 in polygon {
        if ((p2.y > pt.y) != (p1.y > pt.y))
            && (pt.x < (p1.x - p2.x) * (pt.y - p2.y) / (p1.y - p2.y) + p2.x)
        {
            inside = !inside;
        }
        p1 = *p2;
    }
    inside
}

// Compute NFP first
let square_a = vec![point(0.0, 0.0), point(10.0, 0.0), point(10.0, 10.0), point(0.0, 10.0)];
let square_b = vec![point(0.0, 0.0), point(5.0, 0.0), point(5.0, 5.0), point(0.0, 5.0)];
let nfp = NFPConvex::nfp(&square_a, &square_b).unwrap();

// Check if placing B at (15.0, 15.0) causes collision
let test_position = point(15.0, 15.0);
if point_in_polygon(test_position, &nfp) {
    println!("Collision detected!");
} else {
    println!("Safe placement");
}
```

## Requirements

- Both input polygons **must be convex**
- Both polygons must have **at least 3 vertices**
- Polygons should be in **counter-clockwise (CCW) orientation** (auto-corrected if needed)
- Polygons must be **closed** (last point connects to first conceptually)

## API Reference

### `NFPConvex::nfp(poly_a, poly_b) -> Result<Vec<Point>, NfpError>`

Computes the No Fit Polygon for two convex polygons using Minkowski sum.

**Arguments:**
- `poly_a` - First convex polygon as slice of points
- `poly_b` - Second convex polygon as slice of points

**Returns:**
- `Ok(Vec<Point>)` - NFP as counter-clockwise convex polygon
- `Err(NfpError)` - If input validation fails

**Time Complexity:** O(n·m log(n·m)) where n = |A| vertices, m = |B| vertices

### `NfpError`

Error enumeration for NFP operations.

**Variants:**
- `EmptyPolygon` - One or both input polygons are empty
- `InsufficientVertices` - One or both polygons have fewer than 3 vertices

**Example:**
```rust
use nfp::prelude::*;

match NFPConvex::nfp(&a, &b) {
    Ok(nfp) => { /* use nfp */ }
    Err(NfpError::EmptyPolygon) => eprintln!("Empty polygon provided"),
    Err(NfpError::InsufficientVertices) => eprintln!("Polygon needs ≥3 vertices"),
}
```

### `Point`

2D coordinate point from the `togo` geometry library.

**Fields:**
- `x: f64` - X coordinate
- `y: f64` - Y coordinate

### `point(x, y) -> Point`

Helper function to create a new Point.

```rust
use nfp::prelude::*;

let p = point(1.5, 2.5);
assert_eq!(p.x, 1.5);
assert_eq!(p.y, 2.5);
```


## Algorithm

The implementation uses the **Minkowski Sum via Vertex Combinations**:

1. Validate inputs (non-empty, convex, ≥3 vertices)
2. Ensure counter-clockwise orientation
3. Negate polygon B (reflection through origin)
4. Generate all vertex sum combinations: {a + b | a ∈ A, b ∈ -B}
5. Compute convex hull of sum points using Graham scan
6. Return convex hull boundary as NFP

## Limitations

- Current version supports **Convex polygons only**


## References

- Minkowski sum: https://en.wikipedia.org/wiki/Minkowski_addition
- No-Fit Polygon: https://en.wikipedia.org/wiki/No-fit_polygon
- Graham scan: https://en.wikipedia.org/wiki/Graham_scan

## Related Projects

NFP is part of the [Nest2D](https://nest2d.com) collection of nesting and packing tools.
