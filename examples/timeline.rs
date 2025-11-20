//! Timeline examples
//!
//! Demonstrates timeline visualizations for chronological events with
//! dates, labels, and descriptions.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Timeline examples");

    // ============================================================
    // Example 1: Company History (Horizontal)
    // ============================================================
    {
        println!("\n📊 Example 1: Company History");

        let mut timeline = Timeline::new();
        timeline
            .add_event(2015.0, "Founded", Some("Company established".to_string()))
            .add_event(2016.0, "Seed Round", Some("$1M raised".to_string()))
            .add_event(2017.0, "First Product", Some("V1.0 launched".to_string()))
            .add_event(2018.0, "Series A", Some("$10M funding".to_string()))
            .add_event(
                2020.0,
                "Expansion",
                Some("International markets".to_string()),
            )
            .add_event(2022.0, "Series B", Some("$50M raised".to_string()))
            .add_event(2024.0, "IPO", Some("Public listing".to_string()));

        let bounds = timeline.bounds().unwrap();
        let mut canvas = SkiaCanvas::new(1200, 500, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        timeline.draw(&mut canvas)?;

        canvas.save_png("examples/images/timeline_company.png")?;
        println!("  ✓ Company history timeline saved");
    }

    // ============================================================
    // Example 2: Project Milestones (Vertical)
    // ============================================================
    {
        println!("\n📊 Example 2: Project Milestones");

        let mut timeline = Timeline::new();
        timeline
            .add_event(1.0, "Planning", Some("Requirements".to_string()))
            .add_event(2.0, "Design", Some("Architecture".to_string()))
            .add_event(3.0, "Development", Some("Sprint 1-3".to_string()))
            .add_event(4.0, "Testing", Some("QA Phase".to_string()))
            .add_event(5.0, "Deployment", Some("Production".to_string()))
            .add_event(6.0, "Monitoring", Some("Performance".to_string()));

        let timeline = timeline
            .orientation(TimelineOrientation::Vertical)
            .line_color(Color::from_hex("#2ecc71").unwrap())
            .marker_color(Color::from_hex("#27ae60").unwrap());

        let bounds = timeline.bounds().unwrap();
        let mut canvas = SkiaCanvas::new(600, 900, bounds)?;
        canvas.fill_background(&Color::from_hex("#ecf0f1").unwrap().to_rgba())?;

        timeline.draw(&mut canvas)?;

        canvas.save_png("examples/images/timeline_project_vertical.png")?;
        println!("  ✓ Project milestones timeline saved");
    }

    // ============================================================
    // Example 3: Historical Events (Colored Markers)
    // ============================================================
    {
        println!("\n📊 Example 3: Historical Events");

        let mut timeline = Timeline::new();
        timeline
            .add_event_colored(
                1969.0,
                "Moon Landing",
                Some("Apollo 11 mission".to_string()),
                Color::from_hex("#3498db").unwrap(),
            )
            .add_event_colored(
                1989.0,
                "Berlin Wall",
                Some("Fall of the wall".to_string()),
                Color::from_hex("#e74c3c").unwrap(),
            )
            .add_event_colored(
                1991.0,
                "Web Created",
                Some("WWW invented".to_string()),
                Color::from_hex("#9b59b6").unwrap(),
            )
            .add_event_colored(
                2000.0,
                "Millennium",
                Some("Y2K transition".to_string()),
                Color::from_hex("#f39c12").unwrap(),
            )
            .add_event_colored(
                2007.0,
                "iPhone",
                Some("First smartphone".to_string()),
                Color::from_hex("#1abc9c").unwrap(),
            )
            .add_event_colored(
                2020.0,
                "Pandemic",
                Some("COVID-19".to_string()),
                Color::from_hex("#e67e22").unwrap(),
            );

        let bounds = timeline.bounds().unwrap();
        let mut canvas = SkiaCanvas::new(1200, 500, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        timeline
            .marker_size(10.0)
            .line_width(3.0)
            .draw(&mut canvas)?;

        canvas.save_png("examples/images/timeline_history.png")?;
        println!("  ✓ Historical events timeline saved");
    }

    // ============================================================
    // Example 4: Product Releases (No Alternating)
    // ============================================================
    {
        println!("\n📊 Example 4: Product Releases");

        let mut timeline = Timeline::new();
        timeline
            .add_event(1.0, "v1.0", Some("Initial release".to_string()))
            .add_event(2.0, "v1.5", Some("Bug fixes".to_string()))
            .add_event(3.5, "v2.0", Some("Major update".to_string()))
            .add_event(5.0, "v2.5", Some("Performance".to_string()))
            .add_event(7.0, "v3.0", Some("New features".to_string()));

        let timeline = timeline
            .alternating_sides(false)
            .line_color(Color::from_hex("#9b59b6").unwrap())
            .marker_color(Color::from_hex("#8e44ad").unwrap())
            .label_offset(25.0);

        let bounds = timeline.bounds().unwrap();
        let mut canvas = SkiaCanvas::new(1000, 400, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        timeline.draw(&mut canvas)?;

        canvas.save_png("examples/images/timeline_releases.png")?;
        println!("  ✓ Product releases timeline saved");
    }

    // ============================================================
    // Example 5: Timeline without Descriptions
    // ============================================================
    {
        println!("\n📊 Example 5: Simple Timeline");

        let mut timeline = Timeline::new();
        timeline
            .add_event(2020.0, "Q1", None)
            .add_event(2020.25, "Q2", None)
            .add_event(2020.5, "Q3", None)
            .add_event(2020.75, "Q4", None)
            .add_event(2021.0, "Q1", None)
            .add_event(2021.25, "Q2", None);

        let timeline = timeline
            .show_descriptions(false)
            .line_color(Color::from_hex("#34495e").unwrap())
            .marker_color(Color::from_hex("#2c3e50").unwrap())
            .marker_size(6.0);

        let bounds = timeline.bounds().unwrap();
        let mut canvas = SkiaCanvas::new(1000, 300, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        timeline.draw(&mut canvas)?;

        canvas.save_png("examples/images/timeline_simple.png")?;
        println!("  ✓ Simple timeline saved");
    }

    // ============================================================
    // Example 6: Technology Evolution
    // ============================================================
    {
        println!("\n📊 Example 6: Technology Evolution");

        let mut timeline = Timeline::new();
        timeline
            .add_event_colored(
                1990.0,
                "HTML",
                Some("Web markup".to_string()),
                Color::from_hex("#e74c3c").unwrap(),
            )
            .add_event_colored(
                1995.0,
                "JavaScript",
                Some("Browser scripting".to_string()),
                Color::from_hex("#f39c12").unwrap(),
            )
            .add_event_colored(
                2000.0,
                "CSS3",
                Some("Styling".to_string()),
                Color::from_hex("#3498db").unwrap(),
            )
            .add_event_colored(
                2006.0,
                "jQuery",
                Some("DOM manipulation".to_string()),
                Color::from_hex("#9b59b6").unwrap(),
            )
            .add_event_colored(
                2010.0,
                "Angular",
                Some("Framework".to_string()),
                Color::from_hex("#2ecc71").unwrap(),
            )
            .add_event_colored(
                2013.0,
                "React",
                Some("UI library".to_string()),
                Color::from_hex("#1abc9c").unwrap(),
            )
            .add_event_colored(
                2014.0,
                "Vue",
                Some("Progressive".to_string()),
                Color::from_hex("#16a085").unwrap(),
            );

        let bounds = timeline.bounds().unwrap();
        let mut canvas = SkiaCanvas::new(1200, 500, bounds)?;
        canvas.fill_background(&Color::from_hex("#2c3e50").unwrap().to_rgba())?;

        let timeline = timeline.line_color(Color::WHITE).line_width(2.5);

        timeline.draw(&mut canvas)?;

        canvas.save_png("examples/images/timeline_tech.png")?;
        println!("  ✓ Technology evolution timeline saved");
    }

    println!("\n✅ All timeline examples completed!");
    println!("\n💡 Timeline Features:");
    println!("  • Horizontal and vertical orientations");
    println!("  • Event markers with custom colors");
    println!("  • Labels and descriptions");
    println!("  • Date/position display");
    println!("  • Connector lines from timeline to labels");
    println!("  • Alternating label positions");
    println!("  • Customizable styling");

    Ok(())
}
