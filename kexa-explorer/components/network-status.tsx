'use client';

import useSWR from 'swr';
import { Activity, Blocks, Network, ShieldCheck } from 'lucide-react';
import { kexaApi } from '@/lib/kexa';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { StatusDot } from '@/components/status-dot';
import { Skeleton } from '@/components/ui/skeleton';

export function NetworkStatus() {
  const swrOpts = { refreshInterval: 10000, keepPreviousData: true } as const;
  const { data: health } = useSWR('health', kexaApi.health, swrOpts);
  const { data: tip } = useSWR('tip', kexaApi.tip, swrOpts);
  const { data: peers } = useSWR('livePeers', kexaApi.livePeers, swrOpts);

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2 text-base text-white">
          <ShieldCheck className="h-4 w-4 text-cyan-300" /> Live Network Status
        </CardTitle>
      </CardHeader>
      <CardContent className="grid gap-3 md:grid-cols-4">
        <div className="rounded-xl border border-white/10 bg-black/20 p-3">
          <div className="mb-2 flex items-center gap-2 text-xs uppercase tracking-wide text-slate-400"><Activity className="h-3.5 w-3.5" /> Health</div>
          {health ? (
            <div className="flex items-center gap-2"><StatusDot status={health.ok ? 'good' : 'bad'} /> <Badge>{health.ok ? 'Healthy' : 'Offline'}</Badge></div>
          ) : <Skeleton className="h-6 w-24" />}
        </div>
        <div className="rounded-xl border border-white/10 bg-black/20 p-3">
          <div className="mb-2 flex items-center gap-2 text-xs uppercase tracking-wide text-slate-400"><Blocks className="h-3.5 w-3.5" /> Height</div>
          <p className="font-mono text-xl">{tip?.height ?? '—'}</p>
        </div>
        <div className="rounded-xl border border-white/10 bg-black/20 p-3">
          <div className="mb-2 text-xs uppercase tracking-wide text-slate-400">Tip Hash</div>
          <p className="truncate font-mono text-xs text-slate-200">{tip?.hash ?? '—'}</p>
        </div>
        <div className="rounded-xl border border-white/10 bg-black/20 p-3">
          <div className="mb-2 flex items-center gap-2 text-xs uppercase tracking-wide text-slate-400"><Network className="h-3.5 w-3.5" /> Peers</div>
          <p className="font-mono text-xl">{peers?.length ?? '—'}</p>
        </div>
      </CardContent>
    </Card>
  );
}
