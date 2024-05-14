import { ThirdwebProvider, ChainId } from '@thirdweb-dev/react';

function App() {
  return (
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
  );
}

export default App;