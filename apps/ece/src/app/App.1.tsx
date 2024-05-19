import { ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline'; // Optional, for baseline CSS reset
import { ThirdwebProvider, ChainId } from '@thirdweb-dev/react';
import theme from './path/to/your/theme'; // Adjust the path accordingly

export function App() {
    return (
        <ThemeProvider theme={theme}>
            <CssBaseline />
            <ThirdwebProvider desiredChainId={ChainId.Mainnet}>
                <header>
                    <h1>Welcome to My App</h1>
                </header>
                <main>
                    {/* Main content goes here */}
                </main>
                <footer>
                    <p>&copy; 2023 My App. All rights reserved.</p>
                </footer>
            </ThirdwebProvider>
        </ThemeProvider>
    );
}