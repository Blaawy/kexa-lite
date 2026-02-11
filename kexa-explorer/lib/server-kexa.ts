import { NextResponse } from 'next/server';

const baseUrl = process.env.KEXA_RPC_URL || 'http://127.0.0.1:18040';

export async function proxyKexa(path: string) {
  const res = await fetch(`${baseUrl}${path}`, {
    cache: 'no-store',
    headers: { Accept: 'application/json' },
  });

  const contentType = res.headers.get('content-type') || '';
  const data = contentType.includes('application/json') ? await res.json() : await res.text();

  return { ok: res.ok, status: res.status, data };
}

export function liveJson(data: unknown, status = 200) {
  return NextResponse.json(data, {
    status,
    headers: {
      'Cache-Control': 'no-store, max-age=0',
    },
  });
}
