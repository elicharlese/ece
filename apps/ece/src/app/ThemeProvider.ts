// React
import React, { createContext, useState, useContext, ReactNode } from 'react';
import ThemeContext from './ThemeContext';


// Define the shape of the context value
interface ThemeContextType {
  theme: string;
  toggleTheme: () => void;
}

// Create a context for the theme
const ThemeContext = createContext<ThemeContextType | undefined>(undefined);


// Create a provider that will hold the state and enable it to be modified
interface ThemeProviderProps {
  children: ReactNode;
}

export const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
  const [theme, setTheme] = useState<string>('light');

  const toggleTheme = () => {
    setTheme((prevTheme) =>
      prevTheme === 'light'
        ? 'dark'
        : prevTheme === 'dark'
        ? 'colored'
        : 'light'
    );
  };
  return (
    <ThemeContext.Provider value={{ theme, toggleTheme }}>
      {children}
    </ThemeContext.Provider>
  );
};

// Create a custom hook that will allow our components to easily access and modify the theme
export const useTheme = (): ThemeContextType => {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
};

import { createTheme } from '@mui/material/styles';

const theme = createTheme({
  palette: {
    primary: {
      main: '#1976d2',
    },
    secondary: {
      main: '#dc004e',
    },
  },
});

export default theme;