use scrypto::prelude::*;

blueprint! {
    struct EventTicketSystem {
    }

    impl EventTicketSystem {
        pub fn instantiate_system() -> ComponentAddress {
            Self {

            }
            .instantiate()
            .globalize()
        }
    }
}