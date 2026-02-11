import Link from 'next/link';

const links = [
  { href: '/', label: 'Home' },
  { href: '/explorer', label: 'Explorer' },
  { href: '/verify', label: 'Verify' },
  { href: '/nodes', label: 'Nodes' },
  { href: '#', label: 'Docs' },
];

export function TopNav() {
  return (
    <header className="sticky top-0 z-50 border-b bg-white/95 backdrop-blur">
      <div className="mx-auto flex h-16 max-w-6xl items-center justify-between px-4">
        <Link href="/" className="font-semibold tracking-tight">KEXA Explorer</Link>
        <nav className="flex items-center gap-4 text-sm text-slate-600">
          {links.map((link) => (
            <Link key={link.label} href={link.href} className="hover:text-slate-900">{link.label}</Link>
          ))}
        </nav>
      </div>
    </header>
  );
}
