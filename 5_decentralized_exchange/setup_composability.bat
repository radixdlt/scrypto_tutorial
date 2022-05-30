set xrd=030000000000000000000000000000000000000000000000000004
set account=020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a
set privkey=7c9fa136d4413fa6173637e883b6998d32e1d675f88cddff9dcbcf331820f4b8
set gumball_machine_package=016954696b35674c6c5bc6c6ad47a157008a8d7f44b65a38ce747e
set btc=03e94785cf450fe791629d2a5bd1d12d27caa326fe95c98fdba664
set gumball=03b73078a3a8f99e1309253dce86a6f30c8eae43c9bd29773a173a
set gumball_machine=02aeb1c8d719bee20e4d2101840182ee8235fd31b83474ead99560
set radiswap_package=013755e610536d0c257d658d5fa9c919048f14dd1f80a1bc35399f
set lp_token=03e524cd8c4d7a827ec3011f8c88d3d1ca12e771e454bc0af6ec3d
set radiswap=027051b0cf755c982ed41561766dc24f450160d4dd90d995bc1c00

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