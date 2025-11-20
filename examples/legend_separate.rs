//! Separate legend canvas example
//!
//! Demonstrates creating separate canvases for plot and legend,
//! which can be saved independently or combined. This is useful for
//! scientific publications that require separate figure and legend files.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Separate legend canvas example");

    // Create sample data
    let data1 = Series::from_function(0.0, 10.0, 50, |x| x.powi(2) * 0.1);
    let data2 = Series::from_function(0.0, 10.0, 50, |x| x * 1.5);
    let data3 = Series::from_function(0.0, 10.0, 50, |x| (x * 0.5).sin() * 3.0 + 5.0);

    // Create line plots
    let line1 = LinePlot::new(data1)
        .color(Color::from_hex("#3498db").unwrap())
        .line_width(2.5)
        .label("Quadratic");

    let line2 = LinePlot::new(data2)
        .color(Color::from_hex("#e74c3c").unwrap())
        .line_width(2.5)
        .label("Linear");

    let line3 = LinePlot::new(data3)
        .color(Color::from_hex("#2ecc71").unwrap())
        .line_width(2.5)
        .label("Sinusoidal");

    // Calculate bounds
    let bounds1 = line1.bounds().unwrap();
    let bounds2 = line2.bounds().unwrap();
    let bounds3 = line3.bounds().unwrap();
    let bounds = bounds1.union(&bounds2).union(&bounds3).with_padding(0.1);

    // ============================================================
    // APPROACH 1: Plot-only canvas (standard 800x600)
    // ============================================================
    let mut plot_canvas = SkiaCanvas::new(800, 600, bounds)?;
    plot_canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("X Axis")
        .tick_count(6)
        .show_grid(true);

    let y_axis = Axis::new(AxisPosition::Left)
        .label("Y Axis")
        .tick_count(8)
        .show_grid(true);

    x_axis.draw(&mut plot_canvas)?;
    y_axis.draw(&mut plot_canvas)?;

    // Draw plots
    line1.draw(&mut plot_canvas)?;
    line2.draw(&mut plot_canvas)?;
    line3.draw(&mut plot_canvas)?;

    // Save plot without legend
    plot_canvas.save_png("examples/images/legend_separate_plot.png")?;
    println!("✓ Plot-only saved (no legend)");

    // ============================================================
    // APPROACH 2: Legend-only canvas (800x100)
    // ============================================================
    let mut legend_canvas = SkiaCanvas::new_legend_canvas(800, 100, bounds)?;
    legend_canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Create legend centered in the legend canvas
    let mut legend = Legend::new().position(LegendPosition::Custom(400, 50)); // Center of 800x100 canvas

    for line in [&line1, &line2, &line3] {
        if let Some(entry) = line.legend_entry() {
            legend = legend.add_entry(entry);
        }
    }

    legend.draw(&mut legend_canvas)?;

    // Save legend separately
    legend_canvas.save_png("examples/images/legend_separate_legend.png")?;
    println!("✓ Legend-only saved (separate file)");

    // ============================================================
    // APPROACH 3: Combined canvas (plot + legend fused)
    // ============================================================
    let combined_canvas = plot_canvas.combine_vertical(&legend_canvas)?;
    combined_canvas.save_png("examples/images/legend_separate_combined.png")?;
    println!("✓ Combined plot+legend saved (fused)");

    println!("\n📊 Three files created:");
    println!("  1. legend_separate_plot.png     - Plot only (800x600)");
    println!("  2. legend_separate_legend.png   - Legend only (800x100)");
    println!("  3. legend_separate_combined.png - Combined (800x700)");

    println!("\n💡 Use cases:");
    println!("  • Manuscript submission: Submit plot and legend as separate files");
    println!("  • Flexible layout: Import separately into LaTeX/Word for custom positioning");
    println!("  • Publication ready: Combined version for presentations/posters");
    println!("  • Journal requirements: Some journals require separate figure files");

    Ok(())
}
