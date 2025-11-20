//! ndarray integration example
//!
//! Demonstrates direct plotting from ndarray arrays,
//! which is essential for scientific computing workflows in Rust.

use ndarray::{array, Array1, Array2};
use velociplot::prelude::*;

#[cfg(feature = "ndarray-support")]
use velociplot::integration::{NdarrayMultiSeries, NdarraySeries};

fn main() -> Result<()> {
    #[cfg(not(feature = "ndarray-support"))]
    {
        println!("⚠️  This example requires the 'ndarray-support' feature");
        println!("Run with: cargo run --example ndarray_integration --features \"raster,ndarray-support\"");
        return Ok(());
    }

    #[cfg(feature = "ndarray-support")]
    {
        println!("🦖 velociplot - ndarray integration example");

        // ============================================================
        // Example 1: Simple 1D array to series
        // ============================================================
        {
            println!("\n📊 Example 1: Simple 1D array (auto x-coordinates)");

            let y: Array1<f64> = array![1.0, 4.0, 9.0, 16.0, 25.0];
            let series = y.to_series()?;

            let plot = LinePlot::new(series)
                .color(Color::from_hex("#3498db").unwrap())
                .line_width(2.5)
                .label("y = x²");

            let bounds = plot.bounds().unwrap().with_padding(0.1);
            let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
            canvas.fill_background(&Color::WHITE.to_rgba())?;

            let x_axis = Axis::new(AxisPosition::Bottom)
                .label("Index")
                .tick_count(6)
                .show_grid(true);

            let y_axis = Axis::new(AxisPosition::Left)
                .label("Value")
                .tick_count(6)
                .show_grid(true);

            x_axis.draw(&mut canvas)?;
            y_axis.draw(&mut canvas)?;
            plot.draw(&mut canvas)?;

            if let Some(entry) = plot.legend_entry() {
                let legend = Legend::new()
                    .add_entry(entry)
                    .position(LegendPosition::UpperLeft);
                legend.draw(&mut canvas)?;
            }

            canvas.save_png("examples/images/ndarray_simple.png")?;
            println!("  ✓ Simple array plot saved");
        }

        // ============================================================
        // Example 2: Custom x and y arrays
        // ============================================================
        {
            println!("\n📊 Example 2: Custom x and y arrays");

            let x = Array1::linspace(0.0, 10.0, 50);
            let y = x.mapv(|val: f64| (val * 0.5).sin() * 5.0);

            let series = y.to_series_with_x(&x)?;

            let plot = LinePlot::new(series)
                .color(Color::from_hex("#e74c3c").unwrap())
                .line_width(2.5)
                .label("sin(0.5x) × 5");

            let bounds = plot.bounds().unwrap().with_padding(0.1);
            let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
            canvas.fill_background(&Color::WHITE.to_rgba())?;

            let x_axis = Axis::new(AxisPosition::Bottom)
                .label("X")
                .tick_count(6)
                .show_grid(true);

            let y_axis = Axis::new(AxisPosition::Left)
                .label("Y")
                .tick_count(8)
                .show_grid(true);

            x_axis.draw(&mut canvas)?;
            y_axis.draw(&mut canvas)?;
            plot.draw(&mut canvas)?;

            if let Some(entry) = plot.legend_entry() {
                let legend = Legend::new()
                    .add_entry(entry)
                    .position(LegendPosition::UpperRight);
                legend.draw(&mut canvas)?;
            }

            canvas.save_png("examples/images/ndarray_custom_xy.png")?;
            println!("  ✓ Custom x,y arrays plot saved");
        }

        // ============================================================
        // Example 3: Multiple series from 2D array columns
        // ============================================================
        {
            println!("\n📊 Example 3: Multiple series from 2D array (columns)");

            // Create a 2D array: first column = x, other columns = different y series
            let x = Array1::linspace(0.0, 10.0, 50);
            let y1 = x.mapv(|v: f64| v.powi(2) * 0.1);
            let y2 = x.mapv(|v: f64| v * 1.5);
            let y3 = x.mapv(|v: f64| (v * 0.5).sin() * 3.0 + 5.0);

            let mut data = Array2::zeros((50, 4));
            data.column_mut(0).assign(&x);
            data.column_mut(1).assign(&y1);
            data.column_mut(2).assign(&y2);
            data.column_mut(3).assign(&y3);

            let series_list = data.to_multi_series()?;

            let plot1 = LinePlot::new(series_list[0].clone())
                .color(Color::from_hex("#3498db").unwrap())
                .line_width(2.5)
                .label("Quadratic");

            let plot2 = LinePlot::new(series_list[1].clone())
                .color(Color::from_hex("#e74c3c").unwrap())
                .line_width(2.5)
                .label("Linear");

            let plot3 = LinePlot::new(series_list[2].clone())
                .color(Color::from_hex("#2ecc71").unwrap())
                .line_width(2.5)
                .label("Sinusoidal");

            let bounds = plot1
                .bounds()
                .unwrap()
                .union(&plot2.bounds().unwrap())
                .union(&plot3.bounds().unwrap())
                .with_padding(0.1);

            let mut canvas = SkiaCanvas::new(800, 720, bounds)?;
            canvas.fill_background(&Color::WHITE.to_rgba())?;

            let x_axis = Axis::new(AxisPosition::Bottom)
                .label("X")
                .tick_count(6)
                .show_grid(true);

            let y_axis = Axis::new(AxisPosition::Left)
                .label("Y")
                .tick_count(8)
                .show_grid(true);

            x_axis.draw(&mut canvas)?;
            y_axis.draw(&mut canvas)?;

            plot1.draw(&mut canvas)?;
            plot2.draw(&mut canvas)?;
            plot3.draw(&mut canvas)?;

            let mut legend = Legend::new();
            for plot in [&plot1, &plot2, &plot3] {
                if let Some(entry) = plot.legend_entry() {
                    legend = legend.add_entry(entry);
                }
            }
            legend.draw(&mut canvas)?;

            canvas.save_png("examples/images/ndarray_multi_series.png")?;
            println!("  ✓ Multiple series from columns saved");
        }

        // ============================================================
        // Example 4: Multiple series from 2D array rows
        // ============================================================
        {
            println!("\n📊 Example 4: Multiple series from 2D array (rows)");

            let data = array![
                [1.0, 2.0, 3.0, 4.0, 5.0],
                [2.0, 4.0, 6.0, 8.0, 10.0],
                [1.0, 4.0, 9.0, 16.0, 25.0],
            ];

            let series_list = data.rows_to_series()?;

            let colors = [
                Color::from_hex("#3498db").unwrap(),
                Color::from_hex("#e74c3c").unwrap(),
                Color::from_hex("#2ecc71").unwrap(),
            ];

            let labels = ["Series 1", "Series 2", "Series 3"];

            let plots: Vec<_> = series_list
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    LinePlot::new(s.clone())
                        .color(colors[i])
                        .line_width(2.5)
                        .label(labels[i])
                })
                .collect();

            let mut bounds = plots[0].bounds().unwrap();
            for plot in &plots[1..] {
                bounds = bounds.union(&plot.bounds().unwrap());
            }
            bounds = bounds.with_padding(0.1);

            let mut canvas = SkiaCanvas::new(800, 720, bounds)?;
            canvas.fill_background(&Color::WHITE.to_rgba())?;

            let x_axis = Axis::new(AxisPosition::Bottom)
                .label("X")
                .tick_count(6)
                .show_grid(true);

            let y_axis = Axis::new(AxisPosition::Left)
                .label("Y")
                .tick_count(8)
                .show_grid(true);

            x_axis.draw(&mut canvas)?;
            y_axis.draw(&mut canvas)?;

            for plot in &plots {
                plot.draw(&mut canvas)?;
            }

            let mut legend = Legend::new();
            for plot in &plots {
                if let Some(entry) = plot.legend_entry() {
                    legend = legend.add_entry(entry);
                }
            }
            legend.draw(&mut canvas)?;

            canvas.save_png("examples/images/ndarray_rows.png")?;
            println!("  ✓ Multiple series from rows saved");
        }

        println!("\n✅ All ndarray integration examples completed!");
        println!("\n💡 Key features:");
        println!("  • Array1 → Series with auto x-coordinates");
        println!("  • Array1 + Array1 → Series with custom x,y");
        println!("  • Array2 columns → Multiple Series");
        println!("  • Array2 rows → Multiple Series");
        println!("\n📚 Use cases:");
        println!("  • Scientific computing with ndarray");
        println!("  • Numerical simulations");
        println!("  • Data analysis workflows");
        println!("  • Integration with numpy-style arrays");
    }

    Ok(())
}
