use anyhow::Result;
use ipnet::Ipv4Net;
use reqwest::blocking::Client;
use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

const ISRAEL_CIDR_URL: &str =
    "https://raw.githubusercontent.com/herrbischoff/country-ip-blocks/master/ipv4/il.cidr";
const CACHE_FILE: &str = "/tmp/aegis_il_cidr.txt"; // In prod, use AppData
const UPDATE_INTERVAL: u64 = 86400; // 24 hours

pub struct GeoIpManager {
    cidrs: Arc<Mutex<Vec<Ipv4Net>>>,
    last_update: Arc<Mutex<SystemTime>>,
}

impl GeoIpManager {
    pub fn new() -> Self {
        Self {
            cidrs: Arc::new(Mutex::new(Vec::new())),
            last_update: Arc::new(Mutex::new(SystemTime::UNIX_EPOCH)),
        }
    }

    pub fn init(&self) -> Result<()> {
        // Try to load from cache first
        if let Ok(content) = std::fs::read_to_string(CACHE_FILE) {
            self.parse_and_update(&content);
            println!("Loaded GeoIP data from cache.");
        }

        // Check if we need update
        self.update_if_needed();
        Ok(())
    }

    fn update_if_needed(&self) {
        let cidrs = self.cidrs.clone();
        let last_update = self.last_update.clone();

        std::thread::spawn(move || {
            let should_update = {
                let last = last_update.lock().unwrap();
                if let Ok(elapsed) = last.elapsed() {
                    elapsed.as_secs() > UPDATE_INTERVAL
                } else {
                    true
                }
            };

            if should_update {
                println!("Fetching fresh GeoIP data for Israel...");
                let client = Client::builder()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .unwrap();
                if let Ok(resp) = client.get(ISRAEL_CIDR_URL).send() {
                    if let Ok(text) = resp.text() {
                        // Cache it
                        let _ = std::fs::write(CACHE_FILE, &text);

                        // Parse
                        let mut list = cidrs.lock().unwrap();
                        list.clear();
                        for line in text.lines() {
                            if let Ok(net) = line.trim().parse::<Ipv4Net>() {
                                list.push(net);
                            }
                        }
                        *last_update.lock().unwrap() = SystemTime::now();
                        println!("GeoIP data updated. {} prefixes loaded.", list.len());
                    }
                }
            }
        });
    }

    fn parse_and_update(&self, content: &str) {
        let mut list = self.cidrs.lock().unwrap();
        list.clear();
        for line in content.lines() {
            if let Ok(net) = line.trim().parse::<Ipv4Net>() {
                list.push(net);
            }
        }
    }

    pub fn is_target(&self, ip: Ipv4Addr) -> bool {
        let list = self.cidrs.lock().unwrap();
        for net in list.iter() {
            if net.contains(&ip) {
                return true;
            }
        }
        false
    }
}
