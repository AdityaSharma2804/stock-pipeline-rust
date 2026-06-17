# Stock Data Pipeline Benchmarks (Rust)

A high-performance financial data processing pipeline developed during an internship at the National Stock Exchange of India (NSE). The project benchmarks multiple approaches for processing 2.25 million stock market records, focusing on throughput, memory efficiency, and scalability.

## Overview

This implementation evaluates Rust for large-scale financial data processing using:

* Protocol Buffers (Protobuf)
* Welford's Online Algorithm
* FxHashMap
* Rayon Parallelism
* SIMD Processing
* Criterion Benchmarking

The pipeline processes synthetic stock market data representing a complete trading session across 100 symbols.

## Dataset

* Symbols: 100
* Records per Symbol: 22,500
* Total Records: 2,250,000
* Format: CSV and Protobuf Binary
* Trading Session: 09:15 AM – 03:30 PM

## Features

### CSV Processing

* Buffered CSV parsing
* SIMD-based aggregation
* Parallel processing using Rayon

### Binary Processing

* Length-prefixed Protocol Buffers
* Streaming aggregation
* O(1) memory per symbol
* Welford online statistics

### Statistics Computed

* Count
* Minimum Price
* Maximum Price
* Mean Price
* Standard Deviation

## Project Structure

```text
src/
├── io/
│   ├── proto_reader.rs
│   └── proto_writer.rs
├── pb/
├── stats.rs
├── lib.rs
└── main.rs

benches/
└── pipeline_bench.rs

proto/
└── stock.proto
```

## Build

```bash
cargo build --release
```

## Run CSV Pipeline

```bash
cargo run --release
```

## Run Benchmarks

```bash
cargo bench
```

## Key Optimizations

* Replaced HashMap with FxHashMap
* Removed unnecessary String cloning
* Reused decode buffers
* Increased reader buffer size to 1 MB
* Streaming protobuf decoding

## Technologies

* Rust 2021
* prost
* rayon
* criterion
* rustc-hash
* csv
* chrono

## Results

The optimized binary pipeline achieved approximately:

* Wall Time: ~142 ms
* Memory Usage: ~6.7 MB
* Dataset Size: 2.25 Million Records

## Author

Aditya Sharma
Manipal University Jaipur
NSE Internship Project
