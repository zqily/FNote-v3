/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"], 
  theme: {
    extend: {
      colors: {
        background: '#0a0a0a', // A very dark gray, softer than pure black
        surface: '#1a1a1a',    // For cards and sidebars
        primary: '#ffffff',    // Primary text color
        secondary: '#a0a0a0',  // Muted text color
        accent: '#1ED760',     // Vibrant green for interactive elements
        'accent-hover': '#1DB954', // A slightly darker green for hover states
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
      },
    },
  },
  plugins: [],
};