$ErrorActionPreference = "Stop"
Write-Host "Running JSON-RPC tests for node3..."

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
        return $response.result
    } catch {
        Write-Host "Error calling $Method : $_" -ForegroundColor Red
        throw
    }
}

# Initialize players with sufficient MTOSHI
Write-Host "Initializing players..."
$result = Invoke-RpcCall -Method "requestTokens" -Params @("player1", "MTOSHI")
if ($result.TokensRequested -and $result.TokensRequested.amount -eq 100) {
    Write-Host "Player1 initialized with MTOSHI" -ForegroundColor Green
} else {
    Write-Host "Player1 initialization failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}
$result = Invoke-RpcCall -Method "requestTokens" -Params @("player2", "MTOSHI")
if ($result.TokensRequested -and $result.TokensRequested.amount -eq 100) {
    Write-Host "Player2 initialized with MTOSHI" -ForegroundColor Green
} else {
    Write-Host "Player2 initialization failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test requestTokens
Write-Host "Testing requestTokens..."
$result = Invoke-RpcCall -Method "requestTokens" -Params @("player1", "Solutio")
if ($result.TokensRequested -and $result.TokensRequested.amount -eq 100) {
    Write-Host "requestTokens passed" -ForegroundColor Green
} else {
    Write-Host "requestTokens failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test getBalance
Write-Host "Testing getBalance..."
$result = Invoke-RpcCall -Method "getBalance" -Params @("player1", "MTOSHI")
if ($result.Balance -and $result.Balance.balance -ge 100) {
    Write-Host "getBalance passed" -ForegroundColor Green
} else {
    Write-Host "getBalance failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test getPlayer
Write-Host "Testing getPlayer..."
$result = Invoke-RpcCall -Method "getPlayer" -Params @("player1")
if ($result.Player -and $result.Player.data.MTOSHI -ge 100) {
    Write-Host "getPlayer passed" -ForegroundColor Green
} else {
    Write-Host "getPlayer failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test getContentStats
Write-Host "Testing getContentStats..."
$result = Invoke-RpcCall -Method "getContentStats" -Params @("player1")
if ($result.ContentStats) {
    Write-Host "getContentStats passed" -ForegroundColor Green
} else {
    Write-Host "getContentStats failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test simulateActivity
Write-Host "Testing simulateActivity..."
$result = Invoke-RpcCall -Method "simulateActivity" -Params @()
if ($result.ActivitySimulated) {
    Write-Host "simulateActivity passed" -ForegroundColor Green
} else {
    Write-Host "simulateActivity failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test mineBlock
Write-Host "Testing mineBlock..."
$result = Invoke-RpcCall -Method "mineBlock" -Params @("player1", 1)
if ($result.BlockMined) {
    Write-Host "mineBlock passed" -ForegroundColor Green
} else {
    Write-Host "mineBlock failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test recordActivity
Write-Host "Testing recordActivity..."
$result = Invoke-RpcCall -Method "recordActivity" -Params @("player1", 4)
if ($result.ActivityRecorded -and $result.ActivityRecorded.points -eq 20) {
    Write-Host "recordActivity passed" -ForegroundColor Green
} else {
    Write-Host "recordActivity failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test battle
Write-Host "Testing battle..."
$result = Invoke-RpcCall -Method "battle" -Params @("player1", "player2")
if ($result.BattleResult) {
    Write-Host "battle passed" -ForegroundColor Green
} else {
    Write-Host "battle failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test tradeItems
Write-Host "Testing tradeItems..."
$result = Invoke-RpcCall -Method "mintNft" -Params @("player1", "Gem1", 100)
if ($result.NftMinted) {
    Write-Host "mintNft for tradeItems passed" -ForegroundColor Green
} else {
    Write-Host "mintNft for tradeItems failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}
$result = Invoke-RpcCall -Method "mintNft" -Params @("player2", "Gem2", 100)
if ($result.NftMinted) {
    Write-Host "mintNft for tradeItems passed" -ForegroundColor Green
} else {
    Write-Host "mintNft for tradeItems failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}
$result = Invoke-RpcCall -Method "tradeItems" -Params @("player1", 0, "player2", 0)
if ($result.ItemsTraded -and $result.ItemsTraded.success) {
    Write-Host "tradeItems passed" -ForegroundColor Green
} else {
    Write-Host "tradeItems failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test distributeMtoshiRewards
Write-Host "Testing distributeMtoshiRewards..."
$result = Invoke-RpcCall -Method "distributeMtoshiRewards" -Params @()
if ($result.MtoshiDistributed -and $result.MtoshiDistributed.total_reward -eq 500) {
    Write-Host "distributeMtoshiRewards passed" -ForegroundColor Green
} else {
    Write-Host "distributeMtoshiRewards failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test showLeaderboard
Write-Host "Testing showLeaderboard..."
$result = Invoke-RpcCall -Method "showLeaderboard" -Params @()
if ($result.Leaderboard -and $result.Leaderboard.players.Length -ge 2) {
    Write-Host "showLeaderboard passed" -ForegroundColor Green
} else {
    Write-Host "showLeaderboard failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test getMtoshiBalance
Write-Host "Testing getMtoshiBalance..."
$result = Invoke-RpcCall -Method "getMtoshiBalance" -Params @("player1")
if ($result.MtoshiBalance -and $result.MtoshiBalance.balance -ge 0) {
    Write-Host "getMtoshiBalance passed" -ForegroundColor Green
} else {
    Write-Host "getMtoshiBalance failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test mintNft
Write-Host "Testing mintNft..."
$result = Invoke-RpcCall -Method "mintNft" -Params @("player1", "UniqueGem", 100)
if ($result.NftMinted -and $result.NftMinted.value -eq 100) {
    Write-Host "mintNft passed" -ForegroundColor Green
} else {
    Write-Host "mintNft failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test sellNft
Write-Host "Testing sellNft..."
$result = Invoke-RpcCall -Method "sellNft" -Params @("player1", 0, 50)
if ($result.NftListed -and $result.NftListed.price -eq 50) {
    Write-Host "sellNft passed" -ForegroundColor Green
} else {
    Write-Host "sellNft failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test buyNft
Write-Host "Testing buyNft..."
$result = Invoke-RpcCall -Method "buyNft" -Params @("player2", "player1", 0)
if ($result.NftBought -and $result.NftBought.nft_index -eq 0) {
    Write-Host "buyNft passed" -ForegroundColor Green
} else {
    Write-Host "buyNft failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test uploadContent
Write-Host "Testing uploadContent..."
$result = Invoke-RpcCall -Method "uploadContent" -Params @("player1", "video1", "My Video")
if ($result.ContentUploaded -and $result.ContentUploaded.content_id -eq "video1") {
    Write-Host "uploadContent passed" -ForegroundColor Green
} else {
    Write-Host "uploadContent failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test likeContent
Write-Host "Testing likeContent..."
$result = Invoke-RpcCall -Method "likeContent" -Params @("player2", "video1")
if ($result.ContentLiked -and $result.ContentLiked.content_id -eq "video1") {
    Write-Host "likeContent passed" -ForegroundColor Green
} else {
    Write-Host "likeContent failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test commentContent
Write-Host "Testing commentContent..."
$result = Invoke-RpcCall -Method "commentContent" -Params @("player2", "video1", "Great video!")
if ($result.ContentCommented -and $result.ContentCommented.content_id -eq "video1") {
    Write-Host "commentContent passed" -ForegroundColor Green
} else {
    Write-Host "commentContent failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test viewContent
Write-Host "Testing viewContent..."
$result = Invoke-RpcCall -Method "viewContent" -Params @("player2", "video1")
if ($result.ContentViewed -and $result.ContentViewed.content_id -eq "video1") {
    Write-Host "viewContent passed" -ForegroundColor Green
} else {
    Write-Host "viewContent failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test postContent
Write-Host "Testing postContent..."
$result = Invoke-RpcCall -Method "postContent" -Params @("player1", "post1", "My first post")
if ($result.ContentPosted -and $result.ContentPosted.post_id -eq "post1") {
    Write-Host "postContent passed" -ForegroundColor Green
} else {
    Write-Host "postContent failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test likePost
Write-Host "Testing likePost..."
$result = Invoke-RpcCall -Method "likePost" -Params @("player2", "post1", $true)
if ($result.PostLiked -and $result.PostLiked.paid -eq $true) {
    Write-Host "likePost passed" -ForegroundColor Green
} else {
    Write-Host "likePost failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test commentPost
Write-Host "Testing commentPost..."
$result = Invoke-RpcCall -Method "commentPost" -Params @("player2", "post1", "Nice post!")
if ($result.PostCommented -and $result.PostCommented.post_id -eq "post1") {
    Write-Host "commentPost passed" -ForegroundColor Green
} else {
    Write-Host "commentPost failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

# Test boostPost
Write-Host "Testing boostPost..."
$result = Invoke-RpcCall -Method "boostPost" -Params @("player1", "post1", 1)
if ($result.PostBoosted -and $result.PostBoosted.tokens -eq 1) {
    Write-Host "boostPost passed" -ForegroundColor Green
} else {
    Write-Host "boostPost failed: $($result | ConvertTo-Json -Depth 4)" -ForegroundColor Red
    exit 1
}

Write-Host "All JSON-RPC tests passed!" -ForegroundColor Green