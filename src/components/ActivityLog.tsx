import { useRef, useEffect } from 'react';

interface ActivityLogProps {
    logs: string[];
}

export function ActivityLog({ logs }: ActivityLogProps) {
    const bottomRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [logs]);

    return (
        <div className="w-full bg-slate-950/80 p-4 font-mono text-[10px] h-48 overflow-y-auto custom-scrollbar">
            <div className="space-y-1.5">
                {logs.length === 0 && (
                    <div className="text-slate-600 italic">No activity recorded yet...</div>
                )}
                {logs.map((log, i) => (
                    <div key={i} className="flex items-start space-x-3 border-l-2 border-slate-800 pl-3 py-0.5 hover:bg-white/5 transition-colors rounded-r">
                        <span className="text-slate-600 shrink-0 select-none">{new Date().toLocaleTimeString()}</span>
                        <span className={
                            log.includes("ERR") || log.includes("CRITICAL") ? "text-red-400" :
                                log.includes("OK") || log.includes("Success") ? "text-emerald-400" :
                                    "text-slate-300"
                        }>
                            {log}
                        </span>
                    </div>
                ))}
                <div ref={bottomRef} />
            </div>
        </div>
    );
}
