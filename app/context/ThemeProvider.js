// React
import React, { createContext, useState, useContext } from 'react';

// Create a context for the theme
const ThemeContext = createContext();

// Create a provider that will hold the state and enable it to be modified
export const ThemeProvider = ({ children }) => {
  const [theme, setTheme] = useState('light');

  const toggleTheme = () => {
    setTheme((prevTheme) => (prevTheme === 'light' ? 'dark' : 'light'));
  };

  return (
    <ThemeContext.Provider value={{ theme, toggleTheme }}>
      {children}
    </ThemeContext.Provider>
  );
};

// Create a custom hook that will allow our components to easily access and modify the theme
export const useTheme = () => useContext(ThemeContext);