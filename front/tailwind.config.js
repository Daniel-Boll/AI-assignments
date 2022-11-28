/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './app/**/*.{js,ts,jsx,tsx}',
    './page/**/*.{js,ts,jsx,tsx}',
    './ui/**/*.{js,ts,jsx,tsx}',
    './lib/**/*.{js,ts,jsx,tsx}',
  ],
  plugins: [require('@tailwindcss/typography'), require('daisyui')],
  daisyui: {
    themes: [
      {
        light: {
          ...require('daisyui/src/colors/themes')['[data-theme=light]'],
          'base-400': '#d9dbdb',
          'base-500': '#ced0d0',
          'base-600': '#cec6c6',
        },
        dark: {
          ...require('daisyui/src/colors/themes')['[data-theme=dark]'],
          'base-400': '#1e232c',
          'base-500': '#1e232c',
          'base-600': '#1c212a',
        },
      },
    ],
  },
};
