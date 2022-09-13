# Hello World
In this example, you will learn the very basic of building blueprints (smart contracts) on Radix using Scrypto. You will learn the structure of a Scrypto blueprint, how to create a new token and how to write methods so that people are able to interact with your component.

## How to run
1. Reset your environment: `resim reset`
2. Create a new account: `resim new-account` -> save into **$account**
3. Build and deploy the blueprint on the local ledger: `resim publish .` -> save into **$package**
4. Call the `instantiate_hello` function to instantiate a component: `resim call-function $package Hello instantiate_hello` -> save into **$component**
5. Call the `free_token` method on the component: `resim call-method $component free_token`
6. Verify that you received a token by running `resim show $account`