import type { Metadata } from 'next';
import { BlockDetailView } from '@/components/block-detail';

export const metadata: Metadata = {
  title: 'Block Details | KEXA Explorer',
  description: 'Inspect block metadata and raw payloads on KEXA mainnet.',
};

export default function BlockPage({ params }: { params: { hash: string } }) {
  return (
    <section className="space-y-4">
      <h1 className="text-3xl font-bold">Block Details</h1>
      <BlockDetailView hash={params.hash} />
    </section>
  );
}
