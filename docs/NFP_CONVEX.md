# NFP for Convex Polygons

## The core concept for convex polygons:

The No-Fit Polygon for two convex polygons can be computed using the **Minkowski Sum** approach. Specifically:

$$\text{NFP}(A, B) = A \oplus (-B)$$

where $\oplus$ is Minkowski sum and $-B$ means B reflected around the origin.

## The algorithm steps:

1. **Reflect B around origin**: Negate all coordinates of B's vertices, then reverse order (to maintain CCW)
   
2. **Compute convex hull of Minkowski sum**: 
   - For each vertex of A and each vertex of reflected B, compute their Pairwise sum (vector addition)
   - This gives a point cloud of all possible positions
   - Compute convex hull of this point cloud
   - The convex hull IS the NFP

3. **Alternatively, simpler edge-merging approach**:
   - Sort all edges from A and B by angle
   - Merge them into one edge sequence (traversing both polygons' boundaries)
   - Traverse this merged sequence while accumulating position to trace the NFP boundary
   - This avoids computing convex hull explicitly

## Recommended: Edge Merging Approach

```
1. Get all edges from A (as vectors)
2. Get all edges from reflected B (as vectors)
3. Combine into one list, sort by angle (atan2)
4. Start at a reference point
5. Traverse: follow each edge in sequence, tracking current position
6. Each position becomes an NFP vertex
```

