/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      margin: {
        'px-0_5': '0.5px',
      }
    },
  },
  plugins: [],
};
