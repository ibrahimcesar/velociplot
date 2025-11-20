//! Legend types example
//!
//! Demonstrates different legend types: LineLegend, PointLegend, SwatchLegend, and combinations.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Legend Types example");

    // ============================================================
    // Example 1: LineLegend (default)
    // ============================================================
    {
        println!("\n📊 Example 1: LineLegend");

        let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.2).collect();
        let y1: Vec<f64> = x.iter().map(|&x| x.sin()).collect();
        let y2: Vec<f64> = x.iter().map(|&x| (x * 0.5).cos()).collect();
        let y3: Vec<f64> = x.iter().map(|&x| (x * 0.3).sin() * 0.5).collect();

        let series1 = Series::new(x.clone(), y1)?;
        let series2 = Series::new(x.clone(), y2)?;
        let series3 = Series::new(x.clone(), y3)?;

        let plot1 = LinePlot::new(series1)
            .color(Color::from_hex("#e74c3c").unwrap())
            .line_width(2.0)
            .label("sin(x)");

        let plot2 = LinePlot::new(series2)
            .color(Color::from_hex("#3498db").unwrap())
            .line_width(2.0)
            .label("cos(0.5x)");

        let plot3 = LinePlot::new(series3)
            .color(Color::from_hex("#2ecc71").unwrap())
            .line_width(2.0)
            .label("0.5·sin(0.3x)");

        let bounds = plot1
            .bounds()
            .unwrap()
            .union(&plot2.bounds().unwrap())
            .union(&plot3.bounds().unwrap())
            .with_padding(0.1);

        let mut canvas = SkiaCanvas::new(900, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        plot1.draw(&mut canvas)?;
        plot2.draw(&mut canvas)?;
        plot3.draw(&mut canvas)?;

        // LineLegend - default legend with lines
        let legend = Legend::new()
            .position(LegendPosition::UpperRight)
            .add_entry(plot1.legend_entry().unwrap())
            .add_entry(plot2.legend_entry().unwrap())
            .add_entry(plot3.legend_entry().unwrap());

        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("x");
        let y_axis = Axis::new(AxisPosition::Left).label("y");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/legend_line.png")?;
        println!("  ✓ LineLegend saved");
    }

    // ============================================================
    // Example 2: PointLegend (scatter plots)
    // ============================================================
    {
        println!("\n📊 Example 2: PointLegend");

        let x1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let y1 = vec![2.3, 3.1, 2.8, 4.2, 3.9, 5.1, 4.8, 5.9];

        let x2 = vec![1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5];
        let y2 = vec![1.8, 2.2, 2.6, 3.0, 3.4, 3.8, 4.2, 4.6];

        let x3 = vec![1.2, 2.8, 3.3, 4.7, 5.2, 6.8, 7.3, 8.8];
        let y3 = vec![3.2, 3.8, 4.3, 4.8, 5.3, 5.8, 6.3, 6.8];

        let scatter1 = ScatterPlot::new(Series::new(x1, y1)?)
            .color(Color::from_hex("#e74c3c").unwrap())
            .marker_shape(MarkerShape::Circle)
            .marker_size(6.0)
            .label("Group A");

        let scatter2 = ScatterPlot::new(Series::new(x2, y2)?)
            .color(Color::from_hex("#3498db").unwrap())
            .marker_shape(MarkerShape::Square)
            .marker_size(6.0)
            .label("Group B");

        let scatter3 = ScatterPlot::new(Series::new(x3, y3)?)
            .color(Color::from_hex("#2ecc71").unwrap())
            .marker_shape(MarkerShape::Triangle)
            .marker_size(7.0)
            .label("Group C");

        let bounds = scatter1
            .bounds()
            .unwrap()
            .union(&scatter2.bounds().unwrap())
            .union(&scatter3.bounds().unwrap())
            .with_padding(0.15);

        let mut canvas = SkiaCanvas::new(900, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        scatter1.draw(&mut canvas)?;
        scatter2.draw(&mut canvas)?;
        scatter3.draw(&mut canvas)?;

        // PointLegend - legend with point markers
        let legend = Legend::new()
            .position(LegendPosition::UpperLeft)
            .add_entry(
                LegendEntry::new("Group A")
                    .color(Color::from_hex("#e74c3c").unwrap())
                    .point_shape(LegendMarker::Circle, 6.0),
            )
            .add_entry(
                LegendEntry::new("Group B")
                    .color(Color::from_hex("#3498db").unwrap())
                    .point_shape(LegendMarker::Square, 6.0),
            )
            .add_entry(
                LegendEntry::new("Group C")
                    .color(Color::from_hex("#2ecc71").unwrap())
                    .point_shape(LegendMarker::Triangle, 7.0),
            );

        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("X Variable");
        let y_axis = Axis::new(AxisPosition::Left).label("Y Variable");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/legend_point.png")?;
        println!("  ✓ PointLegend saved");
    }

    // ============================================================
    // Example 3: SwatchLegend (categorical data)
    // ============================================================
    {
        println!("\n📊 Example 3: SwatchLegend");

        let categories = vec!["Q1", "Q2", "Q3", "Q4"];
        let values = vec![45.0, 52.0, 48.0, 61.0];
        let colors = vec![
            Color::from_hex("#3498db").unwrap(),
            Color::from_hex("#e74c3c").unwrap(),
            Color::from_hex("#f39c12").unwrap(),
            Color::from_hex("#2ecc71").unwrap(),
        ];

        let x: Vec<f64> = (0..4).map(|i| i as f64).collect();
        let mut all_bounds: Option<Bounds> = None;

        for (i, (&val, color)) in values.iter().zip(&colors).enumerate() {
            let bar = BarPlot::new(Series::new(vec![x[i]], vec![val])?)
                .color(*color)
                .bar_width(0.6)
                .orientation(BarOrientation::Vertical);

            if let Some(b) = bar.bounds() {
                all_bounds = Some(match all_bounds {
                    Some(existing) => existing.union(&b),
                    None => b,
                });
            }
        }

        let bounds = all_bounds.unwrap().with_padding(0.15);
        let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        // Draw bars
        for (i, (&val, color)) in values.iter().zip(&colors).enumerate() {
            let bar = BarPlot::new(Series::new(vec![x[i]], vec![val])?)
                .color(*color)
                .bar_width(0.6)
                .orientation(BarOrientation::Vertical);
            bar.draw(&mut canvas)?;
        }

        // SwatchLegend - legend with filled rectangles (swatches)
        let mut legend = Legend::new().position(LegendPosition::UpperRight);

        for (label, color) in categories.iter().zip(&colors) {
            legend = legend.add_entry(LegendEntry::new(*label).color(*color).swatch_shape());
        }

        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Quarter");
        let y_axis = Axis::new(AxisPosition::Left).label("Revenue (M$)");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/legend_swatch.png")?;
        println!("  ✓ SwatchLegend saved");
    }

    // ============================================================
    // Example 4: Mixed Legend (lines and points)
    // ============================================================
    {
        println!("\n📊 Example 4: Mixed Legend");

        let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.2).collect();
        let y1: Vec<f64> = x.iter().map(|&x| x.sin()).collect();
        let y2: Vec<f64> = x.iter().map(|&x| (x * 0.5).cos()).collect();

        // Scatter points
        let scatter_x = vec![2.0, 4.0, 6.0, 8.0];
        let scatter_y = vec![0.9, 0.2, -0.3, 0.1];

        let line1 = LinePlot::new(Series::new(x.clone(), y1)?)
            .color(Color::from_hex("#e74c3c").unwrap())
            .line_width(2.0);

        let line2 = LinePlot::new(Series::new(x, y2)?)
            .color(Color::from_hex("#3498db").unwrap())
            .line_width(2.0);

        let scatter = ScatterPlot::new(Series::new(scatter_x, scatter_y)?)
            .color(Color::from_hex("#f39c12").unwrap())
            .marker_shape(MarkerShape::Diamond)
            .marker_size(8.0);

        let bounds = line1
            .bounds()
            .unwrap()
            .union(&line2.bounds().unwrap())
            .union(&scatter.bounds().unwrap())
            .with_padding(0.1);

        let mut canvas = SkiaCanvas::new(900, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        line1.draw(&mut canvas)?;
        line2.draw(&mut canvas)?;
        scatter.draw(&mut canvas)?;

        // Mixed legend with different entry types
        let legend = Legend::new()
            .position(LegendPosition::LowerRight)
            .add_entry(
                LegendEntry::new("sin(x) trend")
                    .color(Color::from_hex("#e74c3c").unwrap())
                    .line_width(2.0),
            )
            .add_entry(
                LegendEntry::new("cos(0.5x) trend")
                    .color(Color::from_hex("#3498db").unwrap())
                    .line_width(2.0),
            )
            .add_entry(
                LegendEntry::new("Data points")
                    .color(Color::from_hex("#f39c12").unwrap())
                    .point_shape(LegendMarker::Diamond, 8.0),
            );

        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("x");
        let y_axis = Axis::new(AxisPosition::Left).label("y");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/legend_mixed.png")?;
        println!("  ✓ Mixed legend saved");
    }

    // ============================================================
    // Example 5: All marker shapes in legend
    // ============================================================
    {
        println!("\n📊 Example 5: All marker shapes");

        let markers_scatter = vec![
            MarkerShape::Circle,
            MarkerShape::Square,
            MarkerShape::Triangle,
            MarkerShape::Diamond,
            MarkerShape::Plus,
            MarkerShape::Cross,
        ];

        let markers_legend = vec![
            LegendMarker::Circle,
            LegendMarker::Square,
            LegendMarker::Triangle,
            LegendMarker::Diamond,
            LegendMarker::Plus,
            LegendMarker::Cross,
        ];

        let labels = vec!["Circle", "Square", "Triangle", "Diamond", "Plus", "Cross"];

        let colors = vec![
            Color::from_hex("#e74c3c").unwrap(),
            Color::from_hex("#3498db").unwrap(),
            Color::from_hex("#2ecc71").unwrap(),
            Color::from_hex("#f39c12").unwrap(),
            Color::from_hex("#9b59b6").unwrap(),
            Color::from_hex("#1abc9c").unwrap(),
        ];

        // Create dummy scatter plots to show the markers
        let mut all_bounds: Option<Bounds> = None;
        for (i, &marker) in markers_scatter.iter().enumerate() {
            let x = vec![i as f64 * 2.0];
            let y = vec![(i as f64 * 0.5).sin()];
            let scatter = ScatterPlot::new(Series::new(x, y)?)
                .color(colors[i])
                .marker_shape(marker)
                .marker_size(8.0);

            if let Some(b) = scatter.bounds() {
                all_bounds = Some(match all_bounds {
                    Some(existing) => existing.union(&b),
                    None => b,
                });
            }
        }

        let bounds = all_bounds.unwrap().with_padding(0.3);
        let mut canvas = SkiaCanvas::new(900, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        // Draw all scatter plots
        for (i, &marker) in markers_scatter.iter().enumerate() {
            let x = vec![i as f64 * 2.0];
            let y = vec![(i as f64 * 0.5).sin()];
            let scatter = ScatterPlot::new(Series::new(x, y)?)
                .color(colors[i])
                .marker_shape(marker)
                .marker_size(8.0);
            scatter.draw(&mut canvas)?;
        }

        // Legend with all marker shapes
        let mut legend = Legend::new().position(LegendPosition::CenterRight);

        for (i, &marker) in markers_legend.iter().enumerate() {
            legend = legend.add_entry(
                LegendEntry::new(labels[i])
                    .color(colors[i])
                    .point_shape(marker, 7.0),
            );
        }

        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Index");
        let y_axis = Axis::new(AxisPosition::Left).label("Value");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/legend_markers.png")?;
        println!("  ✓ All markers legend saved");
    }

    println!("\n✅ All legend type examples completed!");
    println!("\n💡 Legend Types Available:");
    println!("  • LineLegend - Lines with labels (default)");
    println!("  • PointLegend - Markers with labels");
    println!("  • SwatchLegend - Filled rectangles with labels");
    println!("  • Mixed - Combine different types in one legend");
    println!("\n📚 Marker Shapes:");
    println!("  • Circle, Square, Triangle");
    println!("  • Diamond, Plus, Cross");

    Ok(())
}
