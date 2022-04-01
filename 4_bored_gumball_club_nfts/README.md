# Bored Gumball Club NFT
In this example, we will take a look at how to create NFTs with Scrypto. This will teach you how to create mintable resources and mint them. 

## Environment variables
Throughout the following steps, addresses will be generated for the account, components and tokens that are created. For simplicity, we included a script that you can run to store those addresses inside environment variables to be able to refer to them easily. To get access to those variables, run the following command:

### Linux, Mac
```bash
source var.sh
```

### Windows (Command Prompt)
```bash
.\var.sh
```

## How to run (Linux, Mac)
1. Reset your environment: `resim reset`
1. Create a new account: `resim new-account`
1. Build and deploy the blueprint on the local ledger: `resim publish .`
1. Call the `instantiate_club` function to instantiate a component with price for random NFTs of 15 XRD and a price for specific NFTs of 30 XRD: `resim call-function $package BoredGumballClub instantiate_club 15 30`
1. By presenting the admin badge, mint three NFTs with different attributes. **Call three times**: `resim call-method $component mint_nft [color_number {1-3}] [hat_number {1-3}] [eyes_number {1-3}] 1,$admin_badge`
1. See the available NFTs stored on the component: `resim show $component`
1. Buy a random one: `resim call-method $component buy_random 15,$xrd`
1. One of the NFTs should now be in your account: `resim show $account`
1. Buy a specific one: `resim call-method $component buy_specific 30,$xrd 00000000000000000000000000000003`

## How to run (Windows)
1. Reset your environment: `resim reset`
1. Create a new account: `resim new-account`
1. Build and deploy the blueprint on the local ledger: `resim publish .`
1. Call the `instantiate_club` function to instantiate a component with price for random NFTs of 15 XRD and a price for specific NFTs of 30 XRD: `resim call-function %package% BoredGumballClub instantiate_club 15 30`
1. By presenting the admin badge, mint three NFTs with different attributes. **Call three times**: `resim call-method %component% mint_nft [color_number {1-3}] [hat_number {1-3}] [eyes_number {1-3}] 1,%admin_badge%`
1. See the available NFTs stored on the component: `resim show %component%`
1. Buy a random one: `resim call-method %component% buy_random 15,%xrd%`
1. One of the NFTs should now be in your account: `resim show %account%`
1. Buy a specific one: `resim call-method %component% buy_specific 30,%xrd% 00000000000000000000000000000003`