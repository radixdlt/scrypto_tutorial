use scrypto::prelude::*;

blueprint! {
    struct GumballMachine {
        gumballs: Vault,
        collected_xrd: Vault,
        price: Decimal,
    }

    impl GumballMachine {
        // We are returning a bucket containing the admin badge along with the
        // instantiated component
        pub fn instantiate_machine(price: Decimal) -> (ComponentAddress, Bucket) {
            // Create an admin "badge"
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Gumball Machine Admin")
                .initial_supply(1);

            // Create a new gumball resource
            let gumballs: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Gumball")
                .metadata("symbol", "GUM")
                .initial_supply(300);

            // Define the access rules for the component we are instantiating
            // Callers will have to prove that they own the admin badge before calling "withdraw_xrd"
            // and everyone will be able to call the "buy_gumball" method
            let access_rules = AccessRules::new()
                .method("withdraw_xrd", rule!(require(admin_badge.resource_address())))
                .default(rule!(allow_all));

            // Instantiate the component and attach the access rules we just defined
            let mut component = Self {
                gumballs: Vault::with_bucket(gumballs),
                collected_xrd: Vault::new(RADIX_TOKEN),
                price: price
            }
            .instantiate();
            component.add_access_check(access_rules);

            // Return the component and the admin badge
            (component.globalize(), admin_badge)
        }

        // Allow someone to provide a bucket of XRD as a payment for GUM tokens
        pub fn buy_gumball(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            let our_share = payment.take(self.price);
            self.collected_xrd.put(our_share);

            let gumball: Bucket = self.gumballs.take(1);
            (gumball, payment)
        }

        pub fn withdraw_xrd(&mut self) -> Bucket {
            // Simply take all resources from the collected_xrd vault and
            // return them
            self.collected_xrd.take_all()
        }
    }
}
