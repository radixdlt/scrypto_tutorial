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
        gumball_nft_def: ResourceDef,
        // Vault to contain the collected XRD
        collected_xrd: Vault,
        // The price the user has to pay for a random NFT
        price_random: Decimal,
        // The price the user has to pay for a specific NFT
        price_specific: Decimal,
        // Resource definition of the admin badge, able to call the `mint_nft` method
        admin_badge: ResourceDef,
        // Vault containing the badge allowing this component to mint
        // new NFT resources
        minting_authority: Vault,
        // Keep track of the number of NFT minted
        nb_nft_minted: u128
    }

    impl BoredGumballClub {
        pub fn instantiate_club(price_random: Decimal, price_specific: Decimal) -> (Component, Bucket) {
            // Create an admin badge that will be returned to the caller
            let admin_badge: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Club admin badge")
                .initial_supply_fungible(1);

            // Create a minting authority badge, that will be kept
            // inside the component to be able to mint NFTs later
            let minting_authority: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "NFT minter authority")
                .metadata("description", "Badge that has the authority to mint new gumball NFTs")
                .initial_supply_fungible(1);

            // Initiate the resource definition of the NFTs.
            // We specify that the `minting_authority` badge, created in the previous step
            // will be able to mint this resource.
            let gumball_def: ResourceDef = ResourceBuilder::new_non_fungible()
                .metadata("name", "Bored Gumball Club NFT")
                .flags(MINTABLE)
                .badge(minting_authority.resource_address(), MAY_MINT)
                .no_initial_supply();

            // Instantiate the component
            let component = Self {
                gumball_nfts: Vault::new(gumball_def.address()),
                gumball_nft_def: gumball_def,
                collected_xrd: Vault::new(RADIX_TOKEN),
                price_random: price_random,
                price_specific: price_specific,
                admin_badge: admin_badge.resource_def(),
                minting_authority: Vault::with_bucket(minting_authority),
                nb_nft_minted: 0
            }
            .instantiate();

            (component, admin_badge)
        }

        // Admins can call this method to mint new gumball
        // NFTs with specific attributes
        #[auth(admin_badge)]
        pub fn mint_nft(&mut self, color: u32, hat: u32, eyes: u32) {
            // The id of the minted NFT will be an increasing number, starting with 1
            let nft_id = NonFungibleKey::from(self.nb_nft_minted + 1);

            // Initiate the data that the NFT will hold
            let nft_data: GumballNftData = GumballNftData{
                color: Color::from(color),
                hat: Hat::from(hat),
                eyes: Eyes::from(eyes)
            };

            // Mint a new NFT
            let new_nft = self.minting_authority.authorize(|badge| {
                return self.gumball_nft_def.mint_non_fungible(
                    &nft_id,
                    nft_data, 
                    badge
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
        pub fn buy_specific(&mut self, mut payment: Bucket, id: u128) -> (Bucket, Bucket) {
            self.collected_xrd.put(payment.take(self.price_specific));

            let nft = self.gumball_nfts.take_non_fungible(&NonFungibleKey::from(id));
            (nft, payment)
        }
    }
}
