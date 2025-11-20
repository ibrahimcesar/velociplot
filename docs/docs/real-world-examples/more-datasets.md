---
sidebar_position: 5
---

# More Public Datasets

A curated list of high-quality public datasets perfect for Velociplot visualizations.

## Government & Official Statistics

### United States

| Dataset | Source | Best For |
|---------|--------|----------|
| **Census Data** | [data.census.gov](https://data.census.gov/) | Demographics, histograms, treemaps |
| **NOAA Weather** | [NOAA](https://www.ncdc.noaa.gov/) | Time series, heatmaps |
| **BLS Labor Statistics** | [BLS](https://www.bls.gov/data/) | Line plots, bar charts |
| **Energy Data (EIA)** | [EIA](https://www.eia.gov/opendata/) | Stacked areas, trends |
| **Transportation (BTS)** | [BTS](https://www.bts.gov/) | Time series, scatter |

### International

| Dataset | Source | Best For |
|---------|--------|----------|
| **World Bank** | [data.worldbank.org](https://data.worldbank.org/) | Multi-country comparisons |
| **OECD** | [stats.oecd.org](https://stats.oecd.org/) | Economic indicators |
| **UN Data** | [data.un.org](http://data.un.org/) | Global statistics |
| **Eurostat** | [ec.europa.eu/eurostat](https://ec.europa.eu/eurostat) | European Union data |

## Science & Research

### Earth & Environment

| Dataset | Source | Best For |
|---------|--------|----------|
| **Climate (IPCC)** | [ipcc-data.org](https://www.ipcc-data.org/) | Climate projections |
| **Earthquakes (USGS)** | [earthquake.usgs.gov](https://earthquake.usgs.gov/) | Geographic scatter, timeline |
| **Ocean Data** | [nodc.noaa.gov](https://www.nodc.noaa.gov/) | Heatmaps, depth profiles |
| **Air Quality** | [aqicn.org/data-platform](https://aqicn.org/data-platform) | Time series, maps |
| **Biodiversity** | [gbif.org](https://www.gbif.org/) | Species distributions |

### Space & Astronomy

| Dataset | Source | Best For |
|---------|--------|----------|
| **Exoplanets** | [exoplanetarchive.ipac.caltech.edu](https://exoplanetarchive.ipac.caltech.edu/) | Scatter, bubble charts |
| **Near-Earth Objects** | [cneos.jpl.nasa.gov](https://cneos.jpl.nasa.gov/) | Timeline, trajectories |
| **Gaia Star Catalog** | [gea.esac.esa.int](https://gea.esac.esa.int/archive/) | HR diagrams, 3D plots |
| **Solar Activity** | [swpc.noaa.gov](https://www.swpc.noaa.gov/) | Time series, cycles |

### Biology & Health

| Dataset | Source | Best For |
|---------|--------|----------|
| **Protein Data Bank** | [rcsb.org](https://www.rcsb.org/) | 3D structures, networks |
| **GenBank** | [ncbi.nlm.nih.gov/genbank](https://www.ncbi.nlm.nih.gov/genbank/) | Sequence analysis |
| **Drug Data (FDA)** | [fda.gov/drugs/drug-approvals](https://www.fda.gov/drugs/drug-approvals) | Timeline, bar charts |
| **Clinical Trials** | [clinicaltrials.gov](https://clinicaltrials.gov/) | Treemaps, timelines |

## Social & Economic

### Finance & Markets

| Dataset | Source | Best For |
|---------|--------|----------|
| **Stock Prices (Yahoo)** | [finance.yahoo.com](https://finance.yahoo.com/) | Time series, candlestick |
| **Crypto (CoinGecko)** | [coingecko.com/api](https://www.coingecko.com/en/api) | Volatility, correlations |
| **Company Financials (SEC)** | [sec.gov/edgar](https://www.sec.gov/edgar) | Financial statements |
| **Commodity Prices** | [investing.com](https://www.investing.com/) | Price trends |

### Transportation

| Dataset | Source | Best For |
|---------|--------|----------|
| **Flight Data** | [openflights.org](https://openflights.org/data.html) | Network graphs, routes |
| **NYC Taxi Trips** | [nyc.gov/tlc](https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page) | Heatmaps, temporal patterns |
| **Bike Sharing** | [citibikenyc.com/system-data](https://citibikenyc.com/system-data) | Flow diagrams |

### Education

| Dataset | Source | Best For |
|---------|--------|----------|
| **College Scorecard** | [collegescorecard.ed.gov](https://collegescorecard.ed.gov/) | Scatter, comparisons |
| **PISA Scores** | [oecd.org/pisa](https://www.oecd.org/pisa/) | International rankings |
| **Research Papers** | [arxiv.org](https://arxiv.org/) | Citation networks |

## Sports & Entertainment

| Dataset | Source | Best For |
|---------|--------|----------|
| **Olympic Games** | [olympedia.org](https://www.olympedia.org/) | Medals over time, records |
| **Baseball (Lahman)** | [seanlahman.com](http://www.seanlahman.com/baseball-archive/statistics/) | Statistics, player comparison |
| **Movies (IMDb)** | [datasets.imdbws.com](https://datasets.imdbws.com/) | Ratings, box office |
| **Spotify Charts** | [spotifycharts.com](https://spotifycharts.com/) | Music trends |

## Dataset Aggregators

### Multi-Domain Collections

| Platform | Description |
|----------|-------------|
| **Kaggle** | [kaggle.com/datasets](https://www.kaggle.com/datasets) - 100K+ datasets, ML-ready |
| **Google Dataset Search** | [datasetsearch.research.google.com](https://datasetsearch.research.google.com/) - Search engine for datasets |
| **Data.gov** | [data.gov](https://data.gov/) - US government open data |
| **EU Open Data** | [data.europa.eu](https://data.europa.eu/) - European data portal |
| **Our World in Data** | [ourworldindata.org](https://ourworldindata.org/) - Research-focused datasets |
| **Awesome Public Datasets** | [github.com/awesomedata](https://github.com/awesomedata/awesome-public-datasets) - Curated list |

## Dataset Selection Criteria

When choosing datasets for visualization:

### ✅ Good Datasets for Learning
- **Clean data**: Minimal missing values
- **Well-documented**: Clear variable definitions
- **Reasonable size**: 1K-1M rows (manageable in memory)
- **Interesting**: Real-world significance
- **Public domain**: No license restrictions

### ⚠️ Watch Out For
- **Privacy concerns**: Avoid personal data
- **License restrictions**: Check commercial use permissions
- **Data quality**: Missing values, inconsistencies
- **Size**: Very large datasets may need sampling
- **Currency**: Outdated data might mislead

## Dataset Citation

Always cite your data sources:

```rust
// In your code comments
/// Data Source: NASA GISS Surface Temperature Analysis
/// URL: https://data.giss.nasa.gov/gistemp/
/// Accessed: 2024-01-15
/// License: Public Domain
```

In your plots:
```rust
canvas.draw_text(
    "Source: NASA GISS (public domain)",
    10.0, canvas_height - 20.0, 8.0,
    &Color::from_hex("#7f8c8d").unwrap().to_rgba()
)?;
```

## Example: Loading CSV with polars

For efficient data loading:

```rust
use polars::prelude::*;
use velociplot::prelude::*;

fn main() -> Result<()> {
    // Load CSV with polars
    let df = CsvReader::from_path("data.csv")?
        .has_header(true)
        .finish()?;
    
    // Extract columns
    let x: Vec<f64> = df.column("x")?
        .f64()?
        .into_no_null_iter()
        .collect();
    
    let y: Vec<f64> = df.column("y")?
        .f64()?
        .into_no_null_iter()
        .collect();
    
    // Create plot
    let plot = ScatterPlot::new(x, y)?;
    
    // ... render
    
    Ok(())
}
```

## APIs for Real-Time Data

Many sources provide APIs:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

```rust
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct DataPoint {
    date: String,
    value: f64,
}

async fn fetch_data(url: &str) -> Result<Vec<DataPoint>> {
    let response = reqwest::get(url).await?;
    let data: Vec<DataPoint> = response.json().await?;
    Ok(data)
}
```

## Tips for Working with Real Data

1. **Exploratory Analysis**: Start with simple plots (scatter, histogram)
2. **Data Cleaning**: Handle missing values, outliers
3. **Sampling**: For large datasets, plot a sample first
4. **Multiple Views**: Show same data with different plot types
5. **Annotations**: Add context (events, thresholds, explanations)
6. **Color Blind Friendly**: Use colorblind-safe palettes
7. **High Resolution**: Publication-quality requires high DPI

## Resources

### Learning
- [The Data Visualisation Catalogue](https://datavizcatalogue.com/) - Choose right plot type
- [Our World in Data Charts](https://ourworldindata.org/charts) - Inspiration

### Tools
- **Data Cleaning**: polars, arrow
- **Statistics**: statrs
- **Geospatial**: geo, geojson
- **Dates/Times**: chrono

---

Ready to visualize real data? Start with [Climate Data →](./climate-data.md)
