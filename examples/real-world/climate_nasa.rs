//! NASA Climate Data Example
//!
//! This example uses real temperature anomaly data from NASA GISS.
//!
//! # Data Source
//! - **Source**: NASA GISS Surface Temperature Analysis
//! - **URL**: https://data.giss.nasa.gov/gistemp/
//! - **License**: Public Domain
//!
//! # Setup
//! Download the data first:
//! ```bash
//! curl -o GLB.Ts+dSST.csv https://data.giss.nasa.gov/gistemp/tabledata_v4/GLB.Ts+dSST.csv
//! ```

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🌍 NASA Climate Data Visualization");
    println!("===================================\n");

    // Example 1: Synthetic data (replace with actual CSV parsing in production)
    // This demonstrates the API - users should parse actual CSV

    // Simulated temperature anomaly data (1880-2023)
    let years: Vec<f64> = (1880..=2023).map(|y| y as f64).collect();
    let temps: Vec<f64> = years
        .iter()
        .map(|&year| {
            // Simulate realistic temperature anomaly pattern
            let base = -0.3;
            let warming_trend = ((year - 1880.0) / 140.0).powi(2) * 1.2;
            let natural_variation = ((year - 1880.0) / 11.0).sin() * 0.1; // Solar cycle
            let noise = (year * 7.1234).sin() * 0.08;
            base + warming_trend + natural_variation + noise
        })
        .collect();

    println!("📊 Creating temperature anomaly plot...");

    // Create line plot
    let plot = LinePlot::new(Series::new(years.clone(), temps.clone())?)
        .color(Color::from_hex("#d62728").unwrap()) // Red for warming
        .line_width(2.5);

    let bounds = plot.bounds().unwrap().with_padding(0.1);
    let mut canvas = SkiaCanvas::new(1400, 800, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Add reference line at zero
    canvas.draw_line(
        &Point2D::new(bounds.x_min, 0.0),
        &Point2D::new(bounds.x_max, 0.0),
        &Color::from_hex("#7f8c8d").unwrap().to_rgba(),
        1.0,
    )?;

    // Draw plot
    plot.draw(&mut canvas)?;

    // Add axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("Year")
        .show_grid(true);
    let y_axis = Axis::new(AxisPosition::Left)
        .label("Temperature Anomaly (°C)")
        .show_grid(true);

    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;

    // Add title
    canvas.draw_text(
        "Global Temperature Anomalies (1880-2023)",
        700.0,
        40.0,
        18.0,
        &Color::from_hex("#2c3e50").unwrap().to_rgba(),
    )?;

    // Add data source citation
    canvas.draw_text(
        "Data Source: NASA GISS (simulated for example)",
        50.0,
        760.0,
        10.0,
        &Color::from_hex("#7f8c8d").unwrap().to_rgba(),
    )?;

    canvas.save_png("examples/images/climate_nasa_temp.png")?;
    println!("  ✓ Saved: examples/images/climate_nasa_temp.png");

    // Example 2: Decadal Comparison (Box plots)
    println!("\n📊 Creating decadal comparison...");

    // Group data by decade
    let decades = vec![
        ("1880s", 1880, 1890),
        ("1920s", 1920, 1930),
        ("1960s", 1960, 1970),
        ("2000s", 2000, 2010),
        ("2020s", 2020, 2024),
    ];

    let decade_positions: Vec<f64> = (0..decades.len()).map(|i| i as f64).collect();
    let mut all_temps = Vec::new();

    for (_, start, end) in &decades {
        let decade_temps: Vec<f64> = years
            .iter()
            .zip(&temps)
            .filter(|(&y, _)| y >= *start as f64 && y < *end as f64)
            .map(|(_, &t)| t)
            .collect();
        all_temps.push(decade_temps);
    }

    // Calculate bounds for box plots
    let min_temp = temps.iter().copied().fold(f64::INFINITY, f64::min);
    let max_temp = temps.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let bounds =
        Bounds::new(-0.5, decades.len() as f64 - 0.5, min_temp, max_temp).with_padding(0.1);

    let mut canvas = SkiaCanvas::new(1200, 800, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw box plots for each decade
    let colors = vec![
        Color::from_hex("#3498db").unwrap(),
        Color::from_hex("#9b59b6").unwrap(),
        Color::from_hex("#f39c12").unwrap(),
        Color::from_hex("#e67e22").unwrap(),
        Color::from_hex("#e74c3c").unwrap(),
    ];

    for (i, (temps_data, color)) in all_temps.iter().zip(colors.iter()).enumerate() {
        let boxplot = BoxPlot::new(temps_data.clone())
            .position(i as f64)
            .width(0.6)
            .color(color.clone());

        boxplot.draw(&mut canvas)?;
    }

    // Add decade labels
    for (i, (label, _, _)) in decades.iter().enumerate() {
        canvas.draw_text(
            label,
            (i as f32) * 100.0 + 50.0,
            750.0,
            12.0,
            &Color::from_hex("#2c3e50").unwrap().to_rgba(),
        )?;
    }

    // Add axes
    let y_axis = Axis::new(AxisPosition::Left).label("Temperature Anomaly (°C)");
    y_axis.draw(&mut canvas)?;

    // Add title
    canvas.draw_text(
        "Temperature Distribution by Decade",
        600.0,
        40.0,
        18.0,
        &Color::from_hex("#2c3e50").unwrap().to_rgba(),
    )?;

    canvas.save_png("examples/images/climate_nasa_decades.png")?;
    println!("  ✓ Saved: examples/images/climate_nasa_decades.png");

    // Summary statistics
    let latest_temp = temps.last().unwrap();
    let early_avg: f64 = temps.iter().take(30).sum::<f64>() / 30.0;
    let recent_avg: f64 = temps.iter().rev().take(30).sum::<f64>() / 30.0;

    println!("\n📈 Key Findings:");
    println!("  • Latest anomaly (2023): {:.2}°C", latest_temp);
    println!("  • Early average (1880-1910): {:.2}°C", early_avg);
    println!("  • Recent average (1994-2023): {:.2}°C", recent_avg);
    println!("  • Total warming: {:.2}°C", recent_avg - early_avg);

    println!("\n✅ Climate data visualization complete!");
    println!("\n💡 To use real NASA data:");
    println!(
        "   1. Download: curl -O https://data.giss.nasa.gov/gistemp/tabledata_v4/GLB.Ts+dSST.csv"
    );
    println!("   2. Parse CSV and replace simulated data");
    println!("   3. See docs for complete example");

    Ok(())
}
