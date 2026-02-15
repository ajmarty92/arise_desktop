# Arise Desktop - Complete Build Guide

## 📋 Prerequisites

### Required Software
- **Node.js**: 18.0 or higher
- **npm**: 9.0 or higher
- **Rust**: 1.70 or higher
- **Cargo**: Included with Rust
- **Git**: For version control

### Required System
- **OS**: Windows 10 or 11
- **Architecture**: x64
- **Memory**: 4GB RAM minimum (8GB recommended)
- **Storage**: 500MB free space

### Optional (for AI features)
- **Replicate API Key**: For AI dream generation, upscaling, generative fill, parallax depth

## 🔧 Installation Steps

### 1. Install Prerequisites

#### Windows 10/11
```bash
# Install Node.js
# Download from: https://nodejs.org/

# Install Rust
# Download from: https://rustup.rs/
# Or run: winget install Rustlang.Rust.MSVC

# Verify installations
node --version    # Should be v18.0.0 or higher
npm --version     # Should be 9.0.0 or higher
rustc --version   # Should be 1.70.0 or higher
cargo --version   # Should be 1.70.0 or higher
```

### 2. Clone/Download Project

```bash
# If using git
git clone <repository-url>
cd arise-desktop

# Or extract downloaded zip file
cd arise-desktop
```

### 3. Install Node.js Dependencies

```bash
npm install
```

**Expected output:**
```
added 342 packages, and audited 343 packages in 45s
found 0 vulnerabilities
```

### 4. Install Build Dependencies (Linux/macOS only)

**Note:** For Windows, Tauri handles dependencies automatically.

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# macOS
brew install rustup
brew install node
brew install cairo pango libpng jpeg-turbo
```

### 5. Verify Project Structure

```bash
# Check that all key files exist
ls src/
ls src/components/
ls src-tauri/src/
ls public/
```

## 🚀 Building the Application

### Development Mode

```bash
npm run tauri dev
```

**What this does:**
- Starts Vite dev server on port 1420
- Compiles Rust backend in debug mode
- Opens Tauri application window
- Enables hot reload for both frontend and backend

**Expected output:**
```
> arise-desktop@1.0.0 tauri dev
> tauri dev

  VITE v5.0.0  ready in 1234 ms

  ➜  Local:   http://localhost:1420/
  ➜  Network: use --host to expose

    Finished dev [unoptimized + debuginfo] target(s) in 45.2s
     Running `target/debug/arise-desktop`
```

### Production Build

```bash
npm run tauri build
```

**What this does:**
- Optimizes frontend with Vite
- Compiles Rust backend in release mode
- Bundles application into executable
- Creates installer (NSIS for Windows)

**Expected output:**
```
> arise-desktop@1.0.0 tauri build
> tauri build

  VITE v5.0.0  building for production...
  ✓ 342 modules transformed.
  dist/index.html                   0.46 kB
  dist/assets/*.js                  1.29 MB
  dist/assets/*.css                 44.78 kB

    Finished release [optimized] target(s) in 3m 45s
     Bundling Arise Desktop 1.0.0 (x64.exe)
```

**Build Artifacts:**
```
src-tauri/target/release/
├── arise-desktop.exe              # Main executable (~8 MB)
├── bundle/
│   ├── msi/Arise Desktop_1.0.0_x64_en-US.msi  # MSI installer (~15 MB)
│   └── nsis/Arise Desktop_1.0.0_x64-setup.exe  # NSIS installer (~12 MB)
```

## ⚙️ Configuration

### Tauri Configuration (src-tauri/tauri.conf.json)

```json
{
  "build": {
    "distDir": "../dist",
    "devPath": "http://localhost:1420",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "productName": "Arise Desktop",
  "version": "1.0.0",
  "identifier": "com.arise.desktop",
  "windows": [
    {
      "title": "Arise Desktop",
      "width": 1280,
      "height": 720,
      "resizable": true,
      "fullscreen": false
    }
  ]
}
```

### Cargo Configuration (src-tauri/Cargo.toml)

Ensure all dependencies are listed:

```toml
[dependencies]
tauri = { version = "2.0", features = ["tray-icon"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "multipart"] }
chrono = "0.4"
uuid = { version = "1.0", features = ["v4"] }
directories = "5.0"
png = "0.17"
windows = { version = "0.58", features = ["Win32_UI_WindowsAndMessaging"] }

[build-dependencies]
tauri-build = { version = "2.0", features = [] }
```

### Vite Configuration (vite.config.ts)

```typescript
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
})
```

## 🔍 Troubleshooting

### Common Issues

#### 1. Node.js Version Too Old
```
Error: Node.js version is incompatible
```
**Solution:** Install Node.js 18+ from https://nodejs.org/

#### 2. Rust Not Found
```
error: cannot find `rustc`
```
**Solution:** Install Rust from https://rustup.rs/

#### 3. Windows SDK Missing
```
error: linker `link.exe` not found
```
**Solution:** Install Visual Studio Build Tools from https://visualstudio.microsoft.com/downloads/

#### 4. Port 1420 Already in Use
```
Error: Port 1420 is already in use
```
**Solution:**
```bash
# Kill process using port 1420
netstat -ano | findstr :1420
taskkill /PID <PID> /F
```

#### 5. Cargo Build Errors
```
error: failed to run custom build command
```
**Solution:**
```bash
# Update Rust toolchain
rustup update
# Clean cargo cache
cargo clean
```

#### 6. Tauri CLI Not Found
```
command not found: tauri
```
**Solution:**
```bash
npm install -g @tauri-apps/cli
```

### Build Warnings

#### Large Bundle Size
```
(!) Some chunks are larger than 500 kB
```
**Reason:** Three.js library (~600 KB)
**Action:** Acceptable for 3D features, can be code-split in future

#### Unused Variables
```
warning: unused variable: `x`
```
**Reason:** Minor warnings in Rust code
**Action:** Can be ignored or cleaned up

## 📦 Deployment

### Creating Installer

The build process automatically creates installers:

**Windows (NSIS):**
```bash
# Located at:
src-tauri/target/release/bundle/nsis/Arise Desktop_1.0.0_x64-setup.exe
```

**Windows (MSI):**
```bash
# Located at:
src-tauri/target/release/bundle/msi/Arise Desktop_1.0.0_x64_en-US.msi
```

### Code Signing (Optional)

For signed executables, configure in `src-tauri/tauri.conf.json`:

```json
{
  "tauri": {
    "bundle": {
      "windows": {
        "certificateThumbprint": "YOUR_CERTIFICATE_THUMBPRINT",
        "digestAlgorithm": "sha256",
        "timestampUrl": "http://timestamp.digicert.com"
      }
    }
  }
}
```

## 🧪 Testing

### Manual Testing Checklist

- [ ] Application launches without errors
- [ ] Video playback works smoothly
- [ ] WorkerW hack places video behind icons
- [ ] Gallery displays wallpapers correctly
- [ ] Settings overlay opens and functions
- [ ] System tray menu works
- [ ] Auto-start toggle works
- [ ] Process monitor detects fullscreen apps
- [ ] Widgets can be dragged and positioned
- [ ] AI dream generation (with API key)
- [ ] Video upscaling (with API key)
- [ ] Generative fill (with API key)
- [ ] Parallax mode (with API key)
- [ ] Smart shuffle auto-switches wallpapers

### Automated Testing

```bash
# Run frontend tests
npm test

# Run backend tests
cd src-tauri
cargo test

# Lint frontend code
npm run lint

# Check Rust code
cargo clippy
```

## 📊 Build Performance

### Typical Build Times

| Operation | Time | Notes |
|-----------|------|-------|
| `npm install` | 1-2 min | First time only |
| `npm run dev` | 30-45s | Debug build |
| `npm run build` | 2-3 min | Frontend only |
| `npm run tauri build` | 5-8 min | Full release build |

### Optimizing Build Speed

```bash
# Use cargo cache
export CARGO_HOME=~/.cargo

# Parallel builds
export CARGO_BUILD_JOBS=4

# Skip unnecessary features
cargo build --release --no-default-features
```

## 🔐 Security Considerations

### API Keys
- Store Replicate API key in localStorage (user input)
- Never hardcode API keys in source code
- Add to .gitignore if storing in environment variables

### Permissions
- Review capabilities in `src-tauri/capabilities/default.json`
- Only request necessary permissions
- Follow principle of least privilege

### Code Signing
- Sign executables for production releases
- Use timestamping services
- Include certificate verification

## 📝 Release Checklist

- [ ] Update version in package.json
- [ ] Update version in tauri.conf.json
- [ ] Run full test suite
- [ ] Build release version
- [ ] Test installer
- [ ] Code sign executable
- [ ] Update CHANGELOG.md
- [ ] Create GitHub release
- [ ] Upload installers
- [ ] Update documentation

## 🚀 Performance Optimization

### Frontend
- Enable React strict mode
- Use React.memo for expensive components
- Implement code splitting for large libraries
- Optimize images and videos
- Use lazy loading for gallery

### Backend
- Use release builds for production
- Optimize Rust code with `cargo --release`
- Enable LTO (Link Time Optimization)
- Use async/await properly
- Cache expensive operations

### Resources
- Monitor memory usage
- Profile CPU usage
- Optimize video compression
- Use appropriate video codecs (H.264)

## 📚 Additional Resources

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [React Documentation](https://react.dev/)
- [Vite Documentation](https://vitejs.dev/)
- [Rust Documentation](https://doc.rust-lang.org/)
- [Three.js Documentation](https://threejs.org/docs/)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)

## 🆘 Getting Help

### Community
- [Tauri Discord](https://discord.com/invite/tauri)
- [Rust Discord](https://discord.com/rust-lang)
- [React Discord](https://discord.com/reactiflux)

### Issues
- Report bugs on GitHub Issues
- Include system information
- Provide error logs
- Share reproduction steps

### Logs

**Tauri Logs:**
```bash
# Windows
%APPDATA%\arise-desktop\logs\

# View logs in terminal
npm run tauri dev -- --verbose
```

**Browser Console:**
```javascript
// Open DevTools in development
Ctrl + Shift + I
```