# WebAssembly Integration Guide

This document outlines how to add interactive Rust playground to the Docusaurus documentation.

## Approach 1: Monaco Editor + Rust Playground Backend

### Setup

1. **Install dependencies**:
```bash
cd docs
npm install @monaco-editor/react monaco-editor
```

2. **Create Playground Component**:

```tsx title="docs/src/components/RustPlayground/index.tsx"
import React, { useState } from 'react';
import Editor from '@monaco-editor/react';

export default function RustPlayground({ initialCode, showOutput = true }) {
  const [code, setCode] = useState(initialCode);
  const [output, setOutput] = useState('');
  const [loading, setLoading] = useState(false);

  const runCode = async () => {
    setLoading(true);
    try {
      // Use Rust Playground API
      const response = await fetch('https://play.rust-lang.org/execute', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          channel: 'stable',
          mode: 'debug',
          edition: '2021',
          crateType: 'bin',
          tests: false,
          code: code,
        }),
      });
      
      const result = await response.json();
      setOutput(result.stdout + result.stderr);
    } catch (error) {
      setOutput(`Error: ${error.message}`);
    }
    setLoading(false);
  };

  return (
    <div className="rust-playground">
      <Editor
        height="300px"
        defaultLanguage="rust"
        value={code}
        onChange={(value) => setCode(value || '')}
        theme="vs-dark"
        options={{
          minimap: { enabled: false },
          fontSize: 14,
        }}
      />
      <button onClick={runCode} disabled={loading}>
        {loading ? 'Running...' : '▶ Run'}
      </button>
      {showOutput && (
        <pre className="output">
          {output || 'Click Run to see output'}
        </pre>
      )}
    </div>
  );
}
```

3. **Use in MDX**:

```mdx
import RustPlayground from '@site/src/components/RustPlayground';

<RustPlayground 
  initialCode={`fn main() {
    println!("Hello from Velociplot!");
}`}
/>
```

**Limitations**:
- Cannot use external crates (Velociplot won't work)
- Only demonstrates Rust syntax
- Requires internet connection

---

## Approach 2: WebAssembly with wasm-bindgen

### Setup

1. **Create WASM package**:

```bash
cargo new --lib velociplot-wasm
cd velociplot-wasm
```

2. **Configure Cargo.toml**:

```toml
[package]
name = "velociplot-wasm"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
velociplot = { path = ".." }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["CanvasRenderingContext2d", "HtmlCanvasElement"] }
serde-wasm-bindgen = "0.6"
```

3. **Implement WASM bindings**:

```rust title="velociplot-wasm/src/lib.rs"
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub struct WasmPlot {
    canvas: HtmlCanvasElement,
}

#[wasm_bindgen]
impl WasmPlot {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<WasmPlot, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()?;
        
        Ok(WasmPlot { canvas })
    }

    pub fn plot_sine(&self) -> Result<(), JsValue> {
        // Create sine wave data
        let x: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
        let y: Vec<f64> = x.iter().map(|&x| x.sin()).collect();
        
        // Render to canvas
        let ctx = self.canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        
        ctx.begin_path();
        for (i, (&xi, &yi)) in x.iter().zip(y.iter()).enumerate() {
            let px = (xi / 10.0) * self.canvas.width() as f64;
            let py = self.canvas.height() as f64 / 2.0 - yi * 50.0;
            
            if i == 0 {
                ctx.move_to(px, py);
            } else {
                ctx.line_to(px, py);
            }
        }
        ctx.stroke();
        
        Ok(())
    }
}
```

4. **Build WASM**:

```bash
wasm-pack build --target web
```

5. **Create React Component**:

```tsx title="docs/src/components/WasmPlot/index.tsx"
import React, { useEffect, useRef } from 'react';

export default function WasmPlot({ plotType = 'sine' }) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    async function initWasm() {
      // Dynamic import of WASM module
      const wasm = await import('../../../velociplot-wasm/pkg');
      
      if (canvasRef.current) {
        const plot = new wasm.WasmPlot(canvasRef.current.id);
        
        switch (plotType) {
          case 'sine':
            plot.plot_sine();
            break;
          // Add more plot types
        }
      }
    }

    initWasm();
  }, [plotType]);

  return (
    <canvas
      ref={canvasRef}
      id="wasm-canvas"
      width={800}
      height={600}
      style={{ border: '1px solid #ccc' }}
    />
  );
}
```

6. **Use in MDX**:

```mdx
import WasmPlot from '@site/src/components/WasmPlot';

<WasmPlot plotType="sine" />
```

---

## Approach 3: Embedded iframe with Rust Playground

Simplest approach - no setup needed:

```mdx
<iframe
  src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main()%20%7B%0A%20%20%20%20println!(%22Hello%2C%20world!%22)%3B%0A%7D"
  width="100%"
  height="500px"
  style={{ border: 'none' }}
/>
```

---

## Recommended Approach

For **Velociplot documentation**, we recommend:

### Phase 1: Static Examples with Links (Current)
- ✅ Show code snippets
- ✅ Display pre-rendered images
- ✅ Link to runnable examples in repo
- ✅ Provide Gitpod/Codespaces buttons

### Phase 2: WebAssembly Integration
- 🔄 Build velociplot-wasm package
- 🔄 Create interactive canvas component
- 🔄 Add preset plot types (sine, scatter, bar)
- 🔄 Allow parameter adjustment via UI

### Phase 3: Full Playground
- 🔄 Monaco editor integration
- 🔄 Compile user code to WASM on-the-fly
- 🔄 Live preview of plots
- 🔄 Export generated images

## Implementation Checklist

- [ ] Create `velociplot-wasm` crate
- [ ] Implement WebCanvas backend for browser
- [ ] Add wasm-bindgen bindings
- [ ] Build and test WASM package
- [ ] Create React/TypeScript components
- [ ] Integrate with Docusaurus
- [ ] Add to documentation build process
- [ ] Document usage for contributors

## Resources

- [wasm-pack Book](https://rustwasm.github.io/wasm-pack/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Rust and WebAssembly](https://rustwasm.github.io/book/)
- [Monaco Editor](https://microsoft.github.io/monaco-editor/)

---

**Note**: WebAssembly support is a great addition but not critical for 1.0 release. Can be added in version 1.1 or 1.2.
