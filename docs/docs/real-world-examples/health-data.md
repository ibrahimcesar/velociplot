---
sidebar_position: 3
---

# Health Data Visualization

Visualize public health data from WHO, CDC, and Our World in Data.

## Dataset: COVID-19 Cases and Deaths

**Source**: [Our World in Data](https://github.com/owid/covid-19-data)  
**License**: Creative Commons BY  
**Description**: Global COVID-19 data updated daily

### Download the Data

```bash
curl -o owid-covid-data.csv "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/owid-covid-data.csv"
```

## Example 1: Cases Over Time (Multiple Countries)

```rust
use velociplot::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> Result<()> {
    // Parse COVID data for specific countries
    let file = File::open("owid-covid-data.csv")?;
    let reader = BufReader::new(file);
    
    let countries = vec!["USA", "GBR", "DEU", "BRA", "IND"];
    let mut data: HashMap<String, (Vec<f64>, Vec<f64>)> = HashMap::new();
    
    for country in &countries {
        data.insert(country.to_string(), (Vec::new(), Vec::new()));
    }
    
    for line in reader.lines().skip(1) {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        
        let country_code = parts[1];
        if let Some((dates, cases)) = data.get_mut(country_code) {
            // Parse date to decimal year
            if let (Some(year), Ok(new_cases)) = (
                parse_date_to_year(parts[2]),
                parts[5].parse::<f64>() // new_cases column
            ) {
                dates.push(year);
                cases.push(new_cases);
            }
        }
    }
    
    // Create multi-series plot
    let mut plot = DateListPlot::empty();
    
    let labels = vec![
        "United States",
        "United Kingdom", 
        "Germany",
        "Brazil",
        "India"
    ];
    
    for (country, label) in countries.iter().zip(labels.iter()) {
        if let Some((dates, cases)) = data.get(country) {
            plot.add_series(
                dates.clone(),
                cases.clone(),
                Some(label.to_string())
            )?;
        }
    }
    
    let bounds = plot.bounds().unwrap().with_padding(0.1);
    let mut canvas = SkiaCanvas::new(1400, 800, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    plot.draw(&mut canvas)?;
    
    // Add axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("Date")
        .show_grid(true);
    let y_axis = Axis::new(AxisPosition::Left)
        .label("Daily New Cases")
        .show_grid(true);
    
    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;
    
    // Legend
    let mut legend = Legend::new()
        .position(LegendPosition::UpperLeft);
    for entry in plot.legend_entries() {
        legend = legend.add_entry(entry);
    }
    legend.draw(&mut canvas)?;
    
    canvas.save_png("covid_cases.png")?;
    
    Ok(())
}

fn parse_date_to_year(date_str: &str) -> Option<f64> {
    // Convert "2020-03-15" to 2020.20 (decimal year)
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() == 3 {
        let year: f64 = parts[0].parse().ok()?;
        let month: f64 = parts[1].parse().ok()?;
        let day: f64 = parts[2].parse().ok()?;
        Some(year + (month - 1.0) / 12.0 + day / 365.0)
    } else {
        None
    }
}
```

### Key Insights

- **Wave Patterns**: Multiple infection waves visible
- **Country Differences**: Varied response and timelines
- **Vaccination Impact**: Case reduction after 2021

## Example 2: Vaccination Progress (Stacked Area)

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // Vaccination data: fully vaccinated, partially vaccinated, unvaccinated
    let dates: Vec<f64> = vec![/* dates */];
    
    let fully_vaxxed = vec![/* percentages over time */];
    let partial_vaxxed = vec![/* percentages over time */];
    let unvaxxed = vec![/* percentages over time */];
    
    // Create stacked area plot
    let plot = StackedAreaPlot::new(
        dates,
        vec![fully_vaxxed, partial_vaxxed, unvaxxed],
        vec![
            "Fully Vaccinated".to_string(),
            "Partially Vaccinated".to_string(),
            "Unvaccinated".to_string()
        ]
    )?;
    
    let bounds = plot.bounds().unwrap().with_padding_top(0.1);
    let mut canvas = SkiaCanvas::new(1400, 800, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    plot.draw(&mut canvas)?;
    
    // Add legend
    let mut legend = Legend::new()
        .position(LegendPosition::UpperLeft);
    for entry in plot.legend_entries() {
        legend = legend.add_entry(entry);
    }
    legend.draw(&mut canvas)?;
    
    canvas.save_png("vaccination_progress.png")?;
    
    Ok(())
}
```

## Example 3: Mortality by Age Group (Grouped Bars)

**Source**: [CDC COVID Data Tracker](https://covid.cdc.gov/covid-data-tracker/)

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // Age groups and mortality data
    let age_groups = vec![
        "0-17", "18-29", "30-39", "40-49",
        "50-64", "65-74", "75-84", "85+"
    ];
    
    let deaths_2020 = vec![50.0, 389.0, 1402.0, 4294.0, 19304.0, 47860.0, 68447.0, 70738.0];
    let deaths_2021 = vec![94.0, 801.0, 3201.0, 8702.0, 34598.0, 56432.0, 67890.0, 84523.0];
    let deaths_2022 = vec![128.0, 923.0, 2845.0, 6321.0, 22109.0, 38764.0, 45098.0, 52341.0];
    
    // Create grouped bar plot
    let x_positions: Vec<f64> = (0..age_groups.len()).map(|i| i as f64).collect();
    
    let mut canvas = SkiaCanvas::new(1400, 800, Bounds::new(0.0, 8.0, 0.0, 90000.0))?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    let colors = vec![
        Color::from_hex("#3498db").unwrap(),
        Color::from_hex("#e74c3c").unwrap(),
        Color::from_hex("#2ecc71").unwrap(),
    ];
    
    let width = 0.25;
    let datasets = vec![deaths_2020, deaths_2021, deaths_2022];
    
    for (i, (data, color)) in datasets.iter().zip(colors.iter()).enumerate() {
        for (j, &value) in data.iter().enumerate() {
            let offset = (i as f64 - 1.0) * width;
            let bar = BarPlot::new(Series::new(vec![j as f64 + offset], vec![value])?)
                .bar_width(width)
                .color(color.clone());
            
            bar.draw(&mut canvas)?;
        }
    }
    
    // Add custom x-axis labels for age groups
    for (i, label) in age_groups.iter().enumerate() {
        canvas.draw_text(
            label,
            (i as f32 * 100.0) + 50.0,
            750.0,
            12.0,
            &Color::from_hex("#2c3e50").unwrap().to_rgba()
        )?;
    }
    
    canvas.save_png("covid_mortality_age.png")?;
    
    Ok(())
}
```

## Example 4: Disease Burden (Bubble Chart)

**Source**: [Global Burden of Disease Study](http://www.healthdata.org/gbd)

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // Disease data: prevalence (x), mortality (y), DALYs (size)
    let diseases = vec![
        ("Heart Disease", 8.9, 17.8, 200.0),
        ("Stroke", 5.5, 11.6, 150.0),
        ("Diabetes", 9.0, 1.5, 90.0),
        ("Cancer", 3.2, 9.6, 180.0),
        ("Respiratory", 6.8, 6.4, 120.0),
        ("Alzheimer's", 2.1, 3.4, 70.0),
    ];
    
    let x: Vec<f64> = diseases.iter().map(|d| d.1).collect();
    let y: Vec<f64> = diseases.iter().map(|d| d.2).collect();
    let sizes: Vec<f64> = diseases.iter().map(|d| d.3).collect();
    
    let bubble = BubbleChart::new(x, y, sizes)?
        .color(Color::from_hex("#e74c3c").unwrap())
        .opacity(0.6);
    
    let bounds = bubble.bounds().unwrap().with_padding(0.15);
    let mut canvas = SkiaCanvas::new(1200, 800, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    bubble.draw(&mut canvas)?;
    
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("Prevalence (% of population)");
    let y_axis = Axis::new(AxisPosition::Left)
        .label("Mortality (% of deaths)");
    
    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;
    
    // Add disease labels
    for (i, (name, _, _, _)) in diseases.iter().enumerate() {
        canvas.draw_text(
            name,
            x[i] as f32, y[i] as f32 + 0.3,
            10.0,
            &Color::from_hex("#2c3e50").unwrap().to_rgba()
        )?;
    }
    
    canvas.save_png("disease_burden.png")?;
    
    Ok(())
}
```

## More Health Datasets

### WHO (World Health Organization)
- [**Global Health Observatory**](https://www.who.int/data/gho) - 1000+ indicators
- [**Life Expectancy**](https://www.who.int/data/gho/data/themes/mortality-and-global-health-estimates/ghe-life-expectancy-and-healthy-life-expectancy)
- [**Vaccination Coverage**](https://www.who.int/teams/immunization-vaccines-and-biologicals/immunization-analysis-and-insights/global-monitoring/immunization-coverage)

### CDC (US)
- [**WONDER Database**](https://wonder.cdc.gov/) - Mortality, natality, cancer
- [**NHANES**](https://www.cdc.gov/nchs/nhanes/) - Nutrition and health survey
- [**BRFSS**](https://www.cdc.gov/brfss/) - Behavioral risk factors

### Our World in Data
- [**Causes of Death**](https://ourworldindata.org/causes-of-death)
- [**Cancer**](https://ourworldindata.org/cancer)
- [**Mental Health**](https://ourworldindata.org/mental-health)

## Tips for Health Data

1. **Population Normalization**: Use rates per 100k, not absolute numbers
2. **Age Standardization**: Adjust for age demographics
3. **Privacy**: Use only aggregated, de-identified public data
4. **Confidence Intervals**: Show uncertainty in estimates
5. **Ethical Considerations**: Be sensitive with mortality data

## Data Privacy

When working with health data:
- ✅ Use only publicly available datasets
- ✅ Work with aggregated data only
- ❌ Never share individual-level health data
- ✅ Follow HIPAA/GDPR guidelines if applicable

---

**Next**: [Astronomy - Exoplanets & Space Data →](./astronomy-data.md)
