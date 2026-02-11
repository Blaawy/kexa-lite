'use client';

import { FormEvent, useMemo, useState } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';
import useSWR from 'swr';
import { formatDistanceToNow } from 'date-fns';
import { AlertTriangle, Blocks, Network, Shield } from 'lucide-react';
import { kexaApi } from '@/lib/kexa';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Button } from '@/components/ui/button';
import { CopyButton } from '@/components/copy-button';
import { Skeleton } from '@/components/ui/skeleton';

const hashRegex = /^[a-fA-F0-9]{64}$/;

export function ExplorerDashboard() {
  const router = useRouter();
  const [hash, setHash] = useState('');
  const [searchError, setSearchError] = useState<string | null>(null);

  const swrOpts = { refreshInterval: 10000, keepPreviousData: true } as const;
  const { data: health, error: healthErr } = useSWR('health', kexaApi.health, swrOpts);
  const { data: tip } = useSWR('tip', kexaApi.tip, swrOpts);
  const { data: blocks, error: blocksErr } = useSWR('blocks', () => kexaApi.blocks(20), swrOpts);
  const { data: peers } = useSWR('livePeers', kexaApi.livePeers, swrOpts);

  const onSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const value = hash.trim();
    if (!hashRegex.test(value)) {
      setSearchError('Enter a valid 64-character hex block hash.');
      return;
    }
    setSearchError(null);
    router.push(`/explorer/block/${value}`);
  };

  const stats = useMemo(
    () => [
      { label: 'Health', value: health?.ok ? 'Healthy' : 'Unknown', icon: Shield },
      { label: 'Height', value: tip?.height?.toString() ?? '—', icon: Blocks },
      { label: 'Peers', value: peers?.length?.toString() ?? '—', icon: Network },
      { label: 'Tip', value: tip?.hash ? `${tip.hash.slice(0, 16)}…` : '—', icon: Blocks },
    ],
    [health?.ok, peers?.length, tip?.hash, tip?.height],
  );

  return (
    <div className="space-y-6">
      <form className="space-y-2" onSubmit={onSubmit}>
        <div className="flex flex-col gap-2 md:flex-row">
          <Input
            value={hash}
            onChange={(e) => setHash(e.target.value)}
            placeholder="Search by block hash (64 hex chars)"
            className="border-white/20 bg-white/5 text-white placeholder:text-slate-400"
          />
          <Button type="submit">Search Block</Button>
        </div>
        {searchError && <p className="text-sm text-rose-300">{searchError}</p>}
      </form>

      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        {stats.map((stat) => (
          <Card key={stat.label}>
            <CardHeader><CardTitle className="flex items-center gap-2 text-slate-300"><stat.icon className="h-4 w-4 text-cyan-300" />{stat.label}</CardTitle></CardHeader>
            <CardContent><p className="font-mono text-lg text-white">{stat.value}</p></CardContent>
          </Card>
        ))}
      </div>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>Latest Blocks (auto-refresh 10s)</CardTitle>
          {blocksErr && <Badge className="border-rose-300/40 bg-rose-400/20 text-rose-100"><AlertTriangle className="mr-1 h-3 w-3" /> Feed unavailable</Badge>}
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Height</TableHead><TableHead>Hash</TableHead><TableHead>Tx</TableHead><TableHead>Time</TableHead><TableHead className="w-14">Copy</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {blocks
                ? blocks.map((b) => {
                    const date = b.timestamp ? new Date(b.timestamp) : null;
                    return (
                      <TableRow key={b.hash} className="cursor-pointer" onClick={() => router.push(`/explorer/block/${b.hash}`)}>
                        <TableCell>{b.height}</TableCell>
                        <TableCell className="font-mono text-xs"><Link href={`/explorer/block/${b.hash}`} className="hover:text-cyan-200" onClick={(e) => e.stopPropagation()}>{b.hash}</Link></TableCell>
                        <TableCell>{b.tx_count ?? 0}</TableCell>
                        <TableCell>{date ? `${formatDistanceToNow(date, { addSuffix: true })}` : '—'}</TableCell>
                        <TableCell><div onClick={(e) => e.stopPropagation()}><CopyButton value={b.hash} label="Copy block hash" /></div></TableCell>
                      </TableRow>
                    );
                  })
                : Array.from({ length: 6 }).map((_, i) => (
                    <TableRow key={i}><TableCell colSpan={5}><Skeleton className="h-6 w-full" /></TableCell></TableRow>
                  ))}
            </TableBody>
          </Table>
          {(healthErr || blocksErr) && <p className="mt-3 text-sm text-rose-200">Some endpoints failed. Check /verify for quick diagnostics.</p>}
        </CardContent>
      </Card>
    </div>
  );
}
