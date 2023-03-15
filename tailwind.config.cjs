const daisyui = require('daisyui');
const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
const config = {
  content: ['src/**/*.{html,js,svelte,ts}'],
  darkMode: 'class',

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
          ...require('daisyui/src/colors/themes')['[data-theme=light]'],
          primary: '#0096FF',
          secondary: '#FF1E56',
          neutral: '#A9FFF7',
          accent: '#E3E7AF',
          'base-100': '#11151C'
        },
        dark: {
          ...require('daisyui/src/colors/themes')['[data-theme=black]'],
          primary: '#0096FF',
          secondary: '#FF1E56',
          neutral: '#A9FFF7',
          accent: '#E3E7AF',
          'base-100': '#11151C'
        }
      }
    ]
  },
  plugins: [daisyui]
};

module.exports = config;
