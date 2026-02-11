import { PeersLiveSchema } from '@/lib/schemas';
import { liveJson, proxyKexa } from '@/lib/server-kexa';

export const dynamic = 'force-dynamic';

export async function GET() {
  try {
    const upstream = await proxyKexa('/peers');
    if (!upstream.ok) return liveJson({ supported: false });
    return liveJson({ supported: true, peers: PeersLiveSchema.parse(upstream.data) });
  } catch {
    return liveJson({ supported: false });
  }
}
