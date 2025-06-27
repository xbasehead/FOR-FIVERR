Write-Host "Testing RPC call: requestTokens to node3..."
$headers = @{ "Content-Type" = "application/json" }
$body = '{\"jsonrpc\": \"2.0\", \"method\": \"requestTokens\", \"params\": [\"player1\", \"MTOSHI\"], \"id\": 1}'
Invoke-WebRequest -Uri http://127.0.0.1:3030 -Method POST -Headers $headers -Body $body -OutFile C:\CatenaMinerariiTestnet\rpc_requestTokens.log
Write-Host "Response saved to rpc_requestTokens.log"
Write-Host "Check node3 window for logs and rpc_requestTokens.log for response."
