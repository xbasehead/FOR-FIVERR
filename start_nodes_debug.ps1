# Clean up any existing node processes
Write-Host "Checking for existing node processes..."
$runningNodes = Get-Process | Where-Object { $_.Path -like "*node*.exe" }
if ($runningNodes) {
    Write-Host "Terminating existing node processes..."
    $runningNodes | ForEach-Object {
        Write-Host "Killing process with PID $(.Id)"
        taskkill /PID $_.Id /F
    }
    Start-Sleep -Seconds 5
}

# Start each node with a 20-second delay
Write-Host "Starting node1..."
Start-Process -FilePath "powershell" -ArgumentList "-NoExit", "-Command", "cd C:\CatenaMinerariiTestnet\node1; cargo run --release | Tee-Object -FilePath node1.log -Append; Read-Host 'Press Enter to exit node1'" -WindowStyle Normal
Start-Sleep -Seconds 20

Write-Host "Starting node2..."
Start-Process -FilePath "powershell" -ArgumentList "-NoExit", "-Command", "cd C:\CatenaMinerariiTestnet\node2; cargo run --release | Tee-Object -FilePath node2.log -Append; Read-Host 'Press Enter to exit node2'" -WindowStyle Normal
Start-Sleep -Seconds 20

Write-Host "Starting node3..."
Start-Process -FilePath "powershell" -ArgumentList "-NoExit", "-Command", "cd C:\CatenaMinerariiTestnet\node3; cargo run --release | Tee-Object -FilePath node3.log -Append; Read-Host 'Press Enter to exit node3'" -WindowStyle Normal
Start-Sleep -Seconds 20

Write-Host "Starting node4..."
Start-Process -FilePath "powershell" -ArgumentList "-NoExit", "-Command", "cd C:\CatenaMinerariiTestnet\node4; cargo run --release | Tee-Object -FilePath node4.log -Append; Read-Host 'Press Enter to exit node4'" -WindowStyle Normal
Start-Sleep -Seconds 20

Write-Host "Starting node5..."
Start-Process -FilePath "powershell" -ArgumentList "-NoExit", "-Command", "cd C:\CatenaMinerariiTestnet\node5; cargo run --release | Tee-Object -FilePath node5.log -Append; Read-Host 'Press Enter to exit node5'" -WindowStyle Normal
Start-Sleep -Seconds 20

Write-Host "All nodes launched. Check individual windows for logs."
