# Gumball Machine
This example will show you how to parameterize the instantiated components by adding arguments to the instantiator function and how to accept payments in your components.

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
1. Call the `instantiate_machine` function to instantiate a component with a gumball price of 25: `resim call-function $package GumballMachine instantiate_machine 25` -> save into **$component** and **$gumball**
1. Call the `buy_gumball` method by sending 25 XRD to it: `resim call-method $component buy_gumball 25,$xrd`
1. Verify that you have received a gumball: `resim show $account`