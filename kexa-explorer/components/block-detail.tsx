'use client';

import useSWR from 'swr';
import { ChevronDown, ExternalLink } from 'lucide-react';
import { useState } from 'react';
import { formatDistanceToNow, format } from 'date-fns';
import { kexaApi } from '@/lib/kexa';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { CopyButton } from '@/components/copy-button';
import { Skeleton } from '@/components/ui/skeleton';

function renderMaybeNumber(input: unknown) {
  if (typeof input === 'number' || typeof input === 'string') return String(input);
  return '—';
}

export function BlockDetailView({ hash }: { hash: string }) {
  const [open, setOpen] = useState(true);
  const { data, error } = useSWR(['block', hash], () => kexaApi.block(hash));

  if (error) return <p className="text-rose-300">Failed to load block.</p>;
  if (!data) return <Skeleton className="h-44 w-full" />;

  const date = data.timestamp ? new Date(data.timestamp) : null;

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader className="space-y-2">
          <CardTitle className="text-slate-200">Block Header</CardTitle>
          <div className="flex flex-wrap items-center gap-2">
            <p className="break-all font-mono text-xs text-white">{data.hash}</p>
            <CopyButton value={data.hash} label="Copy block hash" />
            <a href={`https://x.com/search?q=${data.hash}`} target="_blank" rel="noreferrer" className="inline-flex items-center text-xs text-cyan-300 hover:text-cyan-200">Share <ExternalLink className="ml-1 h-3.5 w-3.5" /></a>
          </div>
        </CardHeader>
        <CardContent className="grid gap-3 text-sm sm:grid-cols-2 lg:grid-cols-4">
          <div><p className="text-slate-400">Height</p><p className="font-mono text-white">{renderMaybeNumber(data.height)}</p></div>
          <div><p className="text-slate-400">Tx Count</p><p className="font-mono text-white">{renderMaybeNumber(data.tx_count ?? 0)}</p></div>
          <div><p className="text-slate-400">Timestamp (relative)</p><p className="text-white">{date ? formatDistanceToNow(date, { addSuffix: true }) : '—'}</p></div>
          <div><p className="text-slate-400">Timestamp (UTC)</p><p className="text-white">{date ? format(date, 'PPpp') : '—'}</p></div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader><CardTitle>Monetary Details</CardTitle></CardHeader>
        <CardContent className="grid gap-3 text-sm sm:grid-cols-3">
          <div><p className="text-slate-400">Coinbase</p><p className="break-all font-mono text-xs">{renderMaybeNumber((data as Record<string, unknown>).coinbase)}</p></div>
          <div><p className="text-slate-400">Fees</p><p className="font-mono">{renderMaybeNumber((data as Record<string, unknown>).fees)}</p></div>
          <div><p className="text-slate-400">Outputs</p><p className="font-mono">{renderMaybeNumber((data as Record<string, unknown>).outputs)}</p></div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <button type="button" className="flex w-full items-center justify-between text-left" onClick={() => setOpen((v) => !v)}>
            <CardTitle>Raw JSON</CardTitle>
            <ChevronDown className={`h-4 w-4 transition-transform ${open ? 'rotate-180' : ''}`} />
          </button>
        </CardHeader>
        {open && <CardContent><pre className="code-block overflow-auto">{JSON.stringify(data, null, 2)}</pre></CardContent>}
      </Card>
    </div>
  );
}
