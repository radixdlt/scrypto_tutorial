# Gumball Machine with auth
In this example, we will add authentication on top of the previous gumball machine example. This way, the component instantiator will be able to call a method to withdraw the collected XRD. This will introduce you to the concept of badges and how to protect your methods so that only certain users can access them.

## Before running this code
Throughout the following steps, addresses will be generated for the account, components and tokens that are created. For simplicity, we included a script that you can run to store those addresses inside environment variables to be able to refer to them easily with the `resim` command or inside transaction manifests. Start by running the following command:

### Linux, Mac
```bash
source var.sh
```

### Windows (PowerShell)
```bash
. .\var.ps1
```

## How to run
1. Reset your environment: `resim reset`
1. Create a new account: `resim new-account`
1. Build and deploy the blueprint on the local ledger: `resim publish .`
1. Call the `instantiate_machine` method to instantiate a component with a gumball price of 25 XRD: `resim call-function $package GumballMachine instantiate_machine 25`
1. Call the `buy_gumball` method on the component multiple times: `resim call-method $component buy_gumball 25,$xrd`
1. See the amount of XRD stored on the component: `resim show $component`
1. Withdraw the stored XRD by running this transaction manifest: `resim run withdraw_xrd.rtm`