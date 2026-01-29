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

            {/* Active Defense Configuration */}
            <div className="col-span-1 md:col-span-2 glass-card p-6 border-t border-white/5 mt-2">
                <div className="flex items-center space-x-3 mb-4">
                    <div className="p-2 bg-red-500/10 rounded-lg">
                        <Power className="w-5 h-5 text-red-500" />
                    </div>
                    <div>
                        <h3 className="text-sm font-semibold text-slate-200 uppercase tracking-wider">Active Defense Response</h3>
                        <p className="text-xs text-slate-500">Message to send when blocking tracking attempts</p>
                    </div>
                </div>

                <form
                    onSubmit={(e) => {
                        e.preventDefault();
                        const formData = new FormData(e.currentTarget);
                        const msg = formData.get('message') as string;
                        if (msg) {
                            import("@tauri-apps/api/core").then(({ invoke }) => {
                                invoke("set_defense_message", { message: msg })
                                    .then(() => alert("Message Updated"))
                                    .catch(err => alert("Failed: " + err));
                            });
                        }
                    }}
                    className="flex gap-2"
                >
                    <input
                        name="message"
                        type="text"
                        defaultValue="ACCESS DENIED: Tracking Attempt Detected."
                        placeholder="Enter custom response message..."
                        className="flex-1 bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-200 focus:outline-none focus:border-indigo-500 transition-colors"
                    />
                    <button
                        type="submit"
                        disabled={loading}
                        className="px-4 py-2 bg-slate-700 hover:bg-indigo-600 text-white text-sm font-medium rounded-lg transition-colors disabled:opacity-50"
                    >
                        Update
                    </button>
                </form>
            </div>
        </div>
    );
}
