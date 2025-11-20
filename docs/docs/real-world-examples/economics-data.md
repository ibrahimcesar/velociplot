---
sidebar_position: 2
---

# Economic Data Visualization

Visualize real economic indicators from the Federal Reserve, World Bank, and BLS.

## Dataset: US Unemployment Rate

**Source**: [FRED (Federal Reserve Economic Data)](https://fred.stlouisfed.org/series/UNRATE)  
**License**: Public Domain (US Government data)  
**Description**: Monthly US unemployment rate (1948-present)

### Download the Data

Visit FRED and download as CSV:
```bash
curl -o unemployment.csv "https://fred.stlouisfed.org/graph/fredgraph.csv?id=UNRATE"
```

## Example 1: Unemployment Trend with Recessions

```rust
use velociplot::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    // Parse CSV
    let file = File::open("unemployment.csv")?;
    let reader = BufReader::new(file);
    
    let mut dates = Vec::new();
    let mut rates = Vec::new();
    
    for line in reader.lines().skip(1) {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        
        // Parse date (YYYY-MM-DD) to decimal year
        if let Some(year_str) = parts[0].split('-').next() {
            if let (Ok(year), Ok(rate)) = (
                year_str.parse::<f64>(),
                parts[1].parse::<f64>()
            ) {
                dates.push(year);
                rates.push(rate);
            }
        }
    }
    
    // Create time series plot
    let plot = DateListPlot::new(dates, rates)?
        .label("Unemployment Rate")
        .color(Color::from_hex("#1f77b4").unwrap())
        .line_width(2.0)
        .show_grid(true);
    
    let bounds = plot.bounds().unwrap().with_padding(0.1);
    let mut canvas = SkiaCanvas::new(1400, 700, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    // Draw plot
    plot.draw(&mut canvas)?;
    
    // Add axes
    let x_axis = Axis::new(AxisPosition::Bottom).label("Year");
    let y_axis = Axis::new(AxisPosition::Left).label("Unemployment Rate (%)");
    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;
    
    // Add legend
    if let Some(entry) = plot.legend_entry() {
        let legend = Legend::new()
            .add_entry(entry)
            .position(LegendPosition::UpperLeft);
        legend.draw(&mut canvas)?;
    }
    
    canvas.save_png("unemployment_trend.png")?;
    
    println!("✓ Created unemployment_trend.png");
    println!("  Current rate: {:.1}%", rates.last().unwrap());
    
    Ok(())
}
```

### Key Insights

- **2008 Financial Crisis**: Peak unemployment ~10%
- **COVID-19 (2020)**: Sharp spike to 14.7%
- **Current Trends**: Recovery patterns visible

## Example 2: GDP Growth (Multiple Countries)

**Source**: [World Bank Open Data](https://data.worldbank.org/indicator/NY.GDP.MKTP.KD.ZG)  
**Dataset**: GDP growth (annual %)

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // Data for US, China, Germany, Brazil (2000-2023)
    let years: Vec<f64> = (2000..=2023).map(|y| y as f64).collect();
    
    // Parse World Bank CSV data
    let us_growth = vec![/* US GDP growth data */];
    let china_growth = vec![/* China GDP growth data */];
    let germany_growth = vec![/* Germany GDP growth data */];
    let brazil_growth = vec![/* Brazil GDP growth data */];
    
    // Create multi-series plot
    let mut plot = DateListPlot::empty();
    plot.add_series(years.clone(), us_growth, Some("United States".to_string()))?;
    plot.add_series(years.clone(), china_growth, Some("China".to_string()))?;
    plot.add_series(years.clone(), germany_growth, Some("Germany".to_string()))?;
    plot.add_series(years, brazil_growth, Some("Brazil".to_string()))?;
    
    let bounds = plot.bounds().unwrap().with_padding(0.1);
    let mut canvas = SkiaCanvas::new(1400, 800, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    plot.draw(&mut canvas)?;
    
    // Add zero line for reference
    canvas.draw_line(
        &Point2D::new(bounds.x_min, 0.0),
        &Point2D::new(bounds.x_max, 0.0),
        &Color::from_hex("#7f8c8d").unwrap().to_rgba(),
        1.0
    )?;
    
    // Legend
    let mut legend = Legend::new().position(LegendPosition::UpperRight);
    for entry in plot.legend_entries() {
        legend = legend.add_entry(entry);
    }
    legend.draw(&mut canvas)?;
    
    canvas.save_png("gdp_growth.png")?;
    
    Ok(())
}
```

## Example 3: Income Distribution (Histogram)

**Source**: [US Census Bureau](https://www.census.gov/data/tables/time-series/demo/income-poverty/historical-income-households.html)

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // Household income data (in thousands)
    let incomes = vec![
        // Data from Current Population Survey
        15.0, 25.0, 35.0, 45.0, 55.0, 65.0, 75.0, 85.0,
        95.0, 110.0, 135.0, 175.0, 250.0
    ];
    
    let frequencies = vec![
        // Frequency for each income bracket
        8.2, 9.1, 8.5, 8.3, 7.9, 7.2, 6.8, 5.9,
        5.1, 8.7, 7.4, 5.2, 11.7
    ];
    
    // Create histogram
    let hist = Histogram::from_bins(incomes.clone(), frequencies)?
        .color(Color::from_hex("#2ecc71").unwrap());
    
    let bounds = hist.bounds().unwrap().with_padding_top(0.1);
    let mut canvas = SkiaCanvas::new(1200, 700, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    hist.draw(&mut canvas)?;
    
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("Household Income ($1000s)");
    let y_axis = Axis::new(AxisPosition::Left)
        .label("Frequency (%)");
    
    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;
    
    canvas.save_png("income_distribution.png")?;
    
    Ok(())
}
```

## Example 4: Sector Employment (Treemap)

**Source**: [Bureau of Labor Statistics](https://www.bls.gov/ces/)

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // US Employment by sector (millions of workers, 2023)
    let mut treemap = Treemap::new();
    
    treemap.add_item("Healthcare", 20.8, Color::from_hex("#3498db").unwrap());
    treemap.add_item("Retail", 15.9, Color::from_hex("#e74c3c").unwrap());
    treemap.add_item("Manufacturing", 12.9, Color::from_hex("#f39c12").unwrap());
    treemap.add_item("Professional Services", 12.4, Color::from_hex("#9b59b6").unwrap());
    treemap.add_item("Leisure", 16.7, Color::from_hex("#1abc9c").unwrap());
    treemap.add_item("Education", 13.8, Color::from_hex("#e67e22").unwrap());
    treemap.add_item("Transportation", 6.2, Color::from_hex("#95a5a6").unwrap());
    treemap.add_item("Construction", 7.9, Color::from_hex("#16a085").unwrap());
    treemap.add_item("Finance", 6.3, Color::from_hex("#d35400").unwrap());
    treemap.add_item("Other", 40.1, Color::from_hex("#7f8c8d").unwrap());
    
    let bounds = treemap.bounds().unwrap();
    let mut canvas = SkiaCanvas::new(1200, 800, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    treemap.draw(&mut canvas)?;
    
    canvas.save_png("sector_employment.png")?;
    
    println!("✓ Created sector employment treemap");
    println!("  Source: BLS Current Employment Statistics");
    
    Ok(())
}
```

## More Economic Datasets

### Federal Reserve Data (FRED)
- [**Inflation Rate (CPI)**](https://fred.stlouisfed.org/series/CPIAUCSL) - Consumer Price Index
- [**Interest Rates**](https://fred.stlouisfed.org/series/DFF) - Federal Funds Rate
- [**Stock Market**](https://fred.stlouisfed.org/series/SP500) - S&P 500 Index

### World Bank
- [**GDP per capita**](https://data.worldbank.org/indicator/NY.GDP.PCAP.CD)
- [**Poverty rates**](https://data.worldbank.org/topic/poverty)
- [**Trade balance**](https://data.worldbank.org/indicator/NE.RSB.GNFS.CD)

### OECD
- [**Productivity**](https://data.oecd.org/lprdty/gdp-per-hour-worked.htm)
- [**Labor force participation**](https://data.oecd.org/emp/labour-force-participation-rate.htm)

## Tips for Economic Data

1. **Seasonally Adjusted**: Use SA (seasonally adjusted) data for trends
2. **Real vs Nominal**: Adjust for inflation when comparing over time
3. **Recession Shading**: Highlight recession periods with background colors
4. **Multiple Scales**: Use dual y-axes for different units
5. **Sources Matter**: Always cite BLS, FRED, World Bank, etc.

## API Access

Most sources provide APIs for programmatic access:

```rust
// Example: FRED API
use reqwest;

async fn fetch_fred_data(series_id: &str, api_key: &str) -> Result<Vec<f64>> {
    let url = format!(
        "https://api.stlouisfed.org/fred/series/observations?series_id={}&api_key={}&file_type=json",
        series_id, api_key
    );
    
    // ... fetch and parse JSON
}
```

---

**Next**: [Health Data - COVID-19 & Disease Trends →](./health-data.md)
