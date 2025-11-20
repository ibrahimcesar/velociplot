//! BarLegend (color bar) examples
//!
//! Demonstrates color bars for heatmaps and gradient visualizations with different
//! orientations, colormaps, and positioning options.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - BarLegend examples");

    // ============================================================
    // Example 1: Vertical BarLegend with Heatmap (Viridis)
    // ============================================================
    {
        println!("\n📊 Example 1: Vertical BarLegend with Heatmap");

        // Create sample data
        let width = 20;
        let height = 15;
        let mut data = vec![vec![0.0; width]; height];

        for i in 0..height {
            for j in 0..width {
                let x = j as f64 / width as f64;
                let y = i as f64 / height as f64;
                data[i][j] = ((x * 4.0).sin() * (y * 4.0).cos() + 1.0) * 50.0;
            }
        }

        let heatmap = Heatmap::new(data.clone())?
            .colormap(Colormap::viridis())
            .show_values(false);

        let bounds = heatmap.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(1000, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        heatmap.draw(&mut canvas)?;

        // Add BarLegend on the right
        let bar_legend = BarLegend::new(0.0, 100.0, Colormap::viridis())
            .label("Temperature (°C)")
            .position(BarLegendPosition::Right)
            .orientation(BarOrientation::Vertical)
            .dimensions(30.0, 250.0)
            .num_ticks(6);

        bar_legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("X Position");
        let y_axis = Axis::new(AxisPosition::Left).label("Y Position");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/bar_legend_vertical.png")?;
        println!("  ✓ Vertical BarLegend saved");
    }

    // ============================================================
    // Example 2: Horizontal BarLegend (Plasma colormap)
    // ============================================================
    {
        println!("\n📊 Example 2: Horizontal BarLegend");

        let width = 25;
        let height = 20;
        let mut data = vec![vec![0.0; width]; height];

        for i in 0..height {
            for j in 0..width {
                let r = ((i as f64 - height as f64 / 2.0).powi(2)
                    + (j as f64 - width as f64 / 2.0).powi(2))
                .sqrt();
                data[i][j] = 100.0 * (-r / 10.0).exp();
            }
        }

        let heatmap = Heatmap::new(data)?
            .colormap(Colormap::plasma())
            .show_values(false);

        let bounds = heatmap.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(900, 800, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        heatmap.draw(&mut canvas)?;

        // Add horizontal BarLegend at bottom
        let bar_legend = BarLegend::new(0.0, 100.0, Colormap::plasma())
            .label("Intensity")
            .position(BarLegendPosition::Bottom)
            .orientation(BarOrientation::Horizontal)
            .dimensions(300.0, 25.0)
            .num_ticks(5);

        bar_legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("X");
        let y_axis = Axis::new(AxisPosition::Left).label("Y");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/bar_legend_horizontal.png")?;
        println!("  ✓ Horizontal BarLegend saved");
    }

    // ============================================================
    // Example 3: Different colormaps comparison
    // ============================================================
    {
        println!("\n📊 Example 3: Colormap comparison");

        let width = 15;
        let height = 15;
        let mut data = vec![vec![0.0; width]; height];

        for i in 0..height {
            for j in 0..width {
                data[i][j] = (i * width + j) as f64;
            }
        }

        let colormaps = vec![
            (Colormap::viridis(), "Viridis"),
            (Colormap::plasma(), "Plasma"),
            (Colormap::inferno(), "Inferno"),
            (Colormap::coolwarm(), "Coolwarm"),
        ];

        for (colormap, name) in colormaps {
            let heatmap = Heatmap::new(data.clone())?
                .colormap(colormap.clone())
                .show_values(false);

            let bounds = heatmap.bounds().unwrap().with_padding(0.15);
            let mut canvas = SkiaCanvas::new(900, 700, bounds)?;
            canvas.fill_background(&Color::WHITE.to_rgba())?;

            heatmap.draw(&mut canvas)?;

            let bar_legend = BarLegend::new(0.0, (width * height) as f64, colormap)
                .label(&format!("{} Scale", name))
                .position(BarLegendPosition::Right)
                .dimensions(35.0, 280.0)
                .num_ticks(5);

            bar_legend.draw(&mut canvas)?;

            let filename = format!("examples/images/bar_legend_{}.png", name.to_lowercase());
            canvas.save_png(&filename)?;
            println!("  ✓ {} colormap saved", name);
        }
    }

    // ============================================================
    // Example 4: Custom positioning
    // ============================================================
    {
        println!("\n📊 Example 4: Custom positioning");

        let width = 18;
        let height = 18;
        let mut data = vec![vec![0.0; width]; height];

        for i in 0..height {
            for j in 0..width {
                let x = j as f64 / width as f64 - 0.5;
                let y = i as f64 / height as f64 - 0.5;
                data[i][j] = (x * x + y * y).sqrt() * 100.0;
            }
        }

        let heatmap = Heatmap::new(data)?
            .colormap(Colormap::magma())
            .show_values(false);

        let bounds = heatmap.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(900, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        heatmap.draw(&mut canvas)?;

        // Custom positioned BarLegend
        let bar_legend = BarLegend::new(0.0, 100.0, Colormap::magma())
            .label("Distance")
            .position(BarLegendPosition::Custom(750, 100))
            .orientation(BarOrientation::Vertical)
            .dimensions(25.0, 200.0)
            .num_ticks(4)
            .show_border(true);

        bar_legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("X");
        let y_axis = Axis::new(AxisPosition::Left).label("Y");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/bar_legend_custom.png")?;
        println!("  ✓ Custom positioned BarLegend saved");
    }

    // ============================================================
    // Example 5: Different tick counts
    // ============================================================
    {
        println!("\n📊 Example 5: Different tick counts");

        let width = 20;
        let height = 15;
        let mut data = vec![vec![0.0; width]; height];

        for i in 0..height {
            for j in 0..width {
                data[i][j] = ((i + j) as f64 / (width + height) as f64) * 200.0 - 100.0;
            }
        }

        let heatmap = Heatmap::new(data)?
            .colormap(Colormap::coolwarm())
            .show_values(false);

        let bounds = heatmap.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(1000, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        heatmap.draw(&mut canvas)?;

        // BarLegend with many ticks
        let bar_legend = BarLegend::new(-100.0, 100.0, Colormap::coolwarm())
            .label("Diverging Scale")
            .position(BarLegendPosition::Right)
            .dimensions(30.0, 300.0)
            .num_ticks(11); // More ticks for precision

        bar_legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("X");
        let y_axis = Axis::new(AxisPosition::Left).label("Y");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/bar_legend_ticks.png")?;
        println!("  ✓ Multi-tick BarLegend saved");
    }

    // ============================================================
    // Example 6: No ticks/values (minimal)
    // ============================================================
    {
        println!("\n📊 Example 6: Minimal BarLegend");

        let width = 22;
        let height = 16;
        let mut data = vec![vec![0.0; width]; height];

        for i in 0..height {
            for j in 0..width {
                data[i][j] = ((i as f64).sin() + (j as f64).cos()) * 25.0 + 50.0;
            }
        }

        let heatmap = Heatmap::new(data)?
            .colormap(Colormap::viridis())
            .show_values(false);

        let bounds = heatmap.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(900, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        heatmap.draw(&mut canvas)?;

        // Minimal BarLegend without ticks/values
        let bar_legend = BarLegend::new(0.0, 100.0, Colormap::viridis())
            .label("Value")
            .position(BarLegendPosition::Right)
            .dimensions(20.0, 250.0)
            .show_ticks(false)
            .show_values(false)
            .show_border(true);

        bar_legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("X");
        let y_axis = Axis::new(AxisPosition::Left).label("Y");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/bar_legend_minimal.png")?;
        println!("  ✓ Minimal BarLegend saved");
    }

    println!("\n✅ All BarLegend examples completed!");
    println!("\n💡 BarLegend Features:");
    println!("  • Vertical and horizontal orientations");
    println!("  • All colormaps supported (Viridis, Plasma, Inferno, Coolwarm, Magma)");
    println!("  • Customizable positioning (Right, Bottom, Custom)");
    println!("  • Adjustable tick counts and labels");
    println!("  • Optional borders, ticks, and value labels");

    Ok(())
}
