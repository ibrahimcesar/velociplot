# Real-World Data Examples

This directory contains examples that demonstrate Velociplot with real public datasets.

## Available Examples

### 🌍 Climate Data - `climate_nasa.rs`

**Dataset**: NASA GISS Global Temperature Anomalies  
**Source**: https://data.giss.nasa.gov/gistemp/

**Plots**:
- Temperature trend line (1880-2023)
- Decadal comparison box plots

**Run**:
```bash
cargo run --example climate_nasa
```

**Outputs**:
- `examples/images/climate_nasa_temp.png`
- `examples/images/climate_nasa_decades.png`

## Using Real Data

The examples use simulated data by default so they work out-of-the-box. To use actual datasets:

1. **Download the data** following instructions in each example
2. **Parse the CSV** using your preferred method:
   ```rust
   use std::fs::File;
   use std::io::{BufRead, BufReader};
   
   let file = File::open("data.csv")?;
   let reader = BufReader::new(file);
   
   for line in reader.lines().skip(1) { // Skip header
       let line = line?;
       let parts: Vec<&str> = line.split(',').collect();
       // Parse your data...
   }
   ```

3. **Or use polars** for more complex datasets:
   ```bash
   cargo add polars --features csv-file
   ```
   ```rust
   use polars::prelude::*;
   
   let df = CsvReader::from_path("data.csv")?
       .has_header(true)
       .finish()?;
   ```

## Data Sources

All examples link to:
- ✅ Public domain or openly licensed datasets
- ✅ Government and research institutions
- ✅ Regularly updated data sources
- ✅ Well-documented data formats

### Popular Sources

| Domain | Source | Examples |
|--------|--------|----------|
| **Climate** | NASA, NOAA, IPCC | Temperature, CO₂, sea level |
| **Economics** | FRED, World Bank, BLS | GDP, unemployment, inflation |
| **Health** | WHO, CDC, OWID | COVID-19, mortality, disease burden |
| **Astronomy** | NASA, ESA, SDSS | Exoplanets, star catalogs, surveys |

## Example Template

Use this template for new real-world examples:

```rust
//! [Dataset Name] Example
//! 
//! # Data Source
//! - **Source**: [Organization]
//! - **URL**: [Direct link]
//! - **License**: [License type]
//! 
//! # Setup
//! ```bash
//! curl -o data.csv [URL]
//! ```

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("📊 [Dataset Name] Visualization");
    
    // 1. Load/simulate data
    let x = vec![/* your data */];
    let y = vec![/* your data */];
    
    // 2. Create plot
    let plot = ScatterPlot::new(x, y)?;
    
    // 3. Render
    let bounds = plot.bounds().unwrap().with_padding(0.1);
    let mut canvas = SkiaCanvas::new(1200, 800, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    plot.draw(&mut canvas)?;
    
    // 4. Add axes, labels, citations
    let x_axis = Axis::new(AxisPosition::Bottom).label("X Label");
    let y_axis = Axis::new(AxisPosition::Left).label("Y Label");
    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;
    
    canvas.draw_text(
        "Source: [Organization] (license)",
        50.0, canvas_height - 20.0, 10.0,
        &Color::from_hex("#7f8c8d").unwrap().to_rgba()
    )?;
    
    // 5. Save
    canvas.save_png("examples/images/your_plot.png")?;
    
    Ok(())
}
```

## Contributing Examples

Want to add a real-world example? We'd love your contribution!

1. Choose a public dataset from a reputable source
2. Create a well-documented example
3. Include citation and license info
4. Add it to Cargo.toml
5. Submit a PR!

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## Documentation

For detailed tutorials on working with real data, see:
- [Documentation Site](https://ibrahimcesar.github.io/velociplot/)
- [Real-World Examples Guide](../../docs/docs/real-world-examples/)

## Citation

When using these examples in your work, please cite both the data source AND Velociplot:

```
Data: NASA Goddard Institute for Space Studies (2024). GISS Surface Temperature Analysis (GISTEMP), version 4. https://data.giss.nasa.gov/gistemp/

Visualization: Velociplot v0.0.1. https://github.com/ibrahimcesar/velociplot
```
