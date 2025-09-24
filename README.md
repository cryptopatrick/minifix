<h1 align="center">
  <br>
    <img 
      src="https://github.com/cryptopatrick/minifix/blob/main/assets/minifix.png" 
      alt="miniFIX" 
      width="200"
    />
  <br>
  miniFIX
  <br>
</h1>

<h4 align="center">
  A bare essentials library for 
  <a href="https://www.fixtrading.org/" target="_blank">
    Financial Information Exchange (FIX)</a>
</h4>

<p align="center">
  <a href="https://crates.io/crates/minifix" target="_blank">
    <img src="https://img.shields.io/crates/v/minifix" alt="Crates.io"/>
  </a>
  <a href="https://crates.io/crates/minifix" target="_blank">
    <img src="https://img.shields.io/crates/d/minifix" alt="Downloads"/>
  </a>
  <a href="https://docs.rs/minifix" target="_blank">
    <img src="https://docs.rs/minifix/badge.svg" alt="Documentation"/>
  </a>
  <a href="LICENSE" target="_blank">
    <img src="https://img.shields.io/github/license/cryptopatrick/minifix.svg" alt="GitHub license"/>
  </a>
</p>

<p align="center">
  <a href="#-what-is-minifix">What is miniFIX</a> •
  <a href="#-features">Features</a> •
  <a href="#-how-to-use">How To Use</a> •
  <a href="#-documentation">Documentation</a> •
  <a href="#-license">License</a>
</p>

<!-- TABLE OF CONTENTS -->
<h2 id="table-of-contents"> :pushpin: Table of Contents</h2>

<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#-what-is-minifix"> What is miniFIX</a></li>
    <li><a href="#-features"> Features</a></li>
    <li><a href="#-how-to-use"> How to Use</a></li>
    <li><a href="#-documentation"> Documentation</a></li>
    <li><a href="#-license">License</a></li>
  </ol>
</details>

## 🤔 What is miniFIX

`minifix` is a high-performance, memory-safe Rust library for parsing, manipulating, and generating FIX (Financial Information Exchange) protocol messages. Designed for speed, safety, and ease of use in financial trading systems, it provides zero-copy parsing, type-safe field access, and comprehensive support for FIX protocol versions 4.0 through 4.4. miniFIX is a derivation of the FerrumFIX crate. Our goal is to create a slimmed-down-bare-essentials version of the original crate. 

> Please note that a lot of the code in miniFIX is drawn from [FerrumFIX](https://github.com/ferrumfix/ferrumfix), and published in accordance with the MIT license (see LICENSE).

### Use Cases

- **Trading Systems**: Parse and generate order messages, execution reports, and market data
- **Market Data Processing**: Handle real-time market data feeds with minimal latency
- **FIX Gateway Development**: Build protocol bridges and message routing systems
- **Financial Analytics**: Process historical trade data and market information
- **Risk Management**: Monitor and analyze trading activity in real-time

### Architecture

miniFIX is organized into several focused crates:

1. **minifix**: Main library with parsing, encoding, and field type handling
2. **minifix-dictionary**: FIX protocol specifications and field definitions  
3. **minifix-codegen**: Code generation for FIX message structures
4. **minisofh**: Simple Open Framing Header (SOFH) support

## 📷 Features

### Core Parsing
- **Zero-copy parsing** - Process messages without unnecessary allocations
- **Type safety** - Compile-time guarantees for field types and message structure
- **High performance** - Optimized for low-latency trading applications
- **Memory safety** - No buffer overflows or undefined behavior

### FIX Protocol Support
- **Multiple FIX versions** - Support for FIX 4.0, 4.1, 4.2, 4.3, and 4.4
- **Message validation** - Comprehensive message structure and field validation
- **Repeating groups** - Full support for FIX repeating group structures
- **Field type handling** - Type-safe field value handling with validation

### Integration & Performance
- **Async support** - Built-in Tokio integration for async applications
- **Session management** - Session handling and heartbeat management
- **JSON encoding** - JSON representation of FIX messages
- **SIMD optimizations** - Vectorized operations where possible

### Example Usage

```rust
use minifix::prelude::*;

// Parse a FIX message
let message = b"8=FIX.4.2|9=49|35=D|49=SENDER|56=TARGET|34=1|52=20230101-12:00:00|55=AAPL|54=1|38=100|10=123|";
let mut decoder = Decoder::new(Dictionary::fix42());
decoder.config_mut().separator = b'|'; // For display (normally SOH)

let msg = decoder.decode(message)?;
println!("Symbol: {:?}", msg.get::<&[u8]>(fix42::SYMBOL)?); // "AAPL"

// Type-safe field access
let seq_num: u32 = msg.get(fix42::MSG_SEQ_NUM)?;           // Integer field
let symbol: &[u8] = msg.get(fix42::SYMBOL)?;               // String field  
let side: fix42::Side = msg.get(fix42::SIDE)?;             // Enumeration field

// Encode a new message
let mut encoder = Encoder::new();
let mut buffer = Vec::new();
let mut new_msg = encoder.start_message(b"FIX.4.3", &mut buffer, b"D");

new_msg.set(fix43::CL_ORD_ID, "ORDER-12345");
new_msg.set(fix43::SYMBOL, "AAPL");
new_msg.set(fix43::SIDE, fix43::Side::Buy);
new_msg.set(fix43::ORDER_QTY, 100);

let (encoded, _) = new_msg.done();
println!("Encoded: {}", String::from_utf8_lossy(encoded));
```

## 🚙 How to Use

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
minifix = "0.1"
```

Or install with cargo:

```bash
cargo add minifix
```

### Optional Features

Enable optional features as needed:

```toml
minifix = { version = "0.1", features = [
    "fix44",          # FIX 4.4 support (includes 4.0-4.3)
    "utils-chrono",   # chrono integration for date/time fields
    "utils-tokio",    # async/Tokio support
    "json-encoding"   # JSON encoding support
]}
```

### Example

```rust
use minifix::prelude::*;
use minifix::tagvalue::{Decoder, Encoder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a FIX message
    let message = b"8=FIX.4.2|9=73|35=D|49=SENDER|56=TARGET|34=1|52=20230101-12:00:00|11=ORDER1|55=AAPL|54=1|38=100|40=2|44=150.50|59=0|10=123|";
    
    let mut decoder = Decoder::new(Dictionary::fix42());
    decoder.config_mut().separator = b'|';
    
    let msg = decoder.decode(message)?;
    
    // Access fields with type safety
    println!("Order ID: {:?}", msg.get::<&[u8]>(fix42::CL_ORD_ID)?);
    println!("Symbol: {:?}", msg.get::<&[u8]>(fix42::SYMBOL)?);  
    println!("Side: {:?}", msg.get::<fix42::Side>(fix42::SIDE)?);
    
    // Encode a new message
    let mut encoder = Encoder::new();
    let mut buffer = Vec::new();
    let mut new_msg = encoder.start_message(b"FIX.4.2", &mut buffer, b"8");
    
    new_msg.set(fix42::ORDER_ID, "EXEC-001");
    new_msg.set(fix42::SYMBOL, "AAPL");
    new_msg.set(fix42::SIDE, fix42::Side::Buy);
    
    let (encoded, _) = new_msg.done();
    println!("Encoded: {}", String::from_utf8_lossy(encoded));
    
    Ok(())
}
```

## 📚 Documentation

Comprehensive documentation is available at [docs.rs/minifix](https://docs.rs/minifix), including:
- API reference for all public types and functions
- Tutorial on FIX message parsing and encoding
- Examples of different message patterns and use cases
- Best practices for financial application development


## 🖊 Author

<a href="https://x.com/cryptopatrick">CryptoPatrick</a>  

Keybase Verification:  
https://keybase.io/cryptopatrick/sigs/8epNh5h2FtIX1UNNmf8YQ-k33M8J-Md4LnAN

## 🐣 Support
Leave a ⭐ if you think this project is cool.  

## 🗄 License
This project is licensed under MIT. See [LICENSE](LICENSE) for details.