import type { Metadata } from 'next';
import './globals.css';
import { TopNav } from '@/components/nav';

export const metadata: Metadata = {
  title: 'KEXA Explorer',
  description: 'Professional mainnet blockchain explorer for KEXA.',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <TopNav />
        <main className="mx-auto max-w-6xl px-4 py-8">{children}</main>
      </body>
    </html>
  );
}
