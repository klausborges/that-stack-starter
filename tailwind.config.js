/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "selector",
  content: ["./templates/**/*.html", "./templates/**/*.tera"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
