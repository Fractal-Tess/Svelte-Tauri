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
          primary: '#e863d6',
          secondary: '#7bc3e0',
          accent: '#cd97fc',
          neutral: '#25343C',
          'base-100': '#F8F8FB',
          info: '#93BADC',
          success: '#12A168',
          warning: '#F4C857',
          error: '#DE3B4E'
        },
        dark: {
          primary: '#137cb5',
          secondary: '#e80d52',
          accent: '#f4cd73',
          neutral: '#2b2b2b',
          'base-100': '#1f1f1f',
          info: '#35B0F3',
          success: '#1B743C',
          warning: '#F38A12',
          error: '#F2215C'
        }
      }
    ]
  },
  plugins: [daisyui]
};

module.exports = config;
