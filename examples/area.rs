//! Area and stacked area chart examples
//!
//! Demonstrates area plots and stacked area charts for showing trends
//! and cumulative values over time.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Area Chart examples");

    // ============================================================
    // Example 1: Simple Area Plot
    // ============================================================
    {
        println!("\n📊 Example 1: Simple Area Plot");

        let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.2).collect();
        let y: Vec<f64> = x.iter().map(|&x| (x * 0.5).sin() * 50.0 + 50.0).collect();

        let area = AreaPlot::new(x, y)?
            .color(Color::from_hex("#3498db").unwrap())
            .opacity(0.6)
            .label("Temperature");

        let bounds = area.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(900, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        area.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Time (hours)");
        let y_axis = Axis::new(AxisPosition::Left).label("Temperature (°F)");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/area_simple.png")?;
        println!("  ✓ Simple area plot saved");
    }

    // ============================================================
    // Example 2: Stacked Area Plot (3 series)
    // ============================================================
    {
        println!("\n📊 Example 2: Stacked Area Plot");

        let x: Vec<f64> = (0..30).map(|i| i as f64).collect();
        let series1: Vec<f64> = x.iter().map(|&x| 20.0 + (x * 0.3).sin() * 5.0).collect();
        let series2: Vec<f64> = x.iter().map(|&x| 15.0 + (x * 0.2).cos() * 3.0).collect();
        let series3: Vec<f64> = x.iter().map(|&x| 10.0 + (x * 0.1).sin() * 2.0).collect();

        let stacked = StackedAreaPlot::new(x, vec![series1, series2, series3])?
            .labels(vec!["Product A", "Product B", "Product C"])
            .colors(vec![
                Color::from_hex("#3498db").unwrap(),
                Color::from_hex("#2ecc71").unwrap(),
                Color::from_hex("#e74c3c").unwrap(),
            ])
            .opacity(0.7);

        let bounds = stacked.bounds().unwrap().with_padding(0.12);
        let mut canvas = SkiaCanvas::new(1000, 650, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        stacked.draw(&mut canvas)?;

        // Add legend
        let mut legend = Legend::new().position(LegendPosition::UpperLeft);
        for entry in stacked.legend_entries() {
            legend = legend.add_entry(entry);
        }
        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Week");
        let y_axis = Axis::new(AxisPosition::Left).label("Sales (units)");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/area_stacked.png")?;
        println!("  ✓ Stacked area plot saved");
    }

    // ============================================================
    // Example 3: Traffic Over Time (5 series)
    // ============================================================
    {
        println!("\n📊 Example 3: Website Traffic Over Time");

        let hours: Vec<f64> = (0..24).map(|i| i as f64).collect();

        // Simulate traffic from different sources throughout the day
        let direct: Vec<f64> = hours
            .iter()
            .map(|&h| 30.0 + 20.0 * ((h - 12.0).abs() / 12.0))
            .collect();

        let organic: Vec<f64> = hours
            .iter()
            .map(|&h| 40.0 + 15.0 * (1.0 - (h - 14.0).abs() / 14.0))
            .collect();

        let social: Vec<f64> = hours
            .iter()
            .map(|&h| 20.0 + 10.0 * (1.0 - (h - 18.0).abs() / 18.0))
            .collect();

        let referral: Vec<f64> = hours
            .iter()
            .map(|&h| 15.0 + 8.0 * ((h * 0.5).sin() + 1.0) / 2.0)
            .collect();

        let email: Vec<f64> = hours
            .iter()
            .map(|&h| 10.0 + 5.0 * ((h * 0.3).cos() + 1.0) / 2.0)
            .collect();

        let stacked = StackedAreaPlot::new(hours, vec![direct, organic, social, referral, email])?
            .labels(vec![
                "Direct",
                "Organic Search",
                "Social Media",
                "Referral",
                "Email",
            ])
            .colors(vec![
                Color::from_hex("#3498db").unwrap(),
                Color::from_hex("#2ecc71").unwrap(),
                Color::from_hex("#f39c12").unwrap(),
                Color::from_hex("#9b59b6").unwrap(),
                Color::from_hex("#e74c3c").unwrap(),
            ])
            .opacity(0.75);

        let bounds = stacked.bounds().unwrap().with_padding(0.12);
        let mut canvas = SkiaCanvas::new(1100, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        stacked.draw(&mut canvas)?;

        let mut legend = Legend::new().position(LegendPosition::UpperRight);
        for entry in stacked.legend_entries() {
            legend = legend.add_entry(entry);
        }
        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Hour of Day");
        let y_axis = Axis::new(AxisPosition::Left).label("Visitors");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/area_traffic.png")?;
        println!("  ✓ Traffic area plot saved");
    }

    // ============================================================
    // Example 4: Resource Usage Over Time
    // ============================================================
    {
        println!("\n📊 Example 4: Resource Usage");

        let time: Vec<f64> = (0..60).map(|i| i as f64).collect();

        let cpu: Vec<f64> = time
            .iter()
            .map(|&t| 30.0 + 20.0 * ((t * 0.1).sin()) + 10.0 * ((t * 0.05).cos()))
            .collect();

        let memory: Vec<f64> = time
            .iter()
            .map(|&t| 40.0 + 15.0 * ((t * 0.08).sin()) + 8.0 * ((t * 0.12).cos()))
            .collect();

        let network: Vec<f64> = time
            .iter()
            .map(|&t| 20.0 + 10.0 * ((t * 0.15).sin()) + 5.0 * ((t * 0.07).cos()))
            .collect();

        let stacked = StackedAreaPlot::new(time, vec![cpu, memory, network])?
            .labels(vec!["CPU", "Memory", "Network"])
            .colors(vec![
                Color::from_hex("#e74c3c").unwrap(),
                Color::from_hex("#f39c12").unwrap(),
                Color::from_hex("#3498db").unwrap(),
            ])
            .opacity(0.7)
            .line_width(2.0);

        let bounds = stacked.bounds().unwrap().with_padding(0.12);
        let mut canvas = SkiaCanvas::new(1000, 650, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        stacked.draw(&mut canvas)?;

        let mut legend = Legend::new().position(LegendPosition::UpperLeft);
        for entry in stacked.legend_entries() {
            legend = legend.add_entry(entry);
        }
        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Time (seconds)");
        let y_axis = Axis::new(AxisPosition::Left).label("Resource Usage (%)");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/area_resources.png")?;
        println!("  ✓ Resource usage area plot saved");
    }

    // ============================================================
    // Example 5: Revenue Growth (Business Metrics)
    // ============================================================
    {
        println!("\n📊 Example 5: Revenue Growth");

        let months: Vec<f64> = (0..12).map(|i| i as f64).collect();

        let subscription: Vec<f64> = months.iter().map(|&m| 50.0 + m * 5.0).collect();

        let licensing: Vec<f64> = months
            .iter()
            .map(|&m| 30.0 + m * 3.0 + (m * 0.5).sin() * 5.0)
            .collect();

        let services: Vec<f64> = months.iter().map(|&m| 20.0 + m * 4.0).collect();

        let stacked = StackedAreaPlot::new(months, vec![subscription, licensing, services])?
            .labels(vec!["Subscription", "Licensing", "Services"])
            .colors(vec![
                Color::from_hex("#2ecc71").unwrap(),
                Color::from_hex("#3498db").unwrap(),
                Color::from_hex("#9b59b6").unwrap(),
            ])
            .opacity(0.7);

        let bounds = stacked.bounds().unwrap().with_padding(0.12);
        let mut canvas = SkiaCanvas::new(1000, 650, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        stacked.draw(&mut canvas)?;

        let mut legend = Legend::new().position(LegendPosition::UpperLeft);
        for entry in stacked.legend_entries() {
            legend = legend.add_entry(entry);
        }
        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Month");
        let y_axis = Axis::new(AxisPosition::Left).label("Revenue (K$)");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/area_revenue.png")?;
        println!("  ✓ Revenue growth area plot saved");
    }

    // ============================================================
    // Example 6: Area with Custom Baseline
    // ============================================================
    {
        println!("\n📊 Example 6: Area with Custom Baseline");

        let x: Vec<f64> = (0..40).map(|i| i as f64 * 0.3).collect();
        let y: Vec<f64> = x.iter().map(|&x| (x * 0.8).sin() * 30.0 + 50.0).collect();

        let area = AreaPlot::new(x, y)?
            .color(Color::from_hex("#9b59b6").unwrap())
            .opacity(0.5)
            .baseline(30.0) // Custom baseline
            .label("Signal");

        let bounds = area.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(900, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        area.draw(&mut canvas)?;

        // Draw baseline line for reference
        let baseline_start = Point2D::new(bounds.x_min, 30.0);
        let baseline_end = Point2D::new(bounds.x_max, 30.0);
        canvas.draw_line(
            &baseline_start,
            &baseline_end,
            &Color::from_hex("#95a5a6").unwrap().to_rgba(),
            1.5,
        )?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Time");
        let y_axis = Axis::new(AxisPosition::Left).label("Value");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/area_baseline.png")?;
        println!("  ✓ Custom baseline area plot saved");
    }

    println!("\n✅ All area chart examples completed!");
    println!("\n💡 Area Chart Features:");
    println!("  • Simple area fills");
    println!("  • Stacked area charts (multiple series)");
    println!("  • Custom baselines");
    println!("  • Adjustable opacity and colors");
    println!("  • Optional boundary lines");
    println!("  • Automatic legend generation");

    Ok(())
}
