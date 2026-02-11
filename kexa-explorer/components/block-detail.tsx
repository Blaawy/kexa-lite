'use client';

import useSWR from 'swr';
import { ChevronDown } from 'lucide-react';
import { useState } from 'react';
import { kexaApi } from '@/lib/kexa';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

export function BlockDetailView({ hash }: { hash: string }) {
  const [open, setOpen] = useState(false);
  const { data, error } = useSWR(['block', hash], () => kexaApi.block(hash));

  if (error) return <p className="text-red-600">Failed to load block.</p>;
  if (!data) return <p className="text-slate-500">Loading block...</p>;

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader><CardTitle>Block {data.height ?? '—'}</CardTitle></CardHeader>
        <CardContent className="space-y-2 text-sm">
          <p><span className="text-slate-500">Hash:</span> <span className="break-all font-mono">{data.hash}</span></p>
          <p><span className="text-slate-500">Timestamp:</span> {data.timestamp ? String(data.timestamp) : '—'}</p>
          <p><span className="text-slate-500">Tx Count:</span> {data.tx_count ?? 0}</p>
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
