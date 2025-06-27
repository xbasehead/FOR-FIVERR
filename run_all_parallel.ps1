# Define the initialization script with the Invoke-RpcCall function
$initScript = {
    function Invoke-RpcCall {
        param (
            [string]$Method,
            [array]$Params
        )
        try {
            $body = @{
                jsonrpc = "2.0"
                method = $Method
                params = $Params
                id = 1
            } | ConvertTo-Json
            $response = Invoke-RestMethod -Uri "http://127.0.0.1:3030" -Method Post -Body $body -ContentType "application/json" -TimeoutSec 5
            Write-Output "Success: $Method - Response: $($response | ConvertTo-Json -Depth 4)"
        } catch {
            Write-Output "Error: $Method - $_"
        }
    }
}

# Define the array of tests
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

# Start background jobs for each test
Write-Host "Starting all RPC calls in parallel..."
$jobs = @()
foreach ($test in $tests) {
    $job = Start-Job -InitializationScript $initScript -ScriptBlock {
        param($method, $params)
        Invoke-RpcCall -Method $method -Params $params
    } -ArgumentList $test.Method, $test.Params
    $jobs += $job
    Write-Host "Started job for $($test.Method)"
}

# Wait for all jobs to complete
Write-Host "Waiting for all jobs to complete..."
Wait-Job -Job $jobs

# Retrieve and display the output from each job
Write-Host "All jobs completed. Retrieving outputs..."
foreach ($job in $jobs) {
    Write-Host "Output for job $($job.Id) - Method: $($tests[$jobs.IndexOf($job)].Method)"
    Receive-Job -Job $job
    Remove-Job -Job $job
}
Write-Host "All RPC calls have been processed."