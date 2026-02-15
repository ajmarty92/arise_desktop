# Arise Desktop - AI-Powered Video Wallpaper Application

<div align="center">

![Arise Desktop](https://img.shields.io/badge/Arise-Desktop-blue)
![Version](https://img.shields.io/badge/version-1.0.0-green)
![License](https://img.shields.io/badge/license-MIT-orange)
![Platform](https://img.shields.io/badge/platform-Windows-blue)

**Transform your Windows desktop with stunning AI-powered video wallpapers**

[Features](#-features) • [Installation](#-installation) • [Documentation](#-documentation) • [Contributing](#contributing)

</div>

---

## ✨ Overview

**Arise Desktop** is a next-generation desktop wallpaper application built with Tauri (Rust + React) that brings your desktop to life with video wallpapers, AI-powered content generation, and smart context-aware features.

### 🎯 Key Capabilities

- **Video Wallpapers** - Play looping videos behind your desktop icons
- **AI Dream Generation** - Create unique wallpapers with text-to-video AI
- **Auto-Remaster** - Upscale and enhance wallpapers with AI
- **Fit to Screen** - Intelligent aspect ratio extension with generative fill
- **Parallax Mode** - Stunning 3D depth effects
- **Smart Shuffle** - Context-aware wallpaper selection based on weather, time, and location
- **Widget System** - Draggable widgets for clock, system stats, and smart shuffle
- **Process Monitoring** - Auto-pause during games or fullscreen apps

## 🚀 Features

### 🎬 Video Wallpaper System
- Full-screen video playback behind desktop icons
- Seamless Windows WorkerW integration
- Smooth playback controls (play/pause, volume, seek)
- Playlist navigation
- Custom video selection from gallery

### 🤖 AI-Powered Features
- **AI Dream** - Generate unique video wallpapers from text prompts
- **Auto-Remaster** - Upscale and enhance video quality with Real-ESRGAN
- **Generative Fill** - Extend wallpapers to fit any screen aspect ratio
- **Parallax Depth** - Create stunning 3D effects with depth estimation

### 🎨 Smart Features
- **Smart Shuffle** - Automatic wallpaper selection based on:
  - Weather conditions (Clear, Cloudy, Rain, Snow, Storm, Fog)
  - Time of day (Dawn, Morning, Afternoon, Evening, Night)
  - Geographic location
  - Mood analysis
- **Widget System** - Draggable widgets:
  - Clock widget with time/date
  - System stats widget (CPU/RAM)
  - Smart shuffle widget with real-time context

### ⚙️ System Features
- **Process Monitor** - Auto-pause during games or high-resource apps
- **System Tray** - Quick access controls from tray icon
- **Auto-Start** - Launch automatically with Windows
- **Performance Mode** - Optimize for different system configurations
- **Glassmorphism UI** - Beautiful, modern design

## 📸 Screenshots

*(Note: Screenshots would be added here in a real project)*

## 🔧 Technology Stack

### Frontend
- **Framework:** React 18 + TypeScript
- **Build Tool:** Vite 5
- **Styling:** Tailwind CSS + Custom Glassmorphism
- **3D Graphics:** React Three Fiber + Three.js
- **Animations:** Framer Motion
- **Icons:** Lucide React

### Backend
- **Framework:** Tauri 2.0
- **Language:** Rust
- **Windows API:** Full integration
- **Async Runtime:** Tokio

### External APIs
- **Replicate** - AI video generation, upscaling, inpainting, depth estimation
- **OpenMeteo** - Weather data (free, no API key)
- **ip-api.com** - Geolocation (free, no API key)

## 📦 Installation

### Prerequisites
- Windows 10 or 11 (x64)
- Node.js 18+
- Rust 1.70+

### Quick Start

```bash
# Clone the repository
git clone <repository-url>
cd arise-desktop

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Download Installer

Download the latest installer from [Releases](https://github.com/yourusername/arise-desktop/releases)

## 🎯 Usage

### Setting a Video Wallpaper

1. Open Arise Desktop
2. Navigate to **Gallery**
3. Select a video from the grid
4. Click **Set as Wallpaper**
5. Enjoy your new animated desktop!

### AI Dream Generation

1. Navigate to **Dream** tab
2. Enter your creative prompt (e.g., "A serene sunset over mountains")
3. Select a style preset (Cinematic, Anime, Realistic, etc.)
4. Click **Generate Dream**
5. Wait for AI to create your unique wallpaper
6. Apply to desktop or save to gallery

### Smart Shuffle

1. Enable **Smart Shuffle Widget** in Settings
2. The widget displays current context (weather, time, location)
3. Click **Auto-Shuffle** to let AI select perfect wallpaper
4. Or manually trigger shuffle from widget

### Fit to Screen

1. Open Gallery and select a static image
2. Click the 📐 (Fit to Screen) button
3. The app analyzes aspect ratio differences
4. Enter custom prompt for AI guidance (optional)
5. Click **Generate** to extend the image
6. Result automatically added to gallery

### Parallax Mode

1. Select a static image from Gallery
2. Click the 🎭 (Parallax) button
3. Generate depth map using AI
4. Preview 3D parallax effect in full screen
5. Apply as wallpaper for stunning depth effect

## 📚 Documentation

- [**Features List**](FEATURES.md) - Complete feature documentation
- [**Architecture**](ARCHITECTURE.md) - Technical architecture details
- [**Build Guide**](BUILD_GUIDE.md) - Build and deployment instructions
- [**API Reference**](API_REFERENCE.md) - API documentation
- [**Smart Shuffle Guide**](SMART_SHUFFLE_GUIDE.md) - Smart shuffle features

## 🏗️ Project Structure

```
arise-desktop/
├── src/                    # Frontend source code
│   ├── components/         # React components
│   │   ├── widgets/       # Widget components
│   │   └── gallery/       # Gallery components
│   ├── lib/               # Utilities and hooks
│   └── styles/            # CSS and stylesheets
├── src-tauri/             # Rust backend
│   ├── src/               # Rust modules
│   └── icons/             # App icons
├── public/                # Static assets
└── docs/                  # Documentation
```

## 🔐 Configuration

### API Keys

For AI features (Dream, Remaster, Fit to Screen, Parallax), you need a Replicate API key:

1. Get your API key from [replicate.com](https://replicate.com/account/api-tokens)
2. Open Arise Desktop
3. Go to **Settings**
4. Enter your API key
5. Save and enjoy AI features!

### Settings

- **Run on Startup** - Auto-launch with Windows
- **Mute Audio** - Mute wallpaper videos
- **Performance Mode** - Optimize for performance
- **Widget Toggles** - Enable/disable widgets

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow existing code style
- Add tests for new features
- Update documentation
- Ensure builds pass before submitting PR

## 📊 Project Statistics

| Metric | Value |
|--------|-------|
| Total Lines of Code | 10,000+ |
| React Components | 15+ |
| Rust Modules | 8 |
| Tauri Commands | 40+ |
| Major Features | 13 |
| Sub-features | 50+ |
| Documentation Lines | 3,800+ |

## 🗺️ Roadmap

### Version 1.1 (Planned)
- [ ] License verification system
- [ ] Additional AI models support
- [ ] Cloud wallpaper library
- [ ] Community sharing features

### Version 2.0 (Future)
- [ ] Mobile companion app
- [ ] Wallpaper scheduling
- [ ] Advanced audio visualization
- [ ] Plugin system
- [ ] Cloud sync

## 🐛 Troubleshooting

### Common Issues

**WorkerW hack not working:**
- Ensure Windows is up to date
- Try restarting the application
- Check if antivirus is blocking

**Video not playing:**
- Verify video format (MP4 recommended)
- Check codec compatibility
- Try different video

**AI features not working:**
- Verify Replicate API key is correct
- Check internet connection
- Ensure sufficient credits in Replicate account

**Performance issues:**
- Enable Performance Mode in Settings
- Lower video resolution
- Close other applications

### Getting Help

- 📖 Check [Documentation](#-documentation)
- 🐛 [Report a bug](https://github.com/yourusername/arise-desktop/issues)
- 💬 [Discord Community](https://discord.gg/yourserver)
- 📧 [Email Support](mailto:support@arisedesktop.com)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Tauri Team** - For the amazing desktop framework
- **React Community** - For excellent UI library
- **Replicate** - For AI model hosting
- **Three.js** - For 3D graphics capabilities
- **OpenMeteo** - For free weather API
- **All Contributors** - Thank you for your contributions!

## 🌟 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=yourusername/arise-desktop&type=Date)](https://star-history.com/#yourusername/arise-desktop&Date)

## 📮 Contact

- **Website:** [arisedesktop.com](https://arisedesktop.com)
- **Twitter:** [@arisedesktop](https://twitter.com/arisedesktop)
- **GitHub:** [arise-desktop](https://github.com/yourusername/arise-desktop)

---

<div align="center">

**Made with ❤️ by the Arise Desktop Team**

[⬆ Back to Top](#-arise-desktop---ai-powered-video-wallpaper-application)

</div>