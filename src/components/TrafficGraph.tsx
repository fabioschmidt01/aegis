import { useEffect, useState } from 'react';
import { AreaChart, Area, XAxis, YAxis, Tooltip, ResponsiveContainer } from 'recharts';
import { listen } from '@tauri-apps/api/event';

interface TrafficData {
    time: string;
    upload: number;
    download: number;
}

interface TrafficEvent {
    up_speed: number;
    down_speed: number;
}

export function TrafficGraph() {
    const [data, setData] = useState<TrafficData[]>([]);

    useEffect(() => {
        // Fill initial data
        const initialData = Array(20).fill({ time: '', upload: 0, download: 0 });
        setData(initialData);

        const unlisten = listen<TrafficEvent>('traffic_update', (event) => {
            const now = new Date().toLocaleTimeString();
            setData(prev => {
                const newData = [...prev.slice(1), {
                    time: now,
                    upload: event.payload.up_speed,
                    download: event.payload.down_speed
                }];
                return newData;
            });
        });

        return () => {
            unlisten.then(f => f());
        };
    }, []);

    const formatBytes = (bytes: number) => {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
    };

    return (
        <div className="w-full h-48 bg-black/20 rounded-xl border border-white/5 p-4 relative overflow-hidden">
            <h3 className="text-xs font-bold text-gray-500 uppercase tracking-widest mb-2 absolute top-4 left-4 z-10">Network Traffic</h3>
            <ResponsiveContainer width="100%" height="100%">
                <AreaChart data={data}>
                    <defs>
                        <linearGradient id="colorDown" x1="0" y1="0" x2="0" y2="1">
                            <stop offset="5%" stopColor="#6366f1" stopOpacity={0.3} />
                            <stop offset="95%" stopColor="#6366f1" stopOpacity={0} />
                        </linearGradient>
                        <linearGradient id="colorUp" x1="0" y1="0" x2="0" y2="1">
                            <stop offset="5%" stopColor="#8b5cf6" stopOpacity={0.3} />
                            <stop offset="95%" stopColor="#8b5cf6" stopOpacity={0} />
                        </linearGradient>
                    </defs>
                    <XAxis dataKey="time" hide />
                    <YAxis hide domain={[0, 'auto']} />
                    <Tooltip
                        contentStyle={{ backgroundColor: '#1e293b', border: '1px solid #334155', borderRadius: '8px' }}
                        itemStyle={{ fontSize: '12px', color: '#cbd5e1' }}
                        formatter={(value: any) => [formatBytes(value), ""]}
                        labelStyle={{ display: 'none' }}
                    />
                    <Area
                        type="monotone"
                        dataKey="download"
                        stroke="#6366f1"
                        fillOpacity={1}
                        fill="url(#colorDown)"
                        isAnimationActive={false}
                        strokeWidth={2}
                    />
                    <Area
                        type="monotone"
                        dataKey="upload"
                        stroke="#8b5cf6"
                        fillOpacity={1}
                        fill="url(#colorUp)"
                        isAnimationActive={false}
                        strokeWidth={2}
                    />
                </AreaChart>
            </ResponsiveContainer>

            <div className="absolute top-4 right-4 flex space-x-4 text-xs font-mono">
                <span className="text-indigo-400 flex items-center">
                    <div className="w-2 h-2 rounded-full bg-indigo-500 mr-2 animate-pulse" />
                    DOWN: {formatBytes(data[data.length - 1]?.download || 0)}/s
                </span>
                <span className="text-purple-400 flex items-center">
                    <div className="w-2 h-2 rounded-full bg-purple-500 mr-2 animate-pulse" />
                    UP: {formatBytes(data[data.length - 1]?.upload || 0)}/s
                </span>
            </div>
        </div>
    );
}
