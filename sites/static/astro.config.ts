import { defineConfig } from 'astro/config'

// Import Astro integrations
import compress from 'astro-compress'
import critters from 'astro-critters'
import icon from 'astro-icon'
import sitemap from '@astrojs/sitemap'
import tailwind from '@astrojs/tailwind'
import turbolinks from '@astrojs/turbolinks'

// https://astro.build/config
export default defineConfig({
  // The production site will be hosted here
  site: 'https://execut.nl/',
  integrations: [
    icon(),
    sitemap(),
    tailwind(),
    turbolinks(),

    // Apply the `critters` integration after TailwindCSS
    critters(),

    // Apply the `compress` integration last for best optimizations
    compress(),
  ],
})
