import { liveJson, proxyKexa } from '@/lib/server-kexa';

export const dynamic = 'force-dynamic';

export async function GET() {
  try {
    const upstream = await proxyKexa('/health');
    if (!upstream.ok) return liveJson({ ok: false, error: 'Upstream health failed' }, upstream.status);

    if (typeof upstream.data === 'string') {
      const healthy = upstream.data.trim().toLowerCase() === 'ok';
      return liveJson({ ok: healthy });
    }

    if (typeof upstream.data === 'object' && upstream.data !== null && 'ok' in upstream.data) {
      return liveJson(upstream.data);
    }

    return liveJson({ ok: true });
  } catch {
    return liveJson({ ok: false, error: 'Unable to reach KEXA node' }, 502);
  }
}
