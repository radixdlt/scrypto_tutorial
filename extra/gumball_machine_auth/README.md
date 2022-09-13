# Gumball Machine with auth
In this example, we will add authentication on top of the previous gumball machine example. This way, the component instantiator will be able to call a method to withdraw the collected XRD. This will introduce you to the concept of badges and how to protect your methods so that only certain users can access them.

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
5. Call the `instantiate_machine` function to instantiate a component with a gumball price of 25: `resim call-function $package GumballMachine instantiate_machine 25` -> save into **$component**, **$admin_badge**, and **$gumball**
6. Call the `buy_gumball` method by sending 25 XRD to it: `resim call-method $component buy_gumball 25,$xrd`
7. See the amount of XRD stored on the component: `resim show $component`
8. Withdraw the stored XRD by running this transaction manifest: `resim run withdraw_xrd.rtm`