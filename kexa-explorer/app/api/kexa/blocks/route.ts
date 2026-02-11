import { BlockSummarySchema } from '@/lib/schemas';
import { liveJson, proxyKexa } from '@/lib/server-kexa';

export const dynamic = 'force-dynamic';

export async function GET(request: Request) {
  try {
    const url = new URL(request.url);
    const limit = Number(url.searchParams.get('limit') ?? '20');
    const safeLimit = Number.isFinite(limit) && limit > 0 ? Math.min(limit, 100) : 20;
    const upstream = await proxyKexa(`/blocks?limit=${safeLimit}`);
    if (!upstream.ok) return liveJson({ error: 'Upstream blocks failed' }, upstream.status);
    const parsed = BlockSummarySchema.array().parse(upstream.data);
    return liveJson(parsed);
  } catch {
    return liveJson({ error: 'Unable to reach KEXA node' }, 502);
  }
}
