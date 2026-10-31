# Building Raito

## Windows Setup

This project requires **Visual Studio Build Tools** with the **"Desktop development with C++" workload** to build on Windows.

### Option 1: Install Visual Studio 2022 Build Tools (Recommended)

1. Download **Visual Studio 2022 Build Tools** from:
   https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022

2. Run the installer and select:
   - **"Desktop development with C++"** workload

3. Click **Install**

4. After installation, build the project:
   ```bash
   cd E:\Documents\TRAVAIL\packages\Raito
   cargo build --release
   ```

### Option 2: Use Visual Studio Developer Command Prompt

After installing Visual Studio Build Tools:

1. Open **"x64 Native Tools Command Prompt for VS 2022"** from the Start Menu
2. Navigate to the project:
   ```cmd
   cd E:\Documents\TRAVAIL\packages\Raito
   cargo build --release
   ```

### Option 3: Build from Git Bash

If you're using Git Bash, the GNU `link` command conflicts with MSVC's `link.exe`. To work around this:

1. Install Visual Studio Build Tools as above
2. From Git Bash, prepend the MSVC paths to PATH:
   ```bash
   export PATH="/c/Program Files (x86)/Microsoft Visual Studio/2022/Community/VC/Bin:$PATH"
   cargo build --release
   ```

OR use the `crun.sh` script which automatically handles this:
```bash
./scripts/crun.sh . -b
```

### Option 4: Use MinGW-w64 (Alternative)

If you prefer to use MinGW instead of MSVC:

1. Install a complete MinGW-w64 toolchain (e.g., MSYS2 with mingw-w64 packages)
2. Switch Rust to use the GNU target:
   ```bash
   rustup default stable-x86_64-pc-windows-gnu
   ```
3. Build with MinGW in PATH:
   ```bash
   PATH="/path/to/mingw64/bin:$PATH" cargo build --release
   ```

Note: The w64devkit that comes with Git for Windows is not sufficient - it's missing some required tools.

## macOS Setup

On macOS, ensure you have Xcode command line tools installed:
```bash
xcode-select --install
```

Then build normally:
```bash
cargo build --release
```

## Linux Setup

On Linux, install the required development packages for your distribution:

### Ubuntu/Debian:
```bash
sudo apt-get install build-essential libssl-dev
```

### Fedora:
```bash
sudo dnf install gcc-c++ openssl-devel
```

Then build normally:
```bash
cargo build --release
```

## Running

After building, you can run the project:

```bash
# Direct execution
./target/release/raito

# Or use the crun script
./scripts/crun.sh .
```

## Troubleshooting

### "linker \`link.exe\` not found"

This error means Visual Studio Build Tools are not installed or not in your PATH.
Install Visual Studio Build Tools with "Desktop development with C++" workload.

### "link: extra operand"

This error occurs in Git Bash because the GNU `link` command (from coreutils) is being used instead of the MSVC linker.
Either:
- Use cmd.exe or PowerShell instead of Git Bash
- Use the Visual Studio Developer Command Prompt
- Prepend MSVC paths to PATH in Git Bash before building

### "cannot find -lgcc_eh"

This error occurs with incomplete MinGW installations. Install a complete MinGW-w64 toolchain or use MSVC instead.
