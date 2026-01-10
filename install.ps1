# ForgeArch Windows Installer
# Usage: iwr https://raw.githubusercontent.com/iamjuaness/ForgeArch/master/install.ps1 | iex

$ErrorActionPreference = "Stop"

# Configuration
$REPO = "iamjuaness/ForgeArch"
$BINARY_NAME = "forge.exe"
$INSTALL_DIR = "$env:USERPROFILE\.local\bin"

Write-Host "🔨 Installing ForgeArch..." -ForegroundColor Cyan

# Detect architecture
$ARCH = if ([System.Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
$TARGET = "$ARCH-pc-windows-msvc"

Write-Host "   Platform: Windows $ARCH" -ForegroundColor Gray

# Get latest release
try {
    $RELEASE_URL = "https://api.github.com/repos/$REPO/releases/latest"
    $RELEASE_DATA = Invoke-RestMethod -Uri $RELEASE_URL -Headers @{ "User-Agent" = "ForgeArch-Installer" }
    $VERSION = $RELEASE_DATA.tag_name
    
    # Find the Windows asset
    $ASSET = $RELEASE_DATA.assets | Where-Object { $_.name -like "*$TARGET.zip" } | Select-Object -First 1
    
    if (-not $ASSET) {
        Write-Host "No Windows build found for $TARGET" -ForegroundColor Red
        exit 1
    }
    
    $DOWNLOAD_URL = $ASSET.browser_download_url
    Write-Host "   Version: $VERSION" -ForegroundColor Gray
    Write-Host "   URL: $DOWNLOAD_URL" -ForegroundColor Gray
} catch {
    Write-Host "Failed to fetch release information" -ForegroundColor Red
    Write-Host "   Error: $_" -ForegroundColor Red
    exit 1
}

# Create install directory
if (-not (Test-Path $INSTALL_DIR)) {
    New-Item -ItemType Directory -Path $INSTALL_DIR -Force | Out-Null
    Write-Host "✓ Created install directory: $INSTALL_DIR" -ForegroundColor Green
}

# Download and extract
$TMP_DIR = "$env:TEMP\forge_install_$(Get-Random)"
New-Item -ItemType Directory -Path $TMP_DIR -Force | Out-Null

try {
    $ZIP_FILE = "$TMP_DIR\forge.zip"
    Write-Host "⬇Downloading ForgeArch..." -ForegroundColor Cyan
    
    Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile $ZIP_FILE -UseBasicParsing
    
    Write-Host "Extracting..." -ForegroundColor Cyan
    Expand-Archive -Path $ZIP_FILE -DestinationPath $TMP_DIR -Force
    
    # Find the binary (might be in subdirectory)
    $BINARY = Get-ChildItem -Path $TMP_DIR -Filter "forge*.exe" -Recurse | Select-Object -First 1
    
    if (-not $BINARY) {
        Write-Host "Binary not found in archive" -ForegroundColor Red
        exit 1
    }
    
    # Copy to install location
    $DEST_PATH = "$INSTALL_DIR\$BINARY_NAME"
    Copy-Item -Path $BINARY.FullName -Destination $DEST_PATH -Force
    
    Write-Host "✓ ForgeArch installed to: $DEST_PATH" -ForegroundColor Green
    
} catch {
    Write-Host "Installation failed" -ForegroundColor Red
    Write-Host "   Error: $_" -ForegroundColor Red
    exit 1
} finally {
    # Cleanup
    Remove-Item -Path $TMP_DIR -Recurse -Force -ErrorAction SilentlyContinue
}

# Configure PATH
$USER_PATH = [Environment]::GetEnvironmentVariable("Path", "User")

if ($USER_PATH -notlike "*$INSTALL_DIR*") {
    Write-Host "Configuring PATH..." -ForegroundColor Cyan
    
    $NEW_PATH = if ($USER_PATH) { "$USER_PATH;$INSTALL_DIR" } else { $INSTALL_DIR }
    [Environment]::SetEnvironmentVariable("Path", $NEW_PATH, "User")
    
    Write-Host "✓ Added $INSTALL_DIR to PATH" -ForegroundColor Green
    Write-Host "" -ForegroundColor Yellow
    Write-Host "IMPORTANT: Close and reopen PowerShell for PATH changes to take effect" -ForegroundColor Yellow
    Write-Host "" -ForegroundColor Yellow
    
    # Update current session PATH
    $env:Path = "$env:Path;$INSTALL_DIR"
    Write-Host "✓ PATH updated for current session" -ForegroundColor Green
} else {
    Write-Host "✓ PATH already configured" -ForegroundColor Green
}

# Verify installation
Write-Host ""
Write-Host "Installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "To use ForgeArch in this session:" -ForegroundColor Cyan
Write-Host "   `$env:Path = `"`$env:Path;$INSTALL_DIR`"" -ForegroundColor Gray
Write-Host ""
Write-Host "To verify installation:" -ForegroundColor Cyan
Write-Host "   forge --version" -ForegroundColor Gray
Write-Host ""
Write-Host "Get started:" -ForegroundColor Cyan
Write-Host "   forge list                           # List available templates" -ForegroundColor Gray
Write-Host "   forge new my-project                 # Create project interactively" -ForegroundColor Gray
Write-Host "   forge new my-api --arch backend-api  # Create with specific template" -ForegroundColor Gray
Write-Host ""
Write-Host "Documentation: https://github.com/$REPO" -ForegroundColor Cyan
