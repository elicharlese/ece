import React from 'react';

interface TokenBalanceProps {
  balance: number;
  symbol: string;
}

const TokenBalanceWidget: React.FC<TokenBalanceProps> = ({ balance, symbol }) => (
  <div>
    <span>{`Balance: ${balance} ${symbol}`}<TokenBalancetokenContractAddress="YOUR_ERC20_CONTRACT_ADDRESS_HERE" /></span>
    
  </div>
);

export default TokenBalance;