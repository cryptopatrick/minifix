/// MiniFixRust Basic Example
///
/// This example demonstrates the core functionality of the minifix crate:
/// 1. Decoding FIX messages
/// 2. Accessing fields with type safety
/// 3. Working with repeating groups
/// 4. Encoding new FIX messages
/// 5. Handling different data types
use minifix::prelude::*;
use minifix::tagvalue::{Decoder, Encoder};

// Sample FIX 4.2 Market Data message with repeating groups (from working example)
const MARKET_DATA_MESSAGE: &[u8] = 
    b"8=FIX.4.2|9=196|35=X|49=A|56=B|34=12|52=20100318-03:21:11.364|262=A|268=2|279=0|269=0|278=BID|55=EUR/USD|270=1.37215|15=EUR|271=2500000|346=1|279=0|269=1|278=OFFER|55=EUR/USD|270=1.37224|15=EUR|271=2503200|346=1|10=171|";

fn main() {
    println!("=== MiniFixRust Basic Example ===\n");

    // Part 1: Decode a FIX message
    decode_example();

    // Part 2: Encode a new FIX message
    encode_example();

    println!("Example completed successfully!");
}

fn decode_example() {
    println!("1. DECODING FIX MESSAGES");
    println!("------------------------");

    // Create a FIX 4.2 dictionary and decoder
    let fix_dictionary = Dictionary::fix42();
    let mut decoder = Decoder::new(fix_dictionary);

    // Configure decoder to use "|" separator for readability (normally SOH/0x01)
    decoder.config_mut().separator = b'|';

    // Decode the message
    let msg = decoder
        .decode(MARKET_DATA_MESSAGE)
        .expect("Failed to decode FIX message");

    println!("✓ Successfully decoded FIX message");

    // Access header fields with type safety
    println!("\n📋 Header Fields:");
    println!(
        "  BeginString: {:?}",
        msg.get::<&[u8]>(fix42::BEGIN_STRING).unwrap()
    );
    println!("  MsgType: {:?}", msg.get::<&[u8]>(fix42::MSG_TYPE).unwrap());
    println!(
        "  SenderCompID: {:?}",
        msg.get::<&[u8]>(fix42::SENDER_COMP_ID).unwrap()
    );
    println!(
        "  TargetCompID: {:?}",
        msg.get::<&[u8]>(fix42::TARGET_COMP_ID).unwrap()
    );
    println!("  MsgSeqNum: {:?}", msg.get::<u32>(fix42::MSG_SEQ_NUM).unwrap());

    // Access body fields
    println!("\n📊 Message Body:");
    println!("  MDReqID: {:?}", msg.get::<&[u8]>(fix42::MD_REQ_ID).unwrap());

    // Work with repeating groups
    println!("\n🔄 Repeating Groups (Market Data Entries):");
    let md_entries = msg.group(fix42::NO_MD_ENTRIES).unwrap();
    println!("  Number of entries: {}", md_entries.len());

    for (i, entry) in md_entries.entries().enumerate() {
        println!("\n  Entry {}:", i + 1);
        println!(
            "    MDUpdateAction: {:?}",
            entry.get::<&[u8]>(fix42::MD_UPDATE_ACTION).unwrap()
        );
        println!(
            "    MDEntryType: {:?}",
            entry.get::<&[u8]>(fix42::MD_ENTRY_TYPE).unwrap()
        );
        println!(
            "    Symbol: {:?}",
            entry.get::<&[u8]>(fix42::SYMBOL).unwrap()
        );
        println!(
            "    MDEntryPx: {:?}",
            entry.get::<&[u8]>(fix42::MD_ENTRY_PX).unwrap()
        );
        println!(
            "    Currency: {:?}",
            entry.get::<&[u8]>(fix42::CURRENCY).unwrap()
        );
        println!(
            "    MDEntrySize: {:?}",
            entry.get::<u32>(fix42::MD_ENTRY_SIZE).unwrap()
        );
    }

    // Demonstrate different ways to access the same field
    println!("\n🎯 Field Access Examples:");
    println!(
        "  Using field constant: {:?}",
        msg.get::<&[u8]>(fix42::SENDER_COMP_ID).unwrap()
    );
    println!("  Using tag number: {:?}", msg.get::<&[u8]>(49).unwrap()); // Tag 49 = SenderCompID

    println!();
}

fn encode_example() {
    println!("2. ENCODING FIX MESSAGES");
    println!("------------------------");

    // Create an encoder
    let mut encoder = Encoder::new();
    let mut buffer = Vec::new();

    // Start building a New Order Single message (FIX 4.3)
    let mut msg = encoder.start_message(b"FIX.4.3", &mut buffer, b"D"); // D = NewOrderSingle

    // Set header fields
    msg.set(fix43::MSG_SEQ_NUM, 123);
    msg.set(fix43::SENDER_COMP_ID, "CLIENT");
    msg.set(fix43::TARGET_COMP_ID, "BROKER");

    // Set order fields with different data types
    msg.set(fix43::CL_ORD_ID, "ORDER-001"); // String
    msg.set(fix43::SYMBOL, "AAPL"); // String
    msg.set(fix43::SIDE, fix43::Side::Buy); // Enum
    msg.set(fix43::ORDER_QTY, 100); // Integer
    msg.set(fix43::ORD_TYPE, fix43::OrdType::Limit); // Enum
    msg.set(fix43::PRICE, 150.25); // Float
    msg.set(fix43::TIME_IN_FORCE, fix43::TimeInForce::Day); // Enum

    // Finish the message (adds checksum automatically)
    let (encoded_msg, _) = msg.done();

    println!("✓ Successfully encoded FIX message");
    println!("\n📤 Encoded Message:");

    // Replace SOH with | for display
    let display_msg =
        String::from_utf8_lossy(encoded_msg).replace('\x01', "|");
    println!("  {}", display_msg);

    // Demonstrate roundtrip: decode what we just encoded
    println!("\n🔄 Roundtrip Test:");
    let fix_dictionary = Dictionary::fix43();
    let mut decoder = Decoder::new(fix_dictionary);

    let decoded_msg =
        decoder.decode(encoded_msg).expect("Failed to decode our own message");

    println!(
        "  Original ClOrdID: {:?}",
        decoded_msg.get::<&[u8]>(fix43::CL_ORD_ID).unwrap()
    );
    println!(
        "  Original Symbol: {:?}",
        decoded_msg.get::<&[u8]>(fix43::SYMBOL).unwrap()
    );
    println!(
        "  Original Side: {:?}",
        decoded_msg.get::<&[u8]>(fix43::SIDE).unwrap()
    );
    println!(
        "  Original OrderQty: {:?}",
        decoded_msg.get::<u32>(fix43::ORDER_QTY).unwrap()
    );
    println!("  ✓ Roundtrip successful!");

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_example() {
        let fix_dictionary = Dictionary::fix42();
        let mut decoder = Decoder::new(fix_dictionary);
        decoder.config_mut().separator = b'|';

        let msg = decoder.decode(MARKET_DATA_MESSAGE).unwrap();

        // Verify key fields
        assert_eq!(msg.get::<&[u8]>(fix42::BEGIN_STRING).unwrap(), b"FIX.4.2");
        assert_eq!(msg.get::<&[u8]>(fix42::MSG_TYPE).unwrap(), b"X");
        assert_eq!(msg.get::<&[u8]>(fix42::SENDER_COMP_ID).unwrap(), b"A");

        // Verify repeating groups
        let md_entries = msg.group(fix42::NO_MD_ENTRIES).unwrap();
        assert_eq!(md_entries.len(), 2);
    }

    #[test]
    fn test_encode_example() {
        let mut encoder = Encoder::new();
        let mut buffer = Vec::new();

        let mut msg = encoder.start_message(b"FIX.4.3", &mut buffer, b"D");
        msg.set(fix43::CL_ORD_ID, "TEST-ORDER");
        msg.set(fix43::SYMBOL, "TEST");
        msg.set(fix43::SIDE, fix43::Side::Buy);

        let (encoded_msg, _) = msg.done();

        // Verify we can decode what we encoded
        let fix_dictionary = Dictionary::fix43();
        let mut decoder = Decoder::new(fix_dictionary);
        let decoded_msg = decoder.decode(encoded_msg).unwrap();

        assert_eq!(
            decoded_msg.get::<&[u8]>(fix43::CL_ORD_ID).unwrap(),
            b"TEST-ORDER"
        );
        assert_eq!(decoded_msg.get::<&[u8]>(fix43::SYMBOL).unwrap(), b"TEST");
        assert_eq!(decoded_msg.get::<&[u8]>(fix43::SIDE).unwrap(), b"1"); // Buy = "1"
    }

    #[test]
    fn run_main() {
        main(); // Ensure the main function runs without panicking
    }
}
