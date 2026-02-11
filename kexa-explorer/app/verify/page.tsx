import type { Metadata } from 'next';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

export const metadata: Metadata = {
  title: 'Verification Contract | KEXA Explorer',
  description: 'Step-by-step commands to verify KEXA mainnet behavior.',
};

const checks = [
  'curl -s http://localhost:3000/api/kexa/health | jq',
  'curl -s http://localhost:3000/api/kexa/tip | jq',
  'curl -s "http://localhost:3000/api/kexa/blocks?limit=1" | jq',
  'curl -s http://localhost:3000/api/kexa/peers/live | jq',
];

export default function VerifyPage() {
  return (
    <section className="space-y-4">
      <h1 className="text-3xl font-bold">Verification Contract</h1>
      <p className="text-slate-600">Use these commands to independently verify mainnet connectivity and state through the explorer proxy.</p>
      <Card>
        <CardHeader><CardTitle>Expected Checks</CardTitle></CardHeader>
        <CardContent>
          <ul className="space-y-3">{checks.map((cmd) => <li key={cmd}><pre className="code-block overflow-auto">{cmd}</pre></li>)}</ul>
        </CardContent>
      </Card>
    </section>
  );
}
