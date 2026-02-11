import type { Metadata } from 'next';
import { NodesView } from '@/components/nodes-view';

export const metadata: Metadata = {
  title: 'Node Peers | KEXA Explorer',
  description: 'Observe live and configured peers on KEXA mainnet.',
};

export default function NodesPage() {
  return (
    <section className="space-y-4">
      <h1 className="text-3xl font-bold">Network Nodes</h1>
      <NodesView />
    </section>
  );
}
