//! Scientific colormaps example
//!
//! Demonstrates the built-in perceptually uniform and scientific colormaps
//! available in velociplot, including viridis, plasma, inferno, magma,
//! coolwarm, jet, and grayscale.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Scientific colormaps example");

    // List of all available colormaps
    let colormaps = vec![
        Colormap::viridis(),
        Colormap::plasma(),
        Colormap::inferno(),
        Colormap::magma(),
        Colormap::coolwarm(),
        Colormap::jet(),
        Colormap::grayscale(),
    ];

    println!("\n📊 Generating colormap visualizations...");

    // Create a visualization for each colormap
    for colormap in &colormaps {
        let name = colormap.name();
        println!("  • {}", name);

        // Create bounds for a horizontal gradient bar
        let bounds = Bounds::new(0.0, 0.0, 100.0, 10.0);
        let mut canvas = SkiaCanvas::new(800, 150, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        // Draw gradient bar showing the colormap using vertical lines
        let bar_height = 60.0;
        let bar_y_start = 40.0;

        // Draw 760 vertical lines to show the gradient (one for each pixel)
        for i in 0..760 {
            let t = i as f32 / 759.0; // Normalized position 0.0 to 1.0
            let color = colormap.get(t);
            let x = i as f32 + 20.0; // Map to canvas width with margins

            // Draw vertical line
            canvas.draw_line_pixels(
                x,
                bar_y_start,
                x,
                bar_y_start + bar_height,
                &color.to_rgba(),
                1.0,
            )?;
        }

        // Draw title
        let title = format!("{} colormap", name);
        canvas.draw_text_pixels(&title, 20.0, 20.0, 18.0, &Color::BLACK.to_rgba())?;

        // Draw labels at start, middle, and end
        canvas.draw_text_pixels(
            "0.0",
            15.0,
            bar_y_start + bar_height + 15.0,
            12.0,
            &Color::BLACK.to_rgba(),
        )?;
        canvas.draw_text_pixels(
            "0.5",
            390.0,
            bar_y_start + bar_height + 15.0,
            12.0,
            &Color::BLACK.to_rgba(),
        )?;
        canvas.draw_text_pixels(
            "1.0",
            765.0,
            bar_y_start + bar_height + 15.0,
            12.0,
            &Color::BLACK.to_rgba(),
        )?;

        let filename = format!("examples/images/colormap_{}.png", name);
        canvas.save_png(&filename)?;
    }

    println!("\n✓ All 7 colormap visualizations saved to examples/images/");

    println!("\n📚 Colormap recommendations:");
    println!("  • viridis, plasma, inferno, magma: Perceptually uniform, colorblind-friendly");
    println!("  • coolwarm: Diverging data (e.g., temperature anomalies)");
    println!("  • grayscale: Print publications, accessibility");
    println!("  • jet: Legacy (not recommended for scientific visualization)");

    println!("\n💡 Usage example:");
    println!("  let colormap = Colormap::viridis();");
    println!("  let color = colormap.get(0.5); // Get color at 50% position");

    Ok(())
}
