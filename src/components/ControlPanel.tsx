import { Power, RefreshCw } from "lucide-react";
import { clsx } from 'clsx';

interface ControlPanelProps {
    active: boolean;
    loading: boolean;
    onToggle: () => void;
    onRefresh: () => void;
}

export function ControlPanel({ active, loading, onToggle, onRefresh }: ControlPanelProps) {
    return (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 w-full">
            {/* Main Toggle Button */}
            <button
                onClick={onToggle}
                disabled={loading}
                className={clsx(
                    "relative group overflow-hidden rounded-2xl p-6 transition-all duration-300 border",
                    active
                        ? "bg-indigo-600 border-indigo-500 hover:bg-indigo-500 shadow-[0_0_20px_rgba(99,102,241,0.3)]"
                        : "bg-surface/50 border-white/5 hover:bg-surface/80 hover:border-white/10"
                )}
            >
                <div className="absolute inset-0 bg-noise opacity-10"></div>
                <div className="relative z-10 flex flex-col items-center justify-center space-y-3">
                    <div className={clsx(
                        "p-4 rounded-full transition-all duration-500",
                        active ? "bg-white/20" : "bg-slate-800"
                    )}>
                        <Power className={clsx("w-8 h-8", active ? "text-white" : "text-slate-400")} />
                    </div>
                    <div className="text-center">
                        <div className={clsx("font-bold text-lg", active ? "text-white" : "text-slate-200")}>
                            {active ? "Deactivate Shield" : "Activate Shield"}
                        </div>
                        <div className={clsx("text-xs opacity-70", active ? "text-indigo-100" : "text-slate-500")}>
                            {active ? "Disconnect from Tor Network" : "Route traffic via Tor"}
                        </div>
                    </div>
                </div>
            </button>

            {/* Refresh Identity Button */}
            <button
                onClick={onRefresh}
                disabled={loading}
                className="glass-card p-6 flex flex-col items-center justify-center space-y-3 hover:-translate-y-1 transition-transform cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed group"
            >
                <div className="p-4 rounded-full bg-slate-800/50 group-hover:bg-indigo-500/20 transition-colors">
                    <RefreshCw className={clsx("w-8 h-8 text-slate-400 group-hover:text-indigo-400 transition-colors", loading && "animate-spin")} />
                </div>
                <div className="text-center">
                    <div className="font-bold text-lg text-slate-200 group-hover:text-indigo-300 transition-colors">New Identity</div>
                    <div className="text-xs text-slate-500 group-hover:text-slate-400 transition-colors">Request new Tor Circuit</div>
                </div>
            </button>
        </div>
    );
}
