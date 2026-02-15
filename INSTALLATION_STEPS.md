# Quick Installation Steps

## Prerequisites
- Node.js >= 18.0.0
- npm >= 9.0.0
- Windows 10/11 (for Tauri features)

## Installation

```bash
# 1. Extract the zip file
unzip arise-desktop-complete.zip
cd arise-desktop-complete

# 2. Install dependencies
npm install
# Note: You may see some peer dependency warnings, but these are safe to ignore
# See package-lock-resolution.md for details

# 3. Verify installation
npm run type-check
npm run lint

# 4. Start development
npm run dev
```

## Build for Production

```bash
npm run build
npm run preview
```

## About the Warnings

You may see some warnings about `@typescript-eslint` packages during installation. These are **safe to ignore** because:

- We use npm overrides to ensure ESLint 9 compatibility
- All functionality works correctly
- See [package-lock-resolution.md](package-lock-resolution.md) for details

## Troubleshooting

If you encounter any issues, see:
- [package-lock-resolution.md](package-lock-resolution.md) - For peer dependency warnings
- [TESTING_GUIDE.md](TESTING_GUIDE.md) - For detailed troubleshooting steps

## All Dependencies Are Latest

- ✅ All packages on latest stable versions
- ✅ ESLint 9 with flat config
- ✅ TypeScript 5.6
- ✅ Vite 6
- ✅ React 18.3
- ✅ No deprecation warnings for core packages

Everything is ready to go! 🚀