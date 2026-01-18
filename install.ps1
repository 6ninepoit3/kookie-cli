#!/usr/bin/env pwsh
# Kookie CLI Installer for Windows
# Run: irm https://raw.githubusercontent.com/yourusername/kookie-cli/main/install.ps1 | iex
# Or locally: .\install.ps1

param(
    [string]$InstallDir = "$env:LOCALAPPDATA\kookie"
)

$ErrorActionPreference = "Stop"

Write-Host ""
Write-Host "🍪 Kookie CLI Installer" -ForegroundColor Cyan
Write-Host "========================" -ForegroundColor Cyan
Write-Host ""

# Create install directory
if (-not (Test-Path $InstallDir)) {
    Write-Host "📁 Creating install directory: $InstallDir" -ForegroundColor Yellow
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Determine source binary
$SourceBinary = $null

# Check if running from repo with built binary
$RepoBinary = Join-Path $PSScriptRoot "target\release\kookie.exe"
if (Test-Path $RepoBinary) {
    $SourceBinary = $RepoBinary
    Write-Host "📦 Found local build: $RepoBinary" -ForegroundColor Green
}

# If no local binary, check current directory
if (-not $SourceBinary) {
    $CurrentDirBinary = Join-Path (Get-Location) "kookie.exe"
    if (Test-Path $CurrentDirBinary) {
        $SourceBinary = $CurrentDirBinary
        Write-Host "📦 Found binary in current directory" -ForegroundColor Green
    }
}

# If still no binary, try to download (placeholder for future GitHub releases)
if (-not $SourceBinary) {
    Write-Host "❌ Error: kookie.exe not found." -ForegroundColor Red
    Write-Host ""
    Write-Host "Please build from source first:" -ForegroundColor Yellow
    Write-Host "  cargo build --release" -ForegroundColor White
    Write-Host ""
    Write-Host "Or place kookie.exe in the current directory." -ForegroundColor Yellow
    exit 1
}

# Copy binary
$DestBinary = Join-Path $InstallDir "kookie.exe"
Write-Host "📋 Installing to: $DestBinary" -ForegroundColor Yellow
Copy-Item -Path $SourceBinary -Destination $DestBinary -Force

# Add to PATH if not already present
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($CurrentPath -notlike "*$InstallDir*") {
    Write-Host "🔧 Adding to PATH..." -ForegroundColor Yellow
    $NewPath = "$CurrentPath;$InstallDir"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    
    # Also update current session
    $env:Path = "$env:Path;$InstallDir"
    
    Write-Host "✅ Added $InstallDir to user PATH" -ForegroundColor Green
} else {
    Write-Host "✅ Already in PATH" -ForegroundColor Green
}

# Verify installation
Write-Host ""
Write-Host "🔍 Verifying installation..." -ForegroundColor Yellow

try {
    $Version = & $DestBinary --version 2>&1
    Write-Host "✅ $Version" -ForegroundColor Green
} catch {
    Write-Host "⚠️  Could not verify installation" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "🎉 Installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Get started:" -ForegroundColor Cyan
Write-Host "  kookie init          # Initialize your vault" -ForegroundColor White
Write-Host "  kookie add --password  # Add a password" -ForegroundColor White
Write-Host "  kookie --help        # See all commands" -ForegroundColor White
Write-Host ""
Write-Host "⚠️  NOTE: Restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
Write-Host ""
