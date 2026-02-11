'use client';

import useSWR from 'swr';
import { Activity, ShieldCheck } from 'lucide-react';
import { kexaApi } from '@/lib/kexa';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

export function NetworkStatus() {
  const { data: health } = useSWR('health', kexaApi.health, { refreshInterval: 10000 });
  const { data: tip } = useSWR('tip', kexaApi.tip, { refreshInterval: 10000 });

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2 text-base"><ShieldCheck className="h-4 w-4" />Live Network Status</CardTitle>
      </CardHeader>
      <CardContent className="grid gap-3 sm:grid-cols-3">
        <div className="rounded-md border p-3">
          <div className="text-xs text-slate-500">Health</div>
          <Badge className={health?.ok ? 'border-emerald-300 bg-emerald-50 text-emerald-700' : 'border-amber-300 bg-amber-50 text-amber-700'}>
            <Activity className="mr-1 h-3 w-3" />{health?.ok ? 'Healthy' : 'Unknown'}
          </Badge>
        </div>
        <div className="rounded-md border p-3">
          <div className="text-xs text-slate-500">Height</div>
          <div className="font-mono text-sm">{tip?.height ?? '—'}</div>
        </div>
        <div className="rounded-md border p-3">
          <div className="text-xs text-slate-500">Tip Hash</div>
          <div className="truncate font-mono text-xs">{tip?.hash ?? '—'}</div>
        </div>
      </CardContent>
    </Card>
  );
}
