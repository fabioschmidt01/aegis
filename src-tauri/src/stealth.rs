use anyhow::{Context, Result};
use std::process::Command;

/// Changes the MAC address of the default interface
pub fn spoof_mac(interface: &str) -> Result<()> {
    // We need to batch these commands to avoid multiple password prompts
    let commands = vec![
        format!("ip link set {} down", interface),
        format!("macchanger -r {}", interface),
        format!("ip link set {} up", interface),
    ];

    run_privileged_batch(&commands).context("Failed to spoof MAC address")
}

/// Restores the MAC address to permanent
pub fn restore_mac(interface: &str) -> Result<()> {
    let commands = vec![
        format!("ip link set {} down", interface),
        format!("macchanger -p {}", interface),
        format!("ip link set {} up", interface),
    ];

    run_privileged_batch(&commands).context("Failed to restore MAC address")
}

/// Randomizes the system hostname
pub fn randomize_hostname() -> Result<String> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_id: u32 = rng.gen_range(1000..9999);
    let new_hostname = format!("anon-{}", random_id);

    // hostnamectl needs authentication/root usually
    // We can run it directly via pkexec
    run_privileged("hostnamectl", &["set-hostname", &new_hostname])
        .context("Failed to set hostname")?;

    Ok(new_hostname)
}

/// Clears system RAM caches (Pandora style)
pub fn wipe_ram() -> Result<()> {
    // sync; echo 3 > /proc/sys/vm/drop_caches
    // executing complex shell command with redirection needs sh -c wrapped in pkexec
    let cmd_str = "sync; echo 3 > /proc/sys/vm/drop_caches";

    // We can reuse the batch logic for a single complex command string or run directly
    run_privileged("sh", &["-c", cmd_str]).context("Failed to wipe RAM")
}

/// Sets system timezone to UTC
pub fn set_utc_timezone() -> Result<()> {
    // ln -sf /usr/share/zoneinfo/UTC /etc/localtime
    run_privileged("ln", &["-sf", "/usr/share/zoneinfo/UTC", "/etc/localtime"])
        .context("Failed to set UTC timezone")
}

/// Cleans system logs (Use with caution)
pub fn clean_logs() -> Result<()> {
    // Truncate logs
    let logs = ["/var/log/syslog", "/var/log/auth.log", "/var/log/kern.log"];

    let mut commands = Vec::new();
    for log in logs.iter() {
        commands.push(format!("truncate -s 0 {}", log));
    }

    run_privileged_batch(&commands).context("Failed to clean system logs")
}

/// Reads the MAC address of the interface (No root needed usually)
pub fn get_mac(interface: &str) -> String {
    use std::fs;
    let path = format!("/sys/class/net/{}/address", interface);
    fs::read_to_string(path)
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string()
}

/// Gets the system hostname (No root needed usually)
pub fn get_hostname() -> String {
    use std::fs;
    fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string()
}

// --- Helpers ---

fn run_privileged(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new("pkexec")
        .arg(cmd)
        .args(args)
        .status()
        .context(format!("Failed to execute privileged command: {}", cmd))?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Command {} failed with status {}",
            cmd,
            status
        ))
    }
}

fn run_privileged_batch(commands: &[String]) -> Result<()> {
    if commands.is_empty() {
        return Ok(());
    }

    let script = commands.join(" && ");

    let status = Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(&script)
        .status()
        .context("Failed to execute privileged batch")?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Privileged batch failed with status {}",
            status
        ))
    }
}
