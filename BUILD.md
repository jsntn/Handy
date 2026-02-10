# Build Instructions

This guide covers how to set up the development environment and build Handy from source across different platforms.

## Prerequisites

### All Platforms

- [Rust](https://rustup.rs/) (latest stable)
- [Bun](https://bun.sh/) package manager
- [Tauri Prerequisites](https://tauri.app/start/prerequisites/)

### Platform-Specific Requirements

#### macOS

- Xcode Command Line Tools
- Install with: `xcode-select --install`

#### Windows

- Microsoft C++ Build Tools
- Visual Studio 2019/2022 with C++ development tools
- Or Visual Studio Build Tools 2019/2022

#### Linux

- Build essentials
- ALSA development libraries
- Install with:

  ```bash
  # Ubuntu/Debian
  sudo apt update
  sudo apt install build-essential libasound2-dev pkg-config libssl-dev libvulkan-dev vulkan-tools glslc libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libgtk-layer-shell0 libgtk-layer-shell-dev patchelf cmake

  # Fedora/RHEL
  sudo dnf groupinstall "Development Tools"
  sudo dnf install alsa-lib-devel pkgconf openssl-devel vulkan-devel \
    gtk3-devel webkit2gtk4.1-devel libappindicator-gtk3-devel librsvg2-devel \
    gtk-layer-shell gtk-layer-shell-devel \
    cmake

  # Arch Linux
  sudo pacman -S base-devel alsa-lib pkgconf openssl vulkan-devel \
    gtk3 webkit2gtk-4.1 libappindicator-gtk3 librsvg gtk-layer-shell \
    cmake
  ```

## Setup Instructions

### 1. Clone the Repository

```bash
git clone git@github.com:cjpais/Handy.git
cd Handy
```

### 2. Install Dependencies

```bash
bun install
```

### 3. Start Dev Server

```bash
bun tauri dev
```

## Building for Distribution

### Build for Production

```bash
bun tauri build
```

This will create platform-specific installers in the `src-tauri/target/release/bundle/` directory.

### Windows Build Outputs

On Windows, the build process creates multiple distribution formats:

- **MSI Installer**: `src-tauri/target/release/bundle/msi/` - Traditional Windows installer
- **NSIS Installer**: `src-tauri/target/release/bundle/nsis/` - Setup executable
- **Portable ZIP**: `src-tauri/target/release/bundle/portable/` - Standalone portable package

#### Portable Build

The portable build is a ZIP archive containing:
- `Handy.exe` - The application executable
- `resources/` - All required resources (models, icons, sounds)
- `PORTABLE_README.txt` - Usage instructions and troubleshooting guide

**To use the portable build:**
1. Extract the ZIP file to any directory
2. Read `PORTABLE_README.txt` for detailed instructions
3. Run `Handy.exe` directly (no installation required)
4. All application data and settings are stored in your user profile

**Note:** The portable build requires:
- Windows 10/11 (x64 or ARM64 depending on the build)
- WebView2 runtime (usually pre-installed on Windows 11)
- Visual C++ Redistributable (usually already present on modern Windows)

### macOS Build Outputs

- **DMG**: `src-tauri/target/[arch]/release/bundle/dmg/` - Disk image for distribution
- **App Bundle**: `src-tauri/target/[arch]/release/bundle/macos/` - Application bundle

### Linux Build Outputs

- **DEB Package**: `src-tauri/target/release/bundle/deb/` - Debian/Ubuntu package
- **RPM Package**: `src-tauri/target/release/bundle/rpm/` - Fedora/RHEL package
- **AppImage**: `src-tauri/target/release/bundle/appimage/` - Portable Linux application

## Downloading Pre-built Binaries

Pre-built binaries, including portable builds, are available from:

1. **GitHub Releases**: https://github.com/cjpais/Handy/releases
   - All release builds include portable ZIP files for Windows
   - Look for files named `Handy_[version]_[arch]_portable.zip`

2. **GitHub Actions Artifacts**: For test builds
   - Navigate to the Actions tab
   - Select a completed workflow run
   - Download artifacts (available for 30 days)
   - Portable ZIPs are included in Windows build artifacts
