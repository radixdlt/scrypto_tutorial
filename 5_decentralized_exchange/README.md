# Decentralized Exchange
In this example, we will mix everything we just learned to make a more concrete blueprint. This blueprint represents an automated marked maker (AMM) based decentralized exchange. People are able to provide and withdraw liquidity, and swap tokens.

## How to run (Linux, Mac)
We will show how you can use the transaction manifest to call multiple components in the same transaction. For this example, we will build a transaction that will first buy a gumball from our gumball machine, then exchange that gumball for some BTC on RadiSwap.

1. Run `source setup_composability.sh`. This will do a general setup and instantiate the gumball machine along with the Radiswap component to prepare your environment 
2. Run the transaction manifest: `resim run composability_example.rtm`

---

## How to run (Windows)
We will show how you can use the transaction manifest to call multiple components in the same transaction. For this example, we will build a transaction that will first buy a gumball from our gumball machine, then exchange that gumball for some BTC on RadiSwap.

1. Run `.\setup_composability.bat`. This will do a general setup and instantiate the gumball machine along with the Radiswap component to prepare your environment 
2. Run the transaction manifest: `resim run composability_example.rtm`
