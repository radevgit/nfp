use nfp::prelude::*;

/// Number of iterations to run the NFP calculation
const ITERATIONS: usize = 1000;

fn main() {
    // Generate two different 100-edge polygons with fixed seeds
    let poly_a = nfp::utils::generate_ellipse_polygon(100, 50.0, 20.0, 2.0, 42);
    let poly_b = nfp::utils::generate_ellipse_polygon(100, 50.0, 20.0, 2.0, 43);

    for _ in 0..ITERATIONS {
        let _ = NFPConvex::nfp(&poly_a, &poly_b);
    }
}

/*
samply record cargo run --release --example nfp_convex_100


 */