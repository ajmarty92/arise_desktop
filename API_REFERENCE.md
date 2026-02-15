# Arise Desktop - API Reference

Complete API documentation for Tauri commands and frontend utilities.

## Table of Contents

- [WorkerW Commands](#workerw-commands)
- [Process Monitor Commands](#process-monitor-commands)
- [System Stats Commands](#system-stats-commands)
- [Smart Shuffle Commands](#smart-shuffle-commands)
- [AI Dream Commands](#ai-dream-commands)
- [Video Remaster Commands](#video-remaster-commands)
- [Generative Fill Commands](#generative-fill-commands)
- [Depth Estimation Commands](#depth-estimation-commands)
- [Auto-Start Commands](#auto-start-commands)
- [Frontend Utilities](#frontend-utilities)

---

## WorkerW Commands

Commands for managing the Windows WorkerW hack to place video behind desktop icons.

### init_worker_w

Initialize the WorkerW hack by finding and spawning the WorkerW window.

**Command:**
```typescript
await invoke('init_worker_w')
```

**Returns:** `Promise<string>`

WorkerW window handle identifier.

**Example:**
```typescript
const handle = await invoke('init_worker_w');
console.log('WorkerW handle:', handle);
```

---

### set_as_wallpaper

Set the application window as wallpaper by parenting it to WorkerW.

**Command:**
```typescript
await invoke('set_as_wallpaper', { handle: string })
```

**Parameters:**
- `handle` (string): WorkerW window handle from `init_worker_w`

**Returns:** `Promise<void>`

**Example:**
```typescript
await invoke('set_as_wallpaper', { handle: '12345' });
```

---

### restore_from_wallpaper

Restore the window from wallpaper mode back to normal window.

**Command:**
```typescript
await invoke('restore_from_wallpaper')
```

**Returns:** `Promise<void>`

**Example:**
```typescript
await invoke('restore_from_wallpaper');
```

---

## Process Monitor Commands

Commands for monitoring system processes and auto-pausing during fullscreen apps.

### start_process_monitor

Start the background process monitor.

**Command:**
```typescript
await invoke('start_process_monitor')
```

**Returns:** `Promise<void>`

**Emits Events:**
- `playback-control` - Emitted when fullscreen app detected

**Event Payload:**
```typescript
{
  shouldPause: boolean;
  reason: string;
}
```

**Example:**
```typescript
await invoke('start_process_monitor');

// Listen for events
const unlisten = await listen('playback-control', (event) => {
  const { shouldPause, reason } = event.payload;
  console.log('Should pause:', shouldPause, 'Reason:', reason);
});
```

---

### stop_process_monitor

Stop the background process monitor.

**Command:**
```typescript
await invoke('stop_process_monitor')
```

**Returns:** `Promise<void>`

**Example:**
```typescript
await invoke('stop_process_monitor');
```

---

### check_system_state

Check if a fullscreen application is currently running.

**Command:**
```typescript
await invoke('check_system_state')
```

**Returns:** `Promise<boolean>`

`true` if fullscreen app detected, `false` otherwise.

**Example:**
```typescript
const isFullscreen = await invoke('check_system_state');
if (isFullscreen) {
  console.log('Fullscreen app detected');
}
```

---

## System Stats Commands

Commands for monitoring system resources (CPU, RAM).

### get_system_stats

Get current system resource usage.

**Command:**
```typescript
await invoke('get_system_stats')
```

**Returns:** `Promise<SystemStats>`

```typescript
interface SystemStats {
  cpu: number;         // CPU usage percentage (0-100)
  ram: number;         // RAM usage percentage (0-100)
  cpuHistory: number[]; // Historical CPU readings
  ramHistory: number[]; // Historical RAM readings
}
```

**Example:**
```typescript
const stats = await invoke('get_system_stats');
console.log(`CPU: ${stats.cpu}%, RAM: ${stats.ram}%`);
```

---

## Smart Shuffle Commands

Commands for context-aware wallpaper selection.

### get_smart_context

Get current context (weather, time, location) based on IP geolocation.

**Command:**
```typescript
await invoke('get_smart_context')
```

**Returns:** `Promise<SmartContext>`

```typescript
interface SmartContext {
  weather: {
    condition: string;  // "Clear", "Cloudy", "Rain", "Snow", "Storm", "Fog"
    temperature: number; // Temperature in Celsius
  };
  time: {
    period: string;     // "Dawn", "Morning", "Afternoon", "Evening", "Night"
    isDay: boolean;
  };
  location: {
    city: string;
    country: string;
  };
  mood: string;        // "Calm", "Energetic", "Melancholic", etc.
  tags: string[];      // Related tags for matching
}
```

**Example:**
```typescript
const context = await invoke('get_smart_context');
console.log(`Weather: ${context.weather.condition}`);
console.log(`Time: ${context.time.period}`);
console.log(`Mood: ${context.mood}`);
```

---

### get_smart_context_for_location

Get context for a specific geographic location.

**Command:**
```typescript
await invoke('get_smart_context_for_location', { 
  lat: number, 
  lon: number 
})
```

**Parameters:**
- `lat` (number): Latitude
- `lon` (number): Longitude

**Returns:** `Promise<SmartContext>`

**Example:**
```typescript
const context = await invoke('get_smart_context_for_location', {
  lat: 40.7128,
  lon: -74.0060
});
```

---

### get_current_time_context

Get current time context without location/weather.

**Command:**
```typescript
await invoke('get_current_time_context')
```

**Returns:** `Promise<TimeContext>`

```typescript
interface TimeContext {
  period: string;  // "Dawn", "Morning", "Afternoon", "Evening", "Night"
  isDay: boolean;
}
```

**Example:**
```typescript
const timeContext = await invoke('get_current_time_context');
console.log(`Period: ${timeContext.period}`);
```

---

## AI Dream Commands

Commands for AI-powered video generation.

### create_dream

Create a new AI-generated video wallpaper.

**Command:**
```typescript
await invoke('create_dream', { 
  prompt: string, 
  style: string,
  apiKey: string 
})
```

**Parameters:**
- `prompt` (string): Text prompt for generation
- `style` (string): Style preset ("cinematic", "anime", "realistic", etc.)
- `apiKey` (string): Replicate API key

**Returns:** `Promise<string>`

Job ID for tracking generation progress.

**Example:**
```typescript
const jobId = await invoke('create_dream', {
  prompt: "A serene sunset over mountains",
  style: "cinematic",
  apiKey: "your_replicate_api_key"
});
console.log('Job ID:', jobId);
```

---

### check_dream_status

Check the status of a dream generation job.

**Command:**
```typescript
await invoke('check_dream_status', { jobId: string })
```

**Parameters:**
- `jobId` (string): Job ID from `create_dream`

**Returns:** `Promise<DreamStatus>`

```typescript
interface DreamStatus {
  status: string;      // "pending", "processing", "succeeded", "failed"
  output: string | null;  // Video URL if succeeded
  error: string | null;   // Error message if failed
  progress: number;    // Progress percentage (0-100)
}
```

**Example:**
```typescript
const status = await invoke('check_dream_status', { jobId: 'job_123' });
console.log('Status:', status.status);
console.log('Progress:', status.progress);
```

---

### download_dream

Download a completed dream video to local storage.

**Command:**
```typescript
await invoke('download_dream', { jobId: string })
```

**Parameters:**
- `jobId` (string): Job ID from `create_dream`

**Returns:** `Promise<string>`

Local file path to downloaded video.

**Example:**
```typescript
const filePath = await invoke('download_dream', { jobId: 'job_123' });
console.log('Downloaded to:', filePath);
```

---

### list_dreams

List all saved dream videos.

**Command:**
```typescript
await invoke('list_dreams')
```

**Returns:** `Promise<DreamMetadata[]>`

```typescript
interface DreamMetadata {
  id: string;
  prompt: string;
  style: string;
  status: string;
  createdAt: string;
  videoUrl?: string;
  error?: string;
}
```

**Example:**
```typescript
const dreams = await invoke('list_dreams');
dreams.forEach(dream => {
  console.log(`${dream.prompt} - ${dream.status}`);
});
```

---

### delete_dream

Delete a saved dream video.

**Command:**
```typescript
await invoke('delete_dream', { dreamId: string })
```

**Parameters:**
- `dreamId` (string): Dream ID to delete

**Returns:** `Promise<void>`

**Example:**
```typescript
await invoke('delete_dream', { dreamId: 'dream_123' });
```

---

## Video Remaster Commands

Commands for AI-powered video upscaling.

### get_video_info

Get information about a video file.

**Command:**
```typescript
await invoke('get_video_info', { videoPath: string })
```

**Parameters:**
- `videoPath` (string): Path to video file

**Returns:** `Promise<VideoInfo>`

```typescript
interface VideoInfo {
  width: number;
  height: number;
  duration: number;
  codec: string;
  bitrate: number;
}
```

**Example:**
```typescript
const info = await invoke('get_video_info', { 
  videoPath: '/path/to/video.mp4' 
});
console.log(`Resolution: ${info.width}x${info.height}`);
```

---

### start_video_upscale

Start AI upscaling of a video.

**Command:**
```typescript
await invoke('start_video_upscale', { 
  videoPath: string,
  apiKey: string 
})
```

**Parameters:**
- `videoPath` (string): Path to video file
- `apiKey` (string): Replicate API key

**Returns:** `Promise<string>`

Job ID for tracking progress.

**Example:**
```typescript
const jobId = await invoke('start_video_upscale', {
  videoPath: '/path/to/video.mp4',
  apiKey: 'your_replicate_api_key'
});
```

---

### check_upscale_status

Check upscaling job status.

**Command:**
```typescript
await invoke('check_upscale_status', { jobId: string })
```

**Parameters:**
- `jobId` (string): Job ID from `start_video_upscale`

**Returns:** `Promise<UpscaleStatus>`

```typescript
interface UpscaleStatus {
  status: string;      // "pending", "processing", "succeeded", "failed"
  output: string | null;  // Video URL if succeeded
  error: string | null;   // Error message if failed
  progress: number;    // Progress percentage (0-100)
}
```

**Example:**
```typescript
const status = await invoke('check_upscale_status', { jobId: 'job_123' });
console.log('Progress:', status.progress);
```

---

### download_upscaled_video

Download upscaled video.

**Command:**
```typescript
await invoke('download_upscaled_video', { jobId: string })
```

**Parameters:**
- `jobId` (string): Job ID from `start_video_upscale`

**Returns:** `Promise<string>`

Local file path to upscaled video.

**Example:**
```typescript
const filePath = await invoke('download_upscaled_video', { 
  jobId: 'job_123' 
});
```

---

### list_remastered_videos

List all remastered videos.

**Command:**
```typescript
await invoke('list_remastered_videos')
```

**Returns:** `Promise<RemasteredVideo[]>`

```typescript
interface RemasteredVideo {
  id: string;
  originalPath: string;
  upscaledPath: string;
  originalResolution: string;
  upscaledResolution: string;
  createdAt: string;
}
```

**Example:**
```typescript
const videos = await invoke('list_remastered_videos');
videos.forEach(video => {
  console.log(`${video.originalResolution} -> ${video.upscaledResolution}`);
});
```

---

## Generative Fill Commands

Commands for AI-powered aspect ratio extension.

### analyze_aspect_fit

Analyze if video needs aspect ratio extension.

**Command:**
```typescript
await invoke('analyze_aspect_fit', { videoPath: string })
```

**Parameters:**
- `videoPath` (string): Path to video file

**Returns:** `Promise<FitAnalysis>`

```typescript
interface FitAnalysis {
  needsExtension: boolean;
  videoAspect: number;
  monitorAspect: number;
  extensionType: 'left-right' | 'top-bottom' | 'none';
  extensionAmount: number;
}
```

**Example:**
```typescript
const analysis = await invoke('analyze_aspect_fit', { 
  videoPath: '/path/to/video.mp4' 
});
console.log('Needs extension:', analysis.needsExtension);
```

---

### get_monitor_info

Get monitor resolution information.

**Command:**
```typescript
await invoke('get_monitor_info')
```

**Returns:** `Promise<MonitorInfo>`

```typescript
interface MonitorInfo {
  width: number;
  height: number;
  refreshRate: number;
  aspectRatio: number;
}
```

**Example:**
```typescript
const monitor = await invoke('get_monitor_info');
console.log(`Resolution: ${monitor.width}x${monitor.height}`);
```

---

### start_generative_fill

Start AI generative fill to extend video.

**Command:**
```typescript
await invoke('start_generative_fill', { 
  videoPath: string, 
  prompt: string,
  apiKey: string 
})
```

**Parameters:**
- `videoPath` (string): Path to video file
- `prompt` (string): Custom prompt for AI guidance
- `apiKey` (string): Replicate API key

**Returns:** `Promise<string>`

Job ID for tracking progress.

**Example:**
```typescript
const jobId = await invoke('start_generative_fill', {
  videoPath: '/path/to/video.mp4',
  prompt: "Seamless extension of the scene",
  apiKey: 'your_replicate_api_key'
});
```

---

### check_fill_status

Check generative fill status.

**Command:**
```typescript
await invoke('check_fill_status', { jobId: string })
```

**Parameters:**
- `jobId` (string): Job ID from `start_generative_fill`

**Returns:** `Promise<FillStatus>`

```typescript
interface FillStatus {
  status: string;      // "pending", "processing", "succeeded", "failed"
  output: string | null;  // Image URL if succeeded
  error: string | null;   // Error message if failed
  progress: number;    // Progress percentage (0-100)
}
```

**Example:**
```typescript
const status = await invoke('check_fill_status', { jobId: 'job_123' });
console.log('Status:', status.status);
```

---

### download_filled_image

Download generative fill result and create video.

**Command:**
```typescript
await invoke('download_filled_image', { jobId: string })
```

**Parameters:**
- `jobId` (string): Job ID from `start_generative_fill`

**Returns:** `Promise<string>`

Local file path to extended video.

**Example:**
```typescript
const filePath = await invoke('download_filled_image', { 
  jobId: 'job_123' 
});
console.log('Extended video:', filePath);
```

---

### get_fills_directory

Get directory path for filled wallpapers.

**Command:**
```typescript
await invoke('get_fills_directory')
```

**Returns:** `Promise<string>`

Directory path string.

**Example:**
```typescript
const dir = await invoke('get_fills_directory');
console.log('Fills directory:', dir);
```

---

### list_filled_wallpapers

List all filled wallpapers.

**Command:**
```typescript
await invoke('list_filled_wallpapers')
```

**Returns:** `Promise<FilledWallpaper[]>`

```typescript
interface FilledWallpaper {
  id: string;
  originalPath: string;
  filledPath: string;
  prompt: string;
  createdAt: string;
}
```

**Example:**
```typescript
const fills = await invoke('list_filled_wallpapers');
fills.forEach(fill => {
  console.log(`${fill.originalPath} -> ${fill.filledPath}`);
});
```

---

## Depth Estimation Commands

Commands for generating depth maps for parallax effects.

### generate_depth_map

Generate depth map for an image.

**Command:**
```typescript
await invoke('generate_depth_map', { 
  imageUrl: string,
  apiKey: string 
})
```

**Parameters:**
- `imageUrl` (string): URL or path to image
- `apiKey` (string): Replicate API key

**Returns:** `Promise<string>`

Job ID for tracking progress.

**Example:**
```typescript
const jobId = await invoke('generate_depth_map', {
  imageUrl: '/path/to/image.jpg',
  apiKey: 'your_replicate_api_key'
});
```

---

### check_depth_map_status

Check depth map generation status.

**Command:**
```typescript
await invoke('check_depth_map_status', { jobId: string })
```

**Parameters:**
- `jobId` (string): Job ID from `generate_depth_map`

**Returns:** `Promise<DepthStatus>`

```typescript
interface DepthStatus {
  status: string;      // "pending", "processing", "succeeded", "failed"
  output: string | null;  // Depth map URL if succeeded
  error: string | null;   // Error message if failed
  progress: number;    // Progress percentage (0-100)
}
```

**Example:**
```typescript
const status = await invoke('check_depth_map_status', { jobId: 'job_123' });
console.log('Progress:', status.progress);
```

---

### get_cached_depth_map

Check if depth map exists in cache.

**Command:**
```typescript
await invoke('get_cached_depth_map', { imageUrl: string })
```

**Parameters:**
- `imageUrl` (string): URL or path to image

**Returns:** `Promise<string | null>`

Depth map file path if exists, null otherwise.

**Example:**
```typescript
const cachedDepth = await invoke('get_cached_depth_map', { 
  imageUrl: '/path/to/image.jpg' 
});
if (cachedDepth) {
  console.log('Cached depth map:', cachedDepth);
}
```

---

### get_depth_maps_directory

Get directory path for depth maps.

**Command:**
```typescript
await invoke('get_depth_maps_directory')
```

**Returns:** `Promise<string>`

Directory path string.

**Example:**
```typescript
const dir = await invoke('get_depth_maps_directory');
console.log('Depth maps directory:', dir);
```

---

## Auto-Start Commands

Commands for managing Windows auto-start functionality.

### enable_auto_start

Enable auto-start with Windows.

**Command:**
```typescript
await invoke('enable_auto_start')
```

**Returns:** `Promise<void>`

**Example:**
```typescript
await invoke('enable_auto_start');
```

---

### disable_auto_start

Disable auto-start with Windows.

**Command:**
```typescript
await invoke('disable_auto_start')
```

**Returns:** `Promise<void>`

**Example:**
```typescript
await invoke('disable_auto_start');
```

---

### is_auto_start_enabled

Check if auto-start is enabled.

**Command:**
```typescript
await invoke('is_auto_start_enabled')
```

**Returns:** `Promise<boolean>`

`true` if auto-start is enabled, `false` otherwise.

**Example:**
```typescript
const isEnabled = await invoke('is_auto_start_enabled');
console.log('Auto-start:', isEnabled ? 'enabled' : 'disabled');
```

---

## Frontend Utilities

### API Client

Centralized API client for all Tauri commands.

**Location:** `src/lib/api.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';

export const api = {
  // WorkerW
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
  createDream: (prompt: string, style: string, apiKey: string) => 
    invoke('create_dream', { prompt, style, apiKey }),
  checkDreamStatus: (jobId: string) => 
    invoke('check_dream_status', { jobId }),
  downloadDream: (jobId: string) => 
    invoke('download_dream', { jobId }),
  listDreams: () => invoke('list_dreams'),
  deleteDream: (dreamId: string) => 
    invoke('delete_dream', { dreamId }),
  
  // Video Remaster
  getVideoInfo: (videoPath: string) => 
    invoke('get_video_info', { videoPath }),
  startVideoUpscale: (videoPath: string, apiKey: string) => 
    invoke('start_video_upscale', { videoPath, apiKey }),
  checkUpscaleStatus: (jobId: string) => 
    invoke('check_upscale_status', { jobId }),
  downloadUpscaledVideo: (jobId: string) => 
    invoke('download_upscaled_video', { jobId }),
  listRemasteredVideos: () => invoke('list_remastered_videos'),
  
  // Generative Fill
  analyzeAspectFit: (videoPath: string) => 
    invoke('analyze_aspect_fit', { videoPath }),
  getMonitorInfo: () => invoke('get_monitor_info'),
  startGenerativeFill: (videoPath: string, prompt: string, apiKey: string) => 
    invoke('start_generative_fill', { videoPath, prompt, apiKey }),
  checkFillStatus: (jobId: string) => 
    invoke('check_fill_status', { jobId }),
  downloadFilledImage: (jobId: string) => 
    invoke('download_filled_image', { jobId }),
  getFillsDirectory: () => invoke('get_fills_directory'),
  listFilledWallpapers: () => invoke('list_filled_wallpapers'),
  
  // Depth Estimation
  generateDepthMap: (imageUrl: string, apiKey: string) => 
    invoke('generate_depth_map', { imageUrl, apiKey }),
  checkDepthMapStatus: (jobId: string) => 
    invoke('check_depth_map_status', { jobId }),
  getCachedDepthMap: (imageUrl: string) => 
    invoke('get_cached_depth_map', { imageUrl }),
  getDepthMapsDirectory: () => invoke('get_depth_maps_directory'),
  
  // Auto-Start
  enableAutoStart: () => invoke('enable_auto_start'),
  disableAutoStart: () => invoke('disable_auto_start'),
  isAutoStartEnabled: () => invoke('is_auto_start_enabled')
};
```

### Custom Hooks

**Location:** `src/lib/hooks.ts`

#### useSystemStats

Hook for monitoring system resources.

```typescript
export const useSystemStats = () => {
  const [stats, setStats] = useState<SystemStats>({
    cpu: 0,
    ram: 0,
    cpuHistory: [],
    ramHistory: []
  });
  
  useEffect(() => {
    const interval = setInterval(async () => {
      const data = await api.getSystemStats();
      setStats(data);
    }, 2000);
    
    return () => clearInterval(interval);
  }, []);
  
  return stats;
};
```

#### useParallaxDepth

Hook for depth map generation and management.

```typescript
export const useParallaxDepth = (imageUrl: string) => {
  const [depthMap, setDepthMap] = useState<string | null>(null);
  const [isGenerating, setIsGenerating] = useState(false);
  const [progress, setProgress] = useState(0);
  const apiKey = localStorage.getItem('replicate_api_key') || '';
  
  const generateDepthMap = async () => {
    setIsGenerating(true);
    try {
      const jobId = await api.generateDepthMap(imageUrl, apiKey);
      
      const pollInterval = setInterval(async () => {
        const status = await api.checkDepthMapStatus(jobId);
        setProgress(status.progress);
        
        if (status.status === 'succeeded' && status.output) {
          setDepthMap(status.output);
          setIsGenerating(false);
          clearInterval(pollInterval);
        } else if (status.status === 'failed') {
          setIsGenerating(false);
          clearInterval(pollInterval);
        }
      }, 1000);
    } catch (error) {
      setIsGenerating(false);
    }
  };
  
  const checkCache = async () => {
    const cached = await api.getCachedDepthMap(imageUrl);
    if (cached) {
      setDepthMap(cached);
    }
  };
  
  useEffect(() => {
    checkCache();
  }, [imageUrl]);
  
  return { depthMap, isGenerating, progress, generateDepthMap };
};
```

### TypeScript Types

**Location:** `src/lib/types.ts`

```typescript
// Wallpaper Types
export interface WallpaperItem {
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

// System Types
export interface SystemStats {
  cpu: number;
  ram: number;
  cpuHistory: number[];
  ramHistory: number[];
}

// Smart Context Types
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

// Dream Types
export interface DreamMetadata {
  id: string;
  prompt: string;
  style: string;
  status: 'pending' | 'processing' | 'succeeded' | 'failed';
  createdAt: string;
  videoUrl?: string;
  error?: string;
}

// Video Info Types
export interface VideoInfo {
  width: number;
  height: number;
  duration: number;
  codec: string;
  bitrate: number;
}
```

## Error Handling

All commands return `Promise` and may reject with error messages.

### Example Error Handling

```typescript
try {
  const handle = await invoke('init_worker_w');
  console.log('Success:', handle);
} catch (error) {
  console.error('Error:', error);
  // Handle error (show user message, retry, etc.)
}
```

### Common Errors

- `WorkerW not found` - WorkerW window could not be found
- `Invalid API key` - Replicate API key is invalid or missing
- `Network error` - Failed to connect to API
- `File not found` - Specified file does not exist
- `Insufficient permissions` - Missing file system or registry permissions

## Event System

### Available Events

- `playback-control` - Emitted when process monitor detects fullscreen app

### Event Payload

```typescript
interface PlaybackControlEvent {
  shouldPause: boolean;
  reason: string;
}
```

### Listening to Events

```typescript
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen('playback-control', (event) => {
  console.log('Received event:', event.payload);
});

// Clean up
unlisten();
```

## Rate Limiting

Some external APIs (Replicate) have rate limits:

- Replicate: ~120 requests per minute for free tier
- OpenMeteo: No rate limit
- ip-api.com: 45 requests per minute

Implement appropriate backoff and retry logic for production use.

## Best Practices

1. **Error Handling:** Always wrap commands in try-catch
2. **Loading States:** Show loading indicators during async operations
3. **Polling:** Use appropriate intervals for status checks (1-2 seconds)
4. **Caching:** Cache results to reduce API calls
5. **Cleanup:** Clean up event listeners and intervals
6. **Validation:** Validate inputs before sending to backend
7. **Security:** Never expose API keys in client-side code

## Support

For issues or questions:
- Check the [Architecture Documentation](ARCHITECTURE.md)
- Review [Build Guide](BUILD_GUIDE.md)
- Open an issue on GitHub

---

**Last Updated:** 2024
**Version:** 1.0.0