import { cn } from '@/lib/utils';

export function StatusDot({ status }: { status: 'good' | 'warn' | 'bad' }) {
  return (
    <span
      className={cn(
        'inline-flex h-2.5 w-2.5 rounded-full',
        status === 'good' && 'bg-emerald-400 shadow-[0_0_14px_rgba(16,185,129,0.9)]',
        status === 'warn' && 'bg-amber-400 shadow-[0_0_14px_rgba(251,191,36,0.9)]',
        status === 'bad' && 'bg-rose-500 shadow-[0_0_14px_rgba(244,63,94,0.9)]',
      )}
    />
  );
}
