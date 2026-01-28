use crate::iptables::IptablesManager;
use anyhow::{Result, Context};
use std::process::Command;
use std::sync::Mutex;

// Constants
const TOR_UID: &str = "debian-tor"; // Default on Debian/Ubuntu
const TRANS_PORT: &str = "9040";
const DNS_PORT: &str = "5353";

pub struct AnonsurfState {
    pub is_active: Mutex<bool>,
}

impl AnonsurfState {
    pub fn new() -> Self {
        Self {
            is_active: Mutex::new(false),
        }
    }
}

pub struct Anonsurf;

impl Anonsurf {
    pub fn start(app: tauri::AppHandle) -> Result<()> {
        Self::emit_log(&app, "Starting Aegis Shield...");

        // 1. Configure Tor if needed
        Self::check_and_configure_tor(&app)?;

        // 2. Start Tor Service
        Self::start_tor_service(&app)?;

        // 3. Backup and Apply Iptables
        Self::emit_log(&app, "Applying Firewall Rules...");
        IptablesManager::backup_rules()?;
        IptablesManager::apply_rules(TOR_UID, DNS_PORT, TRANS_PORT)?;

        Self::emit_log(&app, "Aegis Shield Activated Successfully.");
        Ok(())
    }

    pub fn stop() -> Result<()> {
        println!("Stopping Anonsurf...");

        // 1. Flush Rules
        IptablesManager::flush_rules()?;

        // 2. (Optional) Stop Tor Service? Usually we keep it running.
        // Self::stop_tor_service()?;

        println!("Anonsurf stopped.");
        Ok(())
    }

    pub fn new_identity() -> Result<()> {
        println!("Requesting new identity...");
        // Use netcat or specialized crate to send signal to 9051
        // Simpler way: reload service (brute force) or use pidof
        // Correct way is: echo -e 'AUTHENTICATE ""\r\nsignal NEWNYM\r\nQUIT' | nc 127.0.0.1 9051
        // But we might not have netcat.
        // Let's rely on systemctl reload tor, which sends SIGHUP, causing newnym? No, SIGHUP reloads config.
        // SIGUSR1 logs stats.
        
        // Let's try sending the signal via `killall -HUP tor` (Reload config) -> Doesn't trigger NEWNYM for circuits directly but often good enough.
        // Better: `kill -SIGINT $(pidof tor)` is stop.
        
        // If we want real NEWNYM without netcat, we might need a TcpStream in Rust.
        Self::send_tor_signal("NEWNYM")?;

        Ok(())
    }

    fn start_tor_service(app: &tauri::AppHandle) -> Result<()> {
        Self::emit_log(app, "Restarting Tor Service...");
        let status = Command::new("systemctl")
            .args(&["restart", "tor"]) // Restart to apply new config
            .status()
            .context("Failed to start Tor service")?;
        
        if !status.success() {
             return Err(anyhow::anyhow!("Failed to start Tor service"));
        }
        Ok(())
    }

    fn emit_log(app: &tauri::AppHandle, msg: &str) {
        use tauri::Emitter;
        let _ = app.emit("app_log", msg);
    }

    fn send_tor_signal(signal: &str) -> Result<()> {
        use std::io::Write;
        use std::net::TcpStream;

        // Try to connect to Control Port 9051
        let mut stream = TcpStream::connect("127.0.0.1:9051")
            .context("Could not connect to Tor Control Port. Is Tor running and ControlPort 9051 enabled?")?;
        
        // Authenticate (assume cookie auth or no auth for localhost if configured)
        // Try passwordless first
        stream.write_all(b"AUTHENTICATE \"\"\r\n")?;
        
        // Send Signal
        let cmd = format!("SIGNAL {}\r\n", signal);
        stream.write_all(cmd.as_bytes())?;
        
        // Quit
        stream.write_all(b"QUIT\r\n")?;

        Ok(())
    }

    fn check_and_configure_tor(app: &tauri::AppHandle) -> Result<()> {
        use std::fs;
        use std::io::Write;

        Self::emit_log(app, "Checking Tor Configuration...");
        let torrc_path = "/etc/tor/torrc";
        let content = fs::read_to_string(torrc_path).context("Failed to read torrc")?;

        // Check if our configuration exists
        if content.contains("TransPort 9040") {
            Self::emit_log(app, "Tor is already configured correctly.");
            return Ok(());
        }

        Self::emit_log(app, "Tor configuration incomplete. Fixing...");
        println!("Configuring Tor for Transparent Proxy...");

        let config_block = r#"
# --- Added by Aegis Privacy Shield ---
VirtualAddrNetworkIPv4 10.192.0.0/10
AutomapHostsOnResolve 1
TransPort 9040 IsolateClientAddr IsolateClientProtocol IsolateDestAddr IsolateDestPort
DNSPort 5353
ControlPort 9051
CookieAuthentication 0
# -----------------------------------
"#;

        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(torrc_path)
            .context("Failed to open torrc for appending")?;
        
        writeln!(file, "{}", config_block)?;
        
        Self::emit_log(app, "Tor configuration updated. Added TransPort/DNSPort.");
        println!("Tor configuration updated.");
        
        Ok(())
    }
}
