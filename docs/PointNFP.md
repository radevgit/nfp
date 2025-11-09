The nfp-points.rs implements No Fit Polygon for vector of points that represent vertices in
closed CCW oriented polylines.

### Algorithm Implementation Approach:
The current implementation uses:

- Offset method: For each edge of polygon B, compute offset points from vertices of polygon A
- Sorting by angle: Sort resulting vertices by angle from centroid to ensure proper CCW ordering
- Deduplication: Remove near-duplicate points (within tolerance)

