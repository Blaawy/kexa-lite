import type { Metadata } from 'next';
import { ExplorerDashboard } from '@/components/explorer-dashboard';

export const metadata: Metadata = {
  title: 'Explorer Dashboard | KEXA Explorer',
  description: 'Live KEXA mainnet dashboard with block feed and network status.',
};

export default function ExplorerPage() {
  return (
    <section className="space-y-4">
      <h1 className="text-3xl font-bold text-white">Explorer Dashboard</h1>
      <p className="text-slate-300">Track latest blocks, network health, and peer activity in real time.</p>
      <ExplorerDashboard />
    </section>
  );
}
