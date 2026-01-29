use crate::geoip::GeoIpManager;
use anyhow::Result;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Emitter, Manager};

pub struct HoneypotState {
    pub is_active: Mutex<bool>,
    pub custom_message: Mutex<String>,
}

impl HoneypotState {
    pub fn new() -> Self {
        Self {
            is_active: Mutex::new(false),
            custom_message: Mutex::new("ACCESS DENIED: Tracking Attempt Detected.".to_string()),
        }
    }
}

pub fn start_honeypot_listener(app: AppHandle, geoip: Arc<GeoIpManager>) {
    thread::spawn(move || {
        // Listen on a high port for now.
        // In production, we'd use iptables to redirect port 80/443 traffic here.
        let listener = match TcpListener::bind("0.0.0.0:7878") {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Failed to bind Honeypot listener: {}", e);
                return;
            }
        };

        println!("Honeypot active on port 7878");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let app_handle = app.clone();
                    let geoip_handle = geoip.clone();

                    thread::spawn(move || {
                        if let Err(e) = handle_connection(stream, app_handle, geoip_handle) {
                            eprintln!("Error handling honeypot connection: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    });
}

fn handle_connection(
    mut stream: TcpStream,
    app: AppHandle,
    geoip: Arc<GeoIpManager>,
) -> Result<()> {
    let peer_addr = stream.peer_addr()?;
    let ip = match peer_addr.ip() {
        std::net::IpAddr::V4(ipv4) => ipv4,
        _ => return Ok(()), // Ignore IPv6 for now or handle later
    };

    // Check if target
    if geoip.is_target(ip) {
        println!("TRACKING DETECTED from: {}", ip);

        // 1. Get Message
        let state = app.state::<HoneypotState>();
        let msg = state.custom_message.lock().unwrap().clone();

        // 2. Send Message
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n{}", msg);
        stream.write_all(response.as_bytes())?;

        // 3. Log to Frontend
        let _ = app.emit(
            "security_alert",
            format!("Blocked tracking attempt from Israel ({})", ip),
        );
    } else {
        // Optional: Just close or pretend to be dead
        // stream.shutdown(std::net::Shutdown::Both)?;
    }

    Ok(())
}
