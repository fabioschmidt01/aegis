/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        background: "#0f172a", // Slate 900
        surface: "#1e293b",    // Slate 800
        primary: "#6366f1",    // Indigo 500
        secondary: "#8b5cf6",  // Violet 500
        accent: "#06b6d4",     // Cyan 500
        warning: "#f59e0b",    // Amber 500
        error: "#ef4444",      // Red 500
        success: "#10b981",    // Emerald 500
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
        mono: ['Fira Code', 'monospace'],
      },
    },
  },
  plugins: [],
}
