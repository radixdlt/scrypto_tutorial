# Decentralized Exchange
In this example, we will mix everything we just learned to make a more concrete blueprint. This blueprint represents an automated marked maker (AMM) based decentralized exchange. People are able to provide and withdraw liquidity, and swap tokens.

## Environment variables
Throughout the following steps, addresses will be generated for the account, components and tokens that are created. For simplicity, we included a script that you can run to store those addresses inside environment variables to be able to refer to them easily. To get access to those variables, run the following command:

### Linux, Mac
```bash
source var.sh
```

### Windows (Command Prompt)
```bash
.\var.bat
```

## How to run (Linux, Mac)
1. Reset your environment: `resim reset`
1. Create a new account: `resim new-account`
1. Create a BTC token: `resim new-token-fixed --name BitCoin --symbol BTC 21000000`
1. Build and deploy the blueprint on the local ledger: `resim publish .`
1. Instantiate a component with the following parameters:
    * A bucket of 100 000 XRD
    * A bucket to 1 000 000 BTC
    * 100 for the initial supply of the LP tokens
    * 0.01 (1%) for the trading fees

    `resim call-function $package Radiswap instantiate_pool 100000,$xrd 1000000,$btc 100 0.01`
1. Swap 100 BTC for XRD: `resim call-method $component swap 100,$btc`
1. Look at the resources in your account: `resim show $account`

### Composing multiple calls in the same transaction
We will show how you can use the transaction manifest to call multiple components in the same transaction. For this example, we will build a transaction that will first buy a gumball from our gumball machine, then exchange that gumball for some BTC on RadiSwap.

1. Setup the environment: `source setup_composability.sh`
2. Run the transaction manifest: `resim run composability_example.rtm`

---

## How to run (Windows)
1. Reset your environment: `resim reset`
1. Create a new account: `resim new-account`
1. Create a BTC token: `resim new-token-fixed --name BitCoin --symbol BTC 21000000`
1. Build and deploy the blueprint on the local ledger: `resim publish .`
1. Instantiate a component with the following parameters:
    * A bucket of 100 000 XRD
    * A bucket to 1 000 000 BTC
    * 100 for the initial supply of the LP tokens
    * 0.01 (1%) for the trading fees

    `resim call-function %package% Radiswap instantiate_pool 100000,%xrd% 1000000,%btc% 100 0.01`
1. Swap 100 BTC for XRD: `resim call-method %component% swap 100,%btc%`
1. Look at the resources in your account: `resim show %account%`

### Composing multiple calls in the same transaction
We will show how you can use the transaction manifest to call multiple components in the same transaction. For this example, we will build a transaction that will first buy a gumball from our gumball machine, then exchange that gumball for some BTC on RadiSwap.

1. Setup the environment: `.\setup_composability.bat`
2. Run the transaction manifest: `resim run composability_example.rtm`
