# Arise Desktop - Technical Architecture

## 🏗️ System Architecture

### High-Level Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     User Interface Layer                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Gallery    │  │   Settings   │  │  Dashboard   │      │
│  │   Component  │  │   Overlay    │  │   Widgets    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                    Application Logic Layer                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  State Mgmt  │  │   Hooks      │  │  API Client  │      │
│  │   (React)    │  │  (Custom)    │  │  (Axios)     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                      Tauri Bridge Layer                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   invoke()   │  │   listen()   │  │   emit()     │      │
│  │   Commands   │  │   Events     │  │   Events     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                       Backend Layer                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  WorkerW     │  │  Process     │  │  Smart       │      │
│  │   Hack       │  │  Monitor     │  │  Shuffle     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  AI Dream    │  │   Video      │  │  Generative  │      │
│  │   Module     │  │  Remaster    │  │    Fill      │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│  ┌──────────────┐                                          │
│  │  Depth       │                                          │
│  │  Estimation  │                                          │
│  └──────────────┘                                          │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                     System Layer                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Windows API │  │  File System │  │  Network     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                   External Services                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Replicate   │  │  OpenMeteo   │  │  ip-api.com  │      │
│  │     API      │  │    API       │  │              │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

## 🎨 Frontend Architecture

### Component Hierarchy

```
App.tsx (Root)
├── WallpaperPlayer.tsx
│   ├── Video Element
│   └── Controls Overlay
├── Gallery.tsx
│   ├── ExpandingCard.tsx
│   │   ├── VideoThumbnail.tsx
│   │   └── SpatialPreview.tsx
│   ├── SearchBar
│   └── FilterControls
├── SettingsOverlay.tsx
│   ├── ToggleSwitch
│   └── Settings Sections
├── DreamTab.tsx
│   ├── PromptInput
│   ├── StyleSelector
│   └── ProgressTracker
├── VideoRemaster.tsx
│   ├── VideoInfo
│   ├── QualitySelector
│   └── ProgressTracker
├── FitToScreen.tsx
│   ├── AspectComparison
│   ├── PromptInput
│   └── ProgressTracker
├── ParallaxWallpaper.tsx
│   ├── Canvas (Three.js)
│   ├── GenerationUI
│   └── PreviewControls
└── Widgets/
    ├── DraggableWidget.tsx
    ├── ClockWidget.tsx
    ├── SystemStatsWidget.tsx
    └── SmartShuffleWidget.tsx
```

### State Management

**Global State (App.tsx):**
```typescript
interface AppState {
  // Wallpaper Management
  wallpapers: WallpaperItem[];
  activeWallpaperUrl: string | null;
  
  // WorkerW Hack State
  workerWHandle: string | null;
  isWallpaperMode: boolean;
  
  // Process Monitor State
  isMonitorRunning: boolean;
  isPausedByMonitor: boolean;
  
  // Playback Control
  shouldPlay: boolean;
  isMuted: boolean;
  isPerformanceMode: boolean;
  
  // UI State
  isSettingsOverlayOpen: boolean;
  viewMode: 'player' | 'gallery' | 'settings';
  
  // Widget State
  showClockWidget: boolean;
  showStatsWidget: boolean;
  showShuffleWidget: boolean;
}
```

**Local Storage:**
```typescript
// Persisted Settings
interface PersistedSettings {
  apiKey: string;
  autoStart: boolean;
  muted: boolean;
  performanceMode: boolean;
  clockWidgetEnabled: boolean;
  statsWidgetEnabled: boolean;
  shuffleWidgetEnabled: boolean;
  widgetPositions: Record<string, {x: number, y: number}>;
}
```

### Custom Hooks

**useSystemStats.ts:**
```typescript
export const useSystemStats = () => {
  const [stats, setStats] = useState<SystemStats>({
    cpu: 0,
    ram: 0,
    cpuHistory: [],
    ramHistory: []
  });
  
  useEffect(() => {
    // Poll system stats every 2 seconds
    const interval = setInterval(async () => {
      const data = await invoke('get_system_stats');
      setStats(data);
    }, 2000);
    
    return () => clearInterval(interval);
  }, []);
  
  return stats;
};
```

**useParallaxDepth.ts:**
```typescript
export const useParallaxDepth = (imageUrl: string) => {
  const [depthMap, setDepthMap] = useState<string | null>(null);
  const [isGenerating, setIsGenerating] = useState(false);
  const [progress, setProgress] = useState(0);
  
  const generateDepthMap = async () => {
    setIsGenerating(true);
    const jobId = await invoke('generate_depth_map', { imageUrl });
    
    // Poll for progress
    const pollInterval = setInterval(async () => {
      const status = await invoke('check_depth_map_status', { jobId });
      setProgress(status.progress);
      
      if (status.status === 'succeeded') {
        setDepthMap(status.output);
        setIsGenerating(false);
        clearInterval(pollInterval);
      }
    }, 1000);
  };
  
  return { depthMap, isGenerating, progress, generateDepthMap };
};
```

### API Client

**api.ts:**
```typescript
import { invoke } from '@tauri-apps/api/core';

export const api = {
  // Wallpaper Commands
  initWorkerW: () => invoke('init_worker_w'),
  setAsWallpaper: (handle: string) => invoke('set_as_wallpaper', { handle }),
  restoreFromWallpaper: () => invoke('restore_from_wallpaper'),
  
  // Process Monitor
  startProcessMonitor: () => invoke('start_process_monitor'),
  stopProcessMonitor: () => invoke('stop_process_monitor'),
  checkSystemState: () => invoke('check_system_state'),
  
  // System Stats
  getSystemStats: () => invoke('get_system_stats'),
  
  // Smart Shuffle
  getSmartContext: () => invoke('get_smart_context'),
  getSmartContextForLocation: (lat: number, lon: number) => 
    invoke('get_smart_context_for_location', { lat, lon }),
  
  // AI Dream
  createDream: (prompt: string, style: string) => 
    invoke('create_dream', { prompt, style }),
  checkDreamStatus: (jobId: string) => 
    invoke('check_dream_status', { jobId }),
  downloadDream: (jobId: string) => 
    invoke('download_dream', { jobId }),
  listDreams: () => invoke('list_dreams'),
  deleteDream: (dreamId: string) => 
    invoke('delete_dream', { dreamId }),
  
  // Video Remaster
  startVideoUpscale: (videoPath: string) => 
    invoke('start_video_upscale', { videoPath }),
  checkUpscaleStatus: (jobId: string) => 
    invoke('check_upscale_status', { jobId }),
  downloadUpscaledVideo: (jobId: string) => 
    invoke('download_upscaled_video', { jobId }),
  listRemasteredVideos: () => invoke('list_remastered_videos'),
  
  // Generative Fill
  analyzeAspectFit: (videoPath: string) => 
    invoke('analyze_aspect_fit', { videoPath }),
  getMonitorInfo: () => invoke('get_monitor_info'),
  startGenerativeFill: (videoPath: string, prompt: string) => 
    invoke('start_generative_fill', { videoPath, prompt }),
  checkFillStatus: (jobId: string) => 
    invoke('check_fill_status', { jobId }),
  downloadFilledImage: (jobId: string) => 
    invoke('download_filled_image', { jobId }),
  
  // Depth Estimation
  generateDepthMap: (imageUrl: string) => 
    invoke('generate_depth_map', { imageUrl }),
  checkDepthMapStatus: (jobId: string) => 
    invoke('check_depth_map_status', { jobId }),
  getCachedDepthMap: (imageUrl: string) => 
    invoke('get_cached_depth_map', { imageUrl }),
  
  // Auto-Start
  enableAutoStart: () => invoke('enable_auto_start'),
  disableAutoStart: () => invoke('disable_auto_start'),
  isAutoStartEnabled: () => invoke('is_auto_start_enabled')
};
```

## 🔧 Backend Architecture

### Module Structure

```
lib.rs (Main Entry Point)
├── workerw (WorkerW Hack Module)
│   ├── find_and_spawn_worker_w()
│   ├── enum_windows_proc()
│   ├── set_window_as_wallpaper()
│   └── restore_window_from_wallpaper()
├── process_monitor (Process Monitoring)
│   ├── detect_fullscreen_applications()
│   ├── detect_high_resource_processes()
│   ├── monitor_loop()
│   └── emit_playback_control_event()
├── smart_shuffle (Smart Shuffle Module)
│   ├── get_weather_data()
│   ├── get_location_data()
│   ├── get_time_context()
│   ├── analyze_context()
│   └── match_wallpapers()
├── ai_dream (AI Dream Generation)
│   ├── create_dream_job()
│   ├── check_job_status()
│   ├── download_result()
│   └── list_dreams()
├── video_remaster (Video Upscaling)
│   ├── get_video_info()
│   ├── start_upscale_job()
│   ├── check_upscale_status()
│   └── list_remastered()
├── generative_fill (Generative Fill)
│   ├── get_monitor_resolution()
│   ├── get_video_dimensions()
│   ├── analyze_fit()
│   ├── extract_first_frame()
│   ├── create_extended_canvas()
│   ├── start_inpainting()
│   └── create_video_from_image()
└── depth_estimation (Depth Estimation)
    ├── get_cached_depth_map()
    ├── start_depth_estimation()
    ├── check_depth_status()
    └── download_depth_map()
```

### Tauri Commands

**Core Commands (WorkerW):**
```rust
#[tauri::command]
async fn init_worker_w() -> Result<String, String> {
    // Initialize WorkerW hack
    let handle = find_and_spawn_worker_w().await?;
    Ok(handle)
}

#[tauri::command]
async fn set_as_wallpaper(handle: String) -> Result<(), String> {
    set_window_as_wallpaper(handle).await
}

#[tauri::command]
async fn restore_from_wallpaper() -> Result<(), String> {
    restore_window_from_wallpaper().await
}
```

**Process Monitor Commands:**
```rust
#[tauri::command]
async fn start_process_monitor(app_handle: tauri::AppHandle) -> Result<(), String> {
    spawn_process_monitor(app_handle).await
}

#[tauri::command]
async fn stop_process_monitor() -> Result<(), String> {
    stop_monitoring().await
}

#[tauri::command]
async fn check_system_state() -> Result<bool, String> {
    check_fullscreen_state().await
}
```

**System Stats Commands:**
```rust
#[tauri::command]
async fn get_system_stats() -> Result<SystemStats, String> {
    let cpu_usage = get_cpu_usage().await?;
    let ram_usage = get_ram_usage().await?;
    Ok(SystemStats {
        cpu: cpu_usage,
        ram: ram_usage,
        cpu_history: vec![],
        ram_history: vec![]
    })
}
```

### Async Runtime

**Tokio Integration:**
```rust
#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            init_worker_w,
            set_as_wallpaper,
            restore_from_wallpaper,
            start_process_monitor,
            stop_process_monitor,
            check_system_state,
            get_system_stats,
            // ... more commands
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Event System

**Frontend Event Listener:**
```typescript
useEffect(() => {
  const unlisten = listen('playback-control', (event) => {
    const { shouldPause } = event.payload as { shouldPause: boolean };
    setShouldPlay(!shouldPause);
    setIsPausedByMonitor(shouldPause);
  });
  
  return () => unlisten.then(fn => fn());
}, []);
```

**Backend Event Emitter:**
```rust
app_handle
    .emit_all("playback-control", json!({
        "shouldPause": true,
        "reason": "Fullscreen application detected"
    }))
    .map_err(|e| format!("Failed to emit event: {}", e))?;
```

## 🗄️ Data Architecture

### File System Structure

```
%APPDATA%\arise-desktop\
├── dreams/
│   ├── {dream_id}/
│   │   ├── metadata.json
│   │   └── video.mp4
├── remastered/
│   ├── {video_id}/
│   │   ├── metadata.json
│   │   └── upscaled.mp4
├── fills/
│   ├── {fill_id}/
│   │   ├── metadata.json
│   │   ├── original.png
│   │   ├── extended.png
│   │   └── video.mp4
├── depth_maps/
│   ├── {image_hash}/
│   │   └── depth.png
└── settings.json
```

### Data Models

**WallpaperItem:**
```typescript
interface WallpaperItem {
  id: string;
  url: string;
  type: 'video' | 'image';
  title: string;
  duration?: number;
  thumbnail?: string;
  tags: string[];
  isStatic?: boolean;
  hasDepthMap?: boolean;
  isUpscaled?: boolean;
  isExtended?: boolean;
}
```

**DreamMetadata:**
```typescript
interface DreamMetadata {
  id: string;
  prompt: string;
  style: string;
  status: 'pending' | 'processing' | 'succeeded' | 'failed';
  createdAt: string;
  videoUrl?: string;
  error?: string;
}
```

**SystemStats:**
```typescript
interface SystemStats {
  cpu: number;        // 0-100
  ram: number;        // 0-100
  cpuHistory: number[];
  ramHistory: number[];
}
```

**SmartContext:**
```typescript
interface SmartContext {
  weather: {
    condition: string;
    temperature: number;
  };
  time: {
    period: string;
    isDay: boolean;
  };
  location: {
    city: string;
    country: string;
  };
  mood: string;
  tags: string[];
}
```

## 🌐 External API Integration

### Replicate API

**AI Dream Generation:**
```rust
async fn create_dream_job(prompt: &str, style: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.replicate.com/v1/predictions")
        .header("Authorization", format!("Token {}", api_key))
        .json(&json!({
            "version": "model-version-id",
            "input": {
                "prompt": format!("{}, {}", prompt, style),
                "num_frames": 24,
                "num_inference_steps": 25
            }
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to create dream: {}", e))?;
    
    let result: serde_json::Value = response.json().await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    Ok(result["id"].as_str().unwrap().to_string())
}
```

**Polling Status:**
```rust
async fn check_job_status(job_id: &str) -> Result<JobStatus, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("https://api.replicate.com/v1/predictions/{}", job_id))
        .header("Authorization", format!("Token {}", api_key))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    Ok(JobStatus {
        status: response["status"].as_str().unwrap().to_string(),
        output: response["output"].as_str().map(|s| s.to_string()),
        error: response["error"].as_str().map(|s| s.to_string()),
        progress: calculate_progress(&response),
    })
}
```

### OpenMeteo API

**Weather Data:**
```rust
async fn get_weather_data(lat: f64, lon: f64) -> Result<WeatherData, String> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        lat, lon
    );
    
    let response = reqwest::get(&url).await?.json::<serde_json::Value>().await?;
    
    Ok(WeatherData {
        condition: map_weather_code(response["current_weather"]["weathercode"].as_i64().unwrap()),
        temperature: response["current_weather"]["temperature"].as_f64().unwrap(),
    })
}
```

### ip-api.com API

**Geolocation:**
```rust
async fn get_location_data() -> Result<LocationData, String> {
    let response = reqwest::get("http://ip-api.com/json/")
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    Ok(LocationData {
        city: response["city"].as_str().unwrap().to_string(),
        country: response["country"].as_str().unwrap().to_string(),
        lat: response["lat"].as_f64().unwrap(),
        lon: response["lon"].as_f64().unwrap(),
    })
}
```

## 🔐 Security Architecture

### API Key Management

**Frontend Storage:**
```typescript
// Store API key in localStorage
const setApiKey = (key: string) => {
  localStorage.setItem('replicate_api_key', key);
};

const getApiKey = (): string | null => {
  return localStorage.getItem('replicate_api_key');
};
```

**Backend Usage:**
```rust
// API key passed from frontend (not stored in backend)
async fn create_dream_job(
    prompt: String,
    style: String,
    api_key: String,
) -> Result<String, String> {
    // Use api_key directly
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.replicate.com/v1/predictions")
        .header("Authorization", format!("Token {}", api_key))
        .json(&json!({ "input": prompt }))
        .send()
        .await?;
    
    // ... process response
}
```

### File System Security

**App Data Directory:**
```rust
use directories::ProjectDirs;

fn get_app_data_dir() -> Result<PathBuf, String> {
    let proj_dirs = ProjectDirs::from("com", "arise", "desktop")
        .ok_or("Failed to get project directories")?;
    
    let data_dir = proj_dirs.data_dir();
    
    if !data_dir.exists() {
        fs::create_dir_all(data_dir)
            .map_err(|e| format!("Failed to create data directory: {}", e))?;
    }
    
    Ok(data_dir.to_path_buf())
}
```

## 📊 Performance Architecture

### Optimization Strategies

**Frontend:**
1. **Code Splitting:** Lazy load heavy components (Three.js)
2. **Memoization:** React.memo for expensive renders
3. **Virtualization:** Virtual list for large galleries
4. **Image Optimization:** WebP format, lazy loading
5. **Debouncing:** Debounce search inputs

**Backend:**
1. **Async Operations:** Non-blocking I/O with tokio
2. **Caching:** Depth maps, video info
3. **Connection Pooling:** Reuse HTTP clients
4. **Background Tasks:** Spawn independent tasks
5. **Resource Limits:** Monitor memory/CPU usage

**Video Playback:**
1. **Hardware Acceleration:** Use GPU for decoding
2. **Adaptive Quality:** Adjust based on system load
3. **Preloading:** Preload next wallpaper
4. **Buffering:** Optimize buffer size

## 🔄 Event Flow

### WorkerW Initialization Flow

```
User Clicks "Set Wallpaper"
    ↓
Frontend invokes init_worker_w()
    ↓
Backend finds Progman window
    ↓
Backend sends 0x052C message to Progman
    ↓
WorkerW window is spawned
    ↓
Backend enumerates windows to find WorkerW
    ↓
Backend returns WorkerW handle
    ↓
Frontend invokes set_as_wallpaper(handle)
    ↓
Backend sets app window as child of WorkerW
    ↓
Video appears behind desktop icons
```

### Smart Shuffle Flow

```
Auto-Shuffle Triggered
    ↓
Get current location (ip-api.com)
    ↓
Get weather data (OpenMeteo)
    ↓
Get time context (local time)
    ↓
Analyze context and generate mood
    ↓
Match wallpapers based on tags
    ↓
Calculate match scores
    ↓
Select best matching wallpaper
    ↓
Switch to new wallpaper
    ↓
Update UI with context info
```

### AI Dream Generation Flow

```
User inputs prompt and style
    ↓
Frontend invokes create_dream(prompt, style, api_key)
    ↓
Backend creates Replicate prediction
    ↓
Backend returns job ID
    ↓
Frontend starts polling interval
    ↓
Backend checks job status every 1s
    ↓
Status → "processing"
    ↓
Progress updates to UI
    ↓
Status → "succeeded"
    ↓
Frontend invokes download_dream(job_id)
    ↓
Backend downloads video to app data
    ↓
Frontend updates gallery
    ↓
User can view and use new wallpaper
```

## 🧪 Testing Architecture

### Unit Tests

**Frontend (React Testing Library):**
```typescript
describe('WallpaperPlayer', () => {
  it('renders video element', () => {
    render(<WallpaperPlayer url="test.mp4" />);
    const video = screen.getByRole('video');
    expect(video).toBeInTheDocument();
  });
  
  it('pauses when shouldPlay is false', () => {
    const { rerender } = render(<WallpaperPlayer url="test.mp4" shouldPlay={true} />);
    rerender(<WallpaperPlayer url="test.mp4" shouldPlay={false} />);
    // Verify video is paused
  });
});
```

**Backend (Rust):**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_weather_code_mapping() {
        let weather = map_weather_code(0);
        assert_eq!(weather, "Clear");
    }
    
    #[tokio::test]
    async fn test_context_analysis() {
        let context = analyze_context("Clear", "Morning", true);
        assert_eq!(context.mood, "Energetic");
    }
}
```

### Integration Tests

**E2E (Tauri Testing):**
```typescript
import { test, expect } from '@playwright/test';

test('complete wallpaper flow', async ({ page }) => {
  // Launch app
  await page.goto('http://localhost:1420');
  
  // Select wallpaper
  await page.click('[data-testid="wallpaper-card"]:first-child');
  await page.click('button:has-text("Set as Wallpaper")');
  
  // Verify wallpaper is set
  await expect(page.locator('[data-testid="wallpaper-active"]')).toBeVisible();
  
  // Open settings
  await page.click('[data-testid="settings-button"]');
  await expect(page.locator('[data-testid="settings-overlay"]')).toBeVisible();
});
```

## 📈 Monitoring & Logging

### Logging

**Frontend:**
```typescript
const logger = {
  info: (message: string, data?: any) => {
    console.log(`[INFO] ${message}`, data);
    // Send to backend for persistent logging
  },
  error: (message: string, error?: any) => {
    console.error(`[ERROR] ${message}`, error);
    // Send error reports
  }
};
```

**Backend:**
```rust
use log::{info, error, warn};

info!("WorkerW initialized successfully");
error!("Failed to find WorkerW window: {}", err);
warn!("High CPU usage detected: {}%", cpu_usage);
```

### Performance Monitoring

**Metrics to Track:**
- Startup time
- Memory usage
- CPU usage
- Video playback smoothness
- API response times
- Battery impact (laptops)

## 🚀 Deployment Architecture

### Build Process

```
Source Code
    ↓
npm run build (Vite)
    ↓
dist/ (Frontend assets)
    ↓
cargo build --release (Rust)
    ↓
target/release/arise-desktop.exe
    ↓
NSIS Installer
    ↓
Arise Desktop Setup.exe
```

### Distribution Channels

1. **GitHub Releases** - Primary distribution
2. **Website Download** - Official website
3. **Package Managers** - Future (winget, chocolatey)
4. **Auto-Updater** - Future implementation

## 🔄 Future Architecture Enhancements

### Planned Improvements

1. **Plugin System:** Allow third-party extensions
2. **Cloud Sync:** Sync settings and wallpapers across devices
3. **Community Hub:** Share and discover wallpapers
4. **AI Assistant:** Voice commands and suggestions
5. **Mobile App:** Companion mobile application
6. **WebSocket Support:** Real-time updates
7. **Database:** SQLite for better data management
8. **Caching Layer:** Redis for API caching
9. **Microservices:** Split backend into services
10. **GraphQL:** Alternative API layer

## 📚 Resources

### Documentation
- [Tauri Architecture](https://tauri.app/v1/guides/architecture/)
- [React Architecture](https://react.dev/learn/thinking-in-react)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [Three.js Architecture](https://threejs.org/manual/#en/introduction/Installation)

### Best Practices
- Clean Code principles
- SOLID principles
- DRY (Don't Repeat Yourself)
- Separation of Concerns
- Single Responsibility Principle