const daisyui = require('daisyui');
const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
const config = {
  content: ['src/**/*.{html,js,svelte,ts}'],

  theme: {
    extend: {
      fontFamily: {
        sans: ['roboto', ...defaultTheme.fontFamily.sans]
      }
    }
  },

  daisyui: {
    themes: [
      {
        light: {
          ...require('daisyui/src/colors/themes')['[data-theme=lofi]'],
          primary: '#45B1E8',
          secondary: '#fbbf24'
        },
        dark: {
          ...require('daisyui/src/colors/themes')['[data-theme=black]'],
          primary: '#45B1E8',
          secondary: '#fbbf24'
        }
      }
    ]
  },
  plugins: [daisyui]
};

module.exports = config;
