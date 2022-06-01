export xrd=030000000000000000000000000000000000000000000000000004
export account=020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a
export privkey=7c9fa136d4413fa6173637e883b6998d32e1d675f88cddff9dcbcf331820f4b8
export gumball_machine_package=014321b343a9afb325c4f664178cae0b94cc5c606480c78e6b0cde
export gumball=032eb3523ed09e791ddbfc45a24e420cfbb765841686890a0c95d0
export gumball_machine=028ec1491855e0041c6c8e9a4e3150eeafd3066a395b7b97f81046
export radiswap_package=013755e610536d0c257d658d5fa9c919048f14dd1f80a1bc35399f
export lp_token=03b6055abc12b91da980d96d53cb9e92dda0d4b0b7b976b5a564fd
export radiswap=02f20fc0ccc0cb90ffadd99f32b6a2e42c355c47a49e68c4732bff

echo "Reseting environment"
resim reset
resim new-account

echo "Setting up gumball machine"
cd ../2_gumball_machine
resim publish .
resim call-function $gumball_machine_package GumballMachine instantiate_machine 25
resim call-method $gumball_machine buy_gumball 25,$xrd
resim call-method $gumball_machine buy_gumball 25,$xrd
resim call-method $gumball_machine buy_gumball 25,$xrd

echo "Setting up RadiSwap"
cd ../5_decentralized_exchange

echo "Creating BTC token"
NEW_TOKEN=$(resim new-token-fixed --name BitCoin --symbol BTC 21000000)
export btc=$(echo "$NEW_TOKEN" | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")

resim publish .
PACKAGE_RESULT=$(resim call-function $radiswap_package Radiswap instantiate_pool 100,$btc 3,$gumball 100 0.01)
export radiswap=$(echo $PACKAGE_RESULT | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
export lp_token=$(echo $PACKAGE_RESULT | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")