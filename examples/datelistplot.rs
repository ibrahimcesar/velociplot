//! DateListPlot examples
//!
//! Demonstrates time series visualization with automatic time axis handling,
//! inspired by Wolfram's DateListPlot.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - DateListPlot examples");

    // ============================================================
    // Example 1: Simple Time Series
    // ============================================================
    {
        println!("\n📊 Example 1: Temperature Over Time");

        let times: Vec<f64> = (0..24).map(|h| h as f64).collect();
        let temps: Vec<f64> = times
            .iter()
            .map(|&h| 20.0 + 8.0 * ((h - 12.0) * std::f64::consts::PI / 12.0).sin())
            .collect();

        let plot = DateListPlot::new(times, temps)?
            .label("Temperature (°C)")
            .color(Color::from_hex("#e74c3c").unwrap())
            .line_width(2.5);

        let bounds = plot.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(1000, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        plot.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Hour of Day");
        let y_axis = Axis::new(AxisPosition::Left).label("Temperature (°C)");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/datelistplot_temperature.png")?;
        println!("  ✓ Temperature time series saved");
    }

    // ============================================================
    // Example 2: Stock Prices (Multiple Series)
    // ============================================================
    {
        println!("\n📊 Example 2: Stock Prices");

        let days: Vec<f64> = (0..60).map(|d| d as f64).collect();

        let stock_a: Vec<f64> = days
            .iter()
            .map(|&d| 100.0 + 15.0 * (d / 10.0).sin() + d * 0.3)
            .collect();

        let stock_b: Vec<f64> = days
            .iter()
            .map(|&d| 95.0 + 12.0 * ((d + 5.0) / 8.0).sin() + d * 0.25)
            .collect();

        let stock_c: Vec<f64> = days
            .iter()
            .map(|&d| 90.0 + 18.0 * ((d + 10.0) / 12.0).sin() + d * 0.35)
            .collect();

        let mut plot = DateListPlot::empty();
        plot.add_series(days.clone(), stock_a, Some("Stock A".to_string()))?;
        plot.add_series(days.clone(), stock_b, Some("Stock B".to_string()))?;
        plot.add_series(days, stock_c, Some("Stock C".to_string()))?;

        let bounds = plot.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(1200, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        plot.draw(&mut canvas)?;

        // Add legend
        let mut legend = Legend::new().position(LegendPosition::UpperLeft);
        for entry in plot.legend_entries() {
            legend = legend.add_entry(entry);
        }
        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Trading Day");
        let y_axis = Axis::new(AxisPosition::Left).label("Price ($)");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/datelistplot_stocks.png")?;
        println!("  ✓ Stock prices time series saved");
    }

    // ============================================================
    // Example 3: Points Style
    // ============================================================
    {
        println!("\n📊 Example 3: Measurements (Points)");

        let times: Vec<f64> = (0..20).map(|i| i as f64 * 0.5).collect();
        let measurements: Vec<f64> = times
            .iter()
            .enumerate()
            .map(|(i, &t)| {
                let noise = ((i as f64 * 7.0).sin() * 2.5);
                50.0 + 20.0 * (t / 3.0).sin() + noise
            })
            .collect();

        let plot = DateListPlot::new(times, measurements)?
            .label("Sensor Data")
            .color(Color::from_hex("#9b59b6").unwrap())
            .style(DateListStyle::Points)
            .point_size(6.0);

        let bounds = plot.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(1000, 600, bounds)?;
        canvas.fill_background(&Color::from_hex("#ecf0f1").unwrap().to_rgba())?;

        plot.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Time");
        let y_axis = Axis::new(AxisPosition::Left).label("Value");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/datelistplot_points.png")?;
        println!("  ✓ Points style time series saved");
    }

    // ============================================================
    // Example 4: Line and Points Combined
    // ============================================================
    {
        println!("\n📊 Example 4: COVID Cases (Line + Points)");

        let weeks: Vec<f64> = (0..52).map(|w| w as f64).collect();
        let cases: Vec<f64> = weeks
            .iter()
            .map(|&w| {
                let base = 1000.0;
                let trend = w * 50.0;
                let wave1 = 800.0 * ((w - 10.0) / 8.0).sin().max(0.0);
                let wave2 = 600.0 * ((w - 30.0) / 10.0).sin().max(0.0);
                base + trend + wave1 + wave2
            })
            .collect();

        let plot = DateListPlot::new(weeks, cases)?
            .label("Weekly Cases")
            .color(Color::from_hex("#e74c3c").unwrap())
            .style(DateListStyle::LinePoints)
            .line_width(2.0)
            .point_size(4.0);

        let bounds = plot.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(1200, 650, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        plot.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Week Number");
        let y_axis = Axis::new(AxisPosition::Left).label("Cases");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/datelistplot_combined.png")?;
        println!("  ✓ Combined style time series saved");
    }

    // ============================================================
    // Example 5: With Grid
    // ============================================================
    {
        println!("\n📊 Example 5: Energy Consumption (With Grid)");

        let hours: Vec<f64> = (0..168).map(|h| h as f64).collect(); // 1 week
        let consumption: Vec<f64> = hours
            .iter()
            .map(|&h| {
                let day_pattern =
                    50.0 + 30.0 * ((h % 24.0 - 12.0) / 12.0 * std::f64::consts::PI).sin();
                let week_variation = 10.0 * (h / 24.0).sin();
                day_pattern + week_variation
            })
            .collect();

        let plot = DateListPlot::new(hours, consumption)?
            .label("Power (kW)")
            .color(Color::from_hex("#2ecc71").unwrap())
            .line_width(1.5)
            .show_grid(true);

        let bounds = plot.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(1400, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        plot.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Hour (Week)");
        let y_axis = Axis::new(AxisPosition::Left).label("Power (kW)");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/datelistplot_grid.png")?;
        println!("  ✓ Grid-enabled time series saved");
    }

    // ============================================================
    // Example 6: Financial Data (Multiple Metrics)
    // ============================================================
    {
        println!("\n📊 Example 6: Financial Metrics");

        let months: Vec<f64> = (0..24).map(|m| m as f64).collect();

        let revenue: Vec<f64> = months
            .iter()
            .map(|&m| 100.0 + m * 8.0 + 20.0 * (m / 6.0).sin())
            .collect();

        let expenses: Vec<f64> = months
            .iter()
            .map(|&m| 60.0 + m * 4.0 + 15.0 * ((m + 2.0) / 6.0).sin())
            .collect();

        let profit: Vec<f64> = revenue
            .iter()
            .zip(&expenses)
            .map(|(&r, &e)| r - e)
            .collect();

        let mut plot = DateListPlot::empty();
        plot.add_series(months.clone(), revenue, Some("Revenue".to_string()))?;
        plot.add_series(months.clone(), expenses, Some("Expenses".to_string()))?;
        plot.add_series(months, profit, Some("Profit".to_string()))?;

        let bounds = plot.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(1200, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        plot.draw(&mut canvas)?;

        let mut legend = Legend::new().position(LegendPosition::UpperLeft);
        for entry in plot.legend_entries() {
            legend = legend.add_entry(entry);
        }
        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Month");
        let y_axis = Axis::new(AxisPosition::Left).label("Amount (K$)");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/datelistplot_financial.png")?;
        println!("  ✓ Financial metrics time series saved");
    }

    println!("\n✅ All DateListPlot examples completed!");
    println!("\n💡 DateListPlot Features:");
    println!("  • Time series visualization");
    println!("  • Multiple plotting styles (Line, Points, LinePoints)");
    println!("  • Multiple series support");
    println!("  • Automatic color assignment");
    println!("  • Grid display option");
    println!("  • Legend integration");
    println!("  • Works with any temporal data");

    Ok(())
}
