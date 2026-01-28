import { Shield, ShieldAlert, Globe, Activity } from "lucide-react";
import { clsx } from 'clsx';


interface StatusCardProps {
    active: boolean;
    ip?: string;
    country?: string;
    ipv6?: string;
    mac?: string;
    hostname?: string;
    loading: boolean;
}

export function StatusCard({ active, ip, country, ipv6, mac, hostname, loading }: StatusCardProps) {
    return (
        <div className="relative w-full">
            {/* Background Glow */}
            <div className={clsx(
                "absolute inset-0 bg-gradient-to-r blur-3xl opacity-20 transition-all duration-700",
                active ? "from-indigo-500 to-purple-600" : "from-slate-700 to-gray-800"
            )} />

            <div className="relative z-10 glass-card p-6 overflow-hidden">
                <div className="flex items-start justify-between">
                    <div className="space-y-1">
                        <h2 className="text-sm font-medium text-slate-400 uppercase tracking-widest">System Status</h2>
                        <div className="flex items-center space-x-3">
                            <div className={clsx(
                                "p-2 rounded-lg transition-colors duration-500",
                                active ? "bg-indigo-500/20 text-indigo-400" : "bg-red-500/10 text-red-500"
                            )}>
                                {active ? <Shield className="w-6 h-6" /> : <ShieldAlert className="w-6 h-6" />}
                            </div>
                            <div>
                                <span className={clsx(
                                    "text-2xl font-bold tracking-tight block",
                                    active ? "text-white" : "text-slate-300"
                                )}>
                                    {active ? "Protected" : "Exposed"}
                                </span>
                                <span className="text-xs text-slate-500">
                                    {active ? "Traffic is encrypted via Tor" : "Your real IP is visible"}
                                </span>
                            </div>
                        </div>
                    </div>

                    {/* Connection Ring Animation */}
                    <div className="relative flex items-center justify-center w-12 h-12">
                        {active && (
                            <>
                                <span className="absolute w-full h-full rounded-full border border-indigo-500/30 animate-[ping_3s_linear_infinite]" />
                                <span className="absolute w-full h-full rounded-full border border-indigo-500/50 animate-[ping_3s_linear_infinite_1.5s]" />
                            </>
                        )}
                        <Activity className={clsx("w-5 h-5 transition-colors", active ? "text-indigo-400" : "text-slate-600")} />
                    </div>
                </div>

                <div className="mt-6 pt-4 border-t border-white/5 grid grid-cols-2 gap-4">
                    <div className="flex items-center space-x-3">
                        <div className="p-2 bg-slate-800/50 rounded-lg text-slate-400">
                            <Globe className="w-4 h-4" />
                        </div>
                        <div>
                            <div className="text-[10px] uppercase text-slate-500 font-semibold">Public IP</div>
                            <div className="text-sm font-mono text-slate-300">
                                {loading ? (
                                    <span className="animate-pulse">Loading...</span>
                                ) : (
                                    ip || "Unavailable"
                                )}
                            </div>
                        </div>
                    </div>

                    <div className="flex items-center space-x-3">
                        <div className="p-2 bg-slate-800/50 rounded-lg text-slate-400">
                            <Shield className="w-4 h-4" />
                        </div>
                        <div>
                            <div className="text-[10px] uppercase text-slate-500 font-semibold">Location</div>
                            <div className="text-sm font-medium text-slate-300 truncate max-w-[120px]">
                                {loading ? (
                                    <span className="animate-pulse">...</span>
                                ) : (
                                    country || "Unknown"
                                )}
                            </div>
                        </div>
                    </div>

                    <div className="flex items-center space-x-3">
                        <div className="p-2 bg-slate-800/50 rounded-lg text-slate-400">
                            <Activity className="w-4 h-4" />
                        </div>
                        <div>
                            <div className="text-[10px] uppercase text-slate-500 font-semibold">IPv6 Status</div>
                            <div className={clsx(
                                "text-sm font-mono truncate max-w-[120px]",
                                ipv6 === "Blocked" || ipv6 === "Secure" ? "text-emerald-400" : "text-red-400"
                            )}>
                                {loading ? (
                                    <span className="animate-pulse">...</span>
                                ) : (
                                    ipv6 || "Checking..."
                                )}
                            </div>
                        </div>
                    </div>

                    <div className="flex items-center space-x-3">
                        <div className="p-2 bg-slate-800/50 rounded-lg text-slate-400">
                            <Shield className="w-4 h-4" />
                        </div>
                        <div>
                            <div className="text-[10px] uppercase text-slate-500 font-semibold">MAC Address</div>
                            <div className="text-xs font-mono text-slate-300 truncate max-w-[120px]">
                                {loading ? "..." : mac || "Unknown"}
                            </div>
                        </div>
                    </div>

                    <div className="flex items-center space-x-3">
                        <div className="p-2 bg-slate-800/50 rounded-lg text-slate-400">
                            <Globe className="w-4 h-4" />
                        </div>
                        <div>
                            <div className="text-[10px] uppercase text-slate-500 font-semibold">Hostname</div>
                            <div className="text-xs font-mono text-slate-300 truncate max-w-[120px]">
                                {loading ? "..." : hostname || "Unknown"}
                            </div>
                        </div>
                    </div>

                </div>
            </div>
        </div>
    );
}
