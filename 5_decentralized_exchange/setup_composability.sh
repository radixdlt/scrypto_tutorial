export xrd=030000000000000000000000000000000000000000000000000004
export account=0293c502780e23621475989d707cd8128e4506362e5fed6ac0c00a
export pubkey=005feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9
export gumball_machine_package=01ff3eae9463d913a0dba37b78896414eadf59ce144a9143c8018f
export btc=03ff3eae9463d913a0dba37b78896414eadf59ce144a9143c8018f
export gumball_machine_package=01d527faee6d0b91e7c1bab500c6a986e5777a25d704acc288d542
export gumball=0347dfe3a58e8a630305f2f3df82949cd70ce49e2cde097b259f8d
export gumball_machine=0239e3954c002dbc7e0157e30af6212c88ad41c8d2886240e98e76
export radiswap_package=011773788de8e4d2947d6592605302d4820ad060ceab06eb2d4711
export lp_token=03d21e9973030d9ccd35e3955f3cf42d79b8733ff22ed2b2b62a87
export radiswap=024f303486f193254b51c6350bfc130a1d8b936da3155a268f145f

echo "Reseting environment"
resim reset
resim new-account
echo "Creating BTC token"
resim new-token-fixed --name BitCoin --symbol BTC 21000000

echo "Setting up gumball machine"
cd ../2_gumball_machine
resim publish .
resim call-function $gumball_machine_package GumballMachine instantiate_machine 25
resim call-method $gumball_machine buy_gumball 25,$xrd
resim call-method $gumball_machine buy_gumball 25,$xrd
resim call-method $gumball_machine buy_gumball 25,$xrd

echo "Setting up RadiSwap"
cd ../5_decentralized_exchange
resim publish .
resim call-function $radiswap_package Radiswap instantiate_pool 100,$btc 3,$gumball 100 0.01