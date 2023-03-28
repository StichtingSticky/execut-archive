/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.astro'],
  theme: {
    extend: {
      keyframes: {
        blink: {
          '0%, 100%': { opacity: 0 },
          '50%': { opacity: 1 },
        },
      },
      animation: { blink: 'blink 1s ease infinite' },
    },
    colors: {
      'white': '#fff',
      'off-white': '#fffaf8',
      'subtle-highlight': '#f2e8e9',
      'red': '#ef4955',
      'red-highlight': '#f7c0c2',
      'light-red': '#ff7276',
      'orange': '#f7974d',
      'orange-highlight': '#f8dfcb',
      'green': '#43a095',
      'green-highlight': '#b3e1dc',
      'brown': '#433032',
      'black': '#000',
    },
    fontFamily: {
      sans: `"Ubuntu", "Helvetica Neue", Helvetica, Arial, sans-serif`,
      serif: `"Lora", Georgia, serif`,
      mono: `"Ubuntu Mono", "Courier New", monospace`,
    },
  },
  plugins: [
    require('@tailwindcss/line-clamp'),
    require('@tailwindcss/typography'),
  ],
}
