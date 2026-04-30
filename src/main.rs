//! Binary for aggregating stock protobuf data.

// use std::time::Instant;
// use std::error::Error;

// use rayon::prelude::*;
// use rustc_hash::FxHashMap;
// use wide::f64x4;
// use csv::ReaderBuilder;

// // ---------------- SIMD FUNCTION ----------------
// fn compute_stats_simd(prices: &[f64]) -> (usize, f64, f64, f64, f64) {
//     let n = prices.len();
//     let nf = n as f64;

//     let mut sum = f64x4::splat(0.0);
//     let mut min = f64x4::splat(f64::INFINITY);
//     let mut max = f64x4::splat(f64::NEG_INFINITY);

//     let chunks = n / 4;

//     for i in 0..chunks {
//         let base = i * 4;
//         let v = f64x4::new([
//             prices[base],
//             prices[base + 1],
//             prices[base + 2],
//             prices[base + 3],
//         ]);

//         sum += v;
//         min = min.min(v);
//         max = max.max(v);
//     }

//     let mut total_sum: f64 = sum.to_array().iter().sum();
//     let mut total_min = min.to_array().iter().fold(f64::INFINITY, |a, &b| a.min(b));
//     let mut total_max = max.to_array().iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

//     for i in (chunks * 4)..n {
//         let v = prices[i];
//         total_sum += v;
//         total_min = total_min.min(v);
//         total_max = total_max.max(v);
//     }

//     let mean = total_sum / nf;

//     let mut var = f64x4::splat(0.0);
//     let simd_mean = f64x4::splat(mean);

//     for i in 0..chunks {
//         let base = i * 4;
//         let v = f64x4::new([
//             prices[base],
//             prices[base + 1],
//             prices[base + 2],
//             prices[base + 3],
//         ]);

//         let diff = v - simd_mean;
//         var += diff * diff;
//     }

//     let mut total_var: f64 = var.to_array().iter().sum();

//     for i in (chunks * 4)..n {
//         let diff = prices[i] - mean;
//         total_var += diff * diff;
//     }

//     let variance = total_var / nf;
//     let std_dev = variance.sqrt();

//     (n, total_min, total_max, mean, std_dev)
// }

// // ---------------- FAST CSV LOADER ----------------
// fn load_fast(path: &str) -> Result<FxHashMap<String, Vec<f64>>, Box<dyn Error>> {
//     let mut rdr = ReaderBuilder::new()
//         .has_headers(true)
//         .buffer_capacity(1024 * 1024) // large buffer
//         .from_path(path)?;

//     let mut groups: FxHashMap<String, Vec<f64>> = FxHashMap::default();

//     let mut record = csv::StringRecord::new();

//     while rdr.read_record(&mut record)? {
//         let symbol = record.get(0).unwrap();
//         let price: f64 = record.get(2).unwrap().parse().unwrap();

//         groups
//             .entry(symbol.to_string())
//             .or_insert_with(|| Vec::with_capacity(23000))
//             .push(price);
//     }

//     Ok(groups)
// }

// //---------------- MAIN ----------------
// fn main() -> Result<(), Box<dyn Error>> {

//     // -------- LOAD --------
//     let t_load = Instant::now();
//     let groups = load_fast("stock_data.csv")?;
//     println!(
//         "Loaded {} symbols, {} rows in {:.2?}",
//         groups.len(),
//         groups.values().map(|v| v.len()).sum::<usize>(),
//         t_load.elapsed()
//     );

//     // -------- COMPUTE --------
//     let t_compute = Instant::now();

//     let mut results: Vec<_> = groups
//         .par_iter()
//         .map(|(sym, prices)| {
//             let (count, min, max, mean, std) = compute_stats_simd(prices);
//             (sym.clone(), count, min, max, mean, std)
//         })
//         .collect();

//     println!("Compute time: {:.2?}", t_compute.elapsed());

//     // -------- SORT --------
//     results.sort_by(|a, b| a.0.cmp(&b.0));

//     // -------- PRINT --------
//     println!(
//         "\n{:<8}  {:>8}  {:>10}  {:>10}  {:>10}  {:>10}",
//         "SYMBOL", "COUNT", "MIN", "MAX", "MEAN", "STD_DEV"
//     );
//     println!("{}", "-".repeat(68));

//     for (sym, count, min, max, mean, std) in &results {
//         println!(
//             "{:<8}  {:>8}  {:>10.2}  {:>10.2}  {:>10.2}  {:>10.4}",
//             sym, count, min, max, mean, std
//         );
//     }

//     println!("\nTotal time: {:.2?}", t_load.elapsed());

//     Ok(())
// }

//----------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// Binary protobuf input path and aggregation loop.

use stock_pipeline::run_aggregation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let per_symbol = run_aggregation("stock_data.bin")?;

    for (symbol, acc) in &per_symbol {
        let (min, max, mean, std_dev) = acc.finalize();
        println!(
            "{}: count={}, min={}, max={}, mean={}, std_dev={}",
            symbol, acc.count, min, max, mean, std_dev
        );
    }

    Ok(())
}

// load and aggregation timing helper (commented out)

// use rustc_hash::FxHashMap;
// use stock_pipeline::{load_all_records, stats::StatsAccumulator};

// pub fn run_aggregation_timed(path: &str) {
//     use std::time::Instant;

//     let t1 = Instant::now();
//     let records = load_all_records(path).unwrap();
//     let load_time = t1.elapsed();

//     let t2 = Instant::now();
//     let mut per_symbol: FxHashMap<String, StatsAccumulator> = FxHashMap::default();
//     for stock in &records {
//         if let Some(acc) = per_symbol.get_mut(&stock.symbol) {
//             acc.update(stock.price);
//         } else {
//             let mut acc = StatsAccumulator::new();
//             acc.update(stock.price);
//             per_symbol.insert(stock.symbol.clone(), acc);
//         }
//     }
//     let agg_time = t2.elapsed();

//     println!("Load time:  {:?}", load_time);
//     println!("Agg time:   {:?}", agg_time);
// }

// fn main() {
//     run_aggregation_timed("stock_data.bin");
// }
