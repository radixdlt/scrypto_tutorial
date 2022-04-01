# Gumball Machine
This example will show you how to parameterize the instantiated components by adding arguments to the instantiator function and how to accept payments in your components.

## Environment variables
Throughout the following steps, addresses will be generated for the account, components and tokens that are created. For simplicity, we included a script that you can run to store those addresses inside environment variables to be able to refer to them easily. To get access to those variables, run the following command:

### Linux, Mac
```bash
source var.sh
```

### Windows (Command prompt)
```bash
.\var.bat
```

## How to run (Linux, Mac)
1. Reset your environment: `resim reset`
1. Create a new account: `resim new-account`
1. Build and deploy the blueprint on the local ledger: `resim publish .`
1. Call the `instantiate_machine` function to instantiate a component with a gumball price of 25: `resim call-function $package GumballMachine instantiate_machine 25`
1. Call the `buy_gumball` method by sending 25 XRD to it: `resim call-method $component buy_gumball 25,$xrd`
1. Verify that you have received a gumball: `resim show $account`

## How to run (Windows)
1. Reset your environment: `resim reset`
1. Create a new account: `resim new-account`
1. Build and deploy the blueprint on the local ledger: `resim publish .`
1. Call the `instantiate_machine` function to instantiate a component with a gumball price of 25: `resim call-function %package% GumballMachine instantiate_machine 25`
1. Call the `buy_gumball` method by sending 25 XRD to it: `resim call-method %component% buy_gumball 25,%xrd%`
1. Verify that you have received a gumball: `resim show %account%`