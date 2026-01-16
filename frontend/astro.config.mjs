// @ts-check
import { defineConfig } from 'astro/config';

import svelte from '@astrojs/svelte';
import tailwindcss from '@tailwindcss/vite';

// https://astro.build/config
export default defineConfig({
  site: 'https://iberi22.github.io',
  base: '/VeedurIA-Ciudadana',
  integrations: [svelte()],

  vite: {
    plugins: [tailwindcss()]
  }
});