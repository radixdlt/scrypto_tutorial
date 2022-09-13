# Decentralized Exchange
In this example, we will mix everything we just learned to make a more concrete blueprint. This blueprint represents an automated marked maker (AMM) based decentralized exchange. People are able to provide and withdraw liquidity, and swap tokens.

## Setting up the environment
We will show how you can use the transaction manifest to call multiple components in the same transaction. For this example, we will build a transaction that will first buy a gumball from our gumball machine, then exchange that gumball for some BTC on RadiSwap. You can run this command to setup the gumball machine and radiswap components:

### Linux, Mac
```bash
source setup_composability.sh
```

### Windows (PowerShell)
```powershell
. .\setup_composability.ps1
```

## How to run
If you are using Linux, Mac: run the transaction manifest: `resim run composability_example.rtm`
If you are using Windows (PowerShell): run the transaction manifest: `resim run composability_example_powershell.rtm`
