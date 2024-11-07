# BH3-Patch (HTTP Redirection / Proxy)

## Getting Started

### Option 1: Download Prebuilt Binaries

1. Go to the [Releases](https://github.com/MikuLeaks/MikuBH3-Patch/releases) page.
2. Download the latest prebuilt binaries.
3. Extract all contents of the `.zip` file directly into the game folder, where the game’s main executable (`BH3.exe`) is located.

### Option 2: Build from Source

1. Ensure you have [Rust installed](https://www.rust-lang.org/tools/install).
2. Clone the repository:

   ```bash
   git clone https://github.com/MikuLeaks/MikuBH3-Patch.git
   cd MikuBH3-Patch
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

4. Copy `bh3-launcher.exe` and `bh3_patch.dll` from `target/release` folder into the game folder.
