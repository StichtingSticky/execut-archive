import { defineConfig } from 'astro/config'

// Import Astro integrations
import compress from 'astro-compress'
import critters from 'astro-critters'
import sitemap from '@astrojs/sitemap'
import tailwind from '@astrojs/tailwind'
import turbolinks from '@astrojs/turbolinks'

// https://astro.build/config
export default defineConfig({
  // Make use of the new `<Image />` component
  experimental: { assets: true },

  // The production site will be hosted here
  site: 'https://execut.nl/',
  integrations: [
    sitemap(),
    tailwind(),
    turbolinks(),

    // Apply the `critters` integration after TailwindCSS
    critters(),

    // Apply the `compress` integration last for best optimizations
    compress(),
  ],
  image: { service: 'astro/assets/services/sharp' },
})
