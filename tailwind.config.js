const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs", "*.html"],
  theme: {
    extend: {
      colors: {
        slate: {
          800: '#1e293b',
          950: '#020617',
        },
        zinc: {
          950: '#09090b',
        },
        cyan: {
          400: '#22d3ee',
        },
        emerald: {
          400: '#34d399',
        }
      },
      fontFamily: {
        sans: ['FiraCode', ...defaultTheme.fontFamily.sans],
        mono: ['FiraCode', ...defaultTheme.fontFamily.mono],
      }
    },
  },
  plugins: [],
}
