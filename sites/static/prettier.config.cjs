/** @type {import('prettier').Config} */
module.exports = {
  semi: false,
  singleQuote: true,
  quoteProps: 'consistent',
  trailingComma: 'all',
  bracketSameLine: true,
  plugins: [
    require('prettier-plugin-astro'),
    require('prettier-plugin-tailwindcss'),
  ],
  overrides: [
    { files: '*.astro', options: { parser: 'astro' } },
  ],
}
