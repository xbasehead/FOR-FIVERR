Write-Host "Starting node1..."
Start-Process -FilePath "powershell" -ArgumentList "-Command", "cd C:\CatenaMinerariiTestnet\node1; cargo run" -WindowStyle Normal
Start-Sleep -Seconds 10

Write-Host "Starting node2..."
Start-Process -FilePath "powershell" -ArgumentList "-Command", "cd C:\CatenaMinerariiTestnet\node2; cargo run" -WindowStyle Normal
Start-Sleep -Seconds 10

Write-Host "Starting node3..."
Start-Process -FilePath "powershell" -ArgumentList "-Command", "cd C:\CatenaMinerariiTestnet\node3; cargo run" -WindowStyle Normal
Write-Host "All nodes launched. Check individual windows for logs."


Write-Host "Starting node4..."
Start-Process -FilePath "powershell" -ArgumentList "-Command", "cd C:\CatenaMinerariiTestnet\node3; cargo run" -WindowStyle Normal
Write-Host "All nodes launched. Check individual windows for logs."


Write-Host "Starting node5..."
Start-Process -FilePath "powershell" -ArgumentList "-Command", "cd C:\CatenaMinerariiTestnet\node3; cargo run" -WindowStyle Normal
Write-Host "All nodes launched. Check individual windows for logs."

