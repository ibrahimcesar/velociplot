//! Stacked bar chart examples
//!
//! Demonstrates stacked bar charts for showing composition and comparing
//! parts of a whole across different categories.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Stacked Bar Chart examples");

    // ============================================================
    // Example 1: Simple Vertical Stacked Bar
    // ============================================================
    {
        println!("\n📊 Example 1: Simple Vertical Stacked Bar");

        let categories = vec!["Q1", "Q2", "Q3", "Q4"];
        let product_a = vec![45.0, 52.0, 48.0, 61.0];
        let product_b = vec![38.0, 42.0, 45.0, 50.0];
        let product_c = vec![25.0, 28.0, 30.0, 35.0];

        let stacked = StackedBarPlot::new(categories, vec![product_a, product_b, product_c])?
            .labels(vec!["Product A", "Product B", "Product C"])
            .colors(vec![
                Color::from_hex("#3498db").unwrap(),
                Color::from_hex("#e74c3c").unwrap(),
                Color::from_hex("#2ecc71").unwrap(),
            ]);

        let bounds = stacked.bounds().unwrap().with_padding_top(0.15);
        let mut canvas = SkiaCanvas::new(900, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        stacked.draw(&mut canvas)?;

        // Add legend
        let mut legend = Legend::new().position(LegendPosition::UpperRight);
        for entry in stacked.legend_entries() {
            legend = legend.add_entry(entry);
        }
        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Quarter");
        let y_axis = Axis::new(AxisPosition::Left).label("Revenue (M$)");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/stacked_bar_vertical.png")?;
        println!("  ✓ Vertical stacked bar saved");
    }

    // ============================================================
    // Example 2: Horizontal Stacked Bar
    // ============================================================
    {
        println!("\n📊 Example 2: Horizontal Stacked Bar");

        let categories = vec!["North", "South", "East", "West"];
        let jan = vec![120.0, 95.0, 110.0, 88.0];
        let feb = vec![135.0, 102.0, 118.0, 95.0];
        let mar = vec![145.0, 108.0, 125.0, 102.0];

        let stacked = StackedBarPlot::new(categories, vec![jan, feb, mar])?
            .labels(vec!["January", "February", "March"])
            .colors(vec![
                Color::from_hex("#3498db").unwrap(),
                Color::from_hex("#9b59b6").unwrap(),
                Color::from_hex("#e74c3c").unwrap(),
            ])
            .orientation(BarOrientation::Horizontal);

        let bounds = stacked.bounds().unwrap().with_padding_right(0.15);
        let mut canvas = SkiaCanvas::new(900, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        stacked.draw(&mut canvas)?;

        let mut legend = Legend::new().position(LegendPosition::UpperRight);
        for entry in stacked.legend_entries() {
            legend = legend.add_entry(entry);
        }
        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Sales (K units)");
        let y_axis = Axis::new(AxisPosition::Left).label("Region");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/stacked_bar_horizontal.png")?;
        println!("  ✓ Horizontal stacked bar saved");
    }

    // ============================================================
    // Example 3: Multiple Series (Population Demographics)
    // ============================================================
    {
        println!("\n📊 Example 3: Population Demographics");

        let years = vec!["2010", "2012", "2014", "2016", "2018", "2020"];
        let age_0_18 = vec![22.0, 21.5, 21.0, 20.5, 20.0, 19.5];
        let age_19_35 = vec![28.0, 28.5, 29.0, 29.5, 30.0, 30.5];
        let age_36_55 = vec![30.0, 30.0, 30.0, 30.0, 30.0, 30.0];
        let age_56_plus = vec![20.0, 20.0, 20.0, 20.0, 20.0, 20.0];

        let stacked =
            StackedBarPlot::new(years, vec![age_0_18, age_19_35, age_36_55, age_56_plus])?
                .labels(vec!["0-18", "19-35", "36-55", "56+"])
                .colors(vec![
                    Color::from_hex("#3498db").unwrap(),
                    Color::from_hex("#2ecc71").unwrap(),
                    Color::from_hex("#f39c12").unwrap(),
                    Color::from_hex("#e74c3c").unwrap(),
                ]);

        let bounds = stacked.bounds().unwrap().with_padding_top(0.12);
        let mut canvas = SkiaCanvas::new(1000, 650, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        stacked.draw(&mut canvas)?;

        let mut legend = Legend::new().position(LegendPosition::UpperLeft);
        for entry in stacked.legend_entries() {
            legend = legend.add_entry(entry);
        }
        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Year");
        let y_axis = Axis::new(AxisPosition::Left).label("Population (%)");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/stacked_bar_demographics.png")?;
        println!("  ✓ Demographics stacked bar saved");
    }

    // ============================================================
    // Example 4: Budget Breakdown
    // ============================================================
    {
        println!("\n📊 Example 4: Budget Breakdown");

        let departments = vec!["Engineering", "Marketing", "Sales", "Operations", "R&D"];
        let salaries = vec![180.0, 120.0, 150.0, 90.0, 140.0];
        let infrastructure = vec![60.0, 30.0, 20.0, 50.0, 80.0];
        let travel = vec![20.0, 40.0, 60.0, 15.0, 25.0];
        let misc = vec![15.0, 25.0, 30.0, 20.0, 18.0];

        let stacked =
            StackedBarPlot::new(departments, vec![salaries, infrastructure, travel, misc])?
                .labels(vec![
                    "Salaries",
                    "Infrastructure",
                    "Travel",
                    "Miscellaneous",
                ])
                .colors(vec![
                    Color::from_hex("#1abc9c").unwrap(),
                    Color::from_hex("#3498db").unwrap(),
                    Color::from_hex("#9b59b6").unwrap(),
                    Color::from_hex("#e67e22").unwrap(),
                ]);

        let bounds = stacked.bounds().unwrap().with_padding_top(0.15);
        let mut canvas = SkiaCanvas::new(1000, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        stacked.draw(&mut canvas)?;

        let mut legend = Legend::new().position(LegendPosition::UpperRight);
        for entry in stacked.legend_entries() {
            legend = legend.add_entry(entry);
        }
        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Department");
        let y_axis = Axis::new(AxisPosition::Left).label("Budget (K$)");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/stacked_bar_budget.png")?;
        println!("  ✓ Budget breakdown stacked bar saved");
    }

    // ============================================================
    // Example 5: Energy Mix Over Time
    // ============================================================
    {
        println!("\n📊 Example 5: Energy Mix");

        let years = vec!["2000", "2005", "2010", "2015", "2020"];
        let coal = vec![50.0, 45.0, 40.0, 32.0, 25.0];
        let natural_gas = vec![20.0, 22.0, 25.0, 28.0, 30.0];
        let nuclear = vec![15.0, 16.0, 17.0, 18.0, 20.0];
        let renewable = vec![10.0, 12.0, 15.0, 20.0, 25.0];
        let other = vec![5.0, 5.0, 3.0, 2.0, 0.0];

        let stacked =
            StackedBarPlot::new(years, vec![coal, natural_gas, nuclear, renewable, other])?
                .labels(vec!["Coal", "Natural Gas", "Nuclear", "Renewable", "Other"])
                .colors(vec![
                    Color::from_hex("#34495e").unwrap(), // Dark gray for coal
                    Color::from_hex("#3498db").unwrap(), // Blue for gas
                    Color::from_hex("#f39c12").unwrap(), // Orange for nuclear
                    Color::from_hex("#2ecc71").unwrap(), // Green for renewable
                    Color::from_hex("#95a5a6").unwrap(), // Light gray for other
                ])
                .bar_width(0.7);

        let bounds = stacked.bounds().unwrap().with_padding_top(0.12);
        let mut canvas = SkiaCanvas::new(1000, 650, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        stacked.draw(&mut canvas)?;

        let mut legend = Legend::new().position(LegendPosition::UpperRight);
        for entry in stacked.legend_entries() {
            legend = legend.add_entry(entry);
        }
        legend.draw(&mut canvas)?;

        let x_axis = Axis::new(AxisPosition::Bottom).label("Year");
        let y_axis = Axis::new(AxisPosition::Left).label("Energy Mix (%)");
        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        canvas.save_png("examples/images/stacked_bar_energy.png")?;
        println!("  ✓ Energy mix stacked bar saved");
    }

    println!("\n✅ All stacked bar examples completed!");
    println!("\n💡 Stacked Bar Features:");
    println!("  • Vertical and horizontal orientations");
    println!("  • Multiple series stacking");
    println!("  • Automatic legend generation");
    println!("  • Custom colors and labels");
    println!("  • Adjustable bar width");

    Ok(())
}
