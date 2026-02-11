import type { Metadata } from 'next';
import { NodesView } from '@/components/nodes-view';

export const metadata: Metadata = {
  title: 'Node Peers | KEXA Explorer',
  description: 'Observe live and configured peers on KEXA mainnet.',
};

export default function NodesPage() {
  return (
    <section className="space-y-4">
      <h1 className="text-3xl font-bold text-white">Network Nodes</h1>
      <p className="text-slate-300">Monitor live peer topology and configured peers from the proxy-backed API.</p>
      <NodesView />
    </section>
  );
}
