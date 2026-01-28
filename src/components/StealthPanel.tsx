import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Fingerprint, Eraser, Ghost, Clock, FileWarning, Check, RotateCcw } from "lucide-react";
import { clsx } from 'clsx';

interface StealthPanelProps {
    onLog: (msg: string) => void;
}

export function StealthPanel({ onLog }: StealthPanelProps) {
    const [loading, setLoading] = useState(false);
    const [activeModules, setActiveModules] = useState<Record<string, boolean>>({});

    const handleAction = async (label: string, command: string, id: string, args: Record<string, unknown> = {}) => {
        setLoading(true);
        onLog(`[REQ] ${label}...`);
        try {
            await invoke<string>(command, args);
            onLog(`[OK] ${label} Applied`);
            // Toggle "active" state visually for some feedback (pseudo-state since these are often one-off actions)
            setActiveModules(prev => ({ ...prev, [id]: true }));
            setTimeout(() => setActiveModules(prev => ({ ...prev, [id]: false })), 2000);
        } catch (e) {
            onLog(`[ERR] ${e}`);
        } finally {
            setLoading(false);
        }
    };

    const StealthButton = ({ icon: Icon, label, id, command, args = {}, color = "indigo" }: any) => {
        const isSuccess = activeModules[id];

        return (
            <button
                onClick={() => handleAction(label, command, id, args)}
                disabled={loading}
                className="group relative flex flex-col items-center justify-center p-4 glass-button hover:bg-slate-800/50 transition-all border border-white/5"
            >
                <div className={clsx(
                    "mb-2 transition-all duration-300 transform group-hover:scale-110",
                    isSuccess ? "text-green-400" : `text-${color}-400`
                )}>
                    {isSuccess ? <Check className="w-5 h-5" /> : <Icon className="w-5 h-5" />}
                </div>
                <span className="text-[10px] font-medium text-slate-400 uppercase tracking-wider group-hover:text-slate-200 transition-colors">
                    {label}
                </span>
            </button>
        );
    };

    return (
        <div className="space-y-3">
            <div className="flex items-center space-x-2 px-1">
                <Ghost className="w-4 h-4 text-slate-500" />
                <h3 className="text-xs font-bold text-slate-500 uppercase tracking-widest">Stealth Capabilities</h3>
            </div>
            <div className="grid grid-cols-3 sm:grid-cols-6 gap-3">
                <StealthButton icon={Fingerprint} label="Spoof MAC" id="mac" command="spoof_mac" args={{ interface: "wlo1" }} color="purple" />
                <StealthButton icon={RotateCcw} label="Reset MAC" id="restore_mac" command="restore_mac" args={{ interface: "wlo1" }} color="pink" />
                <StealthButton icon={Ghost} label="Hostname" id="host" command="randomize_hostname" color="blue" />
                <StealthButton icon={Clock} label="UTC Time" id="time" command="set_utc" color="yellow" />
                <StealthButton icon={Eraser} label="Wipe RAM" id="ram" command="wipe_ram" color="red" />
                <StealthButton icon={FileWarning} label="Clean Logs" id="logs" command="clean_logs" color="orange" />
            </div>
        </div>
    );
}
