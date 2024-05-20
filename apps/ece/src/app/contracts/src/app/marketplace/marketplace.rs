use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};

// TODO: Spinup the contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // Define necessary fields for the contract 
    owner_id: AccountId,
    nft_factory: NFTFactory,
    products: UnorderedMap<u64, Product>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            nft_factory: NFTFactory::default(),
            products: UnorderedMap::new(b"p".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            nft_factory: NFTFactory::default(),
            products: UnorderedMap::new(b"p".to_vec()),
        }
    }
}

// TODO: Smart contract for NFT Factory
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NFTFactory {
    nfts: UnorderedMap<u64, NFT>,
    next_id: u64,
}

impl Default for NFTFactory {
    fn default() -> Self {
        Self {
            nfts: UnorderedMap::new(b"n".to_vec()),
            next_id: 0,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NFT {
    id: u64,
    owner_id: AccountId,
    metadata: String,
}

#[near_bindgen]
impl NFTFactory {
    pub fn create_nft(&mut self, owner_id: AccountId, metadata: String) -> u64 {
        let nft = NFT {
            id: self.next_id,
            owner_id: owner_id.clone(),
            metadata,
        };
        self.nfts.insert(&self.next_id, &nft);
        self.next_id += 1;
        env::log(format!("NFT {} created for {}", nft.id, owner_id).as_bytes());
        nft.id
    }

    pub fn transfer_nft(&mut self, nft_id: u64, new_owner_id: AccountId) {
        let mut nft = self.nfts.get(&nft_id).expect("NFT not found");
        nft.owner_id = new_owner_id.clone();
        self.nfts.insert(&nft_id, &nft);
        env::log(format!("NFT {} transferred to {}", nft_id, new_owner_id).as_bytes());
    }

    pub fn get_nft(&self, nft_id: u64) -> Option<NFT> {
        self.nfts.get(&nft_id)
    }
}

// TODO: Smart Contract for Product Page
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Product {
    id: u64,
    owner_id: AccountId,
    metadata: String,
    price: Balance,
}

#[near_bindgen]
impl Product {
    pub fn get_product_details(&self) -> String {
        format!("{:?}", self)
    }
}

// TODO: Smart Contract for Product Listing
#[near_bindgen]
impl Contract {
    pub fn list_product(&mut self, id: u64, owner_id: AccountId, metadata: String, price: Balance) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only the contract owner can list products."
        );
        let product = Product {
            id,
            owner_id: owner_id.clone(),
            metadata,
            price,
        };
        self.products.insert(&id, &product);
        env::log(format!("Product {} listed by {}", id, owner_id).as_bytes());
    }

    pub fn purchase_product(&mut self, product_id: u64) {
        let buyer_id = env::predecessor_account_id();
        let product = self.products.get(&product_id).expect("Product not found");
        assert!(
            env::attached_deposit() >= product.price,
            "Not enough deposit to purchase the product."
        );
        let seller_id = product.owner_id.clone();
        self.products.remove(&product_id);
        Promise::new(seller_id).transfer(product.price);
        env::log(format!("Product {} purchased by {}", product.id, buyer_id).as_bytes());
    }

    pub fn get_product(&self, product_id: u64) -> Option<Product> {
        self.products.get(&product_id)
    }
}

// TODO: Cleanup the code