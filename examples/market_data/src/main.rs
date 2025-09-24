use minifix::definitions::fix44;
/// Market Data Processing Example
///
/// This example demonstrates advanced FIX protocol features:
/// 1. Processing market data snapshots and incremental updates
/// 2. Working with complex repeating groups
/// 3. Handling multiple currency pairs
/// 4. Type-safe field access with enums
/// 5. Real-world message formats
use minifix::prelude::*;
use minifix::tagvalue::{Decoder, Encoder, Message};
use std::collections::{HashMap, HashSet};

// Sample market data messages from real trading sessions
const MARKET_DATA_SNAPSHOT: &[u8] = b"8=FIX.4.4|9=335|35=W|49=BLOOMBERG|56=CLIENT1|34=125|52=20231201-14:30:15.750|262=EURUSD-SNAP|268=4|279=0|269=0|278=BID|55=EUR/USD|270=1.08945|15=EUR|271=5000000|279=1|269=1|278=OFFER|55=EUR/USD|270=1.08950|15=EUR|271=4800000|279=2|269=0|278=BID|55=EUR/USD|270=1.08940|15=EUR|271=3200000|279=3|269=1|278=OFFER|55=EUR/USD|270=1.08955|15=EUR|271=2100000|10=174|";

const MARKET_DATA_INCREMENTAL: &[u8] = b"8=FIX.4.4|9=208|35=X|49=BLOOMBERG|56=CLIENT1|34=126|52=20231201-14:30:16.125|262=EURUSD-INC|268=2|279=0|269=0|278=BID|55=EUR/USD|270=1.08948|15=EUR|271=6000000|279=1|269=1|278=OFFER|55=EUR/USD|270=1.08952|15=EUR|271=5500000|10=007|";

const MARKET_DATA_MULTI_CCY: &[u8] = b"8=FIX.4.4|9=457|35=W|49=REFINITIV|56=CLIENT1|34=87|52=20231201-14:30:17.250|262=MULTI-SNAP|268=6|279=0|269=0|278=BID|55=EUR/USD|270=1.08945|15=EUR|271=5000000|279=1|269=1|278=OFFER|55=EUR/USD|270=1.08950|15=EUR|271=4800000|279=2|269=0|278=BID|55=GBP/USD|270=1.26785|15=GBP|271=3000000|279=3|269=1|278=OFFER|55=GBP/USD|270=1.26790|15=GBP|271=2800000|279=4|269=0|278=BID|55=USD/JPY|270=149.85|15=USD|271=8000000|279=5|269=1|278=OFFER|55=USD/JPY|270=149.90|15=USD|271=7500000|10=029|";

#[derive(Debug)]
struct PriceLevel {
    side: String,
    price: f64,
    size: u64,
    currency: String,
}

#[derive(Debug)]
struct MarketDataBook {
    symbol: String,
    levels: Vec<PriceLevel>,
}

fn main() {
    println!("=== Market Data Processing Example ===\n");

    // Process different types of market data messages
    process_market_data_snapshot();
    process_market_data_incremental();
    process_multi_currency_snapshot();

    // Demonstrate market data subscription workflow
    market_data_subscription_workflow();

    println!("Market data example completed successfully!");
}

fn process_market_data_snapshot() {
    println!("1. PROCESSING MARKET DATA SNAPSHOT");
    println!("----------------------------------");

    let mut decoder = Decoder::new(Dictionary::fix44());
    decoder.config_mut().separator = b'|';

    let msg = decoder
        .decode(MARKET_DATA_SNAPSHOT)
        .expect("Failed to decode market data snapshot");

    // Extract header information
    println!("📊 Market Data Snapshot:");
    println!(
        "  Source: {:?}",
        msg.get::<&[u8]>(fix44::SENDER_COMP_ID).unwrap()
    );
    println!(
        "  Request ID: {:?}",
        msg.get::<&[u8]>(fix44::MD_REQ_ID).unwrap()
    );

    // Process market data entries
    let mut book = process_md_entries(&msg, "Market Data Snapshot").unwrap();

    // Sort levels by price for display
    book.levels.sort_by(|a, b| {
        match a.side.as_str() {
            "BID" => b.price.partial_cmp(&a.price).unwrap(), // Bids: highest first
            "OFFER" => a.price.partial_cmp(&b.price).unwrap(), // Offers: lowest first
            _ => std::cmp::Ordering::Equal,
        }
    });

    println!("\n💰 Order Book for {}:", book.symbol);
    print_order_book(&book);
    println!();
}

fn process_market_data_incremental() {
    println!("2. PROCESSING INCREMENTAL UPDATE");
    println!("--------------------------------");

    let mut decoder = Decoder::new(Dictionary::fix44());
    decoder.config_mut().separator = b'|';

    let msg = decoder
        .decode(MARKET_DATA_INCREMENTAL)
        .expect("Failed to decode incremental update");

    println!("📈 Market Data Incremental Update:");
    println!(
        "  Source: {:?}",
        msg.get::<&[u8]>(fix44::SENDER_COMP_ID).unwrap()
    );
    println!(
        "  Request ID: {:?}",
        msg.get::<&[u8]>(fix44::MD_REQ_ID).unwrap()
    );

    let book = process_md_entries(&msg, "Incremental Update").unwrap();

    println!("\n🔄 Price Updates:");
    for level in &book.levels {
        println!(
            "    {} {} @ {} size {} {}",
            book.symbol, level.side, level.price, level.size, level.currency
        );
    }
    println!();
}

fn process_multi_currency_snapshot() {
    println!("3. PROCESSING MULTI-CURRENCY SNAPSHOT");
    println!("-------------------------------------");

    let mut decoder = Decoder::new(Dictionary::fix44());
    decoder.config_mut().separator = b'|';

    let msg = decoder
        .decode(MARKET_DATA_MULTI_CCY)
        .expect("Failed to decode multi-currency snapshot");

    println!("🌍 Multi-Currency Market Data:");
    println!(
        "  Source: {:?}",
        msg.get::<&[u8]>(fix44::SENDER_COMP_ID).unwrap()
    );

    // Group entries by symbol
    let mut books: HashMap<String, MarketDataBook> = HashMap::new();

    let entries = msg.group(fix44::NO_MD_ENTRIES).unwrap();
    for entry in entries.entries() {
        let symbol = String::from_utf8_lossy(
            entry.get::<&[u8]>(fix44::SYMBOL).unwrap(),
        )
        .to_string();
        let side_byte = entry.get::<&[u8]>(fix44::MD_ENTRY_TYPE).unwrap();
        let side = match side_byte {
            b"0" => "BID",
            b"1" => "OFFER",
            _ => "UNKNOWN",
        }
        .to_string();

        let price_str = std::str::from_utf8(
            entry.get::<&[u8]>(fix44::MD_ENTRY_PX).unwrap(),
        )
        .unwrap();
        let price: f64 = price_str.parse().unwrap();
        let size: u64 = entry.get::<u32>(fix44::MD_ENTRY_SIZE).unwrap() as u64;
        let currency = String::from_utf8_lossy(
            entry.get::<&[u8]>(fix44::CURRENCY).unwrap(),
        )
        .to_string();

        let level = PriceLevel { side, price, size, currency };

        books
            .entry(symbol.clone())
            .or_insert_with(|| MarketDataBook {
                symbol: symbol.clone(),
                levels: Vec::new(),
            })
            .levels
            .push(level);
    }

    // Display each currency pair's book
    for (symbol, mut book) in books {
        book.levels.sort_by(|a, b| match a.side.as_str() {
            "BID" => b.price.partial_cmp(&a.price).unwrap(),
            "OFFER" => a.price.partial_cmp(&b.price).unwrap(),
            _ => std::cmp::Ordering::Equal,
        });

        println!("\n📈 {} Order Book:", symbol);
        print_order_book(&book);
    }

    println!();
}

fn market_data_subscription_workflow() {
    println!("4. MARKET DATA SUBSCRIPTION WORKFLOW");
    println!("------------------------------------");

    // Demonstrate creating market data requests
    let mut encoder = Encoder::new();
    let mut buffer = Vec::new();

    // Create Market Data Request (V)
    let mut md_request = encoder.start_message(b"FIX.4.4", &mut buffer, b"V");

    md_request.set(fix44::MD_REQ_ID, "MD-REQ-001");
    md_request.set(fix44::SUBSCRIPTION_REQUEST_TYPE, "1"); // Snapshot + Updates
    md_request.set(fix44::MARKET_DEPTH, 5); // Top 5 levels
    md_request.set(fix44::MD_UPDATE_TYPE, "0"); // Full refresh

    // Add symbols to request (this would need repeating group support)
    // For now, just set a single symbol
    md_request.set(fix44::SYMBOL, "EUR/USD");

    let (request_msg, _) = md_request.done();

    println!("📤 Market Data Request:");
    let display_msg =
        String::from_utf8_lossy(request_msg).replace('\x01', "|");
    println!("  {}", display_msg);

    // Create Market Data Reject (Y) - for error handling demo
    let mut buffer2 = Vec::new();
    let mut md_reject = encoder.start_message(b"FIX.4.4", &mut buffer2, b"Y");

    md_reject.set(fix44::MD_REQ_ID, "MD-REQ-001");
    md_reject.set(fix44::MD_REQ_REJ_REASON, "4"); // Unsupported market depth
    md_reject.set(fix44::TEXT, "Market depth of 5 not supported");

    let (reject_msg, _) = md_reject.done();

    println!("\n❌ Market Data Reject Example:");
    let display_reject =
        String::from_utf8_lossy(reject_msg).replace('\x01', "|");
    println!("  {}", display_reject);

    println!("\n✅ Subscription workflow demonstration complete");
}

fn process_md_entries(
    msg: &Message<&[u8]>,
    msg_type: &str,
) -> Result<MarketDataBook, String> {
    let entries = msg
        .group(fix44::NO_MD_ENTRIES)
        .map_err(|e| format!("failed to access MD entries: {e:?}"))?;
    println!("  {} entries in {}", entries.len(), msg_type);

    let mut levels = Vec::new();
    let mut symbol = String::new();

    for (i, entry) in entries.entries().enumerate() {
        // Get symbol from first entry
        if symbol.is_empty() {
            let raw_symbol = entry
                .get::<&[u8]>(fix44::SYMBOL)
                .map_err(|e| format!("missing symbol: {e:?}"))?;
            symbol = String::from_utf8_lossy(raw_symbol).to_string();
        }

        let side_byte = entry
            .get::<&[u8]>(fix44::MD_ENTRY_TYPE)
            .map_err(|e| format!("missing entry type: {e:?}"))?;
        let side = match side_byte {
            b"0" => "BID",
            b"1" => "OFFER",
            _ => "UNKNOWN",
        }
        .to_string();

        let price_str_bytes = entry
            .get::<&[u8]>(fix44::MD_ENTRY_PX)
            .map_err(|e| format!("missing price: {e:?}"))?;
        let price_str = std::str::from_utf8(price_str_bytes)
            .map_err(|e| format!("invalid UTF-8 in price: {e}"))?;
        let price: f64 = price_str
            .parse()
            .map_err(|e| format!("invalid price value: {e}"))?;
        let size: u64 = entry
            .get::<u32>(fix44::MD_ENTRY_SIZE)
            .map_err(|e| format!("missing size: {e:?}"))?
            as u64;
        let currency_bytes = entry
            .get::<&[u8]>(fix44::CURRENCY)
            .map_err(|e| format!("missing currency: {e:?}"))?;
        let currency = String::from_utf8_lossy(currency_bytes).to_string();

        println!(
            "    Entry {}: {} @ {} size {} {}",
            i + 1,
            match side_byte {
                b"0" => "BID",
                b"1" => "OFFER",
                _ => "?",
            },
            price_str,
            size,
            currency.as_str()
        );

        levels.push(PriceLevel { side, price, size, currency });
    }

    Ok(MarketDataBook { symbol, levels })
}

fn print_order_book(book: &MarketDataBook) {
    let bids: Vec<_> =
        book.levels.iter().filter(|l| l.side == "BID").collect();
    let offers: Vec<_> =
        book.levels.iter().filter(|l| l.side == "OFFER").collect();

    println!(
        "    {:>12} | {:^8} | {:^12} | {:^8} | {:<12}",
        "Size", "Bid", "Symbol", "Offer", "Size"
    );
    println!(
        "    {:-^12}-+-{:-^8}-+-{:-^12}-+-{:-^8}-+-{:-<12}",
        "", "", "", "", ""
    );

    let max_levels = std::cmp::max(bids.len(), offers.len());

    for i in 0..max_levels {
        let (bid_size, bid_price, bid_currency) = if let Some(bid) = bids.get(i) {
            (bid.size, bid.price, bid.currency.as_str())
        } else {
            (0, 0.0, "")
        };
        let (offer_size, offer_price, offer_currency) = if let Some(offer) = offers.get(i) {
            (offer.size, offer.price, offer.currency.as_str())
        } else {
            (0, 0.0, "")
        };

        let symbol_display =
            if i == max_levels / 2 { &book.symbol } else { "" };

        println!(
            "    {:>12} | {:>8.5} | {:^12} | {:<8.5} | {:<12}",
            if bid_size > 0 {
                format!("{} {}", bid_size, bid_currency)
            } else {
                String::new()
            },
            if bid_price > 0.0 {
                format!("{:.5}", bid_price)
            } else {
                String::new()
            },
            symbol_display,
            if offer_price > 0.0 {
                format!("{:.5}", offer_price)
            } else {
                String::new()
            },
            if offer_size > 0 {
                format!("{} {}", offer_size, offer_currency)
            } else {
                String::new()
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_data_parsing() {
        let mut decoder = Decoder::new(Dictionary::fix44());
        decoder.config_mut().separator = b'|';

        let msg = decoder.decode(MARKET_DATA_SNAPSHOT).unwrap();

        // Verify message structure
        assert_eq!(msg.get::<&[u8]>(fix44::MSG_TYPE).unwrap(), b"W");
        assert_eq!(
            msg.get::<&[u8]>(fix44::SENDER_COMP_ID).unwrap(),
            b"BLOOMBERG"
        );

        // Verify repeating groups
        let entries = msg.group(fix44::NO_MD_ENTRIES).unwrap();
        assert_eq!(entries.len(), 4);

        // Check first entry
        let first_entry = entries.entries().next().unwrap();
        assert_eq!(
            first_entry.get::<&[u8]>(fix44::SYMBOL).unwrap(),
            b"EUR/USD"
        );
        assert_eq!(
            first_entry.get::<&[u8]>(fix44::MD_ENTRY_TYPE).unwrap(),
            b"0"
        ); // BID
    }

    #[test]
    fn test_multi_currency_parsing() {
        let mut decoder = Decoder::new(Dictionary::fix44());
        decoder.config_mut().separator = b'|';

        let msg = decoder.decode(MARKET_DATA_MULTI_CCY).unwrap();
        let entries = msg.group(fix44::NO_MD_ENTRIES).unwrap();

        // Should have 6 entries across 3 currency pairs
        assert_eq!(entries.len(), 6);

        // Collect unique symbols
        let mut symbols: HashSet<String> = HashSet::new();
        for entry in entries.entries() {
            let symbol = entry.get::<&[u8]>(fix44::SYMBOL).unwrap();
            symbols.insert(std::str::from_utf8(symbol).unwrap().to_string());
        }

        assert_eq!(symbols.len(), 3); // EUR/USD, GBP/USD, USD/JPY
        assert!(symbols.contains("EUR/USD"));
        assert!(symbols.contains("GBP/USD"));
        assert!(symbols.contains("USD/JPY"));
    }
}
