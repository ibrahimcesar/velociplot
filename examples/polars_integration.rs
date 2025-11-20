//! Polars integration example
//!
//! Demonstrates direct plotting from Polars DataFrames,
//! which is essential for data analysis workflows in Rust.

use polars::prelude::*;
use velociplot::prelude::*;

#[cfg(feature = "polars-support")]
use velociplot::integration::PolarsSeries;

fn main() -> Result<()> {
    #[cfg(not(feature = "polars-support"))]
    {
        println!("⚠️  This example requires the 'polars-support' feature");
        println!(
            "Run with: cargo run --example polars_integration --features \"raster,polars-support\""
        );
        return Ok(());
    }

    #[cfg(feature = "polars-support")]
    {
        println!("🦖 velociplot - Polars integration example");

        // ============================================================
        // Example 1: Simple DataFrame column to series
        // ============================================================
        {
            println!("\n📊 Example 1: Simple DataFrame column (auto x-coordinates)");

            let df = df! {
                "temperature" => &[20.0, 21.5, 23.0, 24.5, 26.0],
            }
            .unwrap();

            let series = df.to_series("temperature")?;

            let plot = LinePlot::new(series)
                .color(Color::from_hex("#e74c3c").unwrap())
                .line_width(2.5)
                .label("Temperature (°C)");

            let bounds = plot.bounds().unwrap().with_padding(0.1);
            let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
            canvas.fill_background(&Color::WHITE.to_rgba())?;

            let x_axis = Axis::new(AxisPosition::Bottom)
                .label("Time Index")
                .tick_count(6)
                .show_grid(true);

            let y_axis = Axis::new(AxisPosition::Left)
                .label("Temperature (°C)")
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

            canvas.save_png("examples/images/polars_simple.png")?;
            println!("  ✓ Simple DataFrame plot saved");
        }

        // ============================================================
        // Example 2: Custom x and y columns
        // ============================================================
        {
            println!("\n📊 Example 2: Custom x and y columns");

            // Create time series data
            let time: Vec<f64> = (0..50).map(|i| i as f64 * 0.2).collect();
            let values: Vec<f64> = time.iter().map(|&t| (t * 0.5_f64).sin() * 5.0).collect();

            let df = df! {
                "time" => &time,
                "value" => &values,
            }
            .unwrap();

            let series = df.to_series_xy("time", "value")?;

            let plot = LinePlot::new(series)
                .color(Color::from_hex("#3498db").unwrap())
                .line_width(2.5)
                .label("Sensor Data");

            let bounds = plot.bounds().unwrap().with_padding(0.1);
            let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
            canvas.fill_background(&Color::WHITE.to_rgba())?;

            let x_axis = Axis::new(AxisPosition::Bottom)
                .label("Time (s)")
                .tick_count(6)
                .show_grid(true);

            let y_axis = Axis::new(AxisPosition::Left)
                .label("Value")
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

            canvas.save_png("examples/images/polars_custom_xy.png")?;
            println!("  ✓ Custom x,y columns plot saved");
        }

        // ============================================================
        // Example 3: Multiple series from DataFrame
        // ============================================================
        {
            println!("\n📊 Example 3: Multiple series from DataFrame columns");

            // Create multi-column time series data
            let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.2).collect();
            let cpu: Vec<f64> = x
                .iter()
                .map(|&t| 50.0 + (t * 0.3_f64).sin() * 20.0)
                .collect();
            let memory: Vec<f64> = x
                .iter()
                .map(|&t| 60.0 + (t * 0.5_f64).cos() * 15.0)
                .collect();
            let disk: Vec<f64> = x
                .iter()
                .map(|&t| 30.0 + (t * 0.2_f64).sin() * 10.0)
                .collect();

            let df = df! {
                "time" => &x,
                "cpu" => &cpu,
                "memory" => &memory,
                "disk" => &disk,
            }
            .unwrap();

            let series_list = df.to_multi_series("time", &["cpu", "memory", "disk"])?;

            let plot1 = LinePlot::new(series_list[0].clone())
                .color(Color::from_hex("#e74c3c").unwrap())
                .line_width(2.5)
                .label("CPU Usage");

            let plot2 = LinePlot::new(series_list[1].clone())
                .color(Color::from_hex("#3498db").unwrap())
                .line_width(2.5)
                .label("Memory Usage");

            let plot3 = LinePlot::new(series_list[2].clone())
                .color(Color::from_hex("#2ecc71").unwrap())
                .line_width(2.5)
                .label("Disk Usage");

            let bounds = plot1
                .bounds()
                .unwrap()
                .union(&plot2.bounds().unwrap())
                .union(&plot3.bounds().unwrap())
                .with_padding(0.1);

            let mut canvas = SkiaCanvas::new(800, 720, bounds)?;
            canvas.fill_background(&Color::WHITE.to_rgba())?;

            let x_axis = Axis::new(AxisPosition::Bottom)
                .label("Time (s)")
                .tick_count(6)
                .show_grid(true);

            let y_axis = Axis::new(AxisPosition::Left)
                .label("Usage (%)")
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

            canvas.save_png("examples/images/polars_multi_series.png")?;
            println!("  ✓ Multiple series from DataFrame saved");
        }

        // ============================================================
        // Example 4: Real-world data analysis scenario
        // ============================================================
        {
            println!("\n📊 Example 4: Real-world data analysis (sales data)");

            // Simulated sales data by quarter
            let quarter_nums: Vec<f64> = (0..4).map(|i| i as f64).collect();
            let product_a: Vec<f64> = vec![150.0, 180.0, 220.0, 250.0];
            let product_b: Vec<f64> = vec![120.0, 140.0, 160.0, 190.0];
            let product_c: Vec<f64> = vec![100.0, 130.0, 170.0, 210.0];

            let df = df! {
                "quarter" => &quarter_nums,
                "product_a" => &product_a,
                "product_b" => &product_b,
                "product_c" => &product_c,
            }
            .unwrap();

            let series_list =
                df.to_multi_series("quarter", &["product_a", "product_b", "product_c"])?;

            let colors = [
                Color::from_hex("#9b59b6").unwrap(),
                Color::from_hex("#e67e22").unwrap(),
                Color::from_hex("#1abc9c").unwrap(),
            ];

            let labels = ["Product A", "Product B", "Product C"];

            let plots: Vec<_> = series_list
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    LinePlot::new(s.clone())
                        .color(colors[i])
                        .line_width(3.0)
                        .label(labels[i])
                })
                .collect();

            let mut bounds = plots[0].bounds().unwrap();
            for plot in &plots[1..] {
                bounds = bounds.union(&plot.bounds().unwrap());
            }
            bounds = bounds.with_padding(0.15);

            let mut canvas = SkiaCanvas::new(800, 720, bounds)?;
            canvas.fill_background(&Color::WHITE.to_rgba())?;

            let x_axis = Axis::new(AxisPosition::Bottom)
                .label("Quarter")
                .tick_count(5)
                .show_grid(true);

            let y_axis = Axis::new(AxisPosition::Left)
                .label("Sales (thousands)")
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

            canvas.save_png("examples/images/polars_sales_analysis.png")?;
            println!("  ✓ Sales analysis plot saved");
        }

        println!("\n✅ All Polars integration examples completed!");
        println!("\n💡 Key features:");
        println!("  • DataFrame column → Series with auto x-coordinates");
        println!("  • DataFrame columns → Series with custom x,y");
        println!("  • Multiple DataFrame columns → Multiple Series");
        println!("  • Direct integration with Polars data pipelines");
        println!("\n📚 Use cases:");
        println!("  • Data analysis and visualization");
        println!("  • Time series analysis");
        println!("  • Business intelligence dashboards");
        println!("  • ETL pipeline visualization");
    }

    Ok(())
}
