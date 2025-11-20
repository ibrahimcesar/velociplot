//! Treemap examples
//!
//! Demonstrates treemaps for visualizing hierarchical data and proportions.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Treemap examples");

    // ============================================================
    // Example 1: Simple Market Share Treemap
    // ============================================================
    {
        println!("\n📊 Example 1: Market Share");

        let mut treemap = Treemap::new();
        treemap
            .add_item("Company A", 35.0, Color::from_hex("#3498db").unwrap())
            .add_item("Company B", 28.0, Color::from_hex("#e74c3c").unwrap())
            .add_item("Company C", 22.0, Color::from_hex("#2ecc71").unwrap())
            .add_item("Company D", 10.0, Color::from_hex("#f39c12").unwrap())
            .add_item("Others", 5.0, Color::from_hex("#95a5a6").unwrap());

        let bounds = treemap.bounds().unwrap();
        let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        treemap.draw(&mut canvas)?;

        canvas.save_png("examples/images/treemap_market_share.png")?;
        println!("  ✓ Market share treemap saved");
    }

    // ============================================================
    // Example 2: Budget Allocation
    // ============================================================
    {
        println!("\n📊 Example 2: Budget Allocation");

        let mut treemap = Treemap::new();
        treemap
            .add_item("Engineering", 45.0, Color::from_hex("#3498db").unwrap())
            .add_item("Marketing", 25.0, Color::from_hex("#e74c3c").unwrap())
            .add_item("Sales", 15.0, Color::from_hex("#2ecc71").unwrap())
            .add_item("Operations", 10.0, Color::from_hex("#f39c12").unwrap())
            .add_item("R&D", 5.0, Color::from_hex("#9b59b6").unwrap());

        let bounds = treemap.bounds().unwrap();
        let mut canvas = SkiaCanvas::new(900, 700, bounds)?;
        canvas.fill_background(&Color::from_hex("#ecf0f1").unwrap().to_rgba())?;

        treemap.draw(&mut canvas)?;

        canvas.save_png("examples/images/treemap_budget.png")?;
        println!("  ✓ Budget allocation treemap saved");
    }

    // ============================================================
    // Example 3: File System Storage
    // ============================================================
    {
        println!("\n📊 Example 3: Storage Breakdown");

        let mut treemap = Treemap::new();
        treemap
            .add_item("Documents", 120.5, Color::from_hex("#3498db").unwrap())
            .add_item("Photos", 89.2, Color::from_hex("#e74c3c").unwrap())
            .add_item("Videos", 156.8, Color::from_hex("#9b59b6").unwrap())
            .add_item("Music", 45.3, Color::from_hex("#1abc9c").unwrap())
            .add_item("Applications", 67.9, Color::from_hex("#f39c12").unwrap())
            .add_item("System", 34.1, Color::from_hex("#34495e").unwrap())
            .add_item("Other", 18.7, Color::from_hex("#95a5a6").unwrap());

        let bounds = treemap.bounds().unwrap();
        let mut canvas = SkiaCanvas::new(1000, 750, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        treemap.draw(&mut canvas)?;

        canvas.save_png("examples/images/treemap_storage.png")?;
        println!("  ✓ Storage treemap saved");
    }

    // ============================================================
    // Example 4: Portfolio Allocation
    // ============================================================
    {
        println!("\n📊 Example 4: Investment Portfolio");

        let mut treemap = Treemap::new();
        treemap
            .add_item("US Stocks", 40.0, Color::from_hex("#3498db").unwrap())
            .add_item(
                "International Stocks",
                25.0,
                Color::from_hex("#2ecc71").unwrap(),
            )
            .add_item("Bonds", 20.0, Color::from_hex("#f39c12").unwrap())
            .add_item("Real Estate", 10.0, Color::from_hex("#e74c3c").unwrap())
            .add_item("Cash", 5.0, Color::from_hex("#95a5a6").unwrap());

        let bounds = treemap.bounds().unwrap();
        let mut canvas = SkiaCanvas::new(900, 650, bounds)?;
        canvas.fill_background(&Color::from_hex("#2c3e50").unwrap().to_rgba())?;

        treemap
            .border_color(Color::from_hex("#2c3e50").unwrap())
            .border_width(2.0)
            .draw(&mut canvas)?;

        canvas.save_png("examples/images/treemap_portfolio.png")?;
        println!("  ✓ Portfolio treemap saved");
    }

    // ============================================================
    // Example 5: Website Traffic Sources
    // ============================================================
    {
        println!("\n📊 Example 5: Traffic Sources");

        let mut treemap = Treemap::new();
        treemap
            .add_item("Organic Search", 38.5, Color::from_hex("#2ecc71").unwrap())
            .add_item("Direct", 24.2, Color::from_hex("#3498db").unwrap())
            .add_item("Social Media", 18.7, Color::from_hex("#e74c3c").unwrap())
            .add_item("Referral", 12.3, Color::from_hex("#f39c12").unwrap())
            .add_item("Email", 4.8, Color::from_hex("#9b59b6").unwrap())
            .add_item("Paid Search", 1.5, Color::from_hex("#1abc9c").unwrap());

        let bounds = treemap.bounds().unwrap();
        let mut canvas = SkiaCanvas::new(1000, 700, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        treemap.padding(1.5).draw(&mut canvas)?;

        canvas.save_png("examples/images/treemap_traffic.png")?;
        println!("  ✓ Traffic sources treemap saved");
    }

    // ============================================================
    // Example 6: No Labels (Values Only)
    // ============================================================
    {
        println!("\n📊 Example 6: Values Only");

        let mut treemap = Treemap::new();
        treemap
            .add_item("Category 1", 45.0, Color::from_hex("#16a085").unwrap())
            .add_item("Category 2", 30.0, Color::from_hex("#27ae60").unwrap())
            .add_item("Category 3", 15.0, Color::from_hex("#2980b9").unwrap())
            .add_item("Category 4", 10.0, Color::from_hex("#8e44ad").unwrap());

        let bounds = treemap.bounds().unwrap();
        let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        treemap
            .show_labels(false)
            .border_width(3.0)
            .draw(&mut canvas)?;

        canvas.save_png("examples/images/treemap_values_only.png")?;
        println!("  ✓ Values-only treemap saved");
    }

    // ============================================================
    // Example 7: Product Categories
    // ============================================================
    {
        println!("\n📊 Example 7: Product Categories");

        let mut treemap = Treemap::new();
        treemap
            .add_item("Electronics", 185.0, Color::from_hex("#3498db").unwrap())
            .add_item("Clothing", 142.0, Color::from_hex("#e74c3c").unwrap())
            .add_item("Home & Garden", 98.0, Color::from_hex("#2ecc71").unwrap())
            .add_item("Sports", 76.0, Color::from_hex("#f39c12").unwrap())
            .add_item("Books", 54.0, Color::from_hex("#9b59b6").unwrap())
            .add_item("Toys", 38.0, Color::from_hex("#1abc9c").unwrap())
            .add_item("Automotive", 29.0, Color::from_hex("#e67e22").unwrap())
            .add_item("Beauty", 22.0, Color::from_hex("#c0392b").unwrap())
            .add_item("Pet Supplies", 15.0, Color::from_hex("#16a085").unwrap())
            .add_item("Other", 8.0, Color::from_hex("#7f8c8d").unwrap());

        let bounds = treemap.bounds().unwrap();
        let mut canvas = SkiaCanvas::new(1200, 800, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        treemap.padding(2.0).border_width(1.5).draw(&mut canvas)?;

        canvas.save_png("examples/images/treemap_products.png")?;
        println!("  ✓ Product categories treemap saved");
    }

    println!("\n✅ All treemap examples completed!");
    println!("\n💡 Treemap Features:");
    println!("  • Proportional rectangles based on values");
    println!("  • Automatic layout optimization");
    println!("  • Custom colors for each category");
    println!("  • Labels and values display");
    println!("  • Configurable borders and padding");
    println!("  • Smart text color (dark/light) selection");

    Ok(())
}
