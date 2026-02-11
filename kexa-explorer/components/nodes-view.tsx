'use client';

import useSWR from 'swr';
import { kexaApi } from '@/lib/kexa';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

export function NodesView() {
  const { data: live } = useSWR('livePeers', kexaApi.livePeers, { refreshInterval: 10000 });
  const { data: configured } = useSWR('configuredPeers', kexaApi.peers, { refreshInterval: 10000 });

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader><CardTitle>Live Peers</CardTitle></CardHeader>
        <CardContent>
          <ul className="space-y-1 font-mono text-sm">{live?.map((peer) => <li key={peer}>{peer}</li>) ?? <li className="text-slate-500">Loading...</li>}</ul>
        </CardContent>
      </Card>
      {configured?.supported && (
        <Card>
          <CardHeader><CardTitle>Configured Peers</CardTitle></CardHeader>
          <CardContent><ul className="space-y-1 font-mono text-sm">{(configured.peers ?? []).map((peer) => <li key={peer}>{peer}</li>)}</ul></CardContent>
        </Card>
      )}
    </div>
  );
}
