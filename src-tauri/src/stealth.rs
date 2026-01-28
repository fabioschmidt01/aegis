use anyhow::{Result, Context};
use std::process::Command;

/// Changes the MAC address of the default interface
pub fn spoof_mac(interface: &str) -> Result<()> {
    // 1. Bring interface DOWN
    Command::new("ip")
        .args(&["link", "set", interface, "down"])
        .status()
        .context("Failed to bring interface down")?;

    // 2. Change MAC
    // macchanger -r <interface>
    Command::new("macchanger")
        .args(&["-r", interface])
        .status()
        .context("Failed to change MAC address")?;

    // 3. Bring interface UP
    Command::new("ip")
        .args(&["link", "set", interface, "up"])
        .status()
        .context("Failed to bring interface up")?;
    
    Ok(())
}

/// Restores the MAC address to permanent
/// Restores the MAC address to permanent
pub fn restore_mac(interface: &str) -> Result<()> {
    Command::new("ip")
        .args(&["link", "set", interface, "down"])
        .status()?;

    // macchanger -p <interface>
    Command::new("macchanger")
        .args(&["-p", interface])
        .status()?;

    Command::new("ip")
        .args(&["link", "set", interface, "up"])
        .status()?;
    
    Ok(())
}

/// Randomizes the system hostname
pub fn randomize_hostname() -> Result<String> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_id: u32 = rng.gen_range(1000..9999);
    let new_hostname = format!("anon-{}", random_id);

    Command::new("hostnamectl")
        .args(&["set-hostname", &new_hostname])
        .status()
        .context("Failed to set hostname")?;
    
    Ok(new_hostname)
}

/// Clears system RAM caches (Pandora style)
pub fn wipe_ram() -> Result<()> {
    // sync; echo 3 > /proc/sys/vm/drop_caches
    // This requires root and shell redirection, easiest via sh -c
    Command::new("sh")
        .arg("-c")
        .arg("sync; echo 3 > /proc/sys/vm/drop_caches")
        .status()
        .context("Failed to wipe RAM")?;
    
    
    Ok(())
}

/// Sets system timezone to UTC
pub fn set_utc_timezone() -> Result<()> {
    // ln -sf /usr/share/zoneinfo/UTC /etc/localtime
    Command::new("ln")
        .args(&["-sf", "/usr/share/zoneinfo/UTC", "/etc/localtime"])
        .status()
        .context("Failed to set UTC timezone")?;
    Ok(())
}

/// Cleans system logs (Use with caution)
pub fn clean_logs() -> Result<()> {
    // Truncate logs
    let logs = ["/var/log/syslog", "/var/log/auth.log", "/var/log/kern.log"];
    
    for log in logs.iter() {
        // truncate -s 0 <log>
        let _ = Command::new("truncate")
            .args(&["-s", "0", log])
            .status();
    }
    // Also history? history is shell specific.
    Ok(())
}

/// Reads the MAC address of the interface
pub fn get_mac(interface: &str) -> String {
    // /sys/class/net/<interface>/address
    use std::fs;
    let path = format!("/sys/class/net/{}/address", interface);
    fs::read_to_string(path).unwrap_or_else(|_| "unknown".to_string()).trim().to_string()
}

/// Gets the system hostname
pub fn get_hostname() -> String {
    // /etc/hostname or hostname command
    use std::fs;
    fs::read_to_string("/etc/hostname").unwrap_or_else(|_| "unknown".to_string()).trim().to_string()
}
