$root = Resolve-Path "$PSScriptRoot/.."
$binPath = "$root/bin"
$originalPath = Get-Location

Write-Host "🚀 Starting build process..."
Write-Host "==> Building backend..."
Set-Location "$root/backend"
cargo build --release

Write-Host "==> Cleaning and creating bin directory..."
Remove-Item -Recurse -Force $binPath -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path $binPath | Out-Null

Write-Host "==> Building client frontend..."
Set-Location "$root/client"
pnpm build

Write-Host "==> Moving backend binary to bin folder..."
Move-Item "$root/backend/target/release/backend.exe" "$binPath/backend.exe"

Write-Host "==> Copying config.json to bin directory..."
Copy-Item "$root/config.json" "$binPath/config.json"

Write-Host "==> Returning to original path..."
Set-Location "$originalPath"

Write-Host "✅ Build process complete."
