import { HealthSchema } from '@/lib/schemas';
import { liveJson, proxyKexa } from '@/lib/server-kexa';

export const dynamic = 'force-dynamic';

export async function GET() {
  try {
    const upstream = await proxyKexa('/health');
    if (!upstream.ok) return liveJson({ error: 'Upstream health failed' }, upstream.status);
    return liveJson(HealthSchema.parse(upstream.data));
  } catch {
    return liveJson({ ok: false, error: 'Unable to reach KEXA node' }, 502);
  }
}
