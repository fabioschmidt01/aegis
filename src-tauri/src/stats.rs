use std::fs;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

#[derive(Clone, serde::Serialize)]
pub struct TrafficStats {
    pub up_speed: u64, // Bytes/sec
    pub down_speed: u64, // Bytes/sec
}

pub fn start_traffic_monitor(app: AppHandle) {
    thread::spawn(move || {
        let mut last_rx = 0u64;
        let mut last_tx = 0u64;

        loop {
            // Read /proc/net/dev to get total bytes
            if let Ok((rx, tx)) = read_total_traffic() {
                if last_rx != 0 {
                    let down_speed = rx - last_rx;
                    let up_speed = tx - last_tx;

                    let stats = TrafficStats {
                        up_speed,
                        down_speed
                    };

                    // Emit to frontend (ignore errors if app closed)
                    let _ = app.emit("traffic_update", stats);
                }
                last_rx = rx;
                last_tx = tx;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn read_total_traffic() -> anyhow::Result<(u64, u64)> {
    let content = fs::read_to_string("/proc/net/dev")?;
    let mut total_rx = 0;
    let mut total_tx = 0;

    for line in content.lines().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 10 { continue; }
        // Format: interface: rx_bytes ... tx_bytes ...
        // part[0] is interface with colon or not (depends on parsing)
        // actually standard format:
        // Inter-|   Receive                                                |  Transmit
        //  face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
        //     lo: 88701888  110555    0    0    0     0          0         0 88701888  110555    0    0    0     0       0          0
        
        // parts[1] is rx_bytes
        // parts[9] is tx_bytes
        
        if let (Ok(rx), Ok(tx)) = (parts[1].parse::<u64>(), parts[9].parse::<u64>()) {
            total_rx += rx;
            total_tx += tx;
        }
    }
    
    Ok((total_rx, total_tx))
}
