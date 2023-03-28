import { defineCollection, image, z } from 'astro:content'

const articles = defineCollection({
  schema: z.object({
    portrait: image(),
    name: z.string(),
    role: z.string(),
  }),
})

const partners = defineCollection({
  schema: z.object({
    name: z.string(),
    industry: z.string(),
    logo: image(),
    tier: z.enum(['Bronze', 'Silver', 'Gold', 'Platinum']),
    contact: z.object({
      website: z.string().url().optional(),
      mail: z.string().email().optional(),
      socials: z.object({
        facebook: z.string().url().optional(),
        github: z.string().url().optional(),
        glassdoor: z.string().url().optional(),
        instagram: z.string().url().optional(),
        linkedin: z.string().url().optional(),
        mastodon: z.string().url().optional(),
        twitter: z.string().url().optional(),
        youtube: z.string().url().optional(),
      }),
    }),
  }),
})

const speakers = defineCollection({
  schema: z.object({
    name: z.string(),
    description: z.string(),
    portrait: image(),
    host: z.boolean().optional().transform((val) => !!val)
  }),
})

const talks = defineCollection({
  schema: z.object({
    title: z.string(),
    time: z.string(),
  }),
})

const workshops = defineCollection({
  schema: z.object({
    title: z.string(),
    time: z.string(),
  }),
})

export const collections = {
  articles,
  partners,
  speakers,
  talks,
  workshops,
}
