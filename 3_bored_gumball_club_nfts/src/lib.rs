use scrypto::prelude::*;

mod attributes;
use attributes::Color;
use attributes::Hat;
use attributes::Eyes;

#[derive(NonFungibleData)]
struct GumballNftData {
    color: Color,
    hat: Hat,
    eyes: Eyes
}

blueprint! {
    struct BoredGumballClub {
        // Vault that will store the minted NFTs
        gumball_nfts: Vault,
        // Resource definition of the NFT
        gumball_nft_def: ResourceAddress,
        // Vault to contain the collected XRD
        collected_xrd: Vault,
        // The price the user has to pay for a random NFT
        price_random: Decimal,
        // The price the user has to pay for a specific NFT
        price_specific: Decimal,
        // Resource definition of the admin badge, able to call the `mint_nft` method
        admin_badge: ResourceAddress,
        // Vault containing the badge allowing this component to mint
        // new NFT resources
        minting_authority: Vault,
        // Keep track of the number of NFT minted
        nb_nft_minted: u64
    }

    impl BoredGumballClub {
        pub fn instantiate_club(price_random: Decimal, price_specific: Decimal) -> (ComponentAddress, Bucket) {
            // Create an admin badge that will be returned to the caller
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Club admin badge")
                .initial_supply(1);

            // Create a minting authority badge, that will be kept
            // inside the component to be able to mint NFTs later
            let minting_authority: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "NFT minter authority")
                .metadata("description", "Badge that has the authority to mint new gumball NFTs")
                .initial_supply(1);

            // Create the resource definition of the NFTs.
            // We specify that the `minting_authority` badge, created in the previous step
            // will be able to mint this resource.
            let gumball_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Bored Gumball Club NFT")
                .mintable(rule!(require(minting_authority.resource_address())), LOCKED)
                .no_initial_supply();

            // Set the access rules for this component.
            // Only someone presenting the admin_badge will be able to call
            // the "mint_nft" method.
            let access_rules = AccessRules::new()
                .method("mint_nft", rule!(require(admin_badge.resource_address())))
                .default(rule!(allow_all));
            
            // Instantiate the component
            let mut component = Self {
                gumball_nfts: Vault::new(gumball_address),
                gumball_nft_def: gumball_address,
                collected_xrd: Vault::new(RADIX_TOKEN),
                price_random: price_random,
                price_specific: price_specific,
                admin_badge: admin_badge.resource_address(),
                minting_authority: Vault::with_bucket(minting_authority),
                nb_nft_minted: 0
            }
            .instantiate();
            component.add_access_check(access_rules);

            (component.globalize(), admin_badge)
        }

        // Admins can call this method to mint new gumball
        // NFTs with specific attributes
        pub fn mint_nft(&mut self, color: Color, hat: Hat, eyes: Eyes) {
            // The id of the minted NFT will be an increasing number, starting with 1
            let nft_id = NonFungibleId::from_u64(self.nb_nft_minted + 1);

            // Initiate the data that the NFT will hold
            let nft_data: GumballNftData = GumballNftData{
                color,
                hat,
                eyes
            };

            // Mint a new NFT
            let new_nft = self.minting_authority.authorize(|| {
                return borrow_resource_manager!(self.gumball_nft_def).mint_non_fungible(
                    &nft_id,
                    nft_data,
                );
            });

            // Insert the newly minted NFT in the component's vault
            self.gumball_nfts.put(new_nft);
            self.nb_nft_minted += 1;
        }

        // Allow users to buy a random NFT stored on this component
        pub fn buy_random(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            self.collected_xrd.put(payment.take(self.price_random));
            let nft = self.gumball_nfts.take(1);

            (nft, payment)
        }

        // Allow users to buy a specific NFT store on this component
        pub fn buy_specific(&mut self, mut payment: Bucket, id: u64) -> (Bucket, Bucket) {
            self.collected_xrd.put(payment.take(self.price_specific));

            let nft = self.gumball_nfts.take_non_fungible(&NonFungibleId::from_u64(id));
            (nft, payment)
        }
    }
}
