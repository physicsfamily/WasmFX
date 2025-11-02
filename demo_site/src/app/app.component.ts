import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatTabsModule } from '@angular/material/tabs';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatIconModule } from '@angular/material/icon';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatDividerModule } from '@angular/material/divider';

// Import computational benchmark functions
import { 
  calculate_primes, matrix_multiply, fibonacci_sequence, compute_hashes, estimate_pi, sort_array, process_text
} from 'wasm_lib';

type BenchmarkType = 'primes' | 'matrix' | 'fibonacci' | 'hash' | 'pi' | 'sort' | 'text';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    CommonModule,
    MatTabsModule,
    MatButtonModule,
    MatCardModule,
    MatProgressSpinnerModule,
    MatIconModule,
    MatToolbarModule,
    MatDividerModule
  ],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css'],
})
export class AppComponent implements OnInit {
  public isLoading = false;
  
  // Tab management  
  public activeTab: number = 0;
  public tabs = [
    { id: 0, name: 'Cryptography', icon: 'security', description: 'Blockchain & Crypto Operations' },
    { id: 1, name: 'Scientific Computing', icon: 'calculate', description: 'Matrix Operations & Simulations' },
    { id: 2, name: '3D Graphics & Gaming', icon: 'videogame_asset', description: 'Real-time 3D Rendering' },
    { id: 3, name: 'Data Processing', icon: 'analytics', description: 'Sorting & Text Analysis' },
    { id: 4, name: 'All Benchmarks', icon: 'speed', description: 'Complete Performance Suite' }
  ];
  
  // Benchmark timings
  public lastBenchmarkTime: number | null = null;
  public lastJsBenchmarkTime: number | null = null;
  public lastWasmBenchmarkTime: number | null = null;
  public benchmarkResult: string = '';

  ngOnInit() {
    console.log('✅ WASM Benchmark Suite Ready');
  }

  setActiveTab(tabId: number) {
    this.activeTab = tabId;
    this.clearResults();
  }

  clearResults() {
    this.lastBenchmarkTime = null;
    this.lastJsBenchmarkTime = null;
    this.lastWasmBenchmarkTime = null;
    this.benchmarkResult = '';
  }

  // ========================================================================
  // COMPUTATIONAL BENCHMARKS - JavaScript Implementations
  // ========================================================================

  private calculatePrimesJS(limit: number): number[] {
    const primes: number[] = [];
    for (let num = 2; num <= limit; num++) {
      let isPrime = true;
      const sqrtNum = Math.sqrt(num);
      for (let i = 2; i <= sqrtNum; i++) {
        if (num % i === 0) {
          isPrime = false;
          break;
        }
      }
      if (isPrime) primes.push(num);
    }
    return primes;
  }

  private matrixMultiplyJS(size: number): Float64Array {
    const matrixA = new Float64Array(size * size);
    const matrixB = new Float64Array(size * size);
    const result = new Float64Array(size * size);

    for (let i = 0; i < size; i++) {
      for (let j = 0; j < size; j++) {
        matrixA[i * size + j] = (i + j) % 10;
        matrixB[i * size + j] = (i * j) % 10;
      }
    }

    for (let i = 0; i < size; i++) {
      for (let j = 0; j < size; j++) {
        let sum = 0;
        for (let k = 0; k < size; k++) {
          sum += matrixA[i * size + k] * matrixB[k * size + j];
        }
        result[i * size + j] = sum;
      }
    }
    return result;
  }

  private fibonacciJS(count: number): BigInt64Array {
    const sequence = new BigInt64Array(count);
    if (count >= 1) sequence[0] = 0n;
    if (count >= 2) sequence[1] = 1n;
    for (let i = 2; i < count; i++) {
      sequence[i] = sequence[i - 1] + sequence[i - 2];
    }
    return sequence;
  }

  private computeHashesJS(iterations: number): number {
    let hash = 0x12345678;
    for (let i = 0; i < iterations; i++) {
      hash = (hash * 1103515245 + 12345) >>> 0;
      hash ^= hash >>> 16;
      hash = (hash * 0x85ebca6b) >>> 0;
      hash ^= hash >>> 13;
      hash = (hash * 0xc2b2ae35) >>> 0;
      hash ^= hash >>> 16;
      hash = (hash + i) >>> 0;
    }
    return hash;
  }

  private estimatePiJS(samples: number): number {
    let insideCircle = 0;
    let seed = 123456789;
    for (let i = 0; i < samples; i++) {
      seed = (seed * 1103515245 + 12345) >>> 0;
      const x = (seed / 0xFFFFFFFF) * 2 - 1;
      seed = (seed * 1103515245 + 12345) >>> 0;
      const y = (seed / 0xFFFFFFFF) * 2 - 1;
      if (x * x + y * y <= 1.0) insideCircle++;
    }
    return (4.0 * insideCircle) / samples;
  }

  private sortArrayJS(size: number): Int32Array {
    const arr = new Int32Array(size);
    let seed = 42;
    for (let i = 0; i < size; i++) {
      seed = (seed * 1103515245 + 12345) >>> 0;
      arr[i] = (seed % 10000);
    }
    arr.sort((a, b) => a - b);
    return arr;
  }

  private processTextJS(iterations: number): string {
    const baseText = "The quick brown fox jumps over the lazy dog";
    let result = "";
    for (let iter = 0; iter < iterations; iter++) {
      for (const c of baseText) {
        if (c === c.toUpperCase() && c !== c.toLowerCase()) {
          result += c.toLowerCase();
        } else if (c === c.toLowerCase() && c !== c.toUpperCase()) {
          result += c.toUpperCase();
        } else {
          result += c;
        }
      }
    }
    return result.substring(0, 100);
  }

  // ========================================================================
  // BENCHMARK RUNNERS
  // ========================================================================

  runBenchmarkJS(type: BenchmarkType) {
    this.isLoading = true;
    this.lastJsBenchmarkTime = null;
    this.benchmarkResult = '';

    setTimeout(() => {
      const startTime = performance.now();
      let result: any;

      switch (type) {
        case 'primes':
          console.log('JavaScript: Prime calculation started...');
          result = this.calculatePrimesJS(100000);
          this.benchmarkResult = `Found ${result.length} primes`;
          break;
        case 'matrix':
          console.log('JavaScript: Matrix multiplication started...');
          result = this.matrixMultiplyJS(300);
          this.benchmarkResult = `Multiplied 300×300 matrices`;
          break;
        case 'fibonacci':
          console.log('JavaScript: Fibonacci started...');
          result = this.fibonacciJS(1000);
          this.benchmarkResult = `Calculated 1000 Fibonacci numbers`;
          break;
        case 'hash':
          console.log('JavaScript: Hash computation started...');
          result = this.computeHashesJS(10000000);
          this.benchmarkResult = `Hash: 0x${result.toString(16)}`;
          break;
        case 'pi':
          console.log('JavaScript: Pi estimation started...');
          result = this.estimatePiJS(10000000);
          this.benchmarkResult = `π ≈ ${result.toFixed(6)}`;
          break;
        case 'sort':
          console.log('JavaScript: Array sorting started...');
          result = this.sortArrayJS(1000000);
          this.benchmarkResult = `Sorted 1M elements`;
          break;
        case 'text':
          console.log('JavaScript: Text processing started...');
          result = this.processTextJS(10000);
          this.benchmarkResult = `Processed text`;
          break;
      }

      const endTime = performance.now();
      this.lastJsBenchmarkTime = parseFloat((endTime - startTime).toFixed(2));
      this.lastBenchmarkTime = this.lastJsBenchmarkTime;
      console.log(`✅ JavaScript: ${this.lastJsBenchmarkTime}ms`);
      this.isLoading = false;
    }, 10);
  }

  runBenchmarkWASM(type: BenchmarkType) {
    this.isLoading = true;
    this.lastWasmBenchmarkTime = null;
    this.benchmarkResult = '';

    setTimeout(() => {
      const startTime = performance.now();
      let result: any;

      switch (type) {
        case 'primes':
          result = calculate_primes(100000);
          this.benchmarkResult = `Found ${result.length} primes`;
          break;
        case 'matrix':
          result = matrix_multiply(300);
          this.benchmarkResult = `Multiplied 300×300 matrices`;
          break;
        case 'fibonacci':
          result = fibonacci_sequence(1000);
          this.benchmarkResult = `Calculated 1000 Fibonacci numbers`;
          break;
        case 'hash':
          result = compute_hashes(10000000);
          this.benchmarkResult = `Hash: 0x${result.toString(16)}`;
          break;
        case 'pi':
          result = estimate_pi(10000000);
          this.benchmarkResult = `π ≈ ${result.toFixed(6)}`;
          break;
        case 'sort':
          result = sort_array(1000000);
          this.benchmarkResult = `Sorted 1M elements`;
          break;
        case 'text':
          result = process_text(10000);
          this.benchmarkResult = `Processed text`;
          break;
      }

      const endTime = performance.now();
      this.lastWasmBenchmarkTime = parseFloat((endTime - startTime).toFixed(2));
      this.lastBenchmarkTime = this.lastWasmBenchmarkTime;
      console.log(`⚡ WASM: ${this.lastWasmBenchmarkTime}ms`);
      this.isLoading = false;
    }, 10);
  }

  getBenchmarkSpeedup(): string | null {
    if (this.lastJsBenchmarkTime && this.lastWasmBenchmarkTime) {
      const speedup = (this.lastJsBenchmarkTime / this.lastWasmBenchmarkTime).toFixed(2);
      return speedup;
    }
    return null;
  }

  getBenchmarkTitle(bench: string): string {
    const titles: Record<string, string> = {
      'hash': '🔐 Crypto Hash',
      'matrix': '🔲 Matrix Multiply',
      'pi': 'π Monte Carlo',
      'primes': '🔢 Prime Numbers',
      'sort': '📊 Array Sort',
      'text': '📝 Text Process',
      'fibonacci': '🌀 Fibonacci'
    };
    return titles[bench] || bench;
  }

  getBenchmarkDesc(bench: string): string {
    const descs: Record<string, string> = {
      'hash': '10M hash iterations',
      'matrix': '300×300 matrix multiplication',
      'pi': 'Estimate π with 10M samples',
      'primes': 'Find primes up to 100K',
      'sort': 'Sort 1M integers',
      'text': 'Process 10K iterations',
      'fibonacci': 'Calculate 1000 Fibonacci numbers'
    };
    return descs[bench] || '';
  }

  getBenchmarks() {
    return [
      { type: 'hash' as BenchmarkType, title: '🔐 Crypto Hash', desc: '10M hash iterations' },
      { type: 'matrix' as BenchmarkType, title: '🔲 Matrix Multiply', desc: '300×300 matrix multiplication' },
      { type: 'pi' as BenchmarkType, title: 'π Monte Carlo', desc: 'Estimate π with 10M samples' },
      { type: 'primes' as BenchmarkType, title: '🔢 Prime Numbers', desc: 'Find primes up to 100K' },
      { type: 'sort' as BenchmarkType, title: '📊 Array Sort', desc: 'Sort 1M integers' },
      { type: 'text' as BenchmarkType, title: '📝 Text Process', desc: 'Process 10K iterations' },
      { type: 'fibonacci' as BenchmarkType, title: '🌀 Fibonacci', desc: 'Calculate 1000 Fibonacci numbers' }
    ];
  }
}
