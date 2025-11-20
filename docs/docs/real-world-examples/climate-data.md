---
sidebar_position: 1
---

# Climate Data Visualization

Learn to visualize real climate data from NASA and NOAA using Velociplot.

## Dataset: Global Temperature Anomalies

**Source**: [NASA GISS Surface Temperature Analysis](https://data.giss.nasa.gov/gistemp/)  
**License**: Public Domain  
**Description**: Global mean temperature anomalies from 1880-2023

### Download the Data

```bash
curl -O https://data.giss.nasa.gov/gistemp/tabledata_v4/GLB.Ts+dSST.csv
```

The CSV contains:
- Year
- Monthly temperature anomalies (°C relative to 1951-1980 baseline)
- Annual mean

## Example 1: Temperature Trend Line

:::tip Runnable Example
Complete code available at: [`examples/real-world/climate_nasa.rs`](https://github.com/ibrahimcesar/velociplot/blob/main/examples/real-world/climate_nasa.rs)

```bash
cargo run --example climate_nasa
```
:::

```rust
use velociplot::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    // Parse CSV data
    let file = File::open("GLB.Ts+dSST.csv")?;
    let reader = BufReader::new(file);
    
    let mut years = Vec::new();
    let mut temps = Vec::new();
    
    for line in reader.lines().skip(1) { // Skip header
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        
        if let (Ok(year), Ok(temp)) = (
            parts[0].parse::<f64>(),
            parts[13].parse::<f64>() // J-D column (annual mean)
        ) {
            if temp != 999.9 { // Filter missing data
                years.push(year);
                temps.push(temp);
            }
        }
    }
    
    // Create line plot
    let plot = LinePlot::new(Series::new(years.clone(), temps.clone())?)
        .color(Color::from_hex("#d62728").unwrap()) // Red for warming
        .line_width(2.0);
    
    let bounds = plot.bounds().unwrap().with_padding(0.1);
    let mut canvas = SkiaCanvas::new(1200, 700, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    // Add axes with labels
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("Year")
        .show_grid(true);
    let y_axis = Axis::new(AxisPosition::Left)
        .label("Temperature Anomaly (°C)")
        .show_grid(true);
    
    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;
    plot.draw(&mut canvas)?;
    
    // Add title using text annotation
    canvas.draw_text(
        "Global Temperature Anomalies (1880-2023)",
        600.0, 30.0, 16.0,
        &Color::from_hex("#2c3e50").unwrap().to_rgba()
    )?;
    
    canvas.save_png("climate_trend.png")?;
    
    println!("✓ Created climate_trend.png");
    println!("  Data source: NASA GISS");
    println!("  Latest anomaly: {:.2}°C ({:.0})", temps.last().unwrap(), years.last().unwrap());
    
    Ok(())
}
```

### Output

![Climate Trend](../../static/img/examples/climate_trend.png)

**Key Insights**:
- Clear warming trend since 1980s
- 2023 shows significant temperature increase
- Pre-1950 data shows more variability

## Example 2: Seasonal Temperature Patterns (Heatmap)

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // Parse monthly data (Jan-Dec for each year)
    let file = File::open("GLB.Ts+dSST.csv")?;
    let reader = BufReader::new(file);
    
    let mut data = Vec::new();
    let mut years_subset = Vec::new();
    
    // Focus on recent decades (2000-2023)
    for line in reader.lines().skip(1) {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        
        if let Ok(year) = parts[0].parse::<i32>() {
            if year >= 2000 && year <= 2023 {
                years_subset.push(year);
                
                // Extract 12 monthly values
                for i in 1..=12 {
                    if let Ok(temp) = parts[i].parse::<f64>() {
                        if temp != 999.9 {
                            data.push(temp);
                        }
                    }
                }
            }
        }
    }
    
    let rows = years_subset.len();
    let cols = 12;
    
    // Create heatmap
    let heatmap = Heatmap::new(data, rows, cols)
        .colormap(Colormap::Inferno);
    
    let bounds = heatmap.bounds().unwrap();
    let mut canvas = SkiaCanvas::new(1200, 800, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    heatmap.draw(&mut canvas)?;
    
    // Add color bar legend
    let bar_legend = BarLegend::new(-1.0, 2.0, Colormap::Inferno)
        .label("Temperature Anomaly (°C)")
        .position(BarLegendPosition::Right);
    bar_legend.draw(&mut canvas)?;
    
    canvas.save_png("climate_heatmap.png")?;
    
    Ok(())
}
```

### Output

The heatmap shows:
- **Rows**: Years (2000-2023)
- **Columns**: Months (Jan-Dec)
- **Color**: Temperature anomaly intensity

**Patterns Visible**:
- Recent years show more intense warming (brighter colors)
- Some months consistently warmer than others
- Clear acceleration in 2020s

## Example 3: Decadal Comparison (Box Plots)

Compare temperature distributions across decades:

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // Group data by decade
    let decades = vec![
        ("1880s", 1880..1890),
        ("1920s", 1920..1930),
        ("1960s", 1960..1970),
        ("2000s", 2000..2010),
        ("2020s", 2020..2024),
    ];
    
    // Parse and group data
    // ... (parsing code)
    
    // Create box plots for each decade
    let mut canvas = SkiaCanvas::new(1200, 700, bounds)?;
    
    for (i, (label, temps)) in decade_data.iter().enumerate() {
        let boxplot = BoxPlot::new(temps.clone())?
            .position(i as f64)
            .width(0.6);
        
        boxplot.draw(&mut canvas)?;
    }
    
    canvas.save_png("climate_decades.png")?;
    
    Ok(())
}
```

## More Climate Datasets

### CO₂ Levels (Mauna Loa)
- **Source**: [NOAA Global Monitoring Laboratory](https://gml.noaa.gov/ccgg/trends/data.html)
- **Perfect for**: Line plots, trend analysis

### Sea Level Rise
- **Source**: [NASA Sea Level](https://climate.nasa.gov/vital-signs/sea-level/)
- **Perfect for**: Area plots, cumulative trends

### Arctic Sea Ice Extent
- **Source**: [NSIDC](https://nsidc.org/data/seaice_index/)
- **Perfect for**: Time series, seasonal patterns

## Tips for Climate Data

1. **Handle Missing Values**: Climate data often has gaps (999.9, NaN)
2. **Show Uncertainty**: Use error bars or confidence bands
3. **Reference Baseline**: Always note the reference period (e.g., 1951-1980)
4. **Cite Sources**: Link to NASA/NOAA in your figures
5. **Update Regularly**: Climate data is updated monthly

## Learn More

- [NASA Climate Data Portal](https://climate.nasa.gov/vital-signs/)
- [NOAA Climate.gov](https://www.climate.gov/maps-data)
- [IPCC Data](https://www.ipcc.ch/data/)

---

**Next**: [Economics - GDP & Employment Data →](./economics-data.md)
