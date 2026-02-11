import type { Metadata } from 'next';
import Link from 'next/link';
import { NetworkStatus } from '@/components/network-status';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { MAINNET_CONSTANTS } from '@/lib/constants';

export const metadata: Metadata = {
  title: 'KEXA — Mainnet Explorer',
  description: 'Mainnet-first KEXA explorer with verification-focused tooling.',
};

export default function HomePage() {
  return (
    <div className="space-y-8">
      <NetworkStatus />
      <section className="space-y-4 py-8">
        <h1 className="text-4xl font-bold tracking-tight">KEXA — Mainnet Explorer</h1>
        <p className="max-w-2xl text-slate-600">A trust-first blockchain explorer experience built for transparent verification and dependable operations.</p>
        <div className="flex flex-wrap gap-3">
          <Link href="/explorer"><Button size="lg">Explorer</Button></Link>
          <Link href="/verify"><Button variant="outline" size="lg">Verify Mainnet</Button></Link>
          <Link href="/nodes"><Button variant="outline" size="lg">Run a Node</Button></Link>
        </div>
      </section>
      <section className="grid gap-4 md:grid-cols-2">
        <Card><CardContent className="pt-4"><h2 className="mb-2 font-semibold">Mainnet Genesis Hash</h2><p className="break-all font-mono text-sm">{MAINNET_CONSTANTS.genesisHash}</p></CardContent></Card>
        <Card><CardContent className="pt-4"><h2 className="mb-2 font-semibold">Economic Parameters</h2><ul className="space-y-1 text-sm text-slate-700"><li>Max supply: {MAINNET_CONSTANTS.maxSupply}</li><li>Subsidy: {MAINNET_CONSTANTS.subsidy}</li><li>Mineable blocks: {MAINNET_CONSTANTS.mineableBlocks}</li></ul></CardContent></Card>
      </section>
    </div>
  );
}
