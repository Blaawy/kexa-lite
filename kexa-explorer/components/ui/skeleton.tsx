'use client';

import { motion } from 'framer-motion';
import { cn } from '@/lib/utils';

export function Skeleton({ className }: { className?: string }) {
  return (
    <motion.div
      initial={{ opacity: 0.5 }}
      animate={{ opacity: [0.45, 0.95, 0.45] }}
      transition={{ duration: 1.4, repeat: Infinity, ease: 'easeInOut' }}
      className={cn('shimmer rounded-lg bg-white/10', className)}
    />
  );
}
