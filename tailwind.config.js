module.exports = {
  purge: ['./src/**/*.{js,jsx,ts,tsx,scss}', './public/index.html'], // Include SCSS files
  darkMode: 'class', // Enable dark mode using a class
  theme: {
    extend: {
      colors: {
        primary: '#1DA1F2',
        secondary: '#14171A',
        accent: '#1ABC9C',
        light: '#F5F5F5', // Soft white for light mode
        dark: '#1A1A1A', // Soft black for dark mode
      },
      gradientColorStops: {
        'black-to-grey': ['#000000', '#D3D3D3'], // Black to light grey gradient
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};