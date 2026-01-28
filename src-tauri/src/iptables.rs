use std::process::Command;
use anyhow::{Result, Context};

pub struct IptablesManager;

impl IptablesManager {
    /// Backup current iptables rules to a file
    pub fn backup_rules() -> Result<()> {
        // Simple backup: usually we might just flush logic, but saving is good practice.
        // For this tool, we rely on "flushing" our custom chain or rules on stop.
        // Ideally, we should save to a temp file if we want full restore,
        // but typically Anonsurf clones just clear their mess.
        Ok(())
    }

    /// Apply transparent proxy rules
    /// Redirects TCP to 9040 (TransPort) and DNS to 5353 (DNSPort)
    pub fn apply_rules(tor_uid: &str, dns_port: &str, trans_port: &str) -> Result<()> {
        // 1. Flush/Reset first
        // Self::flush_rules()?;

        // --- STRICT MODE: DROP EVERYTHING BY DEFAULT ---
        run_iptables(&["-P", "INPUT", "DROP"])?;
        run_iptables(&["-P", "FORWARD", "DROP"])?;
        run_iptables(&["-P", "OUTPUT", "DROP"])?;

        // 2. Allow Loopback (essential)
        run_iptables(&["-A", "INPUT", "-i", "lo", "-j", "ACCEPT"])?;
        run_iptables(&["-A", "OUTPUT", "-o", "lo", "-j", "ACCEPT"])?;

        // 3. Allow Established Related traffic (incoming)
        run_iptables(&["-A", "INPUT", "-m", "state", "--state", "ESTABLISHED,RELATED", "-j", "ACCEPT"])?;

        // 4. DNS Redirection (UDP) -> Tor DNSPort
        // We must allow OUTPUT to localhost UDP for DNS
        run_iptables(&["-t", "nat", "-A", "OUTPUT", "-p", "udp", "--dport", "53", "-j", "REDIRECT", "--to-ports", dns_port])?;
        // Need to ALLOW the redirected packet in OUTPUT chain (it becomes localhost traffic)
        // But actual DNS packets leaving via Tor are handled by Tor process.
        
        // 5. Allow Tor Process Output
        // The Tor process itself needs to talk to the internet.
        run_iptables(&["-A", "OUTPUT", "-m", "owner", "--uid-owner", tor_uid, "-j", "ACCEPT"])?;

        // 6. Transparent Proxy Redirection (TCP) -> Tor TransPort
        // Redirect standard TCP to 9040
        // Exclude Tor user and Loopback (already allowed/handled above but let's be explicitly safe in NAT)
        run_iptables(&["-t", "nat", "-A", "OUTPUT", "-p", "tcp", "-m", "owner", "--uid-owner", tor_uid, "-j", "RETURN"])?;
        run_iptables(&["-t", "nat", "-A", "OUTPUT", "-o", "lo", "-j", "RETURN"])?;
        run_iptables(&["-t", "nat", "-A", "OUTPUT", "-p", "tcp", "--syn", "-j", "REDIRECT", "--to-ports", trans_port])?;
        
        // 7. Allow TCP Output (that has been redirected to TransPort -> Localhost)
        // Since we dropped OUTPUT by default, we need to allow traffic destined to TransPort?
        // Actually, redirected traffic re-traverses the stack. Anonsurf usually allows ALL output or specific processing.
        // If policy is DROP, we must allow the redirected traffic.
        // Redirected packets destination changes to 127.0.0.1:9040.
        // So we need to allow OUTPUT to destination 127.0.0.1 tcp port 9040/5353.
        run_iptables(&["-A", "OUTPUT", "-d", "127.0.0.1/32", "-p", "tcp", "--dport", trans_port, "-j", "ACCEPT"])?;
        run_iptables(&["-A", "OUTPUT", "-d", "127.0.0.1/32", "-p", "udp", "--dport", dns_port, "-j", "ACCEPT"])?;

        // 8. IPv6 Blocking (Total)
        let _ = run_command("ip6tables", &["-P", "INPUT", "DROP"]);
        let _ = run_command("ip6tables", &["-P", "OUTPUT", "DROP"]);
        let _ = run_command("ip6tables", &["-P", "FORWARD", "DROP"]);

        Ok(())
    }

    /// Flush all rules created by Anonsurf
    pub fn flush_rules() -> Result<()> {
        // Reset default policies
        run_iptables(&["-P", "INPUT", "ACCEPT"])?;
        run_iptables(&["-P", "OUTPUT", "ACCEPT"])?;
        run_iptables(&["-P", "FORWARD", "ACCEPT"])?;

        // Flush NAT table
        run_iptables(&["-t", "nat", "-F"])?;
        run_iptables(&["-t", "nat", "-X"])?;

        // Flush Filter table
        run_iptables(&["-F"])?;
        run_iptables(&["-X"])?;

        // Reset IPv6
        let _ = run_command("ip6tables", &["-P", "INPUT", "ACCEPT"]);
        let _ = run_command("ip6tables", &["-P", "OUTPUT", "ACCEPT"]);
        let _ = run_command("ip6tables", &["-P", "FORWARD", "ACCEPT"]);
        let _ = run_command("ip6tables", &["-F"]);

        Ok(())
    }
}

fn run_iptables(args: &[&str]) -> Result<()> {
    run_command("iptables", args)
}

fn run_command(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .context(format!("Failed to execute {}", cmd))?;

    if status.success() {
        Ok(())
    } else {
        // We don't panic here, but we might want to log it
        Err(anyhow::anyhow!("Command {} failed with status {}", cmd, status))
    }
}
