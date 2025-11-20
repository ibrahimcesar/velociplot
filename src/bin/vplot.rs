//! velociplot CLI - Quick plotting from the command line

use clap::{Parser, Subcommand};
use std::error::Error as StdError;
use std::fs::File;
use std::path::Path;
use std::result::Result as StdResult;

type CliResult<T> = StdResult<T, Box<dyn StdError>>;

#[cfg(feature = "cli")]
use csv::ReaderBuilder;
#[cfg(feature = "cli")]
use serde_json::Value;

#[cfg(feature = "cli")]
use velociplot::prelude::*;

/// velociplot CLI - Scientific plotting at velociraptor speed 🦖
#[derive(Parser)]
#[command(name = "vplot")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Plot data from file
    Plot {
        /// Input data file (CSV, JSON)
        #[arg(value_name = "FILE")]
        input: String,

        /// Plot type
        #[arg(short, long, value_enum, default_value = "line")]
        plot_type: PlotType,

        /// X-axis column name or index (0-based)
        #[arg(short = 'x', long)]
        x_column: Option<String>,

        /// Y-axis column name(s) or index (0-based). Can be specified multiple times for multiple series.
        #[arg(short = 'y', long)]
        y_column: Vec<String>,

        /// Output file (PNG)
        #[arg(short, long, default_value = "plot.png")]
        output: String,

        /// X-axis label
        #[arg(long)]
        xlabel: Option<String>,

        /// Y-axis label
        #[arg(long)]
        ylabel: Option<String>,

        /// Width in pixels
        #[arg(long, default_value = "800")]
        width: u32,

        /// Height in pixels
        #[arg(long, default_value = "600")]
        height: u32,

        /// Show legend
        #[arg(long)]
        legend: bool,

        /// Show grid
        #[arg(long)]
        grid: bool,
    },

    /// Generate code template for a plot type
    Template {
        /// Plot type to generate template for
        #[arg(value_enum)]
        plot_type: PlotType,

        /// Output file path (default: stdout)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Show version info
    Version,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum PlotType {
    /// Line plot
    Line,
    /// Scatter plot
    Scatter,
    /// Bar chart
    Bar,
    /// Histogram
    Histogram,
}

#[cfg(feature = "cli")]
fn main() -> CliResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Plot {
            input,
            plot_type,
            x_column,
            y_column,
            output,
            xlabel,
            ylabel,
            width,
            height,
            legend,
            grid,
        } => {
            println!("🦖 velociplot - Scientific plotting at velociraptor speed");
            println!();

            // Parse data from file
            let (x_data, y_data_sets, labels) = parse_data_file(&input, x_column.as_deref(), &y_column)?;

            println!("📊 Data loaded:");
            println!("   {} data points", x_data.len());
            println!("   {} series", y_data_sets.len());
            println!();

            // Generate plot
            generate_plot(
                &x_data,
                &y_data_sets,
                &labels,
                plot_type,
                &output,
                xlabel.as_deref(),
                ylabel.as_deref(),
                width,
                height,
                legend,
                grid,
            )?;

            println!("✅ Plot saved to: {}", output);
            Ok(())
        }
        Commands::Template { plot_type, output } => {
            let template = generate_template(&plot_type);

            if let Some(path) = output {
                std::fs::write(&path, template)?;
                println!("✅ Template saved to: {}", path);
            } else {
                println!("{}", template);
            }
            Ok(())
        }
        Commands::Version => {
            println!("velociplot v{}", env!("CARGO_PKG_VERSION"));
            println!("Fast, publication-quality scientific plotting");
            Ok(())
        }
    }
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("Error: CLI support not enabled. Rebuild with --features cli");
    std::process::exit(1);
}

#[cfg(feature = "cli")]
fn parse_data_file(
    path: &str,
    x_col: Option<&str>,
    y_cols: &[String],
) -> CliResult<(Vec<f64>, Vec<Vec<f64>>, Vec<String>)> {
    let extension = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    match extension.to_lowercase().as_str() {
        "csv" => parse_csv(path, x_col, y_cols),
        "json" => parse_json(path, x_col, y_cols),
        _ => Err(format!("Unsupported file format: {}", extension).into()),
    }
}

#[cfg(feature = "cli")]
fn parse_csv(
    path: &str,
    x_col: Option<&str>,
    y_cols: &[String],
) -> CliResult<(Vec<f64>, Vec<Vec<f64>>, Vec<String>)> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Get headers
    let headers = reader.headers()?.clone();
    let header_vec: Vec<String> = headers.iter().map(|s| s.to_string()).collect();

    // Determine column indices
    let x_idx = if let Some(col) = x_col {
        // Try parsing as index first, then as name
        col.parse::<usize>().ok().or_else(|| {
            header_vec.iter().position(|h| h == col)
        }).ok_or_else(|| format!("X column not found: {}", col))?
    } else {
        0 // Default to first column
    };

    let y_indices: CliResult<Vec<usize>> = if y_cols.is_empty() {
        // Default to all columns except x
        Ok((0..header_vec.len()).filter(|&i| i != x_idx).collect())
    } else {
        y_cols
            .iter()
            .map(|col| {
                col.parse::<usize>().ok().or_else(|| {
                    header_vec.iter().position(|h| h == col)
                }).ok_or_else(|| format!("Y column not found: {}", col).into())
            })
            .collect()
    };

    let y_indices = y_indices?;

    // Get labels for series
    let labels: Vec<String> = y_indices
        .iter()
        .map(|&idx| header_vec.get(idx).cloned().unwrap_or_else(|| format!("Series {}", idx)))
        .collect();

    // Read data
    let mut x_data = Vec::new();
    let mut y_data_sets: Vec<Vec<f64>> = vec![Vec::new(); y_indices.len()];

    for result in reader.records() {
        let record = result?;

        // Parse x value
        let x_str = record.get(x_idx).ok_or("X column out of range")?;
        let x: f64 = x_str.parse().map_err(|_| format!("Invalid number in X column: {}", x_str))?;
        x_data.push(x);

        // Parse y values
        for (i, &y_idx) in y_indices.iter().enumerate() {
            let y_str = record.get(y_idx).ok_or("Y column out of range")?;
            let y: f64 = y_str.parse().map_err(|_| format!("Invalid number in Y column: {}", y_str))?;
            y_data_sets[i].push(y);
        }
    }

    Ok((x_data, y_data_sets, labels))
}

#[cfg(feature = "cli")]
fn parse_json(
    path: &str,
    x_col: Option<&str>,
    y_cols: &[String],
) -> CliResult<(Vec<f64>, Vec<Vec<f64>>, Vec<String>)> {
    let file = File::open(path)?;
    let json: Value = serde_json::from_reader(file)?;

    // Expect array of objects
    let array = json.as_array().ok_or("JSON must be an array of objects")?;

    if array.is_empty() {
        return Err("JSON array is empty".into());
    }

    // Determine column names
    let first_obj = array[0].as_object().ok_or("JSON array must contain objects")?;
    let available_keys: Vec<String> = first_obj.keys().cloned().collect();

    let x_key = x_col.unwrap_or_else(|| available_keys.first().map(|s| s.as_str()).unwrap_or("x"));

    let y_keys: Vec<String> = if y_cols.is_empty() {
        available_keys.iter().filter(|k| k.as_str() != x_key).cloned().collect()
    } else {
        y_cols.to_vec()
    };

    let labels = y_keys.clone();

    // Parse data
    let mut x_data = Vec::new();
    let mut y_data_sets: Vec<Vec<f64>> = vec![Vec::new(); y_keys.len()];

    for obj_val in array {
        let obj = obj_val.as_object().ok_or("Expected object in array")?;

        // Parse x
        let x = obj
            .get(x_key)
            .and_then(|v| v.as_f64())
            .ok_or_else(|| format!("Missing or invalid X value: {}", x_key))?;
        x_data.push(x);

        // Parse y values
        for (i, y_key) in y_keys.iter().enumerate() {
            let y = obj
                .get(y_key)
                .and_then(|v| v.as_f64())
                .ok_or_else(|| format!("Missing or invalid Y value: {}", y_key))?;
            y_data_sets[i].push(y);
        }
    }

    Ok((x_data, y_data_sets, labels))
}

#[cfg(feature = "cli")]
fn generate_plot(
    x_data: &[f64],
    y_data_sets: &[Vec<f64>],
    labels: &[String],
    plot_type: PlotType,
    output: &str,
    xlabel: Option<&str>,
    ylabel: Option<&str>,
    width: u32,
    height: u32,
    show_legend: bool,
    show_grid: bool,
) -> CliResult<()> {
    // Default colors
    let colors = [
        Color::from_hex("#3498db").unwrap(), // Blue
        Color::from_hex("#e74c3c").unwrap(), // Red
        Color::from_hex("#2ecc71").unwrap(), // Green
        Color::from_hex("#f39c12").unwrap(), // Orange
        Color::from_hex("#9b59b6").unwrap(), // Purple
        Color::from_hex("#1abc9c").unwrap(), // Teal
    ];

    match plot_type {
        PlotType::Line => generate_line_plot(
            x_data, y_data_sets, labels, &colors, output, xlabel, ylabel,
            width, height, show_legend, show_grid
        ),
        PlotType::Scatter => generate_scatter_plot(
            x_data, y_data_sets, labels, &colors, output, xlabel, ylabel,
            width, height, show_legend, show_grid
        ),
        PlotType::Bar => generate_bar_plot(
            x_data, y_data_sets, labels, &colors, output, xlabel, ylabel,
            width, height, show_grid
        ),
        PlotType::Histogram => generate_histogram(
            y_data_sets, &colors, output, xlabel, ylabel,
            width, height, show_grid
        ),
    }
}

#[cfg(feature = "cli")]
fn generate_line_plot(
    x_data: &[f64],
    y_data_sets: &[Vec<f64>],
    labels: &[String],
    colors: &[Color],
    output: &str,
    xlabel: Option<&str>,
    ylabel: Option<&str>,
    width: u32,
    height: u32,
    show_legend: bool,
    show_grid: bool,
) -> CliResult<()> {
    let mut plots = Vec::new();
    for (i, y_data) in y_data_sets.iter().enumerate() {
        let series = Series::new(x_data.to_vec(), y_data.clone())
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;
        let plot = LinePlot::new(series)
            .color(colors[i % colors.len()].clone())
            .line_width(2.0)
            .label(&labels[i]);
        plots.push(plot);
    }

    let mut bounds = plots[0].bounds().ok_or("Failed to calculate bounds")?;
    for plot in &plots[1..] {
        bounds = bounds.union(&plot.bounds().ok_or("Failed to calculate bounds")?);
    }
    bounds = bounds.with_padding(0.1);

    let mut canvas = SkiaCanvas::new(width, height, bounds)
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    canvas.fill_background(&Color::WHITE.to_rgba())
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    let x_axis = Axis::new(AxisPosition::Bottom)
        .label(xlabel.unwrap_or("X"))
        .show_grid(show_grid);
    let y_axis = Axis::new(AxisPosition::Left)
        .label(ylabel.unwrap_or("Y"))
        .show_grid(show_grid);

    x_axis.draw(&mut canvas).map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    y_axis.draw(&mut canvas).map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    for plot in &plots {
        plot.draw(&mut canvas).map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    }

    if show_legend && y_data_sets.len() > 1 {
        let mut legend = Legend::new().position(LegendPosition::UpperRight);
        for plot in &plots {
            if let Some(entry) = plot.legend_entry() {
                legend = legend.add_entry(entry);
            }
        }
        legend.draw(&mut canvas).map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    }

    canvas.save_png(output).map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    Ok(())
}

#[cfg(feature = "cli")]
fn generate_scatter_plot(
    x_data: &[f64],
    y_data_sets: &[Vec<f64>],
    labels: &[String],
    colors: &[Color],
    output: &str,
    xlabel: Option<&str>,
    ylabel: Option<&str>,
    width: u32,
    height: u32,
    show_legend: bool,
    show_grid: bool,
) -> CliResult<()> {
    let mut plots = Vec::new();
    for (i, y_data) in y_data_sets.iter().enumerate() {
        let series = Series::new(x_data.to_vec(), y_data.clone())
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;
        let plot = ScatterPlot::new(series)
            .color(colors[i % colors.len()].clone())
            .marker_size(6.0)
            .label(&labels[i]);
        plots.push(plot);
    }

    let mut bounds = plots[0].bounds().ok_or("Failed to calculate bounds")?;
    for plot in &plots[1..] {
        bounds = bounds.union(&plot.bounds().ok_or("Failed to calculate bounds")?);
    }
    bounds = bounds.with_padding(0.1);

    let mut canvas = SkiaCanvas::new(width, height, bounds)
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    canvas.fill_background(&Color::WHITE.to_rgba())
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    let x_axis = Axis::new(AxisPosition::Bottom)
        .label(xlabel.unwrap_or("X"))
        .show_grid(show_grid);
    let y_axis = Axis::new(AxisPosition::Left)
        .label(ylabel.unwrap_or("Y"))
        .show_grid(show_grid);

    x_axis.draw(&mut canvas).map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    y_axis.draw(&mut canvas).map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    for plot in &plots {
        plot.draw(&mut canvas).map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    }

    if show_legend && y_data_sets.len() > 1 {
        let mut legend = Legend::new().position(LegendPosition::UpperRight);
        for plot in &plots {
            if let Some(entry) = plot.legend_entry() {
                legend = legend.add_entry(entry);
            }
        }
        legend.draw(&mut canvas).map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    }

    canvas.save_png(output).map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    Ok(())
}

#[cfg(feature = "cli")]
fn generate_bar_plot(
    x_data: &[f64],
    y_data_sets: &[Vec<f64>],
    labels: &[String],
    colors: &[Color],
    output: &str,
    xlabel: Option<&str>,
    ylabel: Option<&str>,
    width: u32,
    height: u32,
    show_grid: bool,
) -> CliResult<()> {
    if y_data_sets.is_empty() {
        return Err("No data available for bar plot".into());
    }

    let series = Series::new(x_data.to_vec(), y_data_sets[0].clone())
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    let plot = BarPlot::new(series)
        .color(colors[0].clone())
        .label(&labels[0]);

    let bounds = plot.bounds().ok_or("Failed to calculate bounds")?.with_padding(0.15);

    let mut canvas = SkiaCanvas::new(width, height, bounds)
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    canvas.fill_background(&Color::WHITE.to_rgba())
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    let x_axis = Axis::new(AxisPosition::Bottom)
        .label(xlabel.unwrap_or("X"))
        .show_grid(show_grid);
    let y_axis = Axis::new(AxisPosition::Left)
        .label(ylabel.unwrap_or("Y"))
        .show_grid(show_grid);

    x_axis.draw(&mut canvas).map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    y_axis.draw(&mut canvas).map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    plot.draw(&mut canvas).map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    canvas.save_png(output).map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    Ok(())
}

#[cfg(feature = "cli")]
fn generate_histogram(
    y_data_sets: &[Vec<f64>],
    colors: &[Color],
    output: &str,
    xlabel: Option<&str>,
    ylabel: Option<&str>,
    width: u32,
    height: u32,
    show_grid: bool,
) -> CliResult<()> {
    if y_data_sets.is_empty() {
        return Err("No data available for histogram".into());
    }

    let plot = Histogram::new(&y_data_sets[0])
        .bins(20)
        .color(colors[0].clone());

    let bounds = plot.bounds().ok_or("Failed to calculate bounds")?.with_padding(0.15);

    let mut canvas = SkiaCanvas::new(width, height, bounds)
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    canvas.fill_background(&Color::WHITE.to_rgba())
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    let x_axis = Axis::new(AxisPosition::Bottom)
        .label(xlabel.unwrap_or("Value"))
        .show_grid(show_grid);
    let y_axis = Axis::new(AxisPosition::Left)
        .label(ylabel.unwrap_or("Frequency"))
        .show_grid(show_grid);

    x_axis.draw(&mut canvas).map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    y_axis.draw(&mut canvas).map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    plot.draw(&mut canvas).map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    canvas.save_png(output).map_err(|e| Box::new(e) as Box<dyn StdError>)?;
    Ok(())
}

#[cfg(feature = "cli")]
fn generate_template(plot_type: &PlotType) -> String {
    match plot_type {
        PlotType::Line => r##"use velociplot::prelude::*;

fn main() -> Result<()> {
    // Create your data
    let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.1).collect();
    let y: Vec<f64> = x.iter().map(|&x| x.sin()).collect();

    // Create series and plot
    let series = Series::new(x, y)?;
    let plot = LinePlot::new(series)
        .color(Color::from_hex("#3498db").unwrap())
        .line_width(2.0)
        .label("sin(x)");

    // Set up canvas
    let bounds = plot.bounds().unwrap().with_padding(0.1);
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
    Axis::new(AxisPosition::Bottom).label("X").show_grid(true).draw(&mut canvas)?;
    Axis::new(AxisPosition::Left).label("Y").show_grid(true).draw(&mut canvas)?;

    // Draw plot
    plot.draw(&mut canvas)?;

    // Save
    canvas.save_png("line_plot.png")?;

    Ok(())
}
"##.to_string(),
        PlotType::Scatter => r##"use velociplot::prelude::*;

fn main() -> Result<()> {
    // Create your data
    let x: Vec<f64> = (0..50).map(|i| i as f64).collect();
    let y: Vec<f64> = x.iter().map(|&x| x + (x * 0.5).sin() * 5.0).collect();

    // Create scatter plot
    let plot = ScatterPlot::new(x, y)?
        .color(Color::from_hex("#e74c3c").unwrap())
        .marker_size(6.0)
        .marker_shape(MarkerShape::Circle);

    // Set up canvas
    let bounds = plot.bounds().unwrap().with_padding(0.1);
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
    Axis::new(AxisPosition::Bottom).label("X").draw(&mut canvas)?;
    Axis::new(AxisPosition::Left).label("Y").draw(&mut canvas)?;

    // Draw plot
    plot.draw(&mut canvas)?;

    // Save
    canvas.save_png("scatter_plot.png")?;

    Ok(())
}
"##.to_string(),
        PlotType::Bar => r##"use velociplot::prelude::*;

fn main() -> Result<()> {
    // Create your data
    let categories: Vec<f64> = (0..5).map(|i| i as f64).collect();
    let values = vec![23.0, 45.0, 38.0, 52.0, 31.0];

    // Create bar plot
    let plot = BarPlot::new(categories, values)?
        .color(Color::from_hex("#2ecc71").unwrap())
        .bar_width(0.6);

    // Set up canvas
    let bounds = plot.bounds().unwrap().with_padding(0.15);
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
    Axis::new(AxisPosition::Bottom).label("Category").draw(&mut canvas)?;
    Axis::new(AxisPosition::Left).label("Value").draw(&mut canvas)?;

    // Draw plot
    plot.draw(&mut canvas)?;

    // Save
    canvas.save_png("bar_plot.png")?;

    Ok(())
}
"##.to_string(),
        PlotType::Histogram => r##"use velociplot::prelude::*;

fn main() -> Result<()> {
    // Create your data
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let data: Vec<f64> = (0..1000)
        .map(|_| {
            // Simulate normal distribution (Box-Muller transform)
            let u1: f64 = rng.gen();
            let u2: f64 = rng.gen();
            (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
        })
        .collect();

    // Create histogram
    let plot = Histogram::new(&data, 30)?
        .color(Color::from_hex("#9b59b6").unwrap());

    // Set up canvas
    let bounds = plot.bounds().unwrap().with_padding(0.15);
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
    Axis::new(AxisPosition::Bottom).label("Value").draw(&mut canvas)?;
    Axis::new(AxisPosition::Left).label("Frequency").draw(&mut canvas)?;

    // Draw plot
    plot.draw(&mut canvas)?;

    // Save
    canvas.save_png("histogram.png")?;

    Ok(())
}
"##.to_string(),
    }
}
