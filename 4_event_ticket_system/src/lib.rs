use scrypto::prelude::*;
use sha2::{Digest, Sha256};

// Here, we define the data that will be present in
// each of the ticket NFTs. Note that only the "used" data will
// be updateable
#[derive(NonFungibleData)]
struct EventTicket {
    seat_row: u16,
    seat_number: u16,
    #[scrypto(mutable)]
    used: bool
}

blueprint! {
    struct EventTicketSystem {
        ticket_nft_address: ResourceAddress,
        token_admin: Vault,
        price: Decimal,
        event_start: u64,
        collected_xrd: Vault,
    }

    impl EventTicketSystem {
        pub fn instantiate_system(price: Decimal, event_start_epoch: u64) -> (ComponentAddress, Bucket) {
            // Create a component_admin badge. This will allow the presenter to call
            // the "allow_resell" method
            let component_admin: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Event Ticket System Admin")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(1);

            // Create the token_admin badge. This will be store on the component's vault 
            // and will allow it to do some actions on the ticket NFTs
            let token_admin: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(1);

            // Create the non fungible resource that will represent the tickets
            let ticket_nft: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Event Ticket")
                .mintable(rule!(require(token_admin.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(token_admin.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), MUTABLE(rule!(require(token_admin.resource_address()))))
                .no_initial_supply();

            // Define the access rules of that component.
            // People presenting the component_admin badge will be able to
            // call the allow_resell method
            let access_rules = AccessRules::new()
                .method("allow_resell", rule!(require(component_admin.resource_address())))
                .default(rule!(allow_all));
                
            // Instantiate the component with a state
            let mut component = Self {
                ticket_nft_address: ticket_nft,
                token_admin: Vault::with_bucket(token_admin),
                price,
                event_start: event_start_epoch,
                collected_xrd: Vault::new(RADIX_TOKEN)
            }
            .instantiate();
            component.add_access_check(access_rules);

            (component.globalize(), component_admin)
        }

        // An admin can call this method to allow people to withdraw the
        // ticket NFT from their account
        pub fn allow_resell(&self, allow_resell: bool) {
            // Get either the rule "allow_all" or "deny_all" depending on the allow_resell parameter
            let rule = if allow_resell { 
                rule!(allow_all) 
            } else { 
                rule!(deny_all) 
            };

            // Update the withdrawable flag globally
            self.token_admin.authorize(|| {
                borrow_resource_manager!(self.ticket_nft_address)
                    .set_withdrawable(rule)
            });
        }

        // Allow someone to enter the event. The "used" flag on their ticket NFT will
        // be updated
        pub fn enter_event(&self, ticket: Proof) {
            let validated_ticket: ValidatedProof = ticket.validate_proof(
                ProofValidationMode::ValidateResourceAddress(self.ticket_nft_address)
            )
            .expect("That's not a ticket NFT!");

            assert!(Runtime::current_epoch() >= self.event_start, "The event has not started yet");

            // Get the data associated with the ticket NFT and update the "used" state
            let non_fungible: NonFungible<EventTicket> = validated_ticket.non_fungible();
            let mut ticket_data = non_fungible.data();

            assert!(!ticket_data.used, "You already used this ticket!");

            ticket_data.used = true;

            // Update the data on that NFT globally
            self.token_admin.authorize(|| {
                borrow_resource_manager!(self.ticket_nft_address).update_non_fungible_data(&non_fungible.id(), ticket_data)
            })
        }

        // Allow someone to buy a ticket NFT
        pub fn buy_ticket(&self, payment: Bucket, seat_row: u16, seat_number: u16) -> (Bucket, Bucket) {
            assert!(Runtime::current_epoch() < self.event_start, "The event is already started!");
            
            // Create a hash of the seat row and seat number to make sure no one can buy
            // the same seat ticket
            let mut hasher = Sha256::new();
            hasher.update(seat_row.to_string());
            hasher.update(seat_number.to_string());
            let seat_row_hash = hasher.finalize();

            let ticket_id = NonFungibleId::from_bytes(seat_row_hash.to_vec());

            // Create a ticket NFT. Note that if someone already got a ticket at this specific
            // seat row and seat number, the transaction will fail because there cannot be two NFTs with
            // the same ID.
            let ticket = self.token_admin.authorize(|| {
                                borrow_resource_manager!(self.ticket_nft_address)
                                    .mint_non_fungible(&ticket_id, EventTicket { seat_row, seat_number, used: false })
                            });

            // Return the ticket and the user's change
            (ticket, payment)
        }
    }
}