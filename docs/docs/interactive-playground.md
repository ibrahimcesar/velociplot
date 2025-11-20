---
sidebar_position: 5
---

# Interactive Playground

:::info Coming Soon
Interactive Rust playground with WebAssembly integration is planned for a future release.
:::

## Rust Playground Options

While we work on integrated WebAssembly support, you can try Velociplot code in these environments:

### 1. Rust Playground (Official)

Use the official [Rust Playground](https://play.rust-lang.org/) for quick experiments:

**Limitations**:
- Cannot import external crates like Velociplot directly
- Good for learning Rust syntax and algorithms
- Cannot render actual plots

### 2. Local Development (Recommended)

The best way to experiment with Velociplot:

```bash
# Clone a template project
git clone https://github.com/ibrahimcesar/velociplot
cd velociplot
cargo run --example basic_line

# Or create your own
cargo new my_experiment
cd my_experiment
# Add velociplot to Cargo.toml
cargo run
```

### 3. GitHub Codespaces

Open the repository in GitHub Codespaces for a full development environment in your browser:

1. Go to [github.com/ibrahimcesar/velociplot](https://github.com/ibrahimcesar/velociplot)
2. Click **Code** → **Codespaces** → **Create codespace**
3. Wait for environment to load
4. Run: `cargo run --example basic_line`

### 4. Gitpod

Similar to Codespaces:

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/ibrahimcesar/velociplot)

## Future: WebAssembly Integration

We're planning to add WebAssembly support for interactive plotting in the browser!

### Planned Features

```rust
// Future WebAssembly API
use velociplot_wasm::prelude::*;

#[wasm_bindgen]
pub fn create_plot(canvas_id: &str) {
    let plot = LinePlot::new(data)?;
    plot.render_to_canvas(canvas_id)?;
}
```

This would enable:
- ✨ Live code editing in the docs
- 📊 Instant plot preview
- 🎨 Interactive customization
- 📱 Mobile-friendly experimentation

### Technical Approach

1. **Compile to WebAssembly**:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

2. **Use wasm-pack**:
   ```bash
   wasm-pack build --target web
   ```

3. **Integrate with Docusaurus**:
   ```jsx
   import init, { create_plot } from './velociplot_wasm';
   
   function PlotDemo() {
     useEffect(() => {
       init().then(() => {
         create_plot('canvas-id');
       });
     }, []);
     
     return <canvas id="canvas-id" />;
   }
   ```

## Current Workaround: Static Examples

For now, all examples are pre-rendered and shown as static images with the corresponding code:

````mdx
### Sine Wave Example

```rust
use velociplot::prelude::*;

let x: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
let y: Vec<f64> = x.iter().map(|&x| x.sin()).collect();

let plot = LinePlot::new(Series::new(x, y)?)?;
// ... render
```

![Sine Wave](../static/img/examples/sine_wave.png)
````

## Try It Yourself

The fastest way to experiment:

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Clone repository
git clone https://github.com/ibrahimcesar/velociplot
cd velociplot

# 3. Run any example
cargo run --example basic_line
cargo run --example scatter
cargo run --example climate_nasa

# 4. Modify and experiment!
```

## Contributing

Want to help add WebAssembly support? We'd love your contribution!

Check out:
- [Contributing Guide](https://github.com/ibrahimcesar/velociplot/blob/main/CONTRIBUTING.md)
- [Open Issues](https://github.com/ibrahimcesar/velociplot/issues)
- [Discussions](https://github.com/ibrahimcesar/velociplot/discussions)

---

**Stay tuned!** Follow the project for updates on interactive playground support. 🦖
