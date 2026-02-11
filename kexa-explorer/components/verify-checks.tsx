'use client';

import { useState } from 'react';
import { CheckCircle2, Loader2, XCircle } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { CopyButton } from '@/components/copy-button';

const checks = [
  '/api/kexa/health',
  '/api/kexa/tip',
  '/api/kexa/blocks?limit=1',
  '/api/kexa/peers/live',
] as const;

export function VerifyChecks() {
  const [status, setStatus] = useState<Record<string, 'idle' | 'loading' | 'pass' | 'fail'>>({});

  const run = async () => {
    for (const path of checks) {
      setStatus((prev) => ({ ...prev, [path]: 'loading' }));
      try {
        const res = await fetch(path, { cache: 'no-store' });
        setStatus((prev) => ({ ...prev, [path]: res.ok ? 'pass' : 'fail' }));
      } catch {
        setStatus((prev) => ({ ...prev, [path]: 'fail' }));
      }
    }
  };

  return (
    <div className="space-y-4">
      <Button onClick={run}>Run checks</Button>
      <ul className="space-y-2">
        {checks.map((path) => {
          const command = `curl -s http://localhost:3000${path}`;
          const current = status[path] ?? 'idle';
          return (
            <li key={path} className="flex flex-wrap items-center justify-between gap-2 rounded-xl border border-white/10 bg-black/20 p-3">
              <code className="text-xs text-slate-200">{command}</code>
              <div className="flex items-center gap-2">
                {current === 'loading' && <Loader2 className="h-4 w-4 animate-spin text-cyan-300" />}
                {current === 'pass' && <CheckCircle2 className="h-4 w-4 text-emerald-400" />}
                {current === 'fail' && <XCircle className="h-4 w-4 text-rose-400" />}
                <CopyButton value={command} label="Copy verify command" />
              </div>
            </li>
          );
        })}
      </ul>
    </div>
  );
}
