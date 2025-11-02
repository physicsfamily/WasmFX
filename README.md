# WasmFX: High-Performance Computational Benchmarks

![Rust](https://img.shields.io/badge/Rust-1.91.0+-orange?logo=rust) ![WebAssembly](https://img.shields.io/badge/WebAssembly-Enabled-654FF0?logo=webassembly) ![Angular](https://img.shields.io/badge/Angular-20+-DD0031?logo=angular) ![Node](https://img.shields.io/badge/Node-24+-339933?logo=node.js) ![Material 3](https://img.shields.io/badge/Material%203-Design-1976D2?logo=material-design) ![License](https://img.shields.io/badge/License-MIT-blue)

**WasmFX** is a WebAssembly performance benchmark suite built with **Rust** and **Angular**. It provides side-by-side performance comparisons between JavaScript and WebAssembly implementations across multiple computational domains including cryptography, scientific computing, data processing, and graphics operations.

This project demonstrates practical WASM integration patterns and serves as a reference implementation for developers working with WebAssembly in production environments.

**[Live Demo](https://wasm.graviton.dev)** | **[GitHub](https://github.com/physicsfamily/WasmFX)**

---

## Features

- **Performance Benchmarks**: Side-by-side comparison of JavaScript and WebAssembly implementations
- **Multiple Domains**: Cryptography, matrix operations, Monte Carlo simulations, prime calculations, sorting, and text processing
- **Material 3 UI**: Clean, responsive interface with light theme
- **Optimized Builds**: WASM modules compiled with wasm-opt and async/await support
- **Performance Metrics**: Timing measurements and speedup ratios
- **Client-Side**: All computations run locally in the browser
- **Responsive**: Works on desktop and mobile devices

---

## 🚀 Quick Start

### Prerequisites

Ensure you have the following installed:

- **Rust** v1.91.0+ ([Install](https://www.rust-lang.org/tools/install))
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustc --version  # Verify installation
  ```

- **wasm-pack** ([Install](https://rustwasm.github.io/wasm-pack/installer/))
  ```bash
  cargo install wasm-pack
  wasm-pack --version  # Verify installation
  ```

- **Node.js** v24+ and **npm** v10+ ([Install](https://nodejs.org/))
  ```bash
  node --version   # Should be v24.x.x or higher
  npm --version    # Should be v10.x.x or higher
  ```

### Installation & Setup

```bash
# 1. Clone the repository
git clone git@github.com:physicsfamily/WasmFX.git
cd WasmFX

# 2. Build the Rust WASM library
cd wasm_lib
wasm-pack build --target bundler

# 3. Install Angular dependencies
cd ../demo_site
npm install

# 4. Start the development server
npm start
```

Open **http://localhost:4200** in your browser to see the interactive benchmark suite!

### Live Demo

Try the live demo at: **https://wasm.graviton.dev**

### Troubleshooting

**Issue**: `wasm-pack: command not found`
- **Solution**: Ensure wasm-pack is installed: `cargo install wasm-pack`

**Issue**: WebAssembly async/await warning
- **Solution**: Already fixed! The project includes custom webpack configuration for proper WASM support.

**Issue**: Port 4200 already in use
- **Solution**: Run `ng serve --port 4201` to use a different port

---

## 📂 Project Structure

```
WasmFX/
├── wasm_lib/                          # Rust WebAssembly Library
│   ├── src/
│   │   └── lib.rs                    # Computational benchmark implementations
│   ├── Cargo.toml                    # Rust dependencies & metadata
│   ├── .cargo/config.toml            # Build optimization settings
│   └── pkg/                          # Generated WASM package (after build)
│
├── demo_site/                         # Angular Demo Application
│   ├── src/
│   │   ├── app/
│   │   │   ├── app.component.ts      # Main component with WASM integration
│   │   │   ├── app.component.html    # Material 3 UI with benchmark tabs
│   │   │   └── app.component.css     # Professional styling
│   │   ├── styles.css                # Global Material 3 theme
│   │   └── index.html
│   ├── angular.json                  # Angular build configuration
│   ├── webpack.config.js             # Custom webpack for WASM support
│   ├── tsconfig.json                 # TypeScript configuration
│   └── package.json
│
└── README.md                          # This file
```

---

## Benchmark Suite

### Included Benchmarks

1. **Cryptographic Hashing** (10M iterations)
   - Bitwise operations and hash computation
   - Applications: Cryptography, blockchain validation

2. **Matrix Multiplication** (300×300 matrices)
   - Linear algebra operations
   - Applications: ML inference, graphics, physics

3. **Monte Carlo Pi Estimation** (10M samples)
   - Statistical simulation with floating-point arithmetic
   - Applications: Scientific computing, simulations

4. **Prime Number Calculation** (up to 100K)
   - Integer arithmetic and primality testing
   - Applications: Number theory, cryptography

5. **Array Sorting** (1M elements)
   - Sorting algorithm implementation
   - Applications: Data processing, analysis

6. **Text Processing** (10K iterations)
   - String manipulation and character operations
   - Applications: Text analysis, processing

7. **Fibonacci Sequence** (1000 numbers)
   - Sequential arithmetic computation
   - Applications: Algorithm benchmarking

### Performance Results

Typical performance ratios (WASM vs JavaScript):
- Cryptography: 3-5x
- Matrix Operations: 4-8x
- Sorting: 2-4x
- Scientific Computing: 5-10x

---

## Architecture

### Data Flow

```
Angular UI
    ↓
JavaScript Implementation ─┐
                           ├─→ Performance Comparison
WASM Implementation ───────┘
    ↓
Results & Metrics
```

### Technology Stack

- **Frontend**: Angular 20 (standalone components)
- **UI**: Material 3 design system
- **WASM**: Rust compiled with wasm-pack
- **Build**: Custom webpack configuration
- **Optimization**: wasm-opt level 3

---

## Development

### Adding a Benchmark

1. Implement in Rust (`wasm_lib/src/lib.rs`):
   ```rust
   #[wasm_bindgen]
   pub fn my_benchmark() -> f64 {
       // Implementation
       42.0
   }
   ```

2. Rebuild WASM:
   ```bash
   cd wasm_lib && wasm-pack build --target bundler
   ```

3. Add to Angular component and UI

### Build Configuration

- **WASM Optimization**: wasm-opt level 3
- **Webpack**: Custom configuration for async WebAssembly
- **TypeScript**: Configured with downlevelIteration support

### Running Benchmarks

1. Open http://localhost:4200
2. Select benchmark from tabs
3. Click JavaScript or WebAssembly button
4. View timing results

---

## Contributing

Contributions welcome. Areas for enhancement:

- Additional benchmark implementations
- Algorithm optimizations
- UI improvements
- Documentation updates

## License

MIT License

## References

- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Documentation](https://rustwasm.github.io/wasm-bindgen/)
- [Angular Documentation](https://angular.io/docs)
- [WebAssembly Specification](https://webassembly.org/)

## Support

For issues or questions, open an issue on GitHub.
