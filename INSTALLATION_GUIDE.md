# Arise Desktop - Installation Guide

Complete step-by-step guide for installing Arise Desktop on Windows.

## 📋 Table of Contents

1. [System Requirements](#system-requirements)
2. [Prerequisites](#prerequisites)
3. [Installation Methods](#installation-methods)
4. [First-Time Setup](#first-time-setup)
5. [Configuration](#configuration)
6. [Troubleshooting](#troubleshooting)

---

## System Requirements

### Minimum Requirements
- **OS:** Windows 10 (64-bit) or later
- **Processor:** Intel Core i3 or equivalent
- **RAM:** 4 GB
- **Storage:** 500 MB free space
- **Graphics:** Integrated graphics with DirectX 11 support

### Recommended Requirements
- **OS:** Windows 11 (64-bit)
- **Processor:** Intel Core i5 or equivalent
- **RAM:** 8 GB
- **Storage:** 2 GB free space
- **Graphics:** Dedicated graphics with DirectX 12 support

---

## Prerequisites

### Required Software

#### 1. Visual C++ Redistributable
Arise Desktop requires the Visual C++ Redistributable for Visual Studio 2015-2022.

**Download:** [Microsoft Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)

**Installation:**
1. Download the x64 version
2. Run the installer
3. Accept the license agreement
4. Click "Install"
5. Wait for installation to complete
6. Click "Close"

#### 2. .NET Framework (Windows 10 only)
Windows 10 requires .NET Framework 4.8 or later.

**Check if installed:**
1. Press `Win + R`
2. Type `winver` and press Enter
3. Look for .NET Framework version

**Download if needed:** [.NET Framework 4.8](https://dotnet.microsoft.com/download/dotnet-framework/thank-you/thank-you-page?returnUrl=/download/dotnet-framework/net48)

---

## Installation Methods

### Method 1: Installer (Recommended)

This is the easiest method for most users.

#### Steps:

1. **Download Installer**
   - Go to the [Releases page](https://github.com/yourusername/arise-desktop/releases)
   - Download the latest `Arise-Desktop-Setup.exe` (NSIS installer)
   - Or `Arise-Desktop-x64-en-US.msi` (MSI installer)

2. **Run Installer**
   - Double-click the downloaded installer
   - If prompted by SmartScreen, click "More info" then "Run anyway"
   - Windows Defender may prompt you - click "Run"

3. **Installation Wizard**
   - Welcome screen: Click "Next"
   - License agreement: Read and accept, click "Next"
   - Installation folder: Default is fine, or choose custom location
   - Start Menu folder: Default is fine
   - Additional tasks: 
     - ✅ Create desktop shortcut (recommended)
     - ✅ Create quick launch shortcut
     - ✅ Run Arise Desktop after installation
   - Ready to install: Click "Install"
   - Wait for installation to complete
   - Click "Finish"

4. **First Launch**
   - Arise Desktop will start automatically
   - You'll see the main application window
   - Proceed to [First-Time Setup](#first-time-setup)

### Method 2: Portable Version

For users who don't want to install or want to run from USB.

#### Steps:

1. **Download Portable Version**
   - Go to the [Releases page](https://github.com/yourusername/arise-desktop/releases)
   - Download `Arise-Desktop-portable.zip`

2. **Extract Archive**
   - Right-click the zip file
   - Select "Extract All..."
   - Choose destination folder
   - Click "Extract"

3. **Run Application**
   - Open the extracted folder
   - Double-click `arise-desktop.exe`
   - Application will start

4. **Create Shortcut (Optional)**
   - Right-click `arise-desktop.exe`
   - Select "Send to" → "Desktop (create shortcut)"

### Method 3: Building from Source

For developers who want to customize or contribute.

#### Steps:

1. **Install Prerequisites**
   - Node.js 18+ from [nodejs.org](https://nodejs.org/)
   - Rust from [rustup.rs](https://rustup.rs/)
   - Git from [git-scm.com](https://git-scm.com/)

2. **Clone Repository**
   ```bash
   git clone https://github.com/yourusername/arise-desktop.git
   cd arise-desktop
   ```

3. **Install Dependencies**
   ```bash
   npm install
   ```

4. **Build Application**
   ```bash
   npm run tauri build
   ```

5. **Find Installer**
   - Installer is in `src-tauri/target/release/bundle/nsis/`
   - Run the installer as in Method 1

See [BUILD_GUIDE.md](BUILD_GUIDE.md) for detailed build instructions.

---

## First-Time Setup

### 1. Application Tour

When you first launch Arise Desktop, you'll see:

- **Gallery Tab** - Browse and select wallpapers
- **Player Tab** - View current wallpaper with controls
- **Dream Tab** - Create AI-generated wallpapers
- **Settings Tab** - Configure application settings

### 2. Adding Wallpapers

#### Using Sample Wallpapers
Arise Desktop comes with pre-installed sample wallpapers:
1. Go to **Gallery** tab
2. Browse the grid of wallpapers
3. Click on any wallpaper to preview
4. Click "Set as Wallpaper" to apply

#### Adding Your Own Wallpapers
1. Go to **Gallery** tab
2. Click "Add Wallpaper" button
3. Select video files (.mp4, .webm) or images (.jpg, .png)
4. Wait for processing
5. Wallpaper appears in gallery

### 3. Setting Your First Wallpaper

1. Navigate to **Gallery** tab
2. Select a wallpaper from the grid
3. Click "Set as Wallpaper" button
4. The video will now play behind your desktop icons
5. Use controls in **Player** tab to adjust playback

### 4. Configure Settings

Go to **Settings** tab and configure:

#### General Settings
- **Run on Startup** - Launch Arise Desktop when Windows starts
- **Mute Audio** - Mute wallpaper audio
- **Performance Mode** - Optimize for better performance

#### Widget Settings
- **Clock Widget** - Show draggable clock
- **Stats Widget** - Show CPU/RAM usage
- **Shuffle Widget** - Show smart shuffle context

#### Process Monitor
- **Auto-Pause on Fullscreen** - Pause video during games
- **Detection Sensitivity** - Adjust fullscreen detection

### 5. Setting Up AI Features (Optional)

To use AI features (Dream, Remaster, Fit to Screen, Parallax):

1. **Get Replicate API Key**
   - Go to [replicate.com/account/api-tokens](https://replicate.com/account/api-tokens)
   - Sign up or log in
   - Click "Create Token"
   - Copy the API key

2. **Enter API Key in Arise Desktop**
   - Go to **Settings** tab
   - Find "Replicate API Key" section
   - Paste your API key
   - Click "Save"
   
3. **Verify API Key**
   - Click "Test Connection" button
   - Wait for verification
   - Success message confirms key is valid

**Note:** Replicate has a free tier with limited credits. Check your account for pricing.

---

## Configuration

### Default File Locations

#### Application Data
```
%APPDATA%\arise-desktop\
├── dreams\              # AI-generated wallpapers
├── remastered\          # Upscaled videos
├── fills\               # Extended wallpapers
├── depth_maps\          # Depth maps for parallax
└── settings.json        # Application settings
```

#### Logs
```
%APPDATA%\arise-desktop\logs\
└── arise-desktop.log    # Application logs
```

### Configuration File (settings.json)

```json
{
  "apiKey": "your-replicate-api-key",
  "autoStart": true,
  "muted": false,
  "performanceMode": false,
  "widgets": {
    "clock": {
      "enabled": true,
      "position": { "x": 100, "y": 100 }
    },
    "stats": {
      "enabled": false,
      "position": { "x": 100, "y": 200 }
    },
    "shuffle": {
      "enabled": true,
      "position": { "x": 100, "y": 300 }
    }
  },
  "processMonitor": {
    "enabled": true,
    "autoPause": true
  }
}
```

### Environment Variables

Arise Desktop supports these environment variables:

```bash
# Development
VITE_DEV_MODE=true
TAURI_DEBUG=true

# API Configuration
REPLICATE_API_KEY=your-key-here

# Performance
TAURI_WEBVIEW_MEMORY_LIMIT=512
```

### Registry Keys (Auto-Start)

Auto-start is managed via Windows Registry:

```
Path: HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run
Key: Arise Desktop
Value: "C:\Program Files\Arise Desktop\arise-desktop.exe"
```

---

## Troubleshooting

### Common Issues

#### 1. Application Won't Start

**Symptoms:** Double-clicking the executable does nothing

**Solutions:**
- Check if antivirus is blocking it
- Run as Administrator
- Verify Visual C++ Redistributable is installed
- Check Windows Event Viewer for errors
- Try reinstalling the application

**Event Viewer Steps:**
1. Press `Win + X`, select "Event Viewer"
2. Navigate to "Windows Logs" → "Application"
3. Look for errors from "arise-desktop.exe"

#### 2. Video Not Playing Behind Icons

**Symptoms:** Video plays in window, not behind desktop icons

**Solutions:**
- Restart the application
- Try clicking "Set as Wallpaper" again
- Check if other wallpaper apps are running (Wallpaper Engine, etc.)
- Verify Windows is up to date
- Try different video format (.mp4 recommended)

**Known Conflicts:**
- Wallpaper Engine - Close it first
- Stardock Fences - May interfere
- Windows Dynamic Lock - Disable temporarily

#### 3. Poor Performance

**Symptoms:** Laggy video, high CPU/RAM usage

**Solutions:**
- Enable "Performance Mode" in Settings
- Lower video resolution
- Use H.264 encoded videos
- Close other applications
- Update graphics drivers
- Disable widgets you don't need

**Video Optimization:**
- Use 1080p videos (4K is heavy)
- H.264 codec is best
- 30 FPS is sufficient (60 FPS is overkill)
- Bitrate: 5-10 Mbps for smooth playback

#### 4. AI Features Not Working

**Symptoms:** AI generation fails or shows errors

**Solutions:**
- Verify Replicate API key is correct
- Check internet connection
- Verify you have credits in Replicate account
- Check Replicate service status (status.replicate.com)
- Try regenerating API key
- Check application logs for error details

**API Key Issues:**
```bash
# Test API key manually (requires curl)
curl -X POST \
  https://api.replicate.com/v1/predictions \
  -H "Authorization: Token YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"version": "test"}'
```

#### 5. Widgets Not Appearing

**Symptoms:** Widgets enabled but not visible

**Solutions:**
- Check if widgets are enabled in Settings
- Look for widgets off-screen (may be outside viewport)
- Reset widget positions by editing settings.json
- Try disabling and re-enabling widgets
- Restart application

**Reset Widget Positions:**
```json
// In settings.json, change positions to:
"position": { "x": 100, "y": 100 }
```

#### 6. Auto-Start Not Working

**Symptoms:** Application doesn't start with Windows

**Solutions:**
- Check if auto-start is enabled in Settings
- Verify Registry key exists
- Check Windows Task Manager → Startup
- Check Windows Event Viewer for startup errors
- Try manually adding to Windows Startup folder

**Manual Startup Folder:**
```
%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\
```
Create a shortcut to `arise-desktop.exe` in this folder.

#### 7. Installation Errors

**Symptoms:** Installer fails or rolls back

**Solutions:**
- Run installer as Administrator
- Temporarily disable antivirus
- Close all other applications
- Free up disk space (need 500 MB+)
- Check Windows Update for pending updates
- Try downloading installer again (may be corrupted)
- Use portable version as alternative

#### 8. Smart Shuffle Not Working

**Symptoms:** Context not detected or wrong wallpapers selected

**Solutions:**
- Check internet connection (needs for weather API)
- Verify location services are enabled
- Check OpenMeteo API status
- Try manual location input
- Add more wallpapers with appropriate tags
- Review smart shuffle logs

**Location Services:**
- Windows Settings → Privacy → Location → Enable
- Check if firewall is blocking location access

### Getting Help

If you can't resolve the issue:

1. **Check Logs**
   ```
   %APPDATA%\arise-desktop\logs\arise-desktop.log
   ```

2. **Search Issues**
   - [GitHub Issues](https://github.com/yourusername/arise-desktop/issues)

3. **Create New Issue**
   Include:
   - Windows version
   - Arise Desktop version
   - Error messages
   - Steps to reproduce
   - Screenshots if applicable
   - Log file excerpt

4. **Community Support**
   - [Discord Server](https://discord.gg/yourserver)
   - [Reddit Community](https://reddit.com/r/arisedesktop)

5. **Email Support**
   - support@arisedesktop.com

---

## Updating Arise Desktop

### Auto-Update (Future Feature)

Version 2.0 will include automatic updates.

### Manual Update

1. **Check for Updates**
   - Visit [Releases page](https://github.com/yourusername/arise-desktop/releases)
   - Check if newer version is available

2. **Download New Version**
   - Download latest installer
   - Don't uninstall old version yet

3. **Install Over Existing**
   - Run new installer
   - It will update existing installation
   - Your settings and wallpapers will be preserved

4. **Verify Update**
   - Launch application
   - Check version in Settings → About
   - Verify features work correctly

5. **Rollback (if needed)**
   - Keep old installer for rollback
   - Uninstall new version
   - Install old version
   - Settings will be preserved

---

## Uninstallation

### Standard Uninstall

1. **Open Settings**
   - Press `Win + I`
   - Go to "Apps" → "Installed apps"

2. **Find Arise Desktop**
   - Search for "Arise Desktop"
   - Click the three dots menu
   - Select "Uninstall"

3. **Confirm Uninstall**
   - Click "Uninstall" button
   - Wait for uninstallation to complete

4. **Clean Up Files**
   - Delete `%APPDATA%\arise-desktop\`
   - Delete desktop shortcut (optional)
   - Delete Start Menu entries (optional)

### Manual Uninstall (if standard fails)

1. **Delete Program Files**
   ```
   C:\Program Files\Arise Desktop\
   ```

2. **Delete App Data**
   ```
   %APPDATA%\arise-desktop\
   ```

3. **Remove Registry Keys**
   - Open Registry Editor (`regedit`)
   - Navigate to:
     ```
     HKEY_CURRENT_USER\Software\Arise Desktop
     ```
   - Delete the folder

4. **Remove Startup Entry**
   - Delete Registry key:
     ```
     HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run\Arise Desktop
     ```

5. **Delete Shortcuts**
   - Desktop shortcut
   - Start Menu entry
   - Quick Launch shortcut

---

## Security Considerations

### Antivirus Exclusions

Some antivirus software may falsely flag Arise Desktop:

**Add Exclusions (Windows Defender):**
1. Open Windows Security
2. Go to "Virus & threat protection"
3. Click "Manage settings"
4. Scroll to "Exclusions"
5. Click "Add or remove exclusions"
6. Add:
   - Folder: `C:\Program Files\Arise Desktop\`
   - Folder: `%APPDATA%\arise-desktop\`
   - Process: `arise-desktop.exe`

### Firewall Settings

Arise Desktop needs internet access for AI features:

**Allow Through Firewall:**
1. Open Windows Firewall
2. Click "Allow an app through Windows Firewall"
3. Find "Arise Desktop"
4. Check both Private and Public networks
5. Click OK

### API Key Security

- Never share your Replicate API key
- Rotate keys periodically
- Monitor usage in Replicate dashboard
- Revoke compromised keys immediately

---

## Performance Tips

### Optimize Video Playback

1. **Use Optimal Video Format**
   - Codec: H.264
   - Container: MP4
   - Resolution: 1920x1080
   - Frame Rate: 30 FPS
   - Bitrate: 5-10 Mbps

2. **Enable Performance Mode**
   - Go to Settings
   - Enable "Performance Mode"
   - This reduces CPU/GPU usage

3. **Disable Unnecessary Widgets**
   - Disable widgets you don't use
   - Fewer widgets = better performance

4. **Close Other Applications**
   - Especially resource-heavy apps
   - Games, video editors, etc.

5. **Update Graphics Drivers**
   - Latest drivers = better performance
   - GPU acceleration support

### System Optimization

1. **Disable Windows Animations**
   - Settings → Accessibility → Visual effects
   - Disable unnecessary animations

2. **Set Power Plan to High Performance**
   - Control Panel → Power Options
   - Select "High performance"

3. **Disable Windows Game Bar**
   - Settings → Gaming → Game Bar
   - Turn off "Open Game Bar"

---

## FAQ

### Q: Is Arise Desktop free?
A: Yes! Arise Desktop is free and open-source. AI features require a free Replicate account.

### Q: Does Arise Desktop work on macOS or Linux?
A: Currently, Arise Desktop is Windows-only due to the WorkerW hack. macOS and Linux versions are planned for the future.

### Q: Can I use my own videos?
A: Yes! You can add any MP4 or WebM video to your gallery.

### Q: Does Arise Desktop affect gaming performance?
A: Arise Desktop includes a process monitor that automatically pauses during games to prevent performance impact.

### Q: How much does AI generation cost?
A: Replicate offers a free tier with limited credits. Paid plans start at $5/month. Check replicate.com/pricing for details.

### Q: Can I use Arise Desktop with multiple monitors?
A: Multi-monitor support is planned for version 1.1.

### Q: Does Arise Desktop collect data?
A: No, Arise Desktop does not collect or send any user data. AI features communicate directly with Replicate API.

### Q: How do I report bugs?
A: Please report bugs on GitHub Issues with detailed information and logs.

---

## Additional Resources

- [README.md](README.md) - Main project documentation
- [FEATURES.md](FEATURES.md) - Feature documentation
- [BUILD_GUIDE.md](BUILD_GUIDE.md) - Build instructions
- [API_REFERENCE.md](API_REFERENCE.md) - API documentation
- [ARCHITECTURE.md](ARCHITECTURE.md) - Technical architecture

---

**Last Updated:** 2024
**Version:** 1.0.0

**Need Help?** Contact support@arisedesktop.com