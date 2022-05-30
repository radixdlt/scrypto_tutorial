use scrypto::prelude::*;

blueprint! {
    struct Radiswap {
        /// The resource definition of LP token.
        lp_resource_def: ResourceAddress,
        /// LP tokens mint badge.
        lp_mint_badge: Vault,
        /// The reserve for token A.
        a_pool: Vault,
        /// The reserve for token B.
        b_pool: Vault,
        /// The fee to apply for every swap
        fee: Decimal
    }

    impl Radiswap {
        /// Creates a Radiswap component for token pair A/B and returns the component address
        /// along with the initial LP tokens.
        pub fn instantiate_pool(
            a_tokens: Bucket,
            b_tokens: Bucket,
            lp_initial_supply: Decimal,
            fee: Decimal,
        ) -> (ComponentAddress, Bucket) {
            // Check arguments
            assert!(
                !a_tokens.is_empty() && !b_tokens.is_empty(),
                "You must pass in an initial supply of each token"
            );
            assert!(
                fee >= Decimal::zero() && fee <= Decimal::one(),
                "Invalid fee in thousandths"
            );

            // Instantiate our LP token and mint an initial supply of them
            let lp_mint_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "LP Token Mint Auth")
                .initial_supply(1);

            let lp_tokens = ResourceBuilder::new_fungible()
                .metadata("symbol", "LP")
                .metadata("name", "Exchange LP token")
                .metadata("url", "https://radiswap.com")
                .mintable(rule!(require(lp_mint_badge.resource_address())), LOCKED)
                .burnable(rule!(require(lp_mint_badge.resource_address())), LOCKED)
                .initial_supply(lp_initial_supply);

            // Instantiate our Radiswap component
            let radiswap = Self {
                lp_resource_def: lp_tokens.resource_address(),
                lp_mint_badge: Vault::with_bucket(lp_mint_badge),
                a_pool: Vault::with_bucket(a_tokens),
                b_pool: Vault::with_bucket(b_tokens),
                fee
            }
            .instantiate();

            // Return the new Radiswap component, as well as the initial supply of LP tokens
            (radiswap.globalize(), lp_tokens)
        }

        /// Adds liquidity to this pool and return the LP tokens representing pool shares
        /// along with any remainder.
        pub fn add_liquidity(&mut self, mut a_tokens: Bucket, mut b_tokens: Bucket) -> (Bucket, Bucket) {
            // The ratio of added liquidity in existing liquidty.
            let a_ratio = a_tokens.amount() / self.a_pool.amount();
            let b_ratio = b_tokens.amount() / self.b_pool.amount();

            let (actual_ratio, remainder) = if a_ratio <= b_ratio {
                // We will claim all input token A's, and only the correct amount of token B
                self.a_pool.put(a_tokens);
                self.b_pool
                    .put(b_tokens.take(self.b_pool.amount() * a_ratio));
                (a_ratio, b_tokens)
            } else {
                // We will claim all input token B's, and only the correct amount of token A
                self.b_pool.put(b_tokens);
                self.a_pool
                    .put(a_tokens.take(self.a_pool.amount() * b_ratio));
                (b_ratio, a_tokens)
            };
            
            let lp_resource_manager = borrow_resource_manager!(self.lp_resource_def);
            let supply_to_mint = lp_resource_manager.total_supply() * actual_ratio;

            // Mint LP tokens according to the share the provider is contributing
            let lp_tokens = self.lp_mint_badge
                .authorize(|| lp_resource_manager.mint(supply_to_mint));

            // Return the LP tokens along with any remainder
            (lp_tokens, remainder)
        }

        /// Removes liquidity from this pool.
        pub fn remove_liquidity(&mut self, lp_tokens: Bucket) -> (Bucket, Bucket) {
            assert!(
                self.lp_resource_def == lp_tokens.resource_address(),
                "Wrong token type passed in"
            );

            // Calculate the share based on the input LP tokens.
            let share = lp_tokens.amount() / borrow_resource_manager!(self.lp_resource_def).total_supply();

            // Withdraw the correct amounts of tokens A and B from reserves
            let a_withdrawn = self.a_pool.take(self.a_pool.amount() * share);
            let b_withdrawn = self.b_pool.take(self.b_pool.amount() * share);

            // Burn the LP tokens received
            self.lp_mint_badge.authorize(|| {
                lp_tokens.burn();
            });

            // Return the withdrawn tokens
            (a_withdrawn, b_withdrawn)
        }

        /// Swaps token A for B, or vice versa.
        pub fn swap(&mut self, input_tokens: Bucket) -> Bucket {
            // Calculate the swap fee
            let fee_amount = input_tokens.amount() * self.fee;
            let input_amount = input_tokens.amount();

            let token_a_manager = borrow_resource_manager!(self.a_pool.resource_address());
            let token_b_manager = borrow_resource_manager!(self.b_pool.resource_address());

            if input_tokens.resource_address() == self.a_pool.resource_address() {
                // Calculate how much of token B we will return
                let b_amount = self.b_pool.amount()
                    - self.a_pool.amount() * self.b_pool.amount()
                        / (input_tokens.amount() - fee_amount + self.a_pool.amount());

                // Put the input tokens into our pool
                self.a_pool.put(input_tokens);

                info!("Exchanging {} {} for {} {}", 
                    input_amount, 
                    token_a_manager.metadata().get("symbol").unwrap(), 
                    b_amount, 
                    token_b_manager.metadata().get("symbol").unwrap());

                // Return the tokens owed
                return self.b_pool.take(b_amount);
            } else {
                // Calculate how much of token A we will return
                let a_amount = self.a_pool.amount()
                    - self.a_pool.amount() * self.b_pool.amount()
                        / (input_tokens.amount() - fee_amount + self.b_pool.amount());

                // Put the input tokens into our pool
                self.b_pool.put(input_tokens);

                info!("Exchanging {} {} for {} {}", 
                    input_amount, 
                    token_b_manager.metadata().get("symbol").unwrap(), 
                    a_amount, 
                    token_a_manager.metadata().get("symbol").unwrap());

                // Return the tokens owed
                return self.a_pool.take(a_amount);
            };
        }
    }
}