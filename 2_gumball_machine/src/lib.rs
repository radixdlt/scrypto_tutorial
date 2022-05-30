use scrypto::prelude::*;

blueprint! {
    struct GumballMachine {
        // Vault to store the remaining gumballs
        gumballs: Vault,
        // Vault to store the XRD payments
        collected_xrd: Vault,
        // Represents the price to buy a gumball
        price: Decimal
    }

    impl GumballMachine {
        pub fn instantiate_machine(price: Decimal) -> ComponentAddress {
            // Create a new gumball resource
            let gumballs: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Gumball")
                .metadata("symbol", "GUM")
                .initial_supply(300);

            // Instantiate a new component and return it
            Self {
                gumballs: Vault::with_bucket(gumballs),
                collected_xrd: Vault::new(RADIX_TOKEN),
                price: price
            }
            .instantiate().globalize()
        }

        // Allow users to buy a gumball by providing enough XRD.
        // Returns a single gumball and the remaining of the payment bucket (the change)
        pub fn buy_gumball(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            info!("Buying a gumball!");
            // Take a portion of the payment bucket depending on the price
            let our_share = payment.take(self.price);

            // Insert the portion of XRD inside the collected_xrd vault
            self.collected_xrd.put(our_share);

            // Take a single gumball
            let gumball: Bucket = self.gumballs.take(1);

            // Return the gumball and the XRD remaining in 
            // the payment bucket (if the user sent too much)
            (gumball, payment)
        }
    }
}
