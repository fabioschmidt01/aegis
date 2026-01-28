import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { StatusCard } from "./components/StatusCard";
import { ControlPanel } from "./components/ControlPanel";
import { TrafficGraph } from "./components/TrafficGraph";
import { StealthPanel } from "./components/StealthPanel";
import { ActivityLog } from "./components/ActivityLog";
import { DonationModal } from "./components/DonationModal";
import { Shield, ChevronUp, ChevronDown, Heart } from "lucide-react";
import "./index.css";

interface SystemIdentity {
  mac: string;
  hostname: string;
}

function App() {
  const [isActive, setIsActive] = useState(false);
  const [ip, setIp] = useState<string | undefined>(undefined);
  const [country, setCountry] = useState<string | undefined>(undefined);
  const [ipv6, setIpv6] = useState<string | undefined>(undefined);
  const [identity, setIdentity] = useState<SystemIdentity>({ mac: "Loading...", hostname: "Loading..." });

  const [loading, setLoading] = useState(false);
  const [dataLoading, setDataLoading] = useState(false);
  const [logs, setLogs] = useState<string[]>([]);
  const [showLogs, setShowLogs] = useState(false);
  const [showDonation, setShowDonation] = useState(false);

  const addLog = (msg: string) => {
    setLogs(prev => [...prev, msg]);
  };

  useEffect(() => {
    addLog("Aegis System Initialized");

    const unlisten = listen<string>('app_log', (event) => {
      addLog(event.payload);
    });

    checkStatus();
    fetchIpData();

    return () => {
      unlisten.then(f => f());
    };
  }, []);

  useEffect(() => {
    const interval = setInterval(() => {
      fetchIpData();
    }, 10000);
    return () => clearInterval(interval);
  }, [isActive]);

  async function checkStatus() {
    try {
      const status = await invoke<boolean>("check_status");
      setIsActive(status);
    } catch (e) {
      console.error(e);
      addLog(`[ERR] Status check: ${e}`);
    }
  }

  async function fetchIpData() {
    setDataLoading(true);

    // 1. Fetch Local Identity (MAC, Hostname) from Backend
    try {
      const id = await invoke<SystemIdentity>("get_system_identity", { interface: "wlo1" });
      setIdentity(id);
    } catch (e) {
      console.warn("Failed to fetch identity", e);
    }

    // 2. Fetch Public IP (IPv4)
    try {
      const response = await fetch("https://ipapi.co/json/");
      const data = await response.json();
      setIp(data.ip);
      setCountry(data.country_name);
    } catch (error) {
      try {
        const fallback = await fetch("https://api.ipify.org?format=json");
        const data = await fallback.json();
        setIp(data.ip);
      } catch (e) {
        setIp("Offline");
      }
    }

    // 3. Fetch IPv6 (Should Fail if Protected)
    try {
      // Short timeout for IPv6 check to detect blocking
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), 3000);

      const v6response = await fetch("https://api6.ipify.org?format=json", {
        signal: controller.signal
      });
      clearTimeout(timeoutId);

      const v6data = await v6response.json();
      setIpv6(v6data.ip); // If we get here, IPv6 is LEAKING or available
    } catch (e) {
      setIpv6("Blocked"); // Ideal state for anonymity
    } finally {
      setDataLoading(false);
    }
  }

  async function toggleAnonsurf() {
    setLoading(true);
    try {
      if (isActive) {
        addLog("Stopping Services...");
        await invoke("stop_anonsurf");
        setIsActive(false);
        addLog("Shield Deactivated");
      } else {
        addLog("Initializing Shield...");
        await invoke("start_anonsurf");
        setIsActive(true);
        addLog("Shield Activated - Traffic Secured");
      }
      setTimeout(fetchIpData, 2000);
    } catch (e) {
      alert("Error: " + e);
      addLog(`[CRITICAL] ${e}`);
    } finally {
      setLoading(false);
    }
  }

  async function refreshIdentity() {
    setLoading(true);
    addLog("Cycling Identity...");
    try {
      await invoke("refresh_identity");
      addLog("New Identity Requested");
      setTimeout(fetchIpData, 3000);
    } catch (e) {
      addLog(`[ERR] Identity: ${e}`);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="min-h-screen bg-background font-sans flex flex-col items-center p-6 relative overflow-x-hidden selection:bg-indigo-500/30">

      {/* Background Ambience */}
      <div className="fixed top-[-20%] left-[-10%] w-[500px] h-[500px] bg-indigo-600/20 rounded-full blur-[120px] pointer-events-none" />
      <div className="fixed bottom-[-20%] right-[-10%] w-[500px] h-[500px] bg-purple-600/10 rounded-full blur-[120px] pointer-events-none" />

      <div className="w-full max-w-2xl z-10 flex flex-col space-y-6">

        {/* Header */}
        <header className="flex items-center justify-between mb-4">
          <div className="flex items-center space-x-3">
            <div className="p-2 bg-indigo-500/10 rounded-xl border border-indigo-500/20">
              <Shield className="w-6 h-6 text-indigo-400" />
            </div>
            <div>
              <h1 className="text-2xl font-bold tracking-tight text-white">Aegis</h1>
              <p className="text-xs text-slate-500 uppercase tracking-wider font-medium">Privacy Shield</p>
            </div>
          </div>
          <div className="text-right">
            <div className="text-[10px] text-slate-600 font-mono">v1.2.0-STABLE</div>
          </div>
        </header>

        {/* Donation Banner */}
        <div className="flex justify-end -mt-4 mb-2">
          <button
            onClick={() => setShowDonation(true)}
            className="flex items-center space-x-2 px-3 py-1.5 bg-gradient-to-r from-pink-500/10 to-purple-500/10 hover:from-pink-500/20 hover:to-purple-500/20 border border-pink-500/20 rounded-full transition-all group"
          >
            <Heart className="w-3.5 h-3.5 text-pink-400 group-hover:scale-110 transition-transform" />
            <span className="text-xs font-semibold text-pink-300">Donate & Support</span>
          </button>
        </div>

        <DonationModal isOpen={showDonation} onClose={() => setShowDonation(false)} />

        {/* Main Dashboard Grid */}
        <div className="grid grid-cols-1 gap-6">
          <StatusCard
            active={isActive}
            ip={ip}
            country={country}
            ipv6={ipv6}
            mac={identity.mac}
            hostname={identity.hostname}
            loading={dataLoading}
          />

          <ControlPanel
            active={isActive}
            onToggle={toggleAnonsurf}
            onRefresh={refreshIdentity}
            loading={loading}
          />
        </div>

        {/* Info & Metrics */}
        <div className="grid grid-cols-1 gap-6">
          <TrafficGraph />
          <StealthPanel onLog={addLog} />
        </div>

        {/* Collapsible Logs */}
        <div className="w-full glass-card overflow-hidden transition-all duration-300 border-t border-white/5">
          <button
            onClick={() => setShowLogs(!showLogs)}
            className="w-full p-3 bg-slate-800/30 hover:bg-slate-800/50 flex items-center justify-between text-xs text-slate-400 font-medium uppercase tracking-wider transition-colors"
          >
            <span>System Activity Log</span>
            {showLogs ? <ChevronUp className="w-4 h-4" /> : <ChevronDown className="w-4 h-4" />}
          </button>

          {showLogs && (
            <div className="p-0 border-t border-white/5">
              <ActivityLog logs={logs} />
            </div>
          )}
        </div>

      </div>
    </div>
  );
}

export default App;
