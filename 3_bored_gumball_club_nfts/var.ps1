$xrd="030000000000000000000000000000000000000000000000000004"
$account="020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a"
$privkey="7c9fa136d4413fa6173637e883b6998d32e1d675f88cddff9dcbcf331820f4b8"
$package="0103bcea9e3a45d944105a5690b9ebcd4e950f4303150440f030bc"
$component="02df419bf4b542f411120bcfbd063313222b6cda1c417b63f89f37"
$admin_badge="03e96a2b3445f0688d42bf5415fa0386214d725f5ef8cd2c82120f"
$gumball_nft="031d5073f6878ac4794477655dcd021973bce9187c7650cbc2f600"

# Write the transaction manifest
$tx=@"
# Create a proof of the admin badge
CALL_METHOD ComponentAddress("$account") "create_proof" ResourceAddress("${admin_badge}");

# Mint three NFTs
CALL_METHOD ComponentAddress("$component") "mint_nft" Enum("Blue") Enum("Beanie") Enum("Laser");
CALL_METHOD ComponentAddress("$component") "mint_nft" Enum("Yellow") Enum("Cowboy") Enum("Sleepy");
CALL_METHOD ComponentAddress("$component") "mint_nft" Enum("Red") Enum("Party") Enum("Eyepatch");
"@

$tx | Out-File -FilePath "mint_initial_nfts.rtm" -Encoding 'ascii'
