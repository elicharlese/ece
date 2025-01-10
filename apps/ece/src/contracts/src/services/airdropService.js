// src/services/airdropService.js
import { updateUserBalance } from './userService';

export const airdropRewards = async (userId, amount) => {
    await updateUserBalance(userId, amount);
};

// Example usage
const performAirdrop = async (userId) => {
    const airdropAmount = 100; // Define your airdrop amount
    await airdropRewards(userId, airdropAmount);
};