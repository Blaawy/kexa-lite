'use client';

import * as React from 'react';
import { motion } from 'framer-motion';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '@/lib/utils';

const buttonVariants = cva(
  'inline-flex items-center justify-center whitespace-nowrap rounded-xl text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-300/70 disabled:pointer-events-none disabled:opacity-50',
  {
    variants: {
      variant: {
        default: 'bg-gradient-to-r from-indigo-500 to-cyan-500 text-white shadow-neon hover:brightness-110',
        secondary: 'border border-white/20 bg-white/10 text-white hover:bg-white/20',
        ghost: 'bg-transparent text-slate-200 hover:bg-white/10',
      },
      size: {
        default: 'h-10 px-4 py-2',
        lg: 'h-11 px-6',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  },
);

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement>, VariantProps<typeof buttonVariants> {}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(({ className, variant, size, ...props }, ref) => {
  return (
    <motion.span whileHover={{ scale: 1.02 }} whileTap={{ scale: 0.98 }} className="inline-flex">
      <button className={cn(buttonVariants({ variant, size, className }))} ref={ref} {...props} />
    </motion.span>
  );
});
Button.displayName = 'Button';

export { Button, buttonVariants };
