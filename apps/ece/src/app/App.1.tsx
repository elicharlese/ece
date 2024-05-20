import React from 'react';
import { ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import { ThirdwebProvider, ChainId } from '@thirdweb-dev/react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import theme from './path/to/your/theme';
import Contribute from './frontend/content/Contribute/Contribute';
import ECE from './frontend/content/ECE/ECE';
import Marketplace from './frontend/content/Marketplace/Marketplace';
import CookiesBanner from './CookiesBanner';

const App: React.FC = () => {
    return (
        <ThemeProvider theme={theme}>
            <CssBaseline />
            <ThirdwebProvider desiredChainId={ChainId.Mainnet}>
                <Router>
                    <header>
                        {/* Metadata: This is where we will include... */}
                        {/* Redux & Cookies */}
                    </header>
                    <main>
                        <Routes>
                            <Route path="/contribute" element={<Contribute />} />
                            <Route path="/ece" element={<ECE />} />
                            <Route path="/marketplace" element={<Marketplace />} />
                            {/* Add more routes as needed */}
                        </Routes>
                    </main>
                    <CookiesBanner />
                    <footer>
                        <p>&copy; 2023 My App. All rights reserved.</p>
                        {/* Scripts: Pixels & Schema Pull */}
                    </footer>
                </Router>
            </ThirdwebProvider>
        </ThemeProvider>
    );
};

export default App;