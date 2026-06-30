# Lattice node setup — Windows
# Run from the repo root in PowerShell

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Write-Host "  Lattice Node — Windows Setup"
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Write-Host ""

# Rust toolchain
if (Get-Command rustup -ErrorAction SilentlyContinue) {
    Write-Host "✓ Rust toolchain found"
} else {
    Write-Host "Installing Rust toolchain..."
    Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
    .\rustup-init.exe -y
    Remove-Item rustup-init.exe
    $env:PATH += ";$env:USERPROFILE\.cargo\bin"
}

Write-Host ""
Write-Host "Building lattice-node (release)..."
cargo build --release

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Write-Host "  Setup complete."
Write-Host ""
Write-Host "  Binary: target\release\lattice-node.exe"
Write-Host ""
Write-Host "  Join a remote node:"
Write-Host "    .\target\release\lattice-node.exe --name win --no-mdns \"
Write-Host "      --bootstrap-peer /dns4/<host>/tcp/<port>/p2p/<peer_id>"
Write-Host ""
Write-Host "  Behind NAT? Advertise your external address:"
Write-Host "    .\target\release\lattice-node.exe --name win --no-mdns \"
Write-Host "      --external-addr /ip4/<public-ip>/tcp/6001 \"
Write-Host "      --bootstrap-peer /dns4/<host>/tcp/<port>/p2p/<peer_id>"
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
