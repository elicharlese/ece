module.exports = {
  purge: ['./src/**/*.{js,jsx,ts,tsx,scss}', './public/index.html'], // Include SCSS files
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      colors: {
        primary: '#1DA1F2',
        secondary: '#14171A',
        accent: '#1ABC9C',
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};