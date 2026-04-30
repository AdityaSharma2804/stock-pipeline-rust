//! Library entry points for loading and aggregating stock protobuf records.

pub mod io;
pub mod pb;
pub mod stats;

use io::proto_reader::read_protobuf_streaming;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use stats::StatsAccumulator;

use crate::pb::stock::Stock; // Update this path if the generated Stock type is in a different module.

/// Load all stock records from a length-prefixed protobuf file.
pub fn load_all_records(path: &str) -> Result<Vec<Stock>, Box<dyn std::error::Error>> {
    let mut records = Vec::new();
    read_protobuf_streaming(path, |stock| {
        records.push(stock);
        Ok(())
    })?;
    Ok(records)
}

/// Aggregate stock statistics sequentially from a protobuf file by symbol.
pub fn run_aggregation(
    path: &str,
) -> Result<FxHashMap<String, StatsAccumulator>, Box<dyn std::error::Error>> {
    let mut per_symbol: FxHashMap<String, StatsAccumulator> = FxHashMap::default();

    read_protobuf_streaming(path, |stock| {
        if let Some(acc) = per_symbol.get_mut(&stock.symbol) {
            acc.update(stock.price);
        } else {
            let mut acc = StatsAccumulator::new();
            acc.update(stock.price);
            per_symbol.insert(stock.symbol, acc);
        }
        Ok(())
    })?;

    Ok(per_symbol)
}

/// Aggregate stock statistics in parallel from a protobuf file by symbol.
pub fn run_aggregation_parallel(
    path: &str,
) -> Result<FxHashMap<String, StatsAccumulator>, Box<dyn std::error::Error>> {
    let records = load_all_records(path)?;

    let per_symbol = records
        .par_iter()
        .fold(
            FxHashMap::default,
            |mut map: FxHashMap<String, StatsAccumulator>, stock| {
                if let Some(acc) = map.get_mut(&stock.symbol) {
                    acc.update(stock.price);
                } else {
                    let mut acc = StatsAccumulator::new();
                    acc.update(stock.price);
                    map.insert(stock.symbol.clone(), acc);
                }
                map
            },
        )
        .reduce(
            FxHashMap::default,
            |mut a: FxHashMap<String, StatsAccumulator>, b| {
                for (symbol, acc) in b {
                    a.entry(symbol)
                        .or_insert_with(StatsAccumulator::new)
                        .merge(acc);
                }
                a
            },
        );

    Ok(per_symbol)
}
