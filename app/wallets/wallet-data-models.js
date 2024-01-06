// Example Data Models for a Web3 Marketplace

// User Profile Model
class UserProfile {
  constructor(userId, name, bio, profilePicture) {
    this.userId = userId;
    this.name = name;
    this.bio = bio;
    this.profilePicture = profilePicture;
  }
}

// Product Model
class Product {
  constructor(productId, owner, title, description, price) {
    this.productId = productId;
    this.owner = owner;
    this.title = title;
    this.description = description;
    this.price = price;
  }
}

// Campaign Model for Crowdfunding
class Campaign {
  constructor(campaignId, creator, goalAmount, currentAmount, endDate) {
    this.campaignId = campaignId;
    this.creator = creator;
    this.goalAmount = goalAmount;
    this.currentAmount = currentAmount;
    this.endDate = endDate;
  }
}

export { UserProfile, Product, Campaign };
