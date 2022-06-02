# Bored Gumball Club NFT
In this example, we will take a look at how to create NFTs with Scrypto. This will teach you how to create mintable resources and mint them. 

## How to run
1. save the XRD resource address: 
    ```bash
    // On Linux and Mac
    export xrd=030000000000000000000000000000000000000000000000000004
    // On Windows Powershell
    $xrd="030000000000000000000000000000000000000000000000000004"
    ```
1. Reset your environment: `resim reset`
1. Create a new account: `resim new-account` -> save into **$account**
1. Build and deploy the blueprint on the local ledger: `resim publish .` -> save into **$package**
1. Call the `instantiate_club` function to instantiate a component with price for random NFTs of 15 XRD and a price for specific NFTs of 30 XRD: `resim call-function $package BoredGumballClub instantiate_club 15 30` -> save into **$component** and **$admin_badge**
1. By presenting the admin badge, mint three NFTs with different attributes: `resim run mint_initial_nfts.rtm`
1. See the available NFTs stored on the component: `resim show $component`
1. Buy a random one: `resim call-method $component buy_random 15,$xrd`
1. One of the NFTs should now be in your account: `resim show $account`
1. Buy a specific one: `resim call-method $component buy_specific 30,$xrd 0000000000000003`