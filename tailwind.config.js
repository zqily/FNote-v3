/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        accent: '#22c55e', // equivalent to green-500
      }
    },
  },
  plugins: [],
}