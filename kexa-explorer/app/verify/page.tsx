import type { Metadata } from 'next';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { VerifyChecks } from '@/components/verify-checks';
import { MAINNET_LIVE_URL } from '@/lib/constants';

export const metadata: Metadata = {
  title: 'Verification Contract | KEXA Explorer',
  description: 'Step-by-step commands to verify KEXA mainnet behavior.',
};

export default function VerifyPage() {
  return (
    <section className="space-y-4">
      <h1 className="text-3xl font-bold text-white">Verification Contract</h1>
      <p className="text-slate-300">Use these commands to independently verify mainnet connectivity and state through the explorer proxy.</p>
      <p className="text-xs text-slate-400">Canonical release: https://github.com/Blaawy/kexa-lite/releases/tag/v0.1.0-rc1</p>
      <p>
        <a
          href={MAINNET_LIVE_URL}
          target="_blank"
          rel="noreferrer"
          className="inline-flex rounded-full border border-cyan-400/50 bg-cyan-400/10 px-4 py-2 text-sm font-medium text-cyan-200 transition-colors hover:border-cyan-300 hover:text-cyan-100"
        >
          Mainnet Live Release (Artifacts + SHA256SUMS)
        </a>
      </p>
      <Card>
        <CardHeader><CardTitle>Expected Checks</CardTitle></CardHeader>
        <CardContent>
          <VerifyChecks />
        </CardContent>
      </Card>
    </section>
  );
}
