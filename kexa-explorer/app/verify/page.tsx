import type { Metadata } from 'next';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { VerifyChecks } from '@/components/verify-checks';

export const metadata: Metadata = {
  title: 'Verification Contract | KEXA Explorer',
  description: 'Step-by-step commands to verify KEXA mainnet behavior.',
};

export default function VerifyPage() {
  return (
    <section className="space-y-4">
      <h1 className="text-3xl font-bold text-white">Verification Contract</h1>
      <p className="text-slate-300">Use these commands to independently verify mainnet connectivity and state through the explorer proxy.</p>
      <Card>
        <CardHeader><CardTitle>Expected Checks</CardTitle></CardHeader>
        <CardContent>
          <VerifyChecks />
        </CardContent>
      </Card>
    </section>
  );
}
