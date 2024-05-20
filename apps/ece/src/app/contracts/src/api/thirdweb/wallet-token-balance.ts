import { useEffect, useState } from 'react';
import { ethers } from 'ethers';
import { useAddress } from '@thirdweb-dev/react';

// ERC20 Token ABI elements needed to fetch balances
const tokenAbi = [
  'function balanceOf(address owner) view returns (uint256)',
  'function decimals() view returns (uint8)',
];

const TokenBalance = ({ tokenContractAddress }) => {
  const [balance, setBalance] = useState('0');
  const userAddress = useAddress();

  useEffect(() => {
    if (!userAddress) return;

    const provider = new ethers.providers.Web3Provider(window.ethereum);
    const contract = new ethers.Contract(tokenContractAddress, tokenAbi, provider);

    const getBalance = async () => {
      const bal = await contract.balanceOf(userAddress);
      const decimals = await contract.decimals();
      setBalance(ethers.utils.formatUnits(bal, decimals));
    };

    getBalance();
  }, [userAddress, tokenContractAddress]);

  return (
    <div>
      Your Balance: {balance}
    </div>
  );
};