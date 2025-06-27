# Define the Invoke-RpcCall function
function Invoke-RpcCall {
    param (
        [string]$Method,
        [array]$Params
    )
    $body = @{
        jsonrpc = "2.0"
        method = $Method
        params = $Params
        id = 1
    } | ConvertTo-Json
    try {
        $response = Invoke-RestMethod -Uri "http://127.0.0.1:3030" -Method Post -Body $body -ContentType "application/json" -TimeoutSec 5
        Write-Host "Raw response for $Method : $($response | ConvertTo-Json -Depth 4)"
    } catch {
        Write-Host "Error calling $Method : $_" -ForegroundColor Red
    }
}

# Define the array of tests based on your output
$tests = @(
    @{Method="requestTokens"; Params=@("player1", "MTOSHI")},
    @{Method="requestTokens"; Params=@("player2", "MTOSHI")},
    @{Method="requestTokens"; Params=@("player1", "Solutio")},
    @{Method="getBalance"; Params=@("player1", "MTOSHI")},
    @{Method="getPlayer"; Params=@("player1")},
    @{Method="getContentStats"; Params=@("player1")},
    @{Method="simulateActivity"; Params=@()},
    @{Method="mineBlock"; Params=@("player1", 1)},
    @{Method="recordActivity"; Params=@("player1", 4)},
    @{Method="battle"; Params=@("player1", "player2")},
    @{Method="mintNft"; Params=@("player1", "Gem1", 100)},
    @{Method="mintNft"; Params=@("player2", "Gem2", 100)},
    @{Method="tradeItems"; Params=@("player1", 0, "player2", 0)},
    @{Method="distributeMtoshiRewards"; Params=@()},
    @{Method="showLeaderboard"; Params=@()},
    @{Method="getMtoshiBalance"; Params=@("player1")},
    @{Method="mintNft"; Params=@("player1", "UniqueGem", 100)},
    @{Method="sellNft"; Params=@("player1", 0, 50)},
    @{Method="buyNft"; Params=@("player2", "player1", 0)},
    @{Method="uploadContent"; Params=@("player1", "video1", "My Video")},
    @{Method="likeContent"; Params=@("player2", "video1")},
    @{Method="commentContent"; Params=@("player2", "video1", "Great video!")},
    @{Method="viewContent"; Params=@("player2", "video1")},
    @{Method="postContent"; Params=@("player1", "post1", "My first post")},
    @{Method="likePost"; Params=@("player2", "post1", $true)},
    @{Method="commentPost"; Params=@("player2", "post1", "Nice post!")},
    @{Method="boostPost"; Params=@("player1", "post1", 1)}
)

# Run the tests with 2-second delays
Write-Host "Starting RPC tests with 2-second delays..."
foreach ($test in $tests) {
    Write-Host "Running test: $($test.Method) with params $($test.Params -join ', ')"
    Invoke-RpcCall -Method $test.Method -Params $test.Params
    Write-Host "Test completed. Waiting 2 seconds..."
    Start-Sleep -Seconds 2
}
Write-Host "All tests completed."