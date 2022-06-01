$xrd="030000000000000000000000000000000000000000000000000004"
$account="020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a"
$privkey="7c9fa136d4413fa6173637e883b6998d32e1d675f88cddff9dcbcf331820f4b8"
$package="01e97b5ffae9aa95c7b8f8c5ba552c2aba75277a8ac22566fe70fc"
$component="028098dbc5ece69dd1158d2226656f2a06925184d59226458e0075"
$admin_badge="0334f2222b49b25ac4fac9454c5b01e0c7106492be6e23820eda47"
$ticket_nft="03a7638e27ab96fcb4c9215ebf5880105f3dd9e3be6214bc0893f9"
$account2="021b1871540abf576a9901994265ca5a46364531c272cc09616c8b"
$privkey2="23d7f42b1cdc1f0d492ebd756ed0fe8003995dda554d99418d47a81813650207"

# Write the transaction manifest
$tx=@"
# Create a proof of the admin badge
CALL_METHOD ComponentAddress("$account") "create_proof" ResourceAddress("$admin_badge");

# Call the allow_resell method
CALL_METHOD ComponentAddress("$component") "allow_resell" true;
"@

$tx | Out-File -FilePath "allow_resell.rtm" -Encoding 'ascii'
