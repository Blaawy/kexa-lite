import type { Config } from 'tailwindcss';

const config: Config = {
  content: [
    './app/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './lib/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        border: 'hsl(220 14% 90%)',
        background: 'hsl(0 0% 100%)',
        foreground: 'hsl(222 47% 11%)',
        muted: 'hsl(210 20% 98%)',
        'muted-foreground': 'hsl(215 16% 47%)',
        primary: 'hsl(221 83% 53%)',
        'primary-foreground': 'hsl(210 40% 98%)',
      },
    },
  },
  plugins: [],
};

export default config;
