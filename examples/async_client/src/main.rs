/// Async FIX Client Example
/// 
/// This example demonstrates asynchronous FIX protocol handling using Tokio:
/// 1. Async message stream processing
/// 2. Connection management and heartbeat handling  
/// 3. Session state management
/// 4. Error handling and reconnection logic
/// 5. Real-world async patterns for trading systems

use minifix::prelude::*;
use minifix::tagvalue::{Decoder, Encoder, Message};
use minifix::definitions::fix44;

use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep;

/// Mock FIX server for demonstration purposes
#[tokio::main]
async fn main() {
    println!("=== Async FIX Client Example ===\n");
    
    // Start a mock FIX server for demonstration
    let server_handle = tokio::spawn(async {
        run_mock_fix_server().await;
    });
    
    // Give the server a moment to start
    sleep(Duration::from_millis(100)).await;
    
    // Run the FIX client
    let client_result = run_fix_client().await;
    
    // Clean up
    server_handle.abort();
    
    match client_result {
        Ok(_) => println!("✅ Async FIX client example completed successfully!"),
        Err(e) => println!("❌ Client error: {}", e),
    }
}

async fn run_fix_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 Connecting to FIX server...");
    
    // Connect to the mock server
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("✅ Connected to FIX server");
    
    let mut client = FixClient::new(stream);
    
    // Logon to the FIX session
    client.logon("CLIENT", "SERVER").await?;
    
    // Send a few test messages
    client.send_new_order_single("AAPL", "BUY", 100, 150.50).await?;
    client.send_new_order_single("MSFT", "SELL", 50, 380.25).await?;
    
    // Process incoming messages for a short time
    println!("📨 Processing incoming messages...");
    
    let mut message_count = 0;
    let timeout = Duration::from_secs(2);
    let start_time = tokio::time::Instant::now();
    
    while start_time.elapsed() < timeout && message_count < 10 {
        match client.receive_message().await {
            Ok(Some(msg)) => {
                message_count += 1;
                handle_incoming_message(&msg)?;
            }
            Ok(None) => {
                // No message received, continue
                sleep(Duration::from_millis(10)).await;
            }
            Err(e) => {
                println!("⚠️  Error receiving message: {}", e);
                break;
            }
        }
    }
    
    println!("📊 Processed {} messages", message_count);
    
    // Logout from the session
    client.logout("End of session").await?;
    
    Ok(())
}

/// Simple FIX client implementation for demonstration
struct FixClient {
    decoder: Decoder,
    encoder: Encoder,
    stream: TcpStream,
    seq_num: u32,
    buffer: Vec<u8>,
}

impl FixClient {
    fn new(stream: TcpStream) -> Self {
        Self {
            decoder: Decoder::new(Dictionary::fix44()),
            encoder: Encoder::new(),
            stream,
            seq_num: 1,
            buffer: Vec::new(),
        }
    }
    
    async fn logon(&mut self, sender: &str, target: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔐 Sending Logon message...");
        
        let encoded = {
            let mut buffer = Vec::new();
            let mut msg = self.encoder.start_message(b"FIX.4.4", &mut buffer, b"A"); // Logon
            
            msg.set(fix44::MSG_SEQ_NUM, self.seq_num);
            msg.set(fix44::SENDER_COMP_ID, sender);
            msg.set(fix44::TARGET_COMP_ID, target);
            msg.set(fix44::ENCRYPT_METHOD, "0"); // None
            msg.set(fix44::HEART_BT_INT, 30); // 30 second heartbeat
            
            let (encoded, _) = msg.done();
            encoded.to_vec()
        };
        
        self.seq_num += 1;
        
        // Send the message
        self.send_raw(&encoded).await?;
        
        println!("✅ Logon sent");
        Ok(())
    }
    
    async fn logout(&mut self, reason: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("👋 Sending Logout message...");
        
        let encoded = {
            let mut buffer = Vec::new();
            let mut msg = self.encoder.start_message(b"FIX.4.4", &mut buffer, b"5"); // Logout
            
            msg.set(fix44::MSG_SEQ_NUM, self.seq_num);
            msg.set(fix44::TEXT, reason);
            
            let (encoded, _) = msg.done();
            encoded.to_vec()
        };
        
        self.seq_num += 1;
        
        self.send_raw(&encoded).await?;
        
        println!("✅ Logout sent");
        Ok(())
    }
    
    async fn send_new_order_single(
        &mut self,
        symbol: &str,
        side: &str,
        qty: u32,
        price: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("📤 Sending New Order Single: {} {} {} @ {}", side, qty, symbol, price);
        
        let encoded = {
            let order_id = format!("ORDER-{}", self.seq_num);
            let price_str = format!("{:.2}", price);
            
            let mut buffer = Vec::new();
            let mut msg = self.encoder.start_message(b"FIX.4.4", &mut buffer, b"D"); // New Order Single
            
            msg.set(fix44::MSG_SEQ_NUM, self.seq_num);
            msg.set(fix44::CL_ORD_ID, order_id.as_str());
            msg.set(fix44::SYMBOL, symbol);
            msg.set(fix44::SIDE, match side {
                "BUY" => "1",
                "SELL" => "2",
                _ => "1",
            });
            msg.set(fix44::ORDER_QTY, qty);
            msg.set(fix44::ORD_TYPE, "2"); // Limit
            msg.set(fix44::PRICE, price_str.as_str());
            msg.set(fix44::TIME_IN_FORCE, "0"); // Day
            
            let (encoded, _) = msg.done();
            encoded.to_vec()
        };
        
        self.seq_num += 1;
        
        self.send_raw(&encoded).await?;
        
        println!("✅ Order sent");
        Ok(())
    }
    
    async fn send_raw(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        use tokio::io::AsyncWriteExt;
        self.stream.write_all(data).await?;
        Ok(())
    }
    
    async fn receive_message(&mut self) -> Result<Option<Message<&[u8]>>, Box<dyn std::error::Error>> {
        
        let mut temp_buf = [0u8; 1024];
        
        // Try to read some data (non-blocking)
        self.stream.readable().await?;
        
        match self.stream.try_read(&mut temp_buf) {
            Ok(0) => return Ok(None), // Connection closed
            Ok(n) => {
                self.buffer.extend_from_slice(&temp_buf[..n]);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                return Ok(None); // No data available
            }
            Err(e) => return Err(Box::new(e)),
        }
        
        // Try to decode a complete message
        if let Ok(msg) = self.decoder.decode(&self.buffer) {
            self.buffer.clear(); // Message successfully parsed
            Ok(Some(msg))
        } else {
            Ok(None) // Incomplete message, need more data
        }
    }
}

fn handle_incoming_message(msg: &Message<&[u8]>) -> Result<(), Box<dyn std::error::Error>> {
    let msg_type = msg.get::<&[u8]>(fix44::MSG_TYPE)?;
    let sender = msg.get::<&[u8]>(fix44::SENDER_COMP_ID)?;
    
    match msg_type {
        b"A" => {
            println!("📨 Received Logon from {:?}", std::str::from_utf8(sender)?);
            let heartbeat_int = msg.get::<u32>(fix44::HEART_BT_INT)?;
            println!("   Heartbeat interval: {} seconds", heartbeat_int);
        }
        b"0" => {
            println!("💓 Received Heartbeat from {:?}", std::str::from_utf8(sender)?);
        }
        b"1" => {
            println!("🧪 Received Test Request from {:?}", std::str::from_utf8(sender)?);
            if let Ok(test_req_id) = msg.get::<&[u8]>(fix44::TEST_REQ_ID) {
                println!("   Test Request ID: {:?}", std::str::from_utf8(test_req_id)?);
            }
        }
        b"8" => {
            println!("📊 Received Execution Report from {:?}", std::str::from_utf8(sender)?);
            if let Ok(order_id) = msg.get::<&[u8]>(fix44::CL_ORD_ID) {
                println!("   Order ID: {:?}", std::str::from_utf8(order_id)?);
            }
            if let Ok(exec_type) = msg.get::<&[u8]>(fix44::EXEC_TYPE) {
                let exec_type_str = match exec_type {
                    b"0" => "New",
                    b"1" => "Partial Fill", 
                    b"2" => "Fill",
                    b"4" => "Canceled",
                    b"8" => "Rejected",
                    _ => "Unknown",
                };
                println!("   Execution Type: {}", exec_type_str);
            }
        }
        b"5" => {
            println!("👋 Received Logout from {:?}", std::str::from_utf8(sender)?);
            if let Ok(text) = msg.get::<&[u8]>(fix44::TEXT) {
                println!("   Reason: {:?}", std::str::from_utf8(text)?);
            }
        }
        _ => {
            println!("📩 Received message type {:?} from {:?}", 
                std::str::from_utf8(msg_type)?, 
                std::str::from_utf8(sender)?);
        }
    }
    
    Ok(())
}

/// Mock FIX server for testing
async fn run_mock_fix_server() {
    println!("🚀 Starting mock FIX server on 127.0.0.1:8080");
    
    let listener = match TcpListener::bind("127.0.0.1:8080").await {
        Ok(listener) => listener,
        Err(e) => {
            println!("Failed to bind server: {}", e);
            return;
        }
    };
    
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            handle_client_connection(stream).await;
        });
    }
}

async fn handle_client_connection(mut stream: TcpStream) {
    println!("👤 Client connected to mock server");
    
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    
    let mut buffer = [0u8; 1024];
    let mut decoder = Decoder::new(Dictionary::fix44());
    let mut encoder = Encoder::new();
    let mut seq_num = 1u32;
    
    // Simple server loop
    for _ in 0..5 { // Handle a few messages then disconnect
        if let Ok(n) = stream.read(&mut buffer).await {
            if n == 0 { break; } // Client disconnected
            
            // Try to parse incoming message
            if let Ok(msg) = decoder.decode(&buffer[..n]) {
                let msg_type = msg.get::<&[u8]>(fix44::MSG_TYPE).unwrap_or(b"?");
                println!("🔍 Server received: {:?}", std::str::from_utf8(msg_type).unwrap_or("?"));
                
                match msg_type {
                    b"A" => { // Logon - respond with Logon acknowledgment
                        let mut response_buf = Vec::new();
                        let mut response = encoder.start_message(b"FIX.4.4", &mut response_buf, b"A");
                        response.set(fix44::MSG_SEQ_NUM, seq_num);
                        response.set(fix44::SENDER_COMP_ID, "SERVER");  
                        response.set(fix44::TARGET_COMP_ID, "CLIENT");
                        response.set(fix44::ENCRYPT_METHOD, "0");
                        response.set(fix44::HEART_BT_INT, 30);
                        let (encoded, _) = response.done();
                        seq_num += 1;
                        
                        let _ = stream.write_all(encoded).await;
                        println!("✅ Server sent Logon response");
                    }
                    b"D" => { // New Order Single - respond with Execution Report
                        let mut response_buf = Vec::new();
                        let mut response = encoder.start_message(b"FIX.4.4", &mut response_buf, b"8");
                        response.set(fix44::MSG_SEQ_NUM, seq_num);
                        response.set(fix44::SENDER_COMP_ID, "SERVER");
                        response.set(fix44::TARGET_COMP_ID, "CLIENT");
                        
                        if let Ok(order_id) = msg.get::<&[u8]>(fix44::CL_ORD_ID) {
                            response.set(fix44::CL_ORD_ID, std::str::from_utf8(order_id).unwrap());
                        }
                        
                        let exec_id = format!("EXEC-{}", seq_num);
                        response.set(fix44::EXEC_ID, exec_id.as_str());
                        response.set(fix44::EXEC_TYPE, "0"); // New
                        response.set(fix44::ORD_STATUS, "0"); // New
                        
                        let (encoded, _) = response.done();
                        seq_num += 1;
                        
                        let _ = stream.write_all(encoded).await;
                        println!("✅ Server sent Execution Report");
                    }
                    b"5" => { // Logout
                        println!("👋 Server received Logout, closing connection");
                        break;
                    }
                    _ => {
                        // Send heartbeat for other message types
                        let mut hb_buf = Vec::new();
                        let mut heartbeat = encoder.start_message(b"FIX.4.4", &mut hb_buf, b"0");
                        heartbeat.set(fix44::MSG_SEQ_NUM, seq_num);
                        heartbeat.set(fix44::SENDER_COMP_ID, "SERVER");
                        heartbeat.set(fix44::TARGET_COMP_ID, "CLIENT");
                        let (encoded, _) = heartbeat.done();
                        seq_num += 1;
                        
                        let _ = stream.write_all(encoded).await;
                        println!("💓 Server sent Heartbeat");
                    }
                }
            }
        }
        
        // Small delay to simulate processing time
        sleep(Duration::from_millis(100)).await;
    }
    
    println!("🔚 Server closing client connection");
}