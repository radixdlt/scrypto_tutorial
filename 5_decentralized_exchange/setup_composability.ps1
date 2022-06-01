$xrd="030000000000000000000000000000000000000000000000000004"
$account="020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a"
$privkey="7c9fa136d4413fa6173637e883b6998d32e1d675f88cddff9dcbcf331820f4b8"
$gumball_machine_package="01ec9d4708d88ed0995819598e479cd98074e4b9da3a63515c5fd8"
$gumball="035c4a4745d8221c7dc6a7020357d60c63705bcae54dd2b09200b3"
$gumball_machine="020de54f99b14e1fd6782365b5dc60e03d9ff7799b8fec9b4bb602"
$radiswap_package="0157d554360e1d53e6aae97256a205038f73f74f67491bf3ea4715"
$lp_token="03b6055abc12b91da980d96d53cb9e92dda0d4b0b7b976b5a564fd"
$radiswap="02f20fc0ccc0cb90ffadd99f32b6a2e42c355c47a49e68c4732bff"

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
$NEW_TOKEN=resim new-token-fixed --name BitCoin --symbol BTC 21000000
$NEW_TOKEN -join " " -match 'Resource: (\w+)'
$btc=$matches[1]

resim publish .
$INSTANTIATE_RESULT=resim call-function $radiswap_package Radiswap instantiate_pool "100,$btc" "3,$gumball" 100 0.01

$INSTANTIATE_RESULT -join " " -match 'Component: (\w+)'
$radiswap=$matches[1]
$INSTANTIATE_RESULT -join " " -match 'Resource: (\w+)$'
$lp_token=$matches[1]

# Write the transaction manifest
$tx=@"
#Take 25 XRD from the account
CALL_METHOD ComponentAddress("$account") "withdraw_by_amount" Decimal("25") ResourceAddress("$xrd");

# Buy a gumball from the GumballMachine component
TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("25") ResourceAddress("$xrd") Bucket("xrd_payment");
CALL_METHOD ComponentAddress("$gumball_machine") "buy_gumball" Bucket("xrd_payment");

# Swap the gumball to BTC using the RadiSwap component
TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("1") ResourceAddress("$gumball") Bucket("gumball");
CALL_METHOD ComponentAddress("$radiswap") "swap" Bucket("gumball");

# Make sure we get more than 10 BTC
ASSERT_WORKTOP_CONTAINS_BY_AMOUNT Decimal("10") ResourceAddress("$btc");

# Deposit all resources back into the account
CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("$account") "deposit_batch";
"@

$tx | Out-File -FilePath "composability_example.rtm" -Encoding 'ascii'