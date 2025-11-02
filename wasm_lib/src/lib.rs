use wasm_bindgen::prelude::*;

// --- Utility: A function to log messages to the browser console ---
// We expose a custom `log` function to JS, but also use it internally.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// A simple macro to make logging easier
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// --- Entry Point: Run when the WASM module is first loaded ---
#[wasm_bindgen(start)]
pub fn run_on_load() {
    console_log!("Rust (WASM) module loaded successfully.");
}

// --- Demo 1: Grayscale Filter ---
// This function is exported to JavaScript.
// It takes a `Vec<u8>` which is the raw RGBA pixel data from a canvas.
// It returns a new `Vec<u8>` with the filter applied.
#[wasm_bindgen]
pub fn apply_grayscale(mut image_data: Vec<u8>) -> Vec<u8> {
    console_log!("Rust: Grayscale filter started...");
    
    // Iterate over the pixel data in chunks of 4 bytes (R, G, B, A)
    // `chunks_exact_mut` gives us mutable slices
    for pixel in image_data.chunks_exact_mut(4) {
        // Apply the luminance formula (a common way to calculate grayscale)
        // (R * 0.299 + G * 0.587 + B * 0.114)
        // We use integer math for speed.
        let gray = ((pixel[0] as u32 * 299 + pixel[1] as u32 * 587 + pixel[2] as u32 * 114) / 1000) as u8;

        // Set R, G, and B values to the new 'gray' value
        pixel[0] = gray; // Red
        pixel[1] = gray; // Green
        pixel[2] = gray; // Blue
        // pixel[3] (Alpha) remains unchanged
    }

    console_log!("Rust: Grayscale filter finished.");
    image_data // Return the modified vector
}

// --- Demo 2: Invert Filter (for another example) ---
#[wasm_bindgen]
pub fn apply_invert(mut image_data: Vec<u8>) -> Vec<u8> {
    console_log!("Rust: Invert filter started...");

    for pixel in image_data.chunks_exact_mut(4) {
        pixel[0] = 255 - pixel[0]; // Invert Red
        pixel[1] = 255 - pixel[1]; // Invert Green
        pixel[2] = 255 - pixel[2]; // Invert Blue
        // pixel[3] (Alpha) remains unchanged
    }

    console_log!("Rust: Invert filter finished.");
    image_data
}

// --- Demo 3: Gaussian Blur (Computationally Intensive) ---
// This filter is MUCH more complex than grayscale/invert
// It performs many floating-point operations per pixel
#[wasm_bindgen]
pub fn apply_blur(mut image_data: Vec<u8>, width: u32, height: u32, radius: u32) -> Vec<u8> {
    console_log!("Rust (WASM): Gaussian blur started...");
    
    let width = width as usize;
    let height = height as usize;
    let radius = radius as i32;
    
    // Create a temporary buffer
    let mut temp = image_data.clone();
    
    // Gaussian blur kernel weights (approximation)
    let sigma = radius as f32 / 3.0;
    let two_sigma_sq = 2.0 * sigma * sigma;
    
    // Horizontal pass - thousands of operations per pixel
    for y in 0..height {
        for x in 0..width {
            let mut r_sum = 0.0;
            let mut g_sum = 0.0;
            let mut b_sum = 0.0;
            let mut weight_sum = 0.0;
            
            for dx in -radius..=radius {
                let nx = (x as i32 + dx).max(0).min(width as i32 - 1) as usize;
                let idx = (y * width + nx) * 4;
                
                // Gaussian weight calculation (expensive!)
                let distance_sq = (dx * dx) as f32;
                let weight = (-distance_sq / two_sigma_sq).exp();
                
                r_sum += image_data[idx] as f32 * weight;
                g_sum += image_data[idx + 1] as f32 * weight;
                b_sum += image_data[idx + 2] as f32 * weight;
                weight_sum += weight;
            }
            
            let idx = (y * width + x) * 4;
            temp[idx] = (r_sum / weight_sum) as u8;
            temp[idx + 1] = (g_sum / weight_sum) as u8;
            temp[idx + 2] = (b_sum / weight_sum) as u8;
        }
    }
    
    // Vertical pass - more thousands of operations
    for y in 0..height {
        for x in 0..width {
            let mut r_sum = 0.0;
            let mut g_sum = 0.0;
            let mut b_sum = 0.0;
            let mut weight_sum = 0.0;
            
            for dy in -radius..=radius {
                let ny = (y as i32 + dy).max(0).min(height as i32 - 1) as usize;
                let idx = (ny * width + x) * 4;
                
                let distance_sq = (dy * dy) as f32;
                let weight = (-distance_sq / two_sigma_sq).exp();
                
                r_sum += temp[idx] as f32 * weight;
                g_sum += temp[idx + 1] as f32 * weight;
                b_sum += temp[idx + 2] as f32 * weight;
                weight_sum += weight;
            }
            
            let idx = (y * width + x) * 4;
            image_data[idx] = (r_sum / weight_sum) as u8;
            image_data[idx + 1] = (g_sum / weight_sum) as u8;
            image_data[idx + 2] = (b_sum / weight_sum) as u8;
        }
    }
    
    console_log!("Rust (WASM): Gaussian blur finished.");
    image_data
}

// --- Demo 4: Sobel Edge Detection (Complex Math) ---
#[wasm_bindgen]
pub fn apply_edge_detection(image_data: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    console_log!("Rust (WASM): Edge detection started...");
    
    let width = width as usize;
    let height = height as usize;
    let mut result = vec![0u8; image_data.len()];
    
    // Sobel operators for edge detection
    let sobel_x = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    let sobel_y = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];
    
    // Process each pixel (except borders)
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut gx = 0.0;
            let mut gy = 0.0;
            
            // Apply 3x3 Sobel kernel - 9 operations per pixel
            for ky in 0..3 {
                for kx in 0..3 {
                    let ny = y + ky - 1;
                    let nx = x + kx - 1;
                    let idx = (ny * width + nx) * 4;
                    
                    // Convert to grayscale first
                    let gray = (image_data[idx] as f32 * 0.299
                        + image_data[idx + 1] as f32 * 0.587
                        + image_data[idx + 2] as f32 * 0.114);
                    
                    gx += gray * sobel_x[ky][kx] as f32;
                    gy += gray * sobel_y[ky][kx] as f32;
                }
            }
            
            // Calculate gradient magnitude (expensive sqrt!)
            let magnitude = (gx * gx + gy * gy).sqrt().min(255.0) as u8;
            
            let idx = (y * width + x) * 4;
            result[idx] = magnitude;
            result[idx + 1] = magnitude;
            result[idx + 2] = magnitude;
            result[idx + 3] = image_data[idx + 3];
        }
    }
    
    console_log!("Rust (WASM): Edge detection finished.");
    result
}

// --- Demo 5: Mandelbrot Set (PURE COMPUTATION - WASM DOMINATES!) ---
// This generates a fractal image from scratch using pure math
// No input image needed - we're generating pixel values computationally
#[wasm_bindgen]
pub fn generate_mandelbrot(width: u32, height: u32, max_iterations: u32) -> Vec<u8> {
    console_log!("Rust (WASM): Mandelbrot generation started...");
    
    let width = width as usize;
    let height = height as usize;
    let max_iter = max_iterations as usize;
    
    let mut result = vec![0u8; width * height * 4];
    
    // Mandelbrot set parameters
    let x_min = -2.5;
    let x_max = 1.0;
    let y_min = -1.0;
    let y_max = 1.0;
    
    let x_scale = (x_max - x_min) / width as f64;
    let y_scale = (y_max - y_min) / height as f64;
    
    for py in 0..height {
        for px in 0..width {
            // Map pixel to complex plane
            let x0 = x_min + px as f64 * x_scale;
            let y0 = y_min + py as f64 * y_scale;
            
            let mut x = 0.0;
            let mut y = 0.0;
            let mut iteration = 0;
            
            // Mandelbrot iteration: z = z² + c
            // This is PURE COMPUTATION - hundreds of operations per pixel!
            while x * x + y * y <= 4.0 && iteration < max_iter {
                let xtemp = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                iteration += 1;
            }
            
            // Color based on iteration count
            let idx = (py * width + px) * 4;
            if iteration == max_iter {
                // Point is in the set - black
                result[idx] = 0;
                result[idx + 1] = 0;
                result[idx + 2] = 0;
            } else {
                // Color based on escape time
                let ratio = iteration as f32 / max_iter as f32;
                result[idx] = (255.0 * (1.0 - ratio)) as u8;      // Red
                result[idx + 1] = (255.0 * ratio.sqrt()) as u8;   // Green  
                result[idx + 2] = (255.0 * ratio) as u8;          // Blue
            }
            result[idx + 3] = 255; // Alpha
        }
    }
    
    console_log!("Rust (WASM): Mandelbrot generation finished.");
    result
}

// --- Demo 6: Matrix Multiplication (INTEGER HEAVY) ---
// Apply a complex convolution kernel - lots of integer math
#[wasm_bindgen]
pub fn apply_sharpen(image_data: Vec<u8>, width: u32, height: u32, strength: u32) -> Vec<u8> {
    console_log!("Rust (WASM): Sharpen filter started...");
    
    let width = width as usize;
    let height = height as usize;
    let strength = strength as i32;
    
    let mut result = vec![0u8; image_data.len()];
    
    // Unsharp mask kernel (5x5) - more complex than Sobel
    let kernel: [[i32; 5]; 5] = [
        [-1, -1, -1, -1, -1],
        [-1,  2,  2,  2, -1],
        [-1,  2,  8,  2, -1],
        [-1,  2,  2,  2, -1],
        [-1, -1, -1, -1, -1],
    ];
    
    let kernel_sum: i32 = 8;
    
    for y in 2..height - 2 {
        for x in 2..width - 2 {
            let mut r_sum: i32 = 0;
            let mut g_sum: i32 = 0;
            let mut b_sum: i32 = 0;
            
            // Apply 5x5 kernel - 25 operations per pixel!
            for ky in 0..5 {
                for kx in 0..5 {
                    let ny = y + ky - 2;
                    let nx = x + kx - 2;
                    let idx = (ny * width + nx) * 4;
                    
                    let k_val = kernel[ky][kx];
                    r_sum += image_data[idx] as i32 * k_val;
                    g_sum += image_data[idx + 1] as i32 * k_val;
                    b_sum += image_data[idx + 2] as i32 * k_val;
                }
            }
            
            // Apply strength and clamp
            let orig_idx = (y * width + x) * 4;
            let orig_r = image_data[orig_idx] as i32;
            let orig_g = image_data[orig_idx + 1] as i32;
            let orig_b = image_data[orig_idx + 2] as i32;
            
            result[orig_idx] = (orig_r + (r_sum * strength) / (kernel_sum * 100)).clamp(0, 255) as u8;
            result[orig_idx + 1] = (orig_g + (g_sum * strength) / (kernel_sum * 100)).clamp(0, 255) as u8;
            result[orig_idx + 2] = (orig_b + (b_sum * strength) / (kernel_sum * 100)).clamp(0, 255) as u8;
            result[orig_idx + 3] = image_data[orig_idx + 3];
        }
    }
    
    console_log!("Rust (WASM): Sharpen filter finished.");
    result
}

// ========================================================================
// PURE COMPUTATIONAL BENCHMARKS (No Image Processing!)
// These demonstrate WASM's raw computational power
// ========================================================================

// --- Benchmark 1: Prime Number Generation (CPU Intensive) ---
#[wasm_bindgen]
pub fn calculate_primes(limit: u32) -> Vec<u32> {
    console_log!("Rust (WASM): Prime calculation started...");
    
    let mut primes = Vec::new();
    
    for num in 2..=limit {
        let mut is_prime = true;
        let sqrt_num = (num as f64).sqrt() as u32;
        
        for i in 2..=sqrt_num {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }
        
        if is_prime {
            primes.push(num);
        }
    }
    
    console_log!("Rust (WASM): Prime calculation finished.");
    primes
}

// --- Benchmark 2: Matrix Multiplication (Linear Algebra) ---
#[wasm_bindgen]
pub fn matrix_multiply(size: u32) -> Vec<f64> {
    console_log!("Rust (WASM): Matrix multiplication started...");
    
    let size = size as usize;
    
    // Create two matrices with random-ish values
    let mut matrix_a = vec![0.0; size * size];
    let mut matrix_b = vec![0.0; size * size];
    let mut result = vec![0.0; size * size];
    
    // Initialize matrices
    for i in 0..size {
        for j in 0..size {
            matrix_a[i * size + j] = ((i + j) % 10) as f64;
            matrix_b[i * size + j] = ((i * j) % 10) as f64;
        }
    }
    
    // Matrix multiplication: C = A × B
    // This is O(n³) - very computationally intensive!
    for i in 0..size {
        for j in 0..size {
            let mut sum = 0.0;
            for k in 0..size {
                sum += matrix_a[i * size + k] * matrix_b[k * size + j];
            }
            result[i * size + j] = sum;
        }
    }
    
    console_log!("Rust (WASM): Matrix multiplication finished.");
    result
}

// --- Benchmark 3: Fibonacci (Recursive/Memoization) ---
#[wasm_bindgen]
pub fn fibonacci_sequence(count: u32) -> Vec<u64> {
    console_log!("Rust (WASM): Fibonacci calculation started...");
    
    let mut sequence = Vec::with_capacity(count as usize);
    
    if count >= 1 {
        sequence.push(0);
    }
    if count >= 2 {
        sequence.push(1);
    }
    
    for i in 2..count as usize {
        let next = sequence[i - 1] + sequence[i - 2];
        sequence.push(next);
    }
    
    console_log!("Rust (WASM): Fibonacci calculation finished.");
    sequence
}

// --- Benchmark 4: SHA-256 Hash-like Computation (Bitwise Operations) ---
#[wasm_bindgen]
pub fn compute_hashes(iterations: u32) -> u32 {
    console_log!("Rust (WASM): Hash computation started...");
    
    let mut hash: u32 = 0x12345678;
    
    for i in 0..iterations {
        // Simulate complex hash operations with bitwise math
        hash = hash.wrapping_mul(1103515245).wrapping_add(12345);
        hash ^= hash >> 16;
        hash = hash.wrapping_mul(0x85ebca6b);
        hash ^= hash >> 13;
        hash = hash.wrapping_mul(0xc2b2ae35);
        hash ^= hash >> 16;
        hash = hash.wrapping_add(i);
    }
    
    console_log!("Rust (WASM): Hash computation finished.");
    hash
}

// --- Benchmark 5: Monte Carlo Pi Estimation (Random + Math) ---
#[wasm_bindgen]
pub fn estimate_pi(samples: u32) -> f64 {
    console_log!("Rust (WASM): Pi estimation started...");
    
    let mut inside_circle = 0u32;
    let mut seed = 123456789u32;
    
    for _ in 0..samples {
        // Simple LCG random number generator
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let x = (seed as f64 / u32::MAX as f64) * 2.0 - 1.0;
        
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let y = (seed as f64 / u32::MAX as f64) * 2.0 - 1.0;
        
        if x * x + y * y <= 1.0 {
            inside_circle += 1;
        }
    }
    
    let pi_estimate = 4.0 * inside_circle as f64 / samples as f64;
    console_log!("Rust (WASM): Pi estimation finished.");
    pi_estimate
}

// --- Benchmark 6: QuickSort (Algorithm Performance) ---
#[wasm_bindgen]
pub fn sort_array(size: u32) -> Vec<i32> {
    console_log!("Rust (WASM): Array sorting started...");
    
    let mut arr = Vec::with_capacity(size as usize);
    let mut seed = 42u32;
    
    // Generate pseudo-random array
    for _ in 0..size {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        arr.push((seed % 10000) as i32);
    }
    
    // Use Rust's optimized sort (Timsort)
    arr.sort_unstable();
    
    console_log!("Rust (WASM): Array sorting finished.");
    arr
}

// --- Benchmark 7: String Processing (Real-world data manipulation) ---
#[wasm_bindgen]
pub fn process_text(iterations: u32) -> String {
    console_log!("Rust (WASM): Text processing started...");
    
    let base_text = "The quick brown fox jumps over the lazy dog";
    let mut result = String::new();
    let mut word_count = 0;
    let mut char_count = 0;
    
    for _ in 0..iterations {
        // Simulate text processing
        for c in base_text.chars() {
            char_count += 1;
            if c.is_whitespace() {
                word_count += 1;
            }
            // Reverse case
            if c.is_uppercase() {
                result.push(c.to_lowercase().next().unwrap());
            } else if c.is_lowercase() {
                result.push(c.to_uppercase().next().unwrap());
            } else {
                result.push(c);
            }
        }
    }
    
    console_log!("Rust (WASM): Text processing finished.");
    result.chars().take(100).collect()
}
