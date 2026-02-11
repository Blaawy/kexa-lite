import type { Metadata } from 'next';
import './globals.css';
import { TopNav } from '@/components/nav';
import { BackgroundVideo } from '@/components/background-video';
import { PageTransition } from '@/components/page-transition';

export const metadata: Metadata = {
  title: 'KEXA Explorer',
  description: 'Professional mainnet blockchain explorer for KEXA.',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <BackgroundVideo />
        <div className="relative z-10 min-h-screen">
          <TopNav />
          <main className="mx-auto max-w-7xl px-4 py-8 sm:px-6">
            <PageTransition>{children}</PageTransition>
          </main>
        </div>
      </body>
    </html>
  );
}
