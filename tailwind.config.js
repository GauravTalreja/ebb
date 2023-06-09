/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.rs", "./static/**/*.html"],
    theme: {
        extend: {},
    },
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
    daisyui: {
        themes: true,
    }
}
