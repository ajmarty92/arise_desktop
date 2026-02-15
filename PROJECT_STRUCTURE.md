# Arise Desktop - Complete Project Structure

## Project Overview
**Arise Desktop** is a Tauri-based desktop wallpaper application with AI-powered features for video wallpapers, generative fill, parallax effects, and smart context-aware wallpaper selection.

## Technology Stack
- **Frontend:** React 18 + TypeScript + Vite
- **Backend:** Rust (Tauri 2.0)
- **Styling:** Tailwind CSS + Custom Glassmorphism
- **3D Graphics:** React Three Fiber + Three.js
- **External APIs:** Replicate (AI), OpenMeteo (Weather), ip-api.com (Geolocation)

## Directory Structure

```
arise-desktop/
├── src/
│   ├── App.tsx                    # Main application with all integrations
│   ├── main.tsx                   # Entry point
│   ├── components/
│   │   ├── WallpaperPlayer.tsx    # Video wallpaper player
│   │   ├── Gallery.tsx            # Wallpaper gallery with glassmorphism
│   │   ├── SettingsOverlay.tsx    # Settings modal
│   │   ├── DreamTab.tsx           # AI dream generation UI
│   │   ├── VideoRemaster.tsx      # Video upscaling modal
│   │   ├── FitToScreen.tsx        # Generative fill UI
│   │   ├── ParallaxWallpaper.tsx  # 3D parallax effect
│   │   ├── widgets/
│   │   │   ├── DraggableWidget.tsx     # Widget wrapper
│   │   │   ├── ClockWidget.tsx         # Clock widget
│   │   │   ├── SystemStatsWidget.tsx   # System stats widget
│   │   │   ├── SmartShuffleWidget.tsx  # Smart shuffle widget
│   │   └── gallery/
│   │       ├── ExpandingCard.tsx       # Expanding card component
│   │       ├── SpatialPreview.tsx      # Spatial preview modal
│   │       └── VideoThumbnail.tsx      # Video thumbnail component
│   ├── lib/
│   │   ├── api.ts                   # API helper functions
│   │   ├── hooks.ts                 # Custom React hooks
│   │   └── types.ts                 # TypeScript type definitions
│   └── styles/
│       ├── App.css                  # Global styles
│       ├── glassmorphism.css        # Glassmorphism utilities
│       └── animations.css           # Custom animations
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs                   # Main Rust backend (2,500+ lines)
│   │   ├── workerw.rs               # WorkerW hack module
│   │   ├── process_monitor.rs       # Process monitoring module
│   │   ├── smart_shuffle.rs         # Smart shuffle module
│   │   ├── ai_dream.rs              # AI dream generation module
│   │   ├── video_remaster.rs        # Video upscaling module
│   │   ├── generative_fill.rs       # Generative fill module
│   │   ├── depth_estimation.rs      # Depth estimation module
│   │   └── icons/
│   │       └── tray/
│   │           ├── tray_play.png    # Play tray icon
│   │           └── tray_pause.png   # Pause tray icon
│   ├── Cargo.toml                   # Rust dependencies
│   ├── tauri.conf.json              # Tauri configuration
│   └── capabilities/
│       └── default.json             # App permissions
├── public/
│   ├── videos/                      # Sample wallpapers
│   └── icons/                       # App icons
├── package.json                     # Node.js dependencies
├── tsconfig.json                    # TypeScript config
├── vite.config.ts                   # Vite config
├── tailwind.config.js               # Tailwind CSS config
├── postcss.config.js                # PostCSS config
└── README.md                        # Project documentation

## Key Statistics
- **Total Lines of Code:** 10,000+
- **React Components:** 15+
- **Rust Modules:** 8
- **Tauri Commands:** 40+
- **Major Features:** 10
- **Sub-features:** 50+
- **Documentation:** 3,800+ lines