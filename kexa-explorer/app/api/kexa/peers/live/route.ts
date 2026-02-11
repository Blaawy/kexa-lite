import { PeersLiveSchema } from '@/lib/schemas';
import { liveJson, proxyKexa } from '@/lib/server-kexa';

export const dynamic = 'force-dynamic';

export async function GET() {
  try {
    const upstream = await proxyKexa('/peers/live');
    if (!upstream.ok) return liveJson({ error: 'Upstream peers/live failed' }, upstream.status);
    return liveJson(PeersLiveSchema.parse(upstream.data));
  } catch {
    return liveJson({ error: 'Unable to reach KEXA node' }, 502);
  }
}
