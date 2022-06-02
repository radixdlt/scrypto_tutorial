$xrd="030000000000000000000000000000000000000000000000000004"

echo "Reseting environment"
resim reset
$NEW_ACCOUNT_RESULT=resim new-account
$NEW_ACCOUNT_RESULT -join " " -match 'Account component address: (\w+)'
$account=$matches[1]

echo "Setting up gumball machine"
cd ../2_gumball_machine
$PUBLISH_GUMBALL=resim publish .
$PUBLISH_GUMBALL -join " " -match 'Success! New Package: (\w+)'
$gumball_machine_package=$matches[1]

$INSTANTIATE_RESULT1=resim call-function $gumball_machine_package GumballMachine instantiate_machine 25
$INSTANTIATE_RESULT1 -join " " -match 'Component: (\w+)'
$gumball_machine=$matches[1]
$INSTANTIATE_RESULT1 -join " " -match 'Resource: (\w+)'
$gumball=$matches[1]

resim call-method $gumball_machine buy_gumball 25,$xrd
resim call-method $gumball_machine buy_gumball 25,$xrd
resim call-method $gumball_machine buy_gumball 25,$xrd

echo "Setting up RadiSwap"
cd ../5_decentralized_exchange

echo "Creating BTC token"
$NEW_TOKEN=resim new-token-fixed --name BitCoin --symbol BTC 21000000
$NEW_TOKEN -join " " -match 'Resource: (\w+)'
$btc=$matches[1]

$PUBLISH_RADISWAP=resim publish .
$PUBLISH_RADISWAP -join " " -match 'Success! New Package: (\w+)'
$radiswap_package=$matches[1]

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