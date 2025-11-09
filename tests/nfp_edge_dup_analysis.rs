use nfp::prelude::*;

#[test]
fn analyze_edge_duplicates_in_minkowski_sum() {
    // Simple 4-vertex square
    let square_a = vec![
        point(0.0, 0.0),
        point(1.0, 0.0),
        point(1.0, 1.0),
        point(0.0, 1.0),
    ];

    let square_b = vec![
        point(0.0, 0.0),
        point(1.0, 0.0),
        point(1.0, 1.0),
        point(0.0, 1.0),
    ];

    // Manually trace through the algorithm
    let mut nfp_vertices = Vec::new();

    println!("\n=== TRACING MINKOWSKI SUM GENERATION ===\n");

    for i in 0..square_b.len() {
        let j = (i + 1) % square_b.len();
        let b_curr = square_b[i];
        let b_next = square_b[j];

        println!("Edge B[{}] -> B[{}]: ({:.2}, {:.2}) -> ({:.2}, {:.2})",
            i, j, b_curr.x, b_curr.y, b_next.x, b_next.y);

        for (a_idx, a_vertex) in square_a.iter().enumerate() {
            let offset_to_start = b_curr.sub(a_vertex);
            let offset_to_end = b_next.sub(a_vertex);

            println!("  A[{}]: ({:.2}, {:.2}) => start offset: ({:.10}, {:.10}), end offset: ({:.10}, {:.10})",
                a_idx, a_vertex.x, a_vertex.y,
                offset_to_start.x, offset_to_start.y,
                offset_to_end.x, offset_to_end.y);

            nfp_vertices.push(offset_to_start);
            nfp_vertices.push(offset_to_end);
        }
        println!();
    }

    println!("Total vertices before dedup: {}", nfp_vertices.len());
    println!("Expected: 4 edges × 4 vertices × 2 offsets = 32\n");

    // Now deduplicate (copy the logic from nfp_points.rs)
    let tolerance = 1e-10;
    nfp_vertices.sort_unstable_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
    });

    let before_dedup = nfp_vertices.len();
    nfp_vertices.dedup_by(|a, b| {
        (a.x - b.x).abs() < tolerance && (a.y - b.y).abs() < tolerance
    });
    let after_dedup = nfp_vertices.len();

    println!("After dedup with tolerance {}: {} -> {} vertices\n", tolerance, before_dedup, after_dedup);

    // For a square × square Minkowski sum, the theoretical NFP should have approximately
    // 4 + 4 = 8 vertices (the convex hull of the sum)
    println!("Theoretical NFP for square × square: 8 vertices");
    println!("Actual result: {} vertices\n", after_dedup);

    if after_dedup == 16 {
        println!("❌ BUG CONFIRMED: Got 16 instead of 8");
        println!("   The algorithm is creating 2 vertices per edge of input polygons");
        println!("   Reason: For each edge of B and each vertex of A, TWO offsets are created");
        println!("   (one for edge start, one for edge end), when only ONE is needed per edge\n");

        // Analyze the actual points
        println!("First 8 generated points:");
        for (i, p) in nfp_vertices.iter().enumerate().take(8) {
            println!("  [{}]: ({:.6}, {:.6})", i, p.x, p.y);
        }

        // Check for near-duplicates with larger tolerance
        let mut near_dups_1e6 = 0;
        let mut near_dups_1e4 = 0;
        let mut near_dups_1e2 = 0;

        for i in 0..nfp_vertices.len() {
            for j in (i+1)..nfp_vertices.len() {
                let dx = (nfp_vertices[i].x - nfp_vertices[j].x).abs();
                let dy = (nfp_vertices[i].y - nfp_vertices[j].y).abs();

                if dx < 1e-6 && dy < 1e-6 { near_dups_1e6 += 1; }
                if dx < 1e-4 && dy < 1e-4 { near_dups_1e4 += 1; }
                if dx < 1e-2 && dy < 1e-2 { near_dups_1e2 += 1; }
            }
        }

        println!("\nDuplicate pairs with larger tolerances:");
        println!("  1e-6:  {}", near_dups_1e6);
        println!("  1e-4:  {}", near_dups_1e4);
        println!("  1e-2:  {}", near_dups_1e2);
    }
}

#[test]
fn verify_algorithm_intent() {
    println!("\n=== UNDERSTANDING MINKOWSKI SUM ===\n");
    println!("For NFP of A ⊕ B (Minkowski sum):");
    println!("- Slide polygon A around the perimeter of polygon B");
    println!("- For EACH EDGE of B, compute the offset points");
    println!("- Each edge E of B contributes ONE vertex when A's reference point is at E");
    println!("\nCurrent algorithm does:");
    println!("- For each EDGE of B (start and end vertices)");
    println!("- For each VERTEX of A");
    println!("- Creates BOTH offset_to_start AND offset_to_end");
    println!("\nPROBLEM:");
    println!("- offset_to_end for edge E[i] == offset_to_start for edge E[i+1]");
    println!("- These SHOULD be deduplicated but aren't with tolerance 1e-10");
    println!("\nSOLUTION:");
    println!("- Only use offset_to_start for each edge (not both start AND end)");
    println!("- Or: Use edge direction vector for Minkowski sum");
    println!("- Or: Increase deduplication tolerance OR use absolute dedup");
}
