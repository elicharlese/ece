import { ThirdwebSDK } from '@3rdweb/sdk';
import { ethers } from 'ethers';

// Set up your provider and signer
const provider = new ethers.providers.Web3Provider(window.ethereum);
const signer = provider.getSigner();

// Initialize the Thirdweb SDK
const sdk = new ThirdwebSDK(signer);

// Crowdfunding Interface
class CrowdfundingInterface {
  constructor(crowdfundingModuleAddress) {
    this.crowdfundingModule = sdk.getCrowdfundingModule(crowdfundingModuleAddress);
  }

  async createCampaign(data) {
    const tx = await this.crowdfundingModule.createCampaign(data);
    return tx;
  }

  async contribute(campaignId, contributionAmount) {
    const tx = await this.crowdfundingModule.contribute(campaignId, contributionAmount);
    return tx;
  }

  async withdrawFunds(campaignId) {
    const tx = await this.crowdfundingModule.withdrawFunds(campaignId);
    return tx;
  }

  // ... other crowdfunding related methods ...
}

export default CrowdfundingInterface;
