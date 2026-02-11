import {
  BlockDetailSchema,
  BlockSummarySchema,
  HealthSchema,
  OptionalPeersSchema,
  PeersLiveSchema,
  TipSchema,
  type BlockDetail,
  type BlockSummary,
  type Health,
  type Tip,
} from '@/lib/schemas';

async function getJson<T>(url: string, parser: { parse: (i: unknown) => T }): Promise<T> {
  const res = await fetch(url, { cache: 'no-store' });
  if (!res.ok) {
    throw new Error(`Request failed: ${res.status}`);
  }
  return parser.parse(await res.json());
}

export const kexaApi = {
  health: () => getJson<Health>('/api/kexa/health', HealthSchema),
  tip: () => getJson<Tip>('/api/kexa/tip', TipSchema),
  blocks: (limit = 20) =>
    getJson<BlockSummary[]>(`/api/kexa/blocks?limit=${limit}`, { parse: (i) => BlockSummarySchema.array().parse(i) }),
  block: (hash: string) => getJson<BlockDetail>(`/api/kexa/block/${hash}`, BlockDetailSchema),
  livePeers: () => getJson<string[]>('/api/kexa/peers/live', PeersLiveSchema),
  peers: () => getJson<{ supported: boolean; peers?: string[] }>('/api/kexa/peers', OptionalPeersSchema),
};
