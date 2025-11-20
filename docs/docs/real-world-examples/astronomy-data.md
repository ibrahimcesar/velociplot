---
sidebar_position: 4
---

# Astronomy & Space Data

Visualize data from NASA, ESA, and astronomical surveys.

## Dataset: NASA Exoplanet Archive

**Source**: [NASA Exoplanet Archive](https://exoplanetarchive.ipac.caltech.edu/)  
**License**: Public Domain  
**Description**: Confirmed exoplanets with orbital and physical properties

### Download the Data

```bash
curl -o exoplanets.csv "https://exoplanetarchive.ipac.caltech.edu/TAP/sync?query=select+*+from+ps&format=csv"
```

## Example 1: Exoplanet Mass vs Radius (Scatter)

```rust
use velociplot::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    // Parse exoplanet data
    let file = File::open("exoplanets.csv")?;
    let reader = BufReader::new(file);
    
    let mut mass = Vec::new();    // Jupiter masses
    let mut radius = Vec::new();  // Jupiter radii
    
    for line in reader.lines().skip(1) {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        
        // Parse mass and radius (filter out nulls)
        if let (Ok(m), Ok(r)) = (
            parts[10].parse::<f64>(),  // pl_bmassj
            parts[12].parse::<f64>(),  // pl_radj
        ) {
            if m > 0.0 && r > 0.0 {
                mass.push(m.log10());      // Log scale
                radius.push(r.log10());
            }
        }
    }
    
    // Create scatter plot
    let scatter = ScatterPlot::new(mass, radius)?
        .marker_shape(MarkerShape::Circle)
        .color(Color::from_hex("#3498db").unwrap())
        .marker_size(4.0)
        .opacity(0.6);
    
    let bounds = scatter.bounds().unwrap().with_padding(0.15);
    let mut canvas = SkiaCanvas::new(1000, 800, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    scatter.draw(&mut canvas)?;
    
    // Add reference lines for Solar System planets
    // Earth: ~0.003 MJ, ~0.09 RJ (log: -2.5, -1.0)
    // Jupiter: 1 MJ, 1 RJ (log: 0, 0)
    canvas.draw_circle(
        &Point2D::new(-2.5, -1.0),
        6.0,
        &Color::from_hex("#2ecc71").unwrap().to_rgba(),
        true
    )?;
    canvas.draw_text("Earth", -2.4, -0.9, 10.0, 
        &Color::from_hex("#2c3e50").unwrap().to_rgba())?;
    
    canvas.draw_circle(
        &Point2D::new(0.0, 0.0),
        6.0,
        &Color::from_hex("#e67e22").unwrap().to_rgba(),
        true
    )?;
    canvas.draw_text("Jupiter", 0.1, 0.1, 10.0,
        &Color::from_hex("#2c3e50").unwrap().to_rgba())?;
    
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("log₁₀(Mass / Jupiter Mass)");
    let y_axis = Axis::new(AxisPosition::Left)
        .label("log₁₀(Radius / Jupiter Radius)");
    
    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;
    
    canvas.save_png("exoplanet_mass_radius.png")?;
    
    println!("✓ Created mass-radius plot");
    println!("  Total exoplanets plotted: {}", mass.len());
    
    Ok(())
}
```

### Key Insights

- **Gas Giants**: Large mass, large radius (upper right)
- **Rocky Planets**: Low mass, small radius (lower left)
- **Hot Jupiters**: Similar to Jupiter but closer to stars
- **Super-Earths**: Between Earth and Neptune

## Example 2: Exoplanet Discovery Timeline

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // Parse discovery years
    let mut discovery_years = Vec::new();
    
    // ... parse data ...
    
    // Create histogram of discoveries per year
    let hist = Histogram::from_data(discovery_years, 30)?
        .color(Color::from_hex("#9b59b6").unwrap());
    
    let bounds = hist.bounds().unwrap().with_padding_top(0.1);
    let mut canvas = SkiaCanvas::new(1400, 700, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    hist.draw(&mut canvas)?;
    
    // Add milestone annotations
    canvas.draw_text(
        "First Exoplanet (51 Peg b)",
        1995.0, 50.0, 10.0,
        &Color::from_hex("#e74c3c").unwrap().to_rgba()
    )?;
    
    canvas.draw_text(
        "Kepler Mission Launch",
        2009.0, 100.0, 10.0,
        &Color::from_hex("#e74c3c").unwrap().to_rgba()
    )?;
    
    let x_axis = Axis::new(AxisPosition::Bottom).label("Discovery Year");
    let y_axis = Axis::new(AxisPosition::Left).label("Number of Discoveries");
    
    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;
    
    canvas.save_png("exoplanet_timeline.png")?;
    
    Ok(())
}
```

## Example 3: Discovery Methods (Treemap)

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // Exoplanet discovery methods and counts (as of 2024)
    let mut treemap = Treemap::new();
    
    treemap.add_item("Transit", 3869.0, Color::from_hex("#3498db").unwrap());
    treemap.add_item("Radial Velocity", 1054.0, Color::from_hex("#e74c3c").unwrap());
    treemap.add_item("Direct Imaging", 72.0, Color::from_hex("#2ecc71").unwrap());
    treemap.add_item("Microlensing", 201.0, Color::from_hex("#f39c12").unwrap());
    treemap.add_item("Other", 148.0, Color::from_hex("#95a5a6").unwrap());
    
    let bounds = treemap.bounds().unwrap();
    let mut canvas = SkiaCanvas::new(1200, 800, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    treemap.draw(&mut canvas)?;
    
    canvas.save_png("discovery_methods.png")?;
    
    println!("✓ Exoplanet discovery methods treemap");
    println!("  Total confirmed exoplanets: 5,344");
    
    Ok(())
}
```

## Example 4: Orbital Period vs Distance (Bubble)

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // Parse: orbital period (days), distance from Earth (parsecs), planet mass (size)
    let periods = vec![/* orbital periods */];
    let distances = vec![/* distances in parsecs */];
    let masses = vec![/* masses for bubble size */];
    
    let bubble = BubbleChart::new(periods, distances, masses)?
        .color(Color::from_hex("#e74c3c").unwrap())
        .opacity(0.5)
        .min_size(2.0)
        .max_size(30.0);
    
    let bounds = bubble.bounds().unwrap().with_padding(0.15);
    let mut canvas = SkiaCanvas::new(1200, 800, bounds)?;
    canvas.fill_background(&Color::from_hex("#0c0e1a").unwrap().to_rgba())?; // Space theme!
    
    bubble.draw(&mut canvas)?;
    
    // Stars in background (decorative)
    for _ in 0..200 {
        let x = random() * bounds.width() + bounds.x_min;
        let y = random() * bounds.height() + bounds.y_min;
        canvas.draw_circle(
            &Point2D::new(x, y),
            1.0,
            &Color::WHITE.to_rgba(),
            true
        )?;
    }
    
    canvas.save_png("orbital_distance.png")?;
    
    Ok(())
}
```

## More Astronomy Datasets

### NASA Databases
- [**Exoplanet Archive**](https://exoplanetarchive.ipac.caltech.edu/) - 5000+ confirmed planets
- [**MAST (Hubble/Webb)**](https://mast.stsci.edu/) - Telescope observations
- [**NED**](http://ned.ipac.caltech.edu/) - Extragalactic database

### ESA (European Space Agency)
- [**Gaia Archive**](https://gea.esac.esa.int/archive/) - 1.8 billion stars
- [**Planck Mission**](https://www.cosmos.esa.int/web/planck) - Cosmic microwave background

### Sky Surveys
- [**Sloan Digital Sky Survey**](https://www.sdss.org/) - Galaxies, quasars, stars
- [**Pan-STARRS**](https://panstarrs.stsci.edu/) - Asteroid tracking

## Example: HR Diagram (Classic Astronomy Plot)

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // Hertzsprung-Russell diagram: Temperature vs Luminosity
    // Data from Gaia DR3
    
    let temps = vec![/* stellar temperatures */];
    let luminosities = vec![/* stellar luminosities */];
    
    // Note: HR diagram traditionally shows temperature decreasing left to right
    let scatter = ScatterPlot::new(temps, luminosities)?
        .marker_shape(MarkerShape::Circle)
        .marker_size(2.0)
        .color(Color::from_hex("#ff9500").unwrap())
        .opacity(0.3);
    
    let bounds = scatter.bounds().unwrap().with_padding(0.1);
    let mut canvas = SkiaCanvas::new(1000, 1000, bounds)?;
    canvas.fill_background(&Color::from_hex("#000814").unwrap().to_rgba())?;
    
    scatter.draw(&mut canvas)?;
    
    // Label stellar classes
    canvas.draw_text("Main Sequence", 5500.0, 1.0, 12.0,
        &Color::WHITE.to_rgba())?;
    canvas.draw_text("Red Giants", 4000.0, 100.0, 12.0,
        &Color::from_hex("#e74c3c").unwrap().to_rgba())?;
    canvas.draw_text("White Dwarfs", 10000.0, 0.01, 12.0,
        &Color::from_hex("#ecf0f1").unwrap().to_rgba())?;
    
    canvas.save_png("hr_diagram.png")?;
    
    Ok(())
}
```

## Tips for Astronomy Data

1. **Log Scales**: Many astronomy quantities span orders of magnitude
2. **Units**: Be clear about units (AU, parsecs, solar masses, etc.)
3. **Coordinate Systems**: RA/Dec, Galactic, Ecliptic
4. **Uncertainties**: Always plot error bars for measurements
5. **Color Schemes**: Consider dark backgrounds for space themes

## Data Access APIs

Many archives provide TAP (Table Access Protocol):

```rust
use reqwest;

async fn query_exoplanets(query: &str) -> Result<String> {
    let url = "https://exoplanetarchive.ipac.caltech.edu/TAP/sync";
    let client = reqwest::Client::new();
    
    let response = client
        .get(url)
        .query(&[
            ("query", query),
            ("format", "csv")
        ])
        .send()
        .await?;
    
    Ok(response.text().await?)
}
```

---

**Next**: [More Datasets & Resources →](./more-datasets.md)
