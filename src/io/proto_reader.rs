//! Streaming reader implementation for length-prefixed protobuf stock records.
// use std::fs::File;
// use std::io::{BufReader, Read};

// use prost::Message;

// use crate::pb::stock::Stock;

// // custom varint decoder
// fn read_varint<R: Read>(reader: &mut R) -> Result<u64, Box<dyn std::error::Error>> {
//     let mut result = 0u64;
//     let mut shift = 0;

//     loop {
//         let mut buf = [0u8; 1];
//         if reader.read_exact(&mut buf).is_err() {
//             return Err("EOF".into());
//         }

//         let byte = buf[0];
//         result |= ((byte & 0x7F) as u64) << shift;

//         if byte & 0x80 == 0 {
//             break;
//         }

//         shift += 7;
//     }

//     Ok(result)
// }

// pub fn read_protobuf(
//     path: &str,
// ) -> Result<Vec<Stock>, Box<dyn std::error::Error>> {

//     let file = File::open(path)?;
//     let mut reader = BufReader::new(file);

//     let mut stocks = Vec::new();            //the lake

//     loop {
//         let len = match read_varint(&mut reader) {
//             Ok(l) => l,
//             Err(_) => break,
//         };

//         let mut buf = vec![0; len as usize];
//         reader.read_exact(&mut buf)?;

//         let stock = Stock::decode(&*buf)?;
//         stocks.push(stock);     //add the stock to the lake
//     }

//     Ok(stocks)      //hand teh entire lake to the caller
// }

// streaming reader implementation

// The caller provides record handling; the reader drives the loop.

use crate::pb::stock::Stock;
use prost::Message;
use std::fs::File;
use std::io::{BufReader, Read};

/// Read a length-prefixed protobuf file and invoke `on_record` for each decoded stock record.
pub fn read_protobuf_streaming<F>(
    path: &str,
    mut on_record: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(Stock) -> Result<(), Box<dyn std::error::Error>>,
{
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(1 << 20, file); // 1MB buffer

    // reuse a single buffer across all records — zero per-record allocation
    let mut buf = Vec::with_capacity(256);

    loop {
        // read varint using already-buffered bytes, no per-byte syscalls
        let len = match read_varint_buffered(&mut reader) {
            Ok(l) => l,
            Err(_) => break,
        };

        // reuse buffer — just resize, no new allocation if within capacity
        buf.clear();
        buf.resize(len as usize, 0u8);
        reader.read_exact(&mut buf)?;

        let stock = Stock::decode(&*buf)?;
        on_record(stock)?;
    }

    Ok(())
}

// Read a varint from the buffered reader.
fn read_varint_buffered<R: Read>(reader: &mut R) -> Result<u64, Box<dyn std::error::Error>> {
    let mut result = 0u64;
    let mut shift = 0;
    let mut byte = [0u8; 1];

    loop {
        reader.read_exact(&mut byte)?;
        result |= ((byte[0] & 0x7F) as u64) << shift;
        if byte[0] & 0x80 == 0 {
            break;
        }
        shift += 7;
    }

    Ok(result)
}
