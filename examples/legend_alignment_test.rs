//! Test different legend vertical alignment values to find the best default
//!
//! # Typography Alignment Explained
//!
//! Fonts have multiple vertical reference points:
//!
//! ```text
//! ┌─────────┐  ← 0.0 (top - ascender height)
//! │    h    │
//! │ ┌─x─┐   │  ← ~0.65 (x-height - visual center)
//! │ │   │   │
//! └─┴───┴───┘  ← ~0.75 (baseline)
//!   └───────┘  ← 1.0 (bottom - descender depth)
//!       g
//! ```
//!
//! This tool generates 6 plots with different alignment values so you can
//! visually compare and choose what looks best for your font.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Legend alignment test");
    println!("\nTypography reference points:");
    println!("  • 0.0  = Top of tallest letters (ascenders: h, k, l)");
    println!("  • 0.5  = Geometric middle of text box");
    println!("  • 0.65 = Middle of x-height (geometric center)");
    println!("  • 0.70 = Optical center (DEFAULT for JetBrains Mono)");
    println!("  • 0.75 = Baseline (where letters sit)");
    println!("  • 1.0  = Bottom of descenders (g, y, p)");
    println!("\nGenerating test plots...");

    // Test different alignment values
    let test_values = [0.50, 0.55, 0.60, 0.65, 0.70, 0.75];

    for &align_value in &test_values {
        create_test_plot(align_value)?;
    }

    println!("\n✓ Created {} test plots", test_values.len());
    println!("\n📊 How to choose:");
    println!("  1. Open examples/images/legend_align_*.png files");
    println!("  2. Look at where the colored line intersects the text");
    println!("  3. Pick the one where the line appears visually centered");
    println!("  4. The line should go through the optical middle of letters like 'a', 'e', 'n'");
    println!("\n💡 Tip: 0.70 and 0.75 work best with JetBrains Mono (our default font)");

    Ok(())
}

fn create_test_plot(align: f32) -> Result<()> {
    let data = Series::from_function(0.0, 5.0, 30, |x| x * x);

    let plot = LinePlot::new(data)
        .color(Color::from_hex("#e74c3c").unwrap())
        .line_width(2.5)
        .label(format!("align={:.2}", align));

    let bounds = plot.bounds().unwrap().with_padding(0.1);
    let mut canvas = SkiaCanvas::new(600, 400, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("X")
        .tick_count(6)
        .show_grid(true);

    let y_axis = Axis::new(AxisPosition::Left)
        .label("Y")
        .tick_count(6)
        .show_grid(true);

    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;
    plot.draw(&mut canvas)?;

    // Create legend with custom alignment
    if let Some(entry) = plot.legend_entry() {
        let legend = Legend::new()
            .add_entry(entry)
            .position(LegendPosition::UpperLeft)
            .line_vertical_align(align);
        legend.draw(&mut canvas)?;
    }

    let filename = format!("examples/images/legend_align_{:.2}.png", align);
    canvas.save_png(&filename)?;
    println!("  Created: {}", filename);

    Ok(())
}
