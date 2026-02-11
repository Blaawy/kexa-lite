'use client';

import useSWR from 'swr';
import { RefreshCcw } from 'lucide-react';
import { kexaApi } from '@/lib/kexa';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { CopyButton } from '@/components/copy-button';

export function NodesView() {
  const { data: live, mutate: refreshLive } = useSWR('livePeers', kexaApi.livePeers, { refreshInterval: 10000 });
  const { data: configured, mutate: refreshConfigured } = useSWR('configuredPeers', kexaApi.peers, { refreshInterval: 20000 });

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>Live Peers</CardTitle>
          <Button variant="secondary" onClick={() => refreshLive()}><RefreshCcw className="mr-2 h-4 w-4" />Refresh</Button>
        </CardHeader>
        <CardContent>
          <ul className="space-y-2 text-sm">
            {live?.length ? live.map((peer) => (
              <li key={peer} className="flex items-center justify-between rounded-xl border border-white/10 bg-black/20 p-2 font-mono">
                <span className="truncate pr-3">{peer}</span>
                <CopyButton value={peer} label="Copy peer" />
              </li>
            )) : <li className="text-slate-400">No live peers reported.</li>}
          </ul>
        </CardContent>
      </Card>
      {configured?.supported && (
        <Card>
          <CardHeader className="flex flex-row items-center justify-between">
            <CardTitle>Configured Peers</CardTitle>
            <Button variant="ghost" onClick={() => refreshConfigured()}><RefreshCcw className="mr-2 h-4 w-4" />Refresh</Button>
          </CardHeader>
          <CardContent>
            <ul className="space-y-2 text-sm">
              {(configured.peers ?? []).map((peer) => (
                <li key={peer} className="flex items-center justify-between rounded-xl border border-white/10 bg-black/20 p-2 font-mono">
                  <span className="truncate pr-3">{peer}</span>
                  <CopyButton value={peer} label="Copy configured peer" />
                </li>
              ))}
            </ul>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
