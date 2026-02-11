'use client';

import { FormEvent, useState } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';
import useSWR from 'swr';
import { kexaApi } from '@/lib/kexa';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Button } from '@/components/ui/button';

function Skeleton({ className = '' }: { className?: string }) {
  return <div className={`animate-pulse rounded bg-slate-200 ${className}`} />;
}

export function ExplorerDashboard() {
  const router = useRouter();
  const [hash, setHash] = useState('');

  const swrOpts = { refreshInterval: 10000, keepPreviousData: true } as const;
  const { data: health } = useSWR('health', kexaApi.health, swrOpts);
  const { data: tip } = useSWR('tip', kexaApi.tip, swrOpts);
  const { data: blocks } = useSWR('blocks', () => kexaApi.blocks(20), swrOpts);
  const { data: peers } = useSWR('livePeers', kexaApi.livePeers, swrOpts);

  const onSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (hash.trim()) router.push(`/explorer/block/${hash.trim()}`);
  };

  return (
    <div className="space-y-6">
      <form className="flex gap-2" onSubmit={onSubmit}>
        <Input value={hash} onChange={(e) => setHash(e.target.value)} placeholder="Paste block hash" />
        <Button type="submit">Search</Button>
      </form>

      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <Card><CardHeader><CardTitle>Health</CardTitle></CardHeader><CardContent>{health ? <Badge className={health.ok ? 'border-emerald-300 bg-emerald-50 text-emerald-700' : 'border-red-300 bg-red-50 text-red-700'}>{health.ok ? 'OK' : 'Fail'}</Badge> : <Skeleton className="h-6 w-20" />}</CardContent></Card>
        <Card><CardHeader><CardTitle>Height</CardTitle></CardHeader><CardContent>{tip ? <span className="font-mono">{tip.height}</span> : <Skeleton className="h-5 w-24" />}</CardContent></Card>
        <Card><CardHeader><CardTitle>Tip Hash</CardTitle></CardHeader><CardContent>{tip ? <span className="block truncate font-mono text-xs">{tip.hash}</span> : <Skeleton className="h-5 w-32" />}</CardContent></Card>
        <Card><CardHeader><CardTitle>Live Peers</CardTitle></CardHeader><CardContent>{peers ? <span className="font-mono">{peers.length}</span> : <Skeleton className="h-5 w-10" />}</CardContent></Card>
      </div>

      <Card>
        <CardHeader><CardTitle>Latest Blocks</CardTitle></CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Height</TableHead><TableHead>Hash</TableHead><TableHead>Tx</TableHead><TableHead>Timestamp</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {blocks
                ? blocks.map((b) => (
                    <TableRow key={b.hash}>
                      <TableCell>{b.height}</TableCell>
                      <TableCell className="font-mono text-xs"><Link href={`/explorer/block/${b.hash}`} className="hover:underline">{b.hash}</Link></TableCell>
                      <TableCell>{b.tx_count ?? 0}</TableCell>
                      <TableCell>{b.timestamp ? String(b.timestamp) : '—'}</TableCell>
                    </TableRow>
                  ))
                : Array.from({ length: 6 }).map((_, i) => (
                    <TableRow key={i}><TableCell colSpan={4}><Skeleton className="h-5 w-full" /></TableCell></TableRow>
                  ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
  );
}
