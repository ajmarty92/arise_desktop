# Arise Desktop - Source Code Examples

This document provides key source code examples from the Arise Desktop project.

## Table of Contents
- [Frontend Examples](#frontend-examples)
- [Backend Examples](#backend-examples)
- [Integration Examples](#integration-examples)

---

## Frontend Examples

### App.tsx - Main Application

```typescript
import React, { useState, useEffect } from 'react';
import { invoke, listen } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import { WallpaperPlayer } from './components/WallpaperPlayer';
import { Gallery } from './components/Gallery';
import { SettingsOverlay } from './components/SettingsOverlay';
import { DreamTab } from './components/DreamTab';
import { VideoRemaster } from './components/VideoRemaster';
import { FitToScreen } from './components/FitToScreen';
import { ParallaxWallpaper } from './components/ParallaxWallpaper';
import { ClockWidget } from './components/widgets/ClockWidget';
import { SystemStatsWidget } from './components/widgets/SystemStatsWidget';
import { SmartShuffleWidget } from './components/widgets/SmartShuffleWidget';
import './styles/App.css';

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

function App() {
  // Wallpaper Management
  const [wallpapers, setWallpapers] = useState<WallpaperItem[]>([]);
  const [activeWallpaperUrl, setActiveWallpaperUrl] = useState<string | null>(null);
  
  // WorkerW Hack State
  const [workerWHandle, setWorkerWHandle] = useState<string | null>(null);
  const [isWallpaperMode, setIsWallpaperMode] = useState(false);
  
  // Process Monitor State
  const [isMonitorRunning, setIsMonitorRunning] = useState(false);
  const [isPausedByMonitor, setIsPausedByMonitor] = useState(false);
  
  // Playback Control
  const [shouldPlay, setShouldPlay] = useState(true);
  const [isMuted, setIsMuted] = useState(false);
  const [isPerformanceMode, setIsPerformanceMode] = useState(false);
  
  // UI State
  const [isSettingsOverlayOpen, setIsSettingsOverlayOpen] = useState(false);
  const [viewMode, setViewMode] = useState<'player' | 'gallery' | 'settings'>('gallery');
  
  // Widget State
  const [showClockWidget, setShowClockWidget] = useState(true);
  const [showStatsWidget, setShowStatsWidget] = useState(false);
  const [showShuffleWidget, setShowShuffleWidget] = useState(true);
  
  // Modal States
  const [dreamWallpaper, setDreamWallpaper] = useState<WallpaperItem | null>(null);
  const [remasterWallpaper, setRemasterWallpaper] = useState<WallpaperItem | null>(null);
  const [fitToScreenWallpaper, setFitToScreenWallpaper] = useState<WallpaperItem | null>(null);
  const [parallaxWallpaper, setParallaxWallpaper] = useState<WallpaperItem | null>(null);

  // Initialize WorkerW on mount
  useEffect(() => {
    initWorkerW();
    loadWallpapers();
    setupProcessMonitor();
    loadSettings();
    
    // Listen for playback control events
    const unlisten = listen('playback-control', (event) => {
      const { shouldPause } = event.payload as { shouldPause: boolean };
      setShouldPlay(!shouldPause);
      setIsPausedByMonitor(shouldPause);
    });
    
    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  const initWorkerW = async () => {
    try {
      const handle = await invoke<string>('init_worker_w');
      setWorkerWHandle(handle);
      console.log('WorkerW initialized:', handle);
    } catch (error) {
      console.error('Failed to initialize WorkerW:', error);
    }
  };

  const loadWallpapers = async () => {
    // Load wallpapers from local storage or default samples
    const sampleWallpapers: WallpaperItem[] = [
      {
        id: '1',
        url: '/videos/nature.mp4',
        type: 'video',
        title: 'Serene Nature',
        duration: 30,
        thumbnail: '/thumbnails/nature.jpg',
        tags: ['nature', 'calm', 'outdoor']
      },
      {
        id: '2',
        url: '/videos/city.mp4',
        type: 'video',
        title: 'City Lights',
        duration: 25,
        thumbnail: '/thumbnails/city.jpg',
        tags: ['urban', 'night', 'energetic']
      },
      {
        id: '3',
        url: '/images/mountain.jpg',
        type: 'image',
        title: 'Mountain Vista',
        isStatic: true,
        thumbnail: '/thumbnails/mountain.jpg',
        tags: ['nature', 'mountain', 'daytime']
      }
    ];
    setWallpapers(sampleWallpapers);
  };

  const setupProcessMonitor = async () => {
    try {
      await invoke('start_process_monitor');
      setIsMonitorRunning(true);
    } catch (error) {
      console.error('Failed to start process monitor:', error);
    }
  };

  const loadSettings = () => {
    const savedSettings = localStorage.getItem('arise_settings');
    if (savedSettings) {
      const settings = JSON.parse(savedSettings);
      setIsMuted(settings.muted || false);
      setIsPerformanceMode(settings.performanceMode || false);
      setShowClockWidget(settings.showClockWidget !== false);
      setShowStatsWidget(settings.showStatsWidget || false);
      setShowShuffleWidget(settings.showShuffleWidget !== false);
    }
  };

  const handleSetWallpaper = async (wallpaper: WallpaperItem) => {
    try {
      if (workerWHandle) {
        await invoke('set_as_wallpaper', { handle: workerWHandle });
        setIsWallpaperMode(true);
        setActiveWallpaperUrl(wallpaper.url);
      }
    } catch (error) {
      console.error('Failed to set wallpaper:', error);
    }
  };

  const handleRestoreWallpaper = async () => {
    try {
      await invoke('restore_from_wallpaper');
      setIsWallpaperMode(false);
      setActiveWallpaperUrl(null);
    } catch (error) {
      console.error('Failed to restore wallpaper:', error);
    }
  };

  const handleDreamComplete = (newWallpaper: WallpaperItem) => {
    setWallpapers(prev => [newWallpaper, ...prev]);
    setDreamWallpaper(null);
  };

  const handleRemasterComplete = (remasteredItem: WallpaperItem) => {
    setWallpapers(prev => prev.map(w => 
      w.id === remasteredItem.id ? remasteredItem : w
    ));
    setRemasterWallpaper(null);
  };

  const handleFitToScreenComplete = (extendedItem: WallpaperItem) => {
    setWallpapers(prev => [extendedItem, ...prev]);
    setFitToScreenWallpaper(null);
  };

  const handleParallaxDepthReady = (wallpaper: WallpaperItem) => {
    setWallpapers(prev => prev.map(w => 
      w.id === wallpaper.id ? { ...w, hasDepthMap: true } : w
    ));
  };

  return (
    <div className="app">
      {/* Main Content */}
      <div className="main-content">
        {viewMode === 'gallery' && (
          <Gallery
            wallpapers={wallpapers}
            onSetWallpaper={handleSetWallpaper}
            onDream={setDreamWallpaper}
            onRemaster={setRemasterWallpaper}
            onFitToScreen={setFitToScreenWallpaper}
            onParallax={setParallaxWallpaper}
          />
        )}
        
        {viewMode === 'player' && activeWallpaperUrl && (
          <WallpaperPlayer
            url={activeWallpaperUrl}
            shouldPlay={shouldPlay}
            muted={isMuted}
            onRestore={handleRestoreWallpaper}
          />
        )}
      </div>

      {/* Widgets */}
      {showClockWidget && <ClockWidget />}
      {showStatsWidget && <SystemStatsWidget />}
      {showShuffleWidget && <SmartShuffleWidget />}

      {/* Settings Overlay */}
      {isSettingsOverlayOpen && (
        <SettingsOverlay
          isOpen={isSettingsOverlayOpen}
          onClose={() => setIsSettingsOverlayOpen(false)}
          settings={{
            muted: isMuted,
            performanceMode: isPerformanceMode,
            showClockWidget,
            showStatsWidget,
            showShuffleWidget
          }}
          onSettingsChange={(newSettings) => {
            setIsMuted(newSettings.muted);
            setIsPerformanceMode(newSettings.performanceMode);
            setShowClockWidget(newSettings.showClockWidget);
            setShowStatsWidget(newSettings.showStatsWidget);
            setShowShuffleWidget(newSettings.showShuffleWidget);
          }}
        />
      )}

      {/* Modals */}
      {dreamWallpaper && (
        <DreamTab
          isOpen={true}
          onClose={() => setDreamWallpaper(null)}
          onComplete={handleDreamComplete}
        />
      )}

      {remasterWallpaper && (
        <VideoRemaster
          isOpen={true}
          onClose={() => setRemasterWallpaper(null)}
          wallpaper={remasterWallpaper}
          onComplete={handleRemasterComplete}
        />
      )}

      {fitToScreenWallpaper && (
        <FitToScreen
          isOpen={true}
          onClose={() => setFitToScreenWallpaper(null)}
          wallpaper={fitToScreenWallpaper}
          onComplete={handleFitToScreenComplete}
        />
      )}

      {parallaxWallpaper && (
        <ParallaxWallpaper
          isOpen={true}
          onClose={() => setParallaxWallpaper(null)}
          wallpaper={parallaxWallpaper}
          onDepthReady={handleParallaxDepthReady}
        />
      )}
    </div>
  );
}

export default App;
```

---

## Backend Examples

### lib.rs - Main Rust Entry Point

```rust
use tauri::Manager;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

// State management
struct AppState {
    workerw_handle: Mutex<Option<String>>,
    is_monitoring: Mutex<bool>,
    active_jobs: RwLock<Vec<JobStatus>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct JobStatus {
    id: String,
    job_type: String,
    status: String,
    progress: u8,
    output: Option<String>,
    error: Option<String>,
}

// WorkerW Commands
#[tauri::command]
async fn init_worker_w(state: tauri::State<'_, AppState>) -> Result<String, String> {
    // Find Progman window
    let progman = find_window("Progman");
    if progman.is_none() {
        return Err("Progman window not found".to_string());
    }

    // Send 0x052C message to spawn WorkerW
    let result = unsafe {
        SendMessageA(progman.unwrap(), 0x052C, 0, 0)
    };

    // Find WorkerW window
    let workerw = find_workerw_window().await?;
    
    // Store handle
    let mut handle = state.workerw_handle.lock().unwrap();
    *handle = Some(workerw.to_string());
    
    Ok(workerw.to_string())
}

#[tauri::command]
async fn set_as_wallpaper(
    handle: String,
    window: tauri::Window,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let hwnd = handle.parse::<usize>()
        .map_err(|e| format!("Invalid handle: {}", e))?;
    
    unsafe {
        // Set window as child of WorkerW
        SetParent(window.hwnd() as HWND, hwnd as HWND);
        
        // Set window position and size
        SetWindowPos(
            window.hwnd() as HWND,
            HWND_TOPMOST,
            0, 0,
            GetSystemMetrics(SM_CXSCREEN),
            GetSystemMetrics(SM_CYSCREEN),
            SWP_SHOWWINDOW
        );
    }
    
    Ok(())
}

#[tauri::command]
async fn restore_from_wallpaper(
    window: tauri::Window,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    unsafe {
        // Restore window to desktop
        SetParent(window.hwnd() as HWND, HWND_DESKTOP);
        
        // Show window
        ShowWindow(window.hwnd() as HWND, SW_SHOW);
    }
    
    Ok(())
}

// Process Monitor Commands
#[tauri::command]
async fn start_process_monitor(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let is_monitoring = state.is_monitoring.lock().unwrap();
    if *is_monitoring {
        return Ok(());
    }
    drop(is_monitoring);
    
    // Start monitoring loop
    tokio::spawn(async move {
        monitor_processes(app_handle).await;
    });
    
    let mut is_monitoring = state.is_monitoring.lock().unwrap();
    *is_monitoring = true;
    
    Ok(())
}

#[tauri::command]
async fn stop_process_monitor(
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let mut is_monitoring = state.is_monitoring.lock().unwrap();
    *is_monitoring = false;
    Ok(())
}

#[tauri::command]
async fn check_system_state() -> Result<bool, String> {
    let has_fullscreen = detect_fullscreen_application().await?;
    Ok(has_fullscreen)
}

// System Stats Commands
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

// Smart Shuffle Commands
#[tauri::command]
async fn get_smart_context() -> Result<SmartContext, String> {
    // Get location
    let location = get_location_data().await?;
    
    // Get weather
    let weather = get_weather_data(location.lat, location.lon).await?;
    
    // Get time context
    let time_context = get_time_context();
    
    // Analyze context
    let context = analyze_context(&weather, &time_context).await?;
    
    Ok(context)
}

// AI Dream Commands
#[tauri::command]
async fn create_dream(
    prompt: String,
    style: String,
    api_key: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let job_id = uuid::Uuid::new_v4().to_string();
    
    // Create job on Replicate
    let replicate_job = create_replicate_job(
        &prompt,
        &style,
        &api_key
    ).await?;
    
    // Store job status
    let job_status = JobStatus {
        id: job_id.clone(),
        job_type: "dream".to_string(),
        status: "pending".to_string(),
        progress: 0,
        output: None,
        error: None,
    };
    
    let mut jobs = state.active_jobs.write().await;
    jobs.push(job_status);
    
    Ok(job_id)
}

#[tauri::command]
async fn check_dream_status(
    job_id: String,
    state: tauri::State<'_, AppState>
) -> Result<DreamStatus, String> {
    let jobs = state.active_jobs.read().await;
    let job = jobs.iter()
        .find(|j| j.id == job_id)
        .ok_or("Job not found")?;
    
    Ok(DreamStatus {
        status: job.status.clone(),
        output: job.output.clone(),
        error: job.error.clone(),
        progress: job.progress,
    })
}

#[tauri::command]
async fn download_dream(
    job_id: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let jobs = state.active_jobs.read().await;
    let job = jobs.iter()
        .find(|j| j.id == job_id)
        .ok_or("Job not found")?;
    
    let output_url = job.output.as_ref()
        .ok_or("No output available")?;
    
    // Download video
    let local_path = download_video(output_url, &format!("dreams/{}", job_id)).await?;
    
    Ok(local_path)
}

// Helper Functions
async fn monitor_processes(app_handle: tauri::AppHandle) {
    loop {
        // Check for fullscreen apps
        let has_fullscreen = detect_fullscreen_application().await.unwrap_or(false);
        
        // Emit event
        let _ = app_handle.emit_all("playback-control", json!({
            "shouldPause": has_fullscreen,
            "reason": if has_fullscreen { 
                "Fullscreen application detected" 
            } else { 
                "No fullscreen application" 
            }
        }));
        
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}

async fn create_replicate_job(
    prompt: &str,
    style: &str,
    api_key: &str
) -> Result<String, String> {
    let client = reqwest::Client::new();
    
    let response = client
        .post("https://api.replicate.com/v1/predictions")
        .header("Authorization", format!("Token {}", api_key))
        .json(&json!({
            "version": "stability-ai/stable-video-diffusion",
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
    
    let job_id = result["id"].as_str()
        .ok_or("No job ID in response")?
        .to_string();
    
    Ok(job_id)
}

// Main entry point
#[tokio::main]
async fn main() {
    let app_state = AppState {
        workerw_handle: Mutex::new(None),
        is_monitoring: Mutex::new(false),
        active_jobs: RwLock::new(vec![]),
    };
    
    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // WorkerW
            init_worker_w,
            set_as_wallpaper,
            restore_from_wallpaper,
            // Process Monitor
            start_process_monitor,
            stop_process_monitor,
            check_system_state,
            // System Stats
            get_system_stats,
            // Smart Shuffle
            get_smart_context,
            // AI Dream
            create_dream,
            check_dream_status,
            download_dream,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Integration Examples

### API Client (src/lib/api.ts)

```typescript
import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

// Type definitions
export interface SystemStats {
  cpu: number;
  ram: number;
  cpuHistory: number[];
  ramHistory: number[];
}

export interface SmartContext {
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

export interface DreamStatus {
  status: 'pending' | 'processing' | 'succeeded' | 'failed';
  output: string | null;
  error: string | null;
  progress: number;
}

// API Client
export const api = {
  // WorkerW Commands
  initWorkerW: (): Promise<string> => invoke('init_worker_w'),
  setAsWallpaper: (handle: string): Promise<void> => 
    invoke('set_as_wallpaper', { handle }),
  restoreFromWallpaper: (): Promise<void> => 
    invoke('restore_from_wallpaper'),
  
  // Process Monitor Commands
  startProcessMonitor: (): Promise<void> => 
    invoke('start_process_monitor'),
  stopProcessMonitor: (): Promise<void> => 
    invoke('stop_process_monitor'),
  checkSystemState: (): Promise<boolean> => 
    invoke('check_system_state'),
  
  // System Stats Commands
  getSystemStats: (): Promise<SystemStats> => 
    invoke('get_system_stats'),
  
  // Smart Shuffle Commands
  getSmartContext: (): Promise<SmartContext> => 
    invoke('get_smart_context'),
  
  // AI Dream Commands
  createDream: (prompt: string, style: string, apiKey: string): Promise<string> => 
    invoke('create_dream', { prompt, style, apiKey }),
  checkDreamStatus: (jobId: string): Promise<DreamStatus> => 
    invoke('check_dream_status', { jobId }),
  downloadDream: (jobId: string): Promise<string> => 
    invoke('download_dream', { jobId }),
};

// Event Listeners
export const events = {
  onPlaybackControl: (callback: (data: { shouldPause: boolean, reason: string }) => void): Promise<UnlistenFn> => 
    listen('playback-control', callback),
};

// Polling Helper
export const pollStatus = async <T>(
  getStatus: () => Promise<T>,
  isComplete: (status: T) => boolean,
  interval: number = 1000
): Promise<T> => {
  while (true) {
    const status = await getStatus();
    if (isComplete(status)) {
      return status;
    }
    await new Promise(resolve => setTimeout(resolve, interval));
  }
};

// Usage Example
export const exampleUsage = async () => {
  // Initialize WorkerW
  const handle = await api.initWorkerW();
  console.log('WorkerW handle:', handle);
  
  // Set as wallpaper
  await api.setAsWallpaper(handle);
  
  // Start process monitor
  await api.startProcessMonitor();
  
  // Listen to playback control events
  const unlisten = await events.onPlaybackControl((data) => {
    console.log('Playback control:', data);
    if (data.shouldPause) {
      console.log('Pausing video:', data.reason);
    }
  });
  
  // Get smart context
  const context = await api.getSmartContext();
  console.log('Current context:', context);
  
  // Create AI dream
  const apiKey = localStorage.getItem('replicate_api_key') || '';
  const jobId = await api.createDream(
    'A serene sunset over mountains',
    'cinematic',
    apiKey
  );
  
  // Poll for completion
  const finalStatus = await pollStatus(
    () => api.checkDreamStatus(jobId),
    (status) => status.status === 'succeeded' || status.status === 'failed'
  );
  
  if (finalStatus.status === 'succeeded') {
    const videoPath = await api.downloadDream(jobId);
    console.log('Dream downloaded:', videoPath);
  }
  
  // Cleanup
  unlisten();
};
```

---

## Component Examples

### WallpaperPlayer Component

```typescript
import React, { useRef, useState, useEffect } from 'react';
import { Play, Pause, Volume2, VolumeX, SkipBack, SkipForward } from 'lucide-react';
import './WallpaperPlayer.css';

interface WallpaperPlayerProps {
  url: string;
  shouldPlay: boolean;
  muted: boolean;
  onRestore?: () => void;
}

export const WallpaperPlayer: React.FC<WallpaperPlayerProps> = ({
  url,
  shouldPlay,
  muted,
  onRestore
}) => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [isMuted, setIsMuted] = useState(muted);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [showControls, setShowControls] = useState(false);

  useEffect(() => {
    const video = videoRef.current;
    if (!video) return;

    if (shouldPlay) {
      video.play();
      setIsPlaying(true);
    } else {
      video.pause();
      setIsPlaying(false);
    }
  }, [shouldPlay]);

  useEffect(() => {
    setIsMuted(muted);
  }, [muted]);

  const togglePlayPause = () => {
    const video = videoRef.current;
    if (!video) return;

    if (isPlaying) {
      video.pause();
      setIsPlaying(false);
    } else {
      video.play();
      setIsPlaying(true);
    }
  };

  const toggleMute = () => {
    const video = videoRef.current;
    if (!video) return;

    video.muted = !video.muted;
    setIsMuted(video.muted);
  };

  const handleTimeUpdate = () => {
    const video = videoRef.current;
    if (!video) return;

    setCurrentTime(video.currentTime);
  };

  const handleLoadedMetadata = () => {
    const video = videoRef.current;
    if (!video) return;

    setDuration(video.duration);
  };

  const handleSeek = (e: React.ChangeEvent<HTMLInputElement>) => {
    const video = videoRef.current;
    if (!video) return;

    const time = parseFloat(e.target.value);
    video.currentTime = time;
    setCurrentTime(time);
  };

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <div className="wallpaper-player">
      <video
        ref={videoRef}
        className="wallpaper-video"
        src={url}
        autoPlay={shouldPlay}
        loop
        muted={isMuted}
        onTimeUpdate={handleTimeUpdate}
        onLoadedMetadata={handleLoadedMetadata}
      />
      
      <div 
        className={`controls-overlay ${showControls ? 'visible' : ''}`}
        onMouseEnter={() => setShowControls(true)}
        onMouseLeave={() => setShowControls(false)}
      >
        <div className="controls-row">
          <button onClick={togglePlayPause} className="control-btn">
            {isPlaying ? <Pause size={24} /> : <Play size={24} />}
          </button>
          
          <button onClick={toggleMute} className="control-btn">
            {isMuted ? <VolumeX size={24} /> : <Volume2 size={24} />}
          </button>
          
          <div className="progress-bar">
            <input
              type="range"
              min="0"
              max={duration}
              value={currentTime}
              onChange={handleSeek}
              className="progress-slider"
            />
            <span className="time-display">
              {formatTime(currentTime)} / {formatTime(duration)}
            </span>
          </div>
          
          {onRestore && (
            <button onClick={onRestore} className="control-btn restore-btn">
              Restore Window
            </button>
          )}
        </div>
      </div>
    </div>
  );
};
```

---

This document provides comprehensive source code examples for the Arise Desktop project. For complete implementation details, refer to the full source code in the project repository.

**Last Updated:** 2024
**Version:** 1.0.0