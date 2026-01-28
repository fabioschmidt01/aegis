import { Copy, Heart, X, ExternalLink, Bitcoin, Wallet } from "lucide-react";
import { useState } from "react";
import { clsx } from 'clsx';

interface DonationModalProps {
    isOpen: boolean;
    onClose: () => void;
}

export function DonationModal({ isOpen, onClose }: DonationModalProps) {
    if (!isOpen) return null;

    const [copied, setCopied] = useState<string | null>(null);

    const addresses = [
        {
            label: "Buy Me a Coffee",
            value: "https://buymeacoffee.com/belydev",
            icon: Heart,
            color: "text-yellow-400",
            isLink: true
        },
        {
            label: "Monero (XMR)",
            value: "466KtH3FTWFYJ2xN9McVzzPnNXZf4GGZr2AQ9eQP6RfuYzd2WkTrckf4ySZF8SsdQQNiyWToG8mTP1DaQfsGTd5p2MkMZTN",
            icon: Wallet,
            color: "text-orange-500",
            isLink: false
        },
        {
            label: "ZCash (ZEC)",
            value: "u1zzjp0gh9ms5wcfd5uqsj47jjad7qufqm4pugqw0l96h0374zu3pfn0we0v2g88p0apap9y38kj5dasjcnl6sll7psjfx7g763ymrd57t",
            icon: Wallet,
            color: "text-indigo-400",
            isLink: false
        },
        {
            label: "Bitcoin (BTC)",
            value: "bc1quz3lk0s2wzcpycz545dkzcn5lqyct9z60maafp",
            icon: Bitcoin,
            color: "text-orange-400",
            isLink: false
        },
    ];

    const handleCopy = (text: string, label: string) => {
        navigator.clipboard.writeText(text);
        setCopied(label);
        setTimeout(() => setCopied(null), 2000);
    };

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
            {/* Backdrop */}
            <div
                className="absolute inset-0 bg-black/60 backdrop-blur-sm"
                onClick={onClose}
            />

            {/* Modal */}
            <div className="relative w-full max-w-md bg-slate-900 border border-white/10 rounded-2xl shadow-2xl overflow-hidden transform transition-all animate-in fade-in zoom-in-95 duration-200">

                <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-pink-500 via-red-500 to-yellow-500" />

                <div className="p-6">
                    <div className="flex justify-between items-center mb-6">
                        <div className="flex items-center space-x-2">
                            <Heart className="w-6 h-6 text-pink-500 fill-pink-500 animate-pulse" />
                            <h2 className="text-xl font-bold text-white">Support Development</h2>
                        </div>
                        <button onClick={onClose} className="text-slate-500 hover:text-white transition-colors">
                            <X className="w-5 h-5" />
                        </button>
                    </div>

                    <p className="text-sm text-slate-400 mb-6">
                        Aegis is free and open source. Your support helps verify security, maintain servers, and add new stealth features.
                    </p>

                    <div className="space-y-3">
                        {addresses.map((item) => (
                            <div key={item.label} className="bg-slate-800/50 rounded-xl p-3 border border-white/5 hover:border-indigo-500/30 transition-colors group">
                                <div className="flex items-center justify-between mb-2">
                                    <div className="flex items-center space-x-2">
                                        <item.icon className={clsx("w-4 h-4", item.color)} />
                                        <span className="text-sm font-medium text-slate-200">{item.label}</span>
                                    </div>
                                    {item.isLink ? (
                                        <a
                                            href={item.value}
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            className="px-3 py-1 bg-yellow-500/10 text-yellow-400 rounded text-xs font-bold hover:bg-yellow-500/20 transition-colors flex items-center space-x-1"
                                        >
                                            <span>Open</span>
                                            <ExternalLink className="w-3 h-3" />
                                        </a>
                                    ) : (
                                        <button
                                            onClick={() => handleCopy(item.value, item.label)}
                                            className={clsx(
                                                "px-3 py-1 rounded text-xs font-bold transition-all flex items-center space-x-1",
                                                copied === item.label
                                                    ? "bg-green-500/20 text-green-400"
                                                    : "bg-indigo-500/10 text-indigo-400 hover:bg-indigo-500/20"
                                            )}
                                        >
                                            {copied === item.label ? (
                                                <span>Copied!</span>
                                            ) : (
                                                <>
                                                    <span>Copy</span>
                                                    <Copy className="w-3 h-3" />
                                                </>
                                            )}
                                        </button>
                                    )}
                                </div>

                                {!item.isLink && (
                                    <div className="font-mono text-[10px] text-slate-500 break-all bg-black/20 p-2 rounded">
                                        {item.value}
                                    </div>
                                )}
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
}
