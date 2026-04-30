//! Convert CSV rows into a length-prefixed protobuf file.

use std::fs::File;
use std::io::{BufWriter, Write};

use chrono::NaiveDateTime;
use csv::ReaderBuilder;
use prost::Message;

use crate::pb::stock::Stock;

/// Write binary protobuf records from a CSV file.
pub fn write_protobuf_from_csv(
    csv_path: &str,
    bin_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = ReaderBuilder::new() // open the CSV reader
        .has_headers(true)
        .from_path(csv_path)?;

    let file = File::create(bin_path)?; // create the binary output file
    let mut writer = BufWriter::new(file);

    let mut skipped = 0;

    for result in reader.records() {
        // iterate CSV records
        let record = result?;

        // parse price from column 2
        let price = match record[2].trim().parse::<f64>() {
            // parse price field
            Ok(p) => p,
            Err(_) => {
                // skip invalid price rows
                println!("Bad price value: {:?}", record);
                skipped += 1;
                continue;
            }
        };

        // parse timestamp from column 1
        let timestamp = match NaiveDateTime::parse_from_str(
            // parse timestamp field
            record[1].trim(),
            "%Y-%m-%dT%H:%M:%S",
        ) {
            Ok(dt) => dt.and_utc().timestamp(),

            Err(_) => {
                println!("Bad timestamp: {:?}", record);
                skipped += 1;
                continue;
            }
        };

        let stock = Stock {
            symbol: record[0].to_string(),
            price,
            timestamp,
        };

        let mut buf = Vec::new();
        stock.encode(&mut buf)?;

        let mut len_buf = Vec::new();
        prost::encoding::encode_varint(buf.len() as u64, &mut len_buf);

        writer.write_all(&len_buf)?;
        writer.write_all(&buf)?;
    }

    writer.flush()?;

    println!("Skipped {} bad rows", skipped);

    Ok(())
}
