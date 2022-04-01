# Hello World
In this example, you will learn the very basic of building blueprints (smart contracts) on Radix using Scrypto. You will learn the structure of a Scrypto blueprint, how to create a new token and how write methods so that people are able to interact with your component.

## Environment variables
Throughout the following steps, addresses will be generated for the account, components and tokens that are created. For simplicity, we included a script that you can run to store those addresses inside environment variables to be able to refer to them easily. To get access to those variables, run the following command:

### Linux, Mac
```bash
source var.sh
```

### Windows (command prompt)
```bash
.\var.bat
```

## How to run (Linux, Mac)
1. Reset your environment: `resim reset`
1. Create a new account: `resim new-account`
1. Build and deploy the blueprint on the local ledger: `resim publish .`
1. Call the `instantiate_hello` function to instantiate a component: `resim call-function $package Hello instantiate_hello`
1. Call the `free_token` method on the component: `resim call-method $component free_token`
1. Verify that you received a token by running `resim show $account`

## How to run (Windows)
1. Reset your environment: `resim reset`
1. Create a new account: `resim new-account`
1. Build and deploy the blueprint on the local ledger: `resim publish .`
1. Call the `instantiate_hello` function to instantiate a component: `resim call-function %package% Hello instantiate_hello`
1. Call the `free_token` method on the component: `resim call-method %component% free_token`
1. Verify that you received a token by running `resim show %account%`