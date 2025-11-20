# Velociplot CLI Guide

The `vplot` command-line tool enables quick plotting from CSV and JSON files without writing Rust code.

## Installation

Build with CLI support:

```bash
cargo build --release --features cli
# Binary will be at: target/release/vplot
```

Or install globally:

```bash
cargo install --path . --features cli
```

## Quick Start

### Plot from CSV

```bash
# Simple line plot
vplot plot data.csv -y temperature --output plot.png

# With grid and labels
vplot plot data.csv -y temperature --grid --xlabel "Time" --ylabel "°C"

# Multiple series with legend
vplot plot data.csv -y temp1 -y temp2 -y temp3 --legend --grid

# Scatter plot
vplot plot data.csv --plot-type scatter -x time -y value

# Bar chart
vplot plot data.csv --plot-type bar -y sales

# Histogram
vplot plot data.csv --plot-type histogram -y measurements
```

### Generate Code Templates

```bash
# Get template code for any plot type
vplot template line
vplot template scatter
vplot template bar
vplot template histogram

# Save to file
vplot template line --output my_plot.rs
```

## Command Reference

### `vplot plot`

Plot data from CSV or JSON files.

**Options:**
- `INPUT` - Input data file (CSV or JSON) [required]
- `-p, --plot-type <TYPE>` - Plot type: line, scatter, bar, histogram [default: line]
- `-x, --x-column <COL>` - X-axis column name or index (0-based)
- `-y, --y-column <COL>` - Y-axis column(s) (can specify multiple times)
- `-o, --output <FILE>` - Output PNG file [default: plot.png]
- `--xlabel <LABEL>` - X-axis label
- `--ylabel <LABEL>` - Y-axis label
- `--width <PIXELS>` - Width in pixels [default: 800]
- `--height <PIXELS>` - Height in pixels [default: 600]
- `--legend` - Show legend (for multiple series)
- `--grid` - Show grid lines

### `vplot template`

Generate Rust code template for a plot type.

**Arguments:**
- `PLOT_TYPE` - Type of plot: line, scatter, bar, histogram [required]

**Options:**
- `-o, --output <FILE>` - Save template to file (default: stdout)

### `vplot version`

Show version information.

## Data Formats

### CSV Format

```csv
x,temperature,humidity
0,20.5,45.2
1,21.3,46.8
2,22.1,48.3
3,21.8,47.5
```

**Column Selection:**
- By name: `-y temperature`
- By index: `-y 1` (0-based)
- Multiple columns: `-y temperature -y humidity`
- Auto-detect: If no `-y` specified, uses all columns except x

### JSON Format

```json
[
  {"time": 0, "value": 1.5},
  {"time": 1, "value": 2.3},
  {"time": 2, "value": 1.8}
]
```

Must be an array of objects with consistent keys.

## Examples

### Example 1: Basic Line Plot

**Data (data.csv):**
```csv
x,y
0,0.0
1,1.0
2,4.0
3,9.0
4,16.0
```

**Command:**
```bash
vplot plot data.csv -y y --xlabel "X" --ylabel "Y = X²" --grid
```

### Example 2: Multiple Series

**Data (temps.csv):**
```csv
time,sensor1,sensor2,sensor3
0,20.1,20.3,20.0
1,21.2,20.8,21.5
2,22.3,21.9,22.1
3,23.1,22.5,22.8
```

**Command:**
```bash
vplot plot temps.csv \\
  -y sensor1 -y sensor2 -y sensor3 \\
  --legend \\
  --grid \\
  --xlabel "Time (hours)" \\
  --ylabel "Temperature (°C)" \\
  --output temperatures.png
```

### Example 3: Scatter Plot with Custom Size

**Command:**
```bash
vplot plot data.csv \\
  --plot-type scatter \\
  -x distance -y speed \\
  --width 1200 \\
  --height 800 \\
  --output scatter.png
```

### Example 4: Histogram

**Data (measurements.csv):**
```csv
value
23.1
24.5
22.8
25.3
23.7
24.1
...
```

**Command:**
```bash
vplot plot measurements.csv \\
  --plot-type histogram \\
  -y value \\
  --xlabel "Measurement" \\
  --ylabel "Frequency" \\
  --output histogram.png
```

### Example 5: Bar Chart

**Data (sales.csv):**
```csv
quarter,revenue
1,125000
2,145000
3,132000
4,167000
```

**Command:**
```bash
vplot plot sales.csv \\
  --plot-type bar \\
  -x quarter -y revenue \\
  --xlabel "Quarter" \\
  --ylabel "Revenue ($)" \\
  --output sales.png
```

## Code Generation

Use `vplot template` to get starter code for your Rust projects:

```bash
# View template
vplot template line

# Save and customize
vplot template scatter --output my_plot.rs
```

Then modify the generated code with your data and requirements.

## Tips

1. **Column Names with Spaces:** Use quotes
   ```bash
   vplot plot data.csv -y "Temperature (°C)"
   ```

2. **Multiple Y Columns:** Specify `-y` multiple times
   ```bash
   vplot plot data.csv -y col1 -y col2 -y col3 --legend
   ```

3. **Auto-detect Columns:** Omit `-y` to plot all non-x columns
   ```bash
   vplot plot data.csv --legend  # Plots all columns except first
   ```

4. **Column Indices:** Use 0-based indices instead of names
   ```bash
   vplot plot data.csv -x 0 -y 1 -y 2
   ```

5. **Custom Colors:** Not yet supported in CLI, use library API for full customization

## Limitations

Current CLI supports:
- ✅ CSV and JSON input
- ✅ PNG output
- ✅ 4 plot types (line, scatter, bar, histogram)
- ✅ Basic styling (grid, labels)

Not yet supported (use library API):
- SVG/PDF output
- Custom colors
- Advanced plot types (heatmap, box plots, etc.)
- Multiple subplots
- Custom fonts
- Fine-grained styling

For advanced features, use the Rust library directly or generate a template with `vplot template`.

## Troubleshooting

**"Column not found"**
- Check column name spelling (case-sensitive)
- Try using column index instead (0-based)
- Verify CSV has headers

**"Invalid number in column"**
- Ensure all data in numeric columns is parseable as f64
- Check for empty cells or non-numeric values

**"JSON must be an array of objects"**
- JSON format must be: `[{}, {}, ...]`
- Each object must have the same keys
- Values must be numbers

**CLI not available**
- Rebuild with: `cargo build --features cli`
- Ensure you're running the `vplot` binary, not main library

## See Also

- [Main Documentation](README.md)
- [Library API Reference](https://docs.rs/velociplot)
- [Examples](examples/)
- [Contributing](CONTRIBUTING.md)

---

For bugs or feature requests, please [open an issue](https://github.com/ibrahimcesar/velociplot/issues).
