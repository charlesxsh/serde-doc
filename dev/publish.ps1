$crates = @("serde-doc")

foreach ($crate in $crates) {
    Write-Host "Publishing $crate..."
    cargo publish -p "$crate"
    Start-Sleep -Seconds 5 # Wait for a few seconds to avoid rate limiting
}

Write-Host "All crates published successfully."