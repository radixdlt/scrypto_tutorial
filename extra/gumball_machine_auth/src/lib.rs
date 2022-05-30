use scrypto::prelude::*;

blueprint! {
    struct GumballMachine {
        gumballs: Vault,
        collected_xrd: Vault,
        price: Decimal,
        // Keep track of the resource definition of the badge that can call admin methods
        admin_badge: ResourceDef
    }

    impl GumballMachine {
        // We are now returning a bucket containing the admin badge along with the
        // instantiated component
        pub fn instantiate_machine(price: Decimal) -> (Component, Bucket) {
            // Create an admin "badge"
            let admin_badge: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Gumball Machine Admin")
                .initial_supply_fungible(1);

            // Create a new gumball resource
            let gumballs: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Gumball")
                .metadata("symbol", "GUM")
                .initial_supply_fungible(300);

            let component = Self {
                gumballs: Vault::with_bucket(gumballs),
                collected_xrd: Vault::new(RADIX_TOKEN),
                price: price,
                admin_badge: admin_badge.resource_def()
            }
            .instantiate();

            // Return the component and the admin badge
            (component, admin_badge)
        }

        // This method is the same as the one in the previous example
        pub fn buy_gumball(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            let our_share = payment.take(self.price);
            self.collected_xrd.put(our_share);

            let gumball: Bucket = self.gumballs.take(1);
            (gumball, payment)
        }

        // This next line forces the caller to present the right 
        // admin badge to call this method
        #[auth(admin_badge)]
        pub fn withdraw_xrd(&mut self) -> Bucket {
            // Simply take all resources from the collected_xrd vault and
            // return them
            self.collected_xrd.take_all()
        }
    }
}
