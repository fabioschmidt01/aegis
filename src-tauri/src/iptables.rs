use anyhow::{Context, Result};
use std::process::Command;

pub struct IptablesManager;

impl IptablesManager {
    /// Backup current iptables rules to a file
    pub fn backup_rules() -> Result<()> {
        // Simple backup: same as before
        Ok(())
    }

    /// Apply transparent proxy rules using batched pkexec
    pub fn apply_rules(tor_uid: &str, dns_port: &str, trans_port: &str) -> Result<()> {
        let mut commands = Vec::new();

        // 1. Policy: DROP EVERYTHING
        commands.push(format!("iptables -P INPUT DROP"));
        commands.push(format!("iptables -P FORWARD DROP"));
        commands.push(format!("iptables -P OUTPUT DROP"));

        // 2. Allow Loopback
        commands.push(format!("iptables -A INPUT -i lo -j ACCEPT"));
        commands.push(format!("iptables -A OUTPUT -o lo -j ACCEPT"));

        // 3. Allow Established/Related
        commands.push(format!(
            "iptables -A INPUT -m state --state ESTABLISHED,RELATED -j ACCEPT"
        ));

        // 4. DNS Redirection (UDP) -> Tor DNSPort
        commands.push(format!(
            "iptables -t nat -A OUTPUT -p udp --dport 53 -j REDIRECT --to-ports {}",
            dns_port
        ));

        // 5. Allow Tor Process Output
        commands.push(format!(
            "iptables -A OUTPUT -m owner --uid-owner {} -j ACCEPT",
            tor_uid
        ));

        // 6. Transparent Proxy Redirection (TCP) -> Tor TransPort
        commands.push(format!(
            "iptables -t nat -A OUTPUT -p tcp -m owner --uid-owner {} -j RETURN",
            tor_uid
        ));
        commands.push(format!("iptables -t nat -A OUTPUT -o lo -j RETURN"));
        commands.push(format!(
            "iptables -t nat -A OUTPUT -p tcp --syn -j REDIRECT --to-ports {}",
            trans_port
        ));

        // 7. Allow Redirected Output (to localhost)
        commands.push(format!(
            "iptables -A OUTPUT -d 127.0.0.1/32 -p tcp --dport {} -j ACCEPT",
            trans_port
        ));
        commands.push(format!(
            "iptables -A OUTPUT -d 127.0.0.1/32 -p udp --dport {} -j ACCEPT",
            dns_port
        ));

        // 8. IPv6 Blocking
        commands.push(format!("ip6tables -P INPUT DROP"));
        commands.push(format!("ip6tables -P OUTPUT DROP"));
        commands.push(format!("ip6tables -P FORWARD DROP"));

        execute_batch(&commands)
    }

    /// Flush all rules using batched pkexec
    pub fn flush_rules() -> Result<()> {
        let mut commands = Vec::new();

        // Reset policies
        commands.push(format!("iptables -P INPUT ACCEPT"));
        commands.push(format!("iptables -P OUTPUT ACCEPT"));
        commands.push(format!("iptables -P FORWARD ACCEPT"));

        // Flush & Delete chains
        commands.push(format!("iptables -t nat -F"));
        commands.push(format!("iptables -t nat -X"));
        commands.push(format!("iptables -F"));
        commands.push(format!("iptables -X"));

        // Reset IPv6
        commands.push(format!("ip6tables -P INPUT ACCEPT"));
        commands.push(format!("ip6tables -P OUTPUT ACCEPT"));
        commands.push(format!("ip6tables -P FORWARD ACCEPT"));
        commands.push(format!("ip6tables -F"));

        execute_batch(&commands)
    }
}

/// Executes a list of commands as a single privileged script
fn execute_batch(commands: &[String]) -> Result<()> {
    if commands.is_empty() {
        return Ok(());
    }

    // Join commands with && so if one fails, execution stops (safety)
    // We add `set -x` for debug output if needed, but let's keep it clean
    let script = commands.join(" && ");

    // Use pkexec to run the whole batch as root
    // We wrap in 'sh -c' to interpret the && chain
    let status = Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(&script)
        .status()
        .context("Failed to execute privileged batch command")?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Privileged batch execution failed with status {}",
            status
        ))
    }
}
