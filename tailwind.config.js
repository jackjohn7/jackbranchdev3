/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./jackbranchdev/public/**/*.html"],
  theme: {
    extend: {
      fontFamily: {
        "inter": ["Inter", "sans-serif"]
      }
    },
  },
  plugins: [
    require("@tailwindcss/forms"),
    require("@tailwindcss/typography")
  ],
}

