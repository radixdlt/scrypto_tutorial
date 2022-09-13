# Bored Gumball Club NFT
In this example, we will take a look at how to create NFTs with Scrypto. This will teach you how to create mintable resources and mint them. 

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
5. Call the `instantiate_club` function to instantiate a component with price for random NFTs of 15 XRD and a price for specific NFTs of 30 XRD: `resim call-function $package BoredGumballClub instantiate_club 15 30` -> save into **$component** and **$admin_badge**
6. By presenting the admin badge, mint three NFTs with different attributes: `resim run mint_initial_nfts.rtm`
7. See the available NFTs stored on the component: `resim show $component`
8. Buy a random one: `resim call-method $component buy_random 15,$xrd`
9. One of the NFTs should now be in your account: `resim show $account`
10. Buy a specific one: `resim call-method $component buy_specific 30,$xrd 0000000000000003`