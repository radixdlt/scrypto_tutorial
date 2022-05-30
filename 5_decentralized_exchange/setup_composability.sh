export xrd=030000000000000000000000000000000000000000000000000004
export account=020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a
export privkey=7c9fa136d4413fa6173637e883b6998d32e1d675f88cddff9dcbcf331820f4b8
export gumball_machine_package=016954696b35674c6c5bc6c6ad47a157008a8d7f44b65a38ce747e
export gumball=03b73078a3a8f99e1309253dce86a6f30c8eae43c9bd29773a173a
export gumball_machine=02aeb1c8d719bee20e4d2101840182ee8235fd31b83474ead99560
export radiswap_package=013755e610536d0c257d658d5fa9c919048f14dd1f80a1bc35399f
export lp_token=03b6055abc12b91da980d96d53cb9e92dda0d4b0b7b976b5a564fd
export radiswap=02f20fc0ccc0cb90ffadd99f32b6a2e42c355c47a49e68c4732bff

echo "Reseting environment"
resim reset
resim new-account
echo "Creating BTC token"
export NEW_TOKEN=$(resim new-token-fixed --name BitCoin --symbol BTC 21000000)
export btc=$(echo "$NEW_TOKEN" | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")

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