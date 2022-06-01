$xrd="030000000000000000000000000000000000000000000000000004"
$account="020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a"
$privkey="7c9fa136d4413fa6173637e883b6998d32e1d675f88cddff9dcbcf331820f4b8"
$package="0146f81c29ad397e4d349c50fba38eb905d154bb04577fb8b52b71"
$component="02a39a04c87b4f31ccb7434802efaeb36a5364500efd65ca03b624"
$admin_badge="03d61247701187ff36e8ceb0d782aa44724af08c3f1fe0979011d2"
$gumball="03f79d2cc9776047f8adb904479271081056e78df698e913e12f3d"

# Write the transaction manifest
$tx=@"
# Create a proof of the admin badge and put in on the auth zone
CALL_METHOD ComponentAddress("$account") "create_proof" ResourceAddress("$admin_badge");

# Call the withdraw_xrd method. When calling this, it will verify that the admin badge is
# present on the auth zone
CALL_METHOD ComponentAddress("$component") "withdraw_xrd";

# Put the XRD present on the worktop back into the account
CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("$account") "deposit_batch";
"@

$tx | Out-File -FilePath "withdraw_xrd.rtm" -Encoding 'ascii'