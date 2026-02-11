import { BlockDetailSchema } from '@/lib/schemas';
import { liveJson, proxyKexa } from '@/lib/server-kexa';

export const dynamic = 'force-dynamic';

export async function GET(_: Request, { params }: { params: { hash: string } }) {
  try {
    const upstream = await proxyKexa(`/block/${params.hash}`);
    if (!upstream.ok) return liveJson({ error: 'Block not found' }, upstream.status);
    return liveJson(BlockDetailSchema.parse(upstream.data));
  } catch {
    return liveJson({ error: 'Unable to reach KEXA node' }, 502);
  }
}
