//! Box plot example
//!
//! Demonstrates box-and-whisker plots for displaying distribution statistics.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Box plot example");

    // ============================================================
    // Example 1: Single box plot
    // ============================================================
    {
        println!("\n📊 Example 1: Single box plot");

        let data = vec![
            2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0, 6.5, 7.0, 7.5, 8.0, 8.5, 9.0, 9.5,
        ];

        let boxplot = BoxPlot::new(data)
            .color(Color::from_hex("#3498db").unwrap())
            .width(0.6)
            .label("Sample Data");

        let bounds = boxplot.bounds().unwrap().with_padding(0.15);
        let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        let x_axis = Axis::new(AxisPosition::Bottom)
            .label("Group")
            .tick_count(3)
            .show_grid(false);

        let y_axis = Axis::new(AxisPosition::Left)
            .label("Value")
            .tick_count(8)
            .show_grid(true);

        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;
        boxplot.draw(&mut canvas)?;

        if let Some(entry) = boxplot.legend_entry() {
            let legend = Legend::new()
                .add_entry(entry)
                .position(LegendPosition::UpperRight);
            legend.draw(&mut canvas)?;
        }

        canvas.save_png("examples/images/box_plot_single.png")?;
        println!("  ✓ Single box plot saved");
    }

    // ============================================================
    // Example 2: Multiple box plots for comparison
    // ============================================================
    {
        println!("\n📊 Example 2: Multiple box plots comparison");

        let group_a = vec![
            15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0,
        ];
        let group_b = vec![
            20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
        ];
        let group_c = vec![
            18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0,
        ];

        let box1 = BoxPlot::new(group_a)
            .position(1.0)
            .color(Color::from_hex("#e74c3c").unwrap())
            .label("Group A");

        let box2 = BoxPlot::new(group_b)
            .position(2.0)
            .color(Color::from_hex("#3498db").unwrap())
            .label("Group B");

        let box3 = BoxPlot::new(group_c)
            .position(3.0)
            .color(Color::from_hex("#2ecc71").unwrap())
            .label("Group C");

        let bounds = box1
            .bounds()
            .unwrap()
            .union(&box2.bounds().unwrap())
            .union(&box3.bounds().unwrap())
            .with_padding(0.15);

        let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        let x_axis = Axis::new(AxisPosition::Bottom)
            .label("Groups")
            .tick_count(4)
            .show_grid(false);

        let y_axis = Axis::new(AxisPosition::Left)
            .label("Measurement")
            .tick_count(8)
            .show_grid(true);

        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        box1.draw(&mut canvas)?;
        box2.draw(&mut canvas)?;
        box3.draw(&mut canvas)?;

        let mut legend = Legend::new();
        for b in [&box1, &box2, &box3] {
            if let Some(entry) = b.legend_entry() {
                legend = legend.add_entry(entry);
            }
        }
        legend.draw(&mut canvas)?;

        canvas.save_png("examples/images/box_plot_multiple.png")?;
        println!("  ✓ Multiple box plots saved");
    }

    // ============================================================
    // Example 3: Box plot with outliers
    // ============================================================
    {
        println!("\n📊 Example 3: Box plot with outliers");

        // Dataset with outliers
        let mut data = vec![
            10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0,
        ];
        // Add outliers
        data.extend(vec![5.0, 35.0, 40.0]);

        let boxplot = BoxPlot::new(data)
            .color(Color::from_hex("#9b59b6").unwrap())
            .show_outliers(true)
            .outlier_method(OutlierMethod::IQR)
            .label("Data with Outliers");

        let bounds = boxplot.bounds().unwrap().with_padding(0.15);
        let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        let x_axis = Axis::new(AxisPosition::Bottom)
            .label("Sample")
            .tick_count(3)
            .show_grid(false);

        let y_axis = Axis::new(AxisPosition::Left)
            .label("Value")
            .tick_count(10)
            .show_grid(true);

        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;
        boxplot.draw(&mut canvas)?;

        if let Some(entry) = boxplot.legend_entry() {
            let legend = Legend::new()
                .add_entry(entry)
                .position(LegendPosition::UpperRight);
            legend.draw(&mut canvas)?;
        }

        canvas.save_png("examples/images/box_plot_outliers.png")?;
        println!("  ✓ Box plot with outliers saved");
    }

    // ============================================================
    // Example 4: Real-world application - Test scores
    // ============================================================
    {
        println!("\n📊 Example 4: Real-world example - Test scores");

        // Test scores for different classes
        let class_a = vec![
            85.0, 87.0, 88.0, 90.0, 91.0, 92.0, 93.0, 94.0, 95.0, 96.0, 97.0, 98.0,
        ];
        let class_b = vec![
            75.0, 78.0, 80.0, 82.0, 84.0, 85.0, 86.0, 88.0, 89.0, 90.0, 92.0, 94.0,
        ];
        let class_c = vec![
            80.0, 82.0, 84.0, 86.0, 88.0, 90.0, 91.0, 92.0, 93.0, 94.0, 95.0, 96.0,
        ];
        let class_d = vec![
            70.0, 72.0, 74.0, 76.0, 78.0, 80.0, 82.0, 84.0, 86.0, 88.0, 90.0, 92.0,
        ];

        let boxes = vec![
            BoxPlot::new(class_a)
                .position(1.0)
                .color(Color::from_hex("#e74c3c").unwrap())
                .label("Class A"),
            BoxPlot::new(class_b)
                .position(2.0)
                .color(Color::from_hex("#3498db").unwrap())
                .label("Class B"),
            BoxPlot::new(class_c)
                .position(3.0)
                .color(Color::from_hex("#2ecc71").unwrap())
                .label("Class C"),
            BoxPlot::new(class_d)
                .position(4.0)
                .color(Color::from_hex("#f39c12").unwrap())
                .label("Class D"),
        ];

        let mut bounds = boxes[0].bounds().unwrap();
        for b in &boxes[1..] {
            bounds = bounds.union(&b.bounds().unwrap());
        }
        bounds = bounds.with_padding(0.15);

        let mut canvas = SkiaCanvas::new(800, 720, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        let x_axis = Axis::new(AxisPosition::Bottom)
            .label("Class")
            .tick_count(5)
            .show_grid(false);

        let y_axis = Axis::new(AxisPosition::Left)
            .label("Test Score (%)")
            .tick_count(8)
            .show_grid(true);

        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        for boxplot in &boxes {
            boxplot.draw(&mut canvas)?;
        }

        let mut legend = Legend::new();
        for boxplot in &boxes {
            if let Some(entry) = boxplot.legend_entry() {
                legend = legend.add_entry(entry);
            }
        }
        legend.draw(&mut canvas)?;

        canvas.save_png("examples/images/box_plot_test_scores.png")?;
        println!("  ✓ Test scores box plot saved");
    }

    println!("\n✅ All box plot examples completed!");
    println!("\n💡 Key features:");
    println!("  • Five-number summary: min, Q1, median, Q3, max");
    println!("  • Outlier detection using IQR method");
    println!("  • Multiple groups for comparison");
    println!("  • Customizable colors and widths");
    println!("\n📚 Use cases:");
    println!("  • Distribution comparison across groups");
    println!("  • Outlier identification");
    println!("  • Quality control and testing");
    println!("  • Statistical analysis visualization");

    Ok(())
}
