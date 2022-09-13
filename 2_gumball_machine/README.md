# Gumball Machine
This example will show you how to parameterize the instantiated components by adding arguments to the instantiator function and how to accept payments in your components.

## How to run
1. save the XRD resource address: 
    ```bash
    // On Linux and Mac
    export xrd=resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag
    // On Windows Powershell
    $xrd="resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
    ```
2. Reset your environment: `resim reset`
3. Create a new account: `resim new-account` -> save into **$account**
4. Build and deploy the blueprint on the local ledger: `resim publish .` -> save into **$package**
5. Call the `instantiate_machine` function to instantiate a component with a gumball price of 25: `resim call-function $package GumballMachine instantiate_machine 25` -> save into **$component** and **$gumball**
6. Call the `buy_gumball` method by sending 25 XRD to it: `resim call-method $component buy_gumball 25,$xrd`
7. Verify that you have received a gumball: `resim show $account`