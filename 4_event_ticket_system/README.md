# Event ticket system
In this example, you will learn how to create more complex resources by building an event ticket system. You will create a non fungible resource representing an event ticket where the token administrator will be able to update its NFT data and set whether people are able to move it out of their wallets to sell it or not.

## How to run
1. save the XRD resource address: 
    ```bash
    // On Linux and Mac
    export xrd=resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag
    // On Windows Powershell
    $xrd="resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
    ```
2. Reset your environment: `resim reset`
3. Create a new account: `resim new-account` -> save into **$account**
4. Build and deploy the blueprint on the local ledger: `resim publish .` -> save into **$package**

### Buying a ticket
1. Instantiate a component with a price of 100 XRD and an event start epoch of 200: `resim call-function $package EventTicketSystem instantiate_system 100 200` -> save into **$component, $admin_badge and $ticket_nft**
2. Buy a ticket at row 10 and seat number 10: `resim call-method $component buy_ticket 100,$xrd 10 10`
3. Try to buy it again, you should see an error because the seat is not available anymore.

### Sending the ticket to someone else
1. Create another account: `resim new-account` -> save into **$account2 and $privkey2**
2. Try to transfer them your ticket NFT: `resim transfer 1 $ticket_nft $account2`. You should see an error because the resource flag `restrict_withdraw` was set to `deny_all`.
3. By showing a proof of the admin badge, call the `allow_resell` method: `resim run allow_resell.rtm`
4. Try to transfer the NFT again. You can verify that it worked by looking at the resources in your account: `resim show $account`

### Entering the event
1. Set account 2 as the default account: `resim set-default-account $account2 $privkey2`
2. Try to enter the event: `resim call-method $component enter_event 1,$ticket_nft`. You should see an error because the event has not started yet.
3. Increase the current epoch to 200: `resim set-current-epoch 200`
4. Call the `enter_event` method again
5. Try to call the same method again, you should see an error because you already used the ticket!