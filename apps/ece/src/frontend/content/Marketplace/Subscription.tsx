import React, { useState, useEffect } from 'react';
import './Subscription.scss';

const Subscription = () => {
  const [selectedTier, setSelectedTier] = useState<string | null>(null);
  const [tokenAmount, setTokenAmount] = useState<number>(0);
  const [conversionRate, setConversionRate] = useState<number>(1); // Assume 1 ECE = 1 USD for simplicity

  useEffect(() => {
    // Fetch the conversion rate from an API or smart contract
    // setConversionRate(fetchedRate);
  }, []);

  const tiers = [
    { name: 'Basic', price: 10, perks: ['Basic Perk 1', 'Basic Perk 2'] },
    { name: 'Standard', price: 20, perks: ['Standard Perk 1', 'Standard Perk 2'] },
    { name: 'Premium', price: 50, perks: ['Premium Perk 1', 'Premium Perk 2'] },
  ];

  const handleSubscribe = (tier: string) => {
    setSelectedTier(tier);
    // Call API to subscribe using ECE tokens
  };

  const handlePurchaseTokens = () => {
    // Call API to purchase tokens
  };

  return (
    <div className="subscription-container">
      <h1>Subscribe to a Plan</h1>
      <p>Conversion Rate: 1 ECE = {conversionRate} USD</p>
      <div className="tiers">
        {tiers.map((tier) => (
          <div key={tier.name} className="tier">
            <h2>{tier.name}</h2>
            <p>Price: {tier.price} USD ({tier.price / conversionRate} ECE)</p>
            <ul>
              {tier.perks.map((perk, index) => (
                <li key={index}>{perk}</li>
              ))}
            </ul>
            <button onClick={() => handleSubscribe(tier.name)}>Subscribe</button>
          </div>
        ))}
      </div>
      <div className="pay-as-you-go">
        <h2>Pay As You Go</h2>
        <input
          type="number"
          value={tokenAmount}
          onChange={(e) => setTokenAmount(Number(e.target.value))}
          placeholder="Enter token amount"
        />
        <button onClick={handlePurchaseTokens}>Purchase Tokens</button>
      </div>
    </div>
  );
};

export default Subscription;