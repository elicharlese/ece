// src/services/cardService.js
import stripe from '../utils/stripe';

export const createCard = async (userId) => {
    const card = await stripe.issuing.cards.create({
        cardholder: userId, // Replace with the actual cardholder ID
        currency: 'usd',
        type: 'virtual',
        spending_controls: {
            allowed_categories: ['food', 'travel'],
            blocked_categories: ['gambling'],
        },
    });
    return card;
};