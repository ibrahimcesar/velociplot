//! Heatmap example
//!
//! Demonstrates 2D matrix visualization with color coding using heatmaps.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Heatmap example");

    // ============================================================
    // Example 1: Simple correlation matrix
    // ============================================================
    {
        println!("\n📊 Example 1: Simple correlation matrix");

        let data = vec![
            vec![1.00, 0.85, 0.42, -0.15],
            vec![0.85, 1.00, 0.58, -0.05],
            vec![0.42, 0.58, 1.00, 0.28],
            vec![-0.15, -0.05, 0.28, 1.00],
        ];

        let heatmap = Heatmap::new(data)?
            .colormap(Colormap::coolwarm())
            .show_values(true)
            .value_format(ValueFormat::Decimal(2));

        let bounds = heatmap.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(800, 800, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        heatmap.draw(&mut canvas)?;

        canvas.save_png("examples/images/heatmap_correlation.png")?;
        println!("  ✓ Correlation matrix saved");
    }

    // ============================================================
    // Example 2: Temperature data (viridis colormap)
    // ============================================================
    {
        println!("\n📊 Example 2: Temperature data");

        // Simulated temperature data
        let mut data = Vec::new();
        for i in 0..10 {
            let mut row = Vec::new();
            for j in 0..15 {
                let temp = 15.0 + 10.0 * ((i as f64 / 5.0).sin() * (j as f64 / 7.0).cos());
                row.push(temp);
            }
            data.push(row);
        }

        let heatmap = Heatmap::new(data)?.colormap(Colormap::viridis());

        let bounds = heatmap.bounds().unwrap().with_padding(0.05);
        let mut canvas = SkiaCanvas::new(1000, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        heatmap.draw(&mut canvas)?;

        canvas.save_png("examples/images/heatmap_temperature.png")?;
        println!("  ✓ Temperature heatmap saved");
    }

    // ============================================================
    // Example 3: Confusion matrix
    // ============================================================
    {
        println!("\n📊 Example 3: Confusion matrix");

        // Confusion matrix for 4-class classification
        let data = vec![
            vec![45.0, 3.0, 1.0, 1.0],
            vec![2.0, 38.0, 5.0, 5.0],
            vec![1.0, 4.0, 42.0, 3.0],
            vec![0.0, 2.0, 2.0, 46.0],
        ];

        let heatmap = Heatmap::new(data)?
            .colormap(Colormap::plasma())
            .show_values(true)
            .value_format(ValueFormat::Integer);

        let bounds = heatmap.bounds().unwrap().with_padding(0.15);
        let mut canvas = SkiaCanvas::new(800, 800, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        heatmap.draw(&mut canvas)?;

        canvas.save_png("examples/images/heatmap_confusion.png")?;
        println!("  ✓ Confusion matrix saved");
    }

    // ============================================================
    // Example 4: Distance matrix
    // ============================================================
    {
        println!("\n📊 Example 4: Distance matrix");

        // Calculate Euclidean distance matrix for 5 points
        let points: Vec<(f64, f64)> =
            vec![(1.0, 2.0), (3.0, 4.0), (6.0, 2.0), (8.0, 7.0), (2.0, 9.0)];

        let mut data = Vec::new();
        for p1 in &points {
            let mut row = Vec::new();
            for p2 in &points {
                let dx = p1.0 - p2.0;
                let dy = p1.1 - p2.1;
                let dist = (dx * dx + dy * dy).sqrt();
                row.push(dist);
            }
            data.push(row);
        }

        let heatmap = Heatmap::new(data)?
            .colormap(Colormap::inferno())
            .show_values(true)
            .value_format(ValueFormat::Decimal(1));

        let bounds = heatmap.bounds().unwrap().with_padding(0.15);
        let mut canvas = SkiaCanvas::new(700, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        heatmap.draw(&mut canvas)?;

        canvas.save_png("examples/images/heatmap_distance.png")?;
        println!("  ✓ Distance matrix saved");
    }

    // ============================================================
    // Example 5: Time series data (large matrix)
    // ============================================================
    {
        println!("\n📊 Example 5: Time series data");

        // Generate synthetic time series data
        let mut data = Vec::new();
        for i in 0..20 {
            let mut row = Vec::new();
            for j in 0..30 {
                let value = 50.0
                    + 20.0 * ((i as f64 * 0.3).sin() * (j as f64 * 0.2).cos())
                    + 10.0 * ((i as f64 * 0.5).cos());
                row.push(value);
            }
            data.push(row);
        }

        let heatmap = Heatmap::new(data)?.colormap(Colormap::magma());

        let bounds = heatmap.bounds().unwrap().with_padding(0.05);
        let mut canvas = SkiaCanvas::new(1200, 800, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        heatmap.draw(&mut canvas)?;

        canvas.save_png("examples/images/heatmap_timeseries.png")?;
        println!("  ✓ Time series heatmap saved");
    }

    // ============================================================
    // Example 6: Small matrix with all colormaps
    // ============================================================
    {
        println!("\n📊 Example 6: Comparing colormaps");

        let data = vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![2.0, 4.0, 6.0, 8.0, 10.0],
            vec![3.0, 6.0, 9.0, 12.0, 15.0],
        ];

        let colormaps = vec![
            ("viridis", Colormap::viridis()),
            ("plasma", Colormap::plasma()),
            ("inferno", Colormap::inferno()),
            ("coolwarm", Colormap::coolwarm()),
        ];

        for (name, colormap) in colormaps {
            let heatmap = Heatmap::new(data.clone())?
                .colormap(colormap)
                .show_values(true)
                .value_format(ValueFormat::Integer);

            let bounds = heatmap.bounds().unwrap().with_padding(0.15);
            let mut canvas = SkiaCanvas::new(700, 500, bounds)?;
            canvas.fill_background(&Color::WHITE.to_rgba())?;

            heatmap.draw(&mut canvas)?;

            let filename = format!("examples/images/heatmap_{}.png", name);
            canvas.save_png(&filename)?;
            println!("  ✓ {} colormap saved", name);
        }
    }

    println!("\n✅ All heatmap examples completed!");
    println!("\n💡 Key features:");
    println!("  • 2D matrix visualization with color coding");
    println!("  • Multiple scientific colormaps");
    println!("  • Optional value display in cells");
    println!("  • Customizable value formatting");
    println!("\n📚 Use cases:");
    println!("  • Correlation matrices");
    println!("  • Confusion matrices (ML/classification)");
    println!("  • Distance/similarity matrices");
    println!("  • Time series data visualization");
    println!("  • Spatial data analysis");

    Ok(())
}
