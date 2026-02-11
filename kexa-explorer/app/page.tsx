import type { Metadata } from 'next';
import Link from 'next/link';
import { ArrowRight, ShieldCheck, Sparkles } from 'lucide-react';
import { NetworkStatus } from '@/components/network-status';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { MAINNET_CONSTANTS } from '@/lib/constants';

export const metadata: Metadata = {
  title: 'KEXA — Mainnet Explorer',
  description: 'Mainnet-first KEXA explorer with verification-focused tooling.',
};

export default function HomePage() {
  return (
    <div className="space-y-8">
      <section className="space-y-5 py-10">
        <p className="inline-flex items-center gap-2 rounded-full border border-cyan-300/30 bg-cyan-400/10 px-4 py-1 text-xs uppercase tracking-[0.2em] text-cyan-200"><Sparkles className="h-3.5 w-3.5" /> Premium Mainnet Intelligence</p>
        <h1 className="max-w-4xl text-4xl font-bold leading-tight text-white md:text-6xl">Explore KEXA with institutional-grade clarity.</h1>
        <p className="max-w-3xl text-lg text-slate-300">Real-time blocks, node telemetry, and verification workflows on top of a secure API proxy architecture.</p>
        <div className="flex flex-wrap gap-3">
          <Link href="/explorer"><Button size="lg">Open Explorer <ArrowRight className="ml-1 h-4 w-4" /></Button></Link>
          <Link href="/verify"><Button variant="secondary" size="lg">Verify Mainnet</Button></Link>
          <Link href="/nodes"><Button variant="ghost" size="lg">Inspect Nodes</Button></Link>
        </div>
      </section>

      <NetworkStatus />

      <section className="grid gap-4 md:grid-cols-2">
        <Card>
          <CardHeader><CardTitle>Mainnet Constants</CardTitle></CardHeader>
          <CardContent className="space-y-2 text-sm text-slate-200">
            <p><span className="text-slate-400">Genesis hash:</span> <span className="break-all font-mono">{MAINNET_CONSTANTS.genesisHash}</span></p>
            <p><span className="text-slate-400">Max supply:</span> {MAINNET_CONSTANTS.maxSupply}</p>
            <p><span className="text-slate-400">Subsidy:</span> {MAINNET_CONSTANTS.subsidy}</p>
            <p><span className="text-slate-400">Mineable blocks:</span> {MAINNET_CONSTANTS.mineableBlocks}</p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader><CardTitle>How to Verify</CardTitle></CardHeader>
          <CardContent className="space-y-3 text-sm text-slate-300">
            <p>Run health/tip/blocks/peers checks via the built-in proxy endpoints and inspect response status in one click.</p>
            <Link href="/verify" className="inline-flex items-center text-cyan-300 hover:text-cyan-200">Open verification steps <ShieldCheck className="ml-1 h-4 w-4" /></Link>
          </CardContent>
        </Card>
      </section>
    </div>
  );
}
