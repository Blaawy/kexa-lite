import { TipSchema } from '@/lib/schemas';
import { liveJson, proxyKexa } from '@/lib/server-kexa';

export const dynamic = 'force-dynamic';

export async function GET() {
  try {
    const upstream = await proxyKexa('/tip');
    if (!upstream.ok) return liveJson({ error: 'Upstream tip failed' }, upstream.status);
    return liveJson(TipSchema.parse(upstream.data));
  } catch {
    return liveJson({ error: 'Unable to reach KEXA node' }, 502);
  }
}
