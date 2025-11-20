//! Bubble chart example
//!
//! Demonstrates 3-variable visualization using bubble charts where
//! marker size represents a third dimension of data.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Bubble Chart example");

    // ============================================================
    // Example 1: Simple bubble chart
    // ============================================================
    {
        println!("\n📊 Example 1: Simple bubble chart");

        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 3.0, 5.0, 4.0];
        let sizes = vec![10.0, 20.0, 15.0, 25.0, 18.0];

        let bubble = BubbleChart::new(x, y, sizes)?
            .color(Color::from_hex("#3498db").unwrap())
            .size_range(5.0, 25.0)
            .label("Data Points");

        let bounds = bubble.bounds().unwrap().with_padding(0.15);
        let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        bubble.draw(&mut canvas)?;

        // Add axes
        let x_axis = Axis::new(AxisPosition::Bottom).label("X Variable");
        let y_axis = Axis::new(AxisPosition::Left).label("Y Variable");

        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/bubble_simple.png")?;
        println!("  ✓ Simple bubble chart saved");
    }

    // ============================================================
    // Example 2: Population vs GDP with country size
    // ============================================================
    {
        println!("\n📊 Example 2: GDP vs Life Expectancy (bubble = population)");

        // Simulated data: GDP per capita, Life expectancy, Population (millions)
        let gdp = vec![
            5000.0, 15000.0, 25000.0, 35000.0, 45000.0, 55000.0, 10000.0, 20000.0,
        ];
        let life_exp = vec![65.0, 72.0, 75.0, 78.0, 80.0, 82.0, 68.0, 74.0];
        let population = vec![50.0, 100.0, 80.0, 60.0, 40.0, 30.0, 150.0, 90.0];

        let bubble = BubbleChart::new(gdp, life_exp, population)?
            .color(Color::from_hex("#e74c3c").unwrap())
            .size_range(5.0, 30.0)
            .opacity(0.7)
            .label("Countries");

        let bounds = bubble.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(1000, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        bubble.draw(&mut canvas)?;

        // Add axes with labels
        let x_axis = Axis::new(AxisPosition::Bottom).label("GDP per Capita ($)");
        let y_axis = Axis::new(AxisPosition::Left).label("Life Expectancy (years)");

        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/bubble_gdp.png")?;
        println!("  ✓ GDP bubble chart saved");
    }

    // ============================================================
    // Example 3: Multiple bubble series
    // ============================================================
    {
        println!("\n📊 Example 3: Multiple bubble series (Product comparison)");

        // Product A: Price, Sales, Market Share
        let price_a = vec![10.0, 15.0, 20.0, 25.0, 30.0];
        let sales_a = vec![100.0, 150.0, 120.0, 180.0, 140.0];
        let share_a = vec![20.0, 25.0, 22.0, 30.0, 24.0];

        // Product B
        let price_b = vec![12.0, 18.0, 22.0, 28.0, 32.0];
        let sales_b = vec![90.0, 130.0, 160.0, 150.0, 170.0];
        let share_b = vec![18.0, 22.0, 28.0, 26.0, 29.0];

        let bubble_a = BubbleChart::new(price_a, sales_a, share_a)?
            .color(Color::from_hex("#3498db").unwrap())
            .size_range(5.0, 25.0)
            .opacity(0.6)
            .label("Product A");

        let bubble_b = BubbleChart::new(price_b, sales_b, share_b)?
            .color(Color::from_hex("#e74c3c").unwrap())
            .size_range(5.0, 25.0)
            .opacity(0.6)
            .label("Product B");

        let bounds_a = bubble_a.bounds().unwrap();
        let bounds_b = bubble_b.bounds().unwrap();
        let bounds = bounds_a.union(&bounds_b).with_padding(0.15);

        let mut canvas = SkiaCanvas::new(1000, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        bubble_a.draw(&mut canvas)?;
        bubble_b.draw(&mut canvas)?;

        // Add axes
        let x_axis = Axis::new(AxisPosition::Bottom).label("Price ($)");
        let y_axis = Axis::new(AxisPosition::Left).label("Sales (units)");

        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        // Add legend
        let legend = Legend::new()
            .position(LegendPosition::UpperRight)
            .add_entry(bubble_a.legend_entry().unwrap())
            .add_entry(bubble_b.legend_entry().unwrap());

        legend.draw(&mut canvas)?;

        canvas.save_png("examples/images/bubble_multiple.png")?;
        println!("  ✓ Multiple series bubble chart saved");
    }

    // ============================================================
    // Example 4: Time series bubble (animated snapshot)
    // ============================================================
    {
        println!("\n📊 Example 4: Time series snapshot (Marketing campaign)");

        // Campaign metrics over time: Budget, ROI, Engagement
        let budget = vec![1000.0, 2000.0, 3000.0, 4000.0, 5000.0, 6000.0];
        let roi = vec![1.2, 1.8, 2.5, 2.2, 3.0, 2.8];
        let engagement = vec![100.0, 250.0, 400.0, 350.0, 500.0, 450.0];

        let bubble = BubbleChart::new(budget, roi, engagement)?
            .color(Color::from_hex("#9b59b6").unwrap())
            .size_range(8.0, 35.0)
            .opacity(0.65)
            .label("Campaign Performance");

        let bounds = bubble.bounds().unwrap().with_padding(0.15);
        let mut canvas = SkiaCanvas::new(900, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        bubble.draw(&mut canvas)?;

        // Add axes
        let x_axis = Axis::new(AxisPosition::Bottom).label("Budget ($)");
        let y_axis = Axis::new(AxisPosition::Left).label("ROI (Return on Investment)");

        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/bubble_timeseries.png")?;
        println!("  ✓ Time series bubble chart saved");
    }

    // ============================================================
    // Example 5: Different marker shapes
    // ============================================================
    {
        println!("\n📊 Example 5: Different marker shapes");

        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let y = vec![2.0, 3.0, 2.5, 4.0, 3.5, 4.5];
        let sizes = vec![15.0, 20.0, 18.0, 22.0, 19.0, 21.0];

        let shapes = vec![
            MarkerShape::Circle,
            MarkerShape::Square,
            MarkerShape::Triangle,
            MarkerShape::Diamond,
            MarkerShape::Plus,
            MarkerShape::Cross,
        ];

        let mut all_bounds: Option<Bounds> = None;

        let mut bubbles = Vec::new();
        for (i, &shape) in shapes.iter().enumerate() {
            let bubble = BubbleChart::new(vec![x[i]], vec![y[i]], vec![sizes[i]])?
                .color(Color::from_hex("#16a085").unwrap())
                .marker_shape(shape)
                .size_range(15.0, 25.0)
                .opacity(0.7);

            if let Some(b) = bubble.bounds() {
                all_bounds = Some(match all_bounds {
                    Some(existing) => existing.union(&b),
                    None => b,
                });
            }
            bubbles.push(bubble);
        }

        let bounds = all_bounds.unwrap().with_padding(0.2);
        let mut canvas = SkiaCanvas::new(900, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        for bubble in &bubbles {
            bubble.draw(&mut canvas)?;
        }

        // Add axes
        let x_axis = Axis::new(AxisPosition::Bottom).label("X Axis");
        let y_axis = Axis::new(AxisPosition::Left).label("Y Axis");

        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/bubble_shapes.png")?;
        println!("  ✓ Marker shapes bubble chart saved");
    }

    // ============================================================
    // Example 6: Scientific data (scatter with size)
    // ============================================================
    {
        println!("\n📊 Example 6: Scientific data (Temperature, Pressure, Volume)");

        // Generate some scientific-looking data
        let temperature: Vec<f64> = (0..20).map(|i| 273.0 + i as f64 * 5.0).collect();
        let pressure: Vec<f64> = temperature
            .iter()
            .map(|&t| 101325.0 * (1.0 + (t - 273.0) * 0.003))
            .collect();
        let volume: Vec<f64> = temperature
            .iter()
            .enumerate()
            .map(|(i, &t)| 1.0 + (t - 273.0) * 0.002 + (i as f64 * 0.05))
            .collect();

        let bubble = BubbleChart::new(temperature, pressure, volume)?
            .color(Color::from_hex("#f39c12").unwrap())
            .size_range(6.0, 20.0)
            .opacity(0.7)
            .label("Measurements");

        let bounds = bubble.bounds().unwrap().with_padding(0.1);
        let mut canvas = SkiaCanvas::new(1000, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        bubble.draw(&mut canvas)?;

        // Add axes
        let x_axis = Axis::new(AxisPosition::Bottom).label("Temperature (K)");
        let y_axis = Axis::new(AxisPosition::Left).label("Pressure (Pa)");

        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/bubble_scientific.png")?;
        println!("  ✓ Scientific bubble chart saved");
    }

    println!("\n✅ All bubble chart examples completed!");
    println!("\n💡 Key features:");
    println!("  • 3-variable visualization (x, y, size)");
    println!("  • Configurable size scaling and ranges");
    println!("  • Adjustable opacity for overlapping bubbles");
    println!("  • Multiple marker shapes");
    println!("  • Multiple series support");
    println!("\n📚 Use cases:");
    println!("  • Economic data (GDP, population, life expectancy)");
    println!("  • Marketing analytics (budget, ROI, engagement)");
    println!("  • Product comparisons (price, sales, market share)");
    println!("  • Scientific measurements (3+ variables)");
    println!("  • Time series with magnitude visualization");

    Ok(())
}
