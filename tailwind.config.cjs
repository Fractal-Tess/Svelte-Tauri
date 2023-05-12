const defaultTheme = require('tailwindcss/defaultTheme');
const heropatterns = require('tailwindcss-hero-patterns/src/patterns');

/** @type {import('tailwindcss').Config} */
const config = {
  content: ['src/**/*.{html,js,svelte,ts}'],
  darkMode: 'class',

  theme: {
    extend: {
      fontFamily: {
        sans: ['roboto', ...defaultTheme.fontFamily.sans]
      }
    },
    heroPatternsOpacities: ['10'],
    heroPatterns: {
      topography: heropatterns.topography
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
  plugins: [require('daisyui'), require('tailwindcss-hero-patterns')]
};

module.exports = config;
