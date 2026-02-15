# Arise Desktop - Testing Guide

Complete testing guide for verifying all dependencies work correctly after updates.

## 🔍 Pre-Installation Checks

### 1. Verify Node.js Version
```bash
node --version
# Should be >= 18.0.0
```

### 2. Verify npm Version
```bash
npm --version
# Should be >= 9.0.0
```

### 3. Clean Environment
```bash
# Remove old node_modules if exists
rm -rf node_modules package-lock.json

# Clean Rust cache if exists
cd src-tauri
cargo clean
cd ..
```

## 📦 Installation Testing

### Step 1: Install Dependencies
```bash
npm install
```

**Expected Result:**
- ✅ No deprecation warnings
- ✅ No peer dependency conflicts
- ✅ All packages install successfully
- ✅ Installation completes without errors

**Common Issues & Solutions:**

**Issue:** `ERESOLVE unable to resolve dependency tree`
- **Cause:** Dependency conflicts
- **Solution:** We've resolved this by removing `eslint-config-react-app` and using native ESLint 9 configuration

**Issue:** `npm warn deprecated`
- **Cause:** Using old packages
- **Solution:** All packages are now latest versions

### Step 2: Verify Installation
```bash
# Check installed packages
npm list --depth=0

# Check for outdated packages
npm outdated
# Expected: No outdated packages
```

### Step 3: Verify TypeScript
```bash
npm run type-check
```

**Expected Result:**
- ✅ No TypeScript errors
- ✅ All types resolve correctly
- ✅ Configuration is valid

### Step 4: Verify ESLint
```bash
npm run lint
```

**Expected Result:**
- ✅ ESLint runs successfully
- ✅ No configuration errors
- ✅ Flat config format is valid
- ✅ All plugins load correctly

**ESLint 9 Verification:**
```bash
# Check ESLint version
npx eslint --version
# Expected: v9.14.0

# Verify flat config
npx eslint --print-config src/main.tsx
```

### Step 5: Verify Prettier
```bash
npm run format:check
```

**Expected Result:**
- ✅ Prettier runs successfully
- ✅ All files are formatted correctly
- ✅ No formatting errors

### Step 6: Verify Vite
```bash
npm run build
```

**Expected Result:**
- ✅ Build completes successfully
- ✅ No build errors
- ✅ dist/ folder is created
- ✅ Assets are generated

## 🧪 Integration Testing

### Test 1: Development Server
```bash
npm run dev
```

**Expected Result:**
- ✅ Server starts on http://localhost:1420
- ✅ No compilation errors
- ✅ HMR (Hot Module Replacement) works
- ✅ Console shows no errors

**Verification:**
1. Open browser to http://localhost:1420
2. Check browser console for errors
3. Make a small change to a file
4. Verify HMR updates the page

### Test 2: Production Build
```bash
npm run build
npm run preview
```

**Expected Result:**
- ✅ Build completes
- ✅ Preview server starts
- ✅ Application works in production mode
- ✅ No console errors

### Test 3: Tauri Development
```bash
npm run tauri dev
```

**Expected Result:**
- ✅ Tauri window opens
- ✅ Frontend loads correctly
- ✅ Rust backend compiles
- ✅ No errors in terminal

**Note:** This test requires Windows and proper Tauri setup.

### Test 4: Tauri Production Build
```bash
npm run tauri build
```

**Expected Result:**
- ✅ Frontend builds
- ✅ Rust backend compiles
- ✅ Executable is created
- ✅ Installer is generated

**Verification:**
```bash
# Check build artifacts
ls -la src-tauri/target/release/
ls -la src-tauri/target/release/bundle/
```

## 🔬 Dependency Compatibility Tests

### ESLint 9 Compatibility

**Test Configuration:**
```bash
# Verify ESLint 9 configuration
cat eslint.config.js

# Verify plugins load
npx eslint --debug src/App.tsx 2>&1 | grep -i plugin
```

**Expected Plugins:**
- ✅ @eslint/js
- ✅ typescript-eslint
- ✅ eslint-plugin-react-hooks
- ✅ eslint-plugin-react-refresh

### TypeScript ESLint v8 Compatibility

**Test:**
```bash
# Verify TypeScript ESLint works
npm run type-check
npm run lint
```

**Expected:**
- ✅ No parser errors
- ✅ No type checking errors
- ✅ All rules apply correctly

### React Integration Tests

**Test 1: React 18.3.1**
```bash
# Check React version
npm list react react-dom

# Create test file
cat > test-react.tsx << 'EOF'
import React from 'react';
import { render } from '@testing-library/react';

const TestComponent: React.FC = () => <div>Test</div>;

test('React works', () => {
  const { getByText } = render(<TestComponent />);
  expect(getByText('Test')).toBeInTheDocument();
});
EOF
```

**Test 2: Framer Motion**
```bash
# Verify Framer Motion loads
npm list framer-motion

# Test in browser console:
import { motion } from 'framer-motion';
console.log('Framer Motion loaded:', typeof motion.div);
```

**Test 3: Three.js & React Three Fiber**
```bash
# Verify Three.js versions
npm list three @react-three/fiber @react-three/drei

# Test in browser console:
import * as THREE from 'three';
console.log('Three.js version:', THREE.REVISION);
```

### Vite 6 Compatibility

**Test:**
```bash
# Check Vite version
npx vite --version

# Verify Vite 6 features
npm run dev -- --debug
```

**Expected:**
- ✅ Fast HMR
- ✅ Optimized builds
- ✅ Modern JavaScript features

## 📊 Performance Tests

### Build Performance
```bash
# Time the build
time npm run build

# Expected: < 30 seconds for clean build
```

### Bundle Size Analysis
```bash
# Analyze bundle size
npm run build
du -sh dist/

# Expected: < 2 MB for typical bundle
```

### Runtime Performance
```bash
# Test in development mode
npm run dev

# Check:
# - Fast page load (< 2 seconds)
# - Smooth animations (60 FPS)
# - Low memory usage
```

## 🐛 Troubleshooting

### Issue: ESLint Configuration Errors

**Symptoms:**
```
Error: Failed to load plugin 'react-hooks' declared in 'eslint.config.js'
```

**Solution:**
```bash
# Reinstall ESLint plugins
npm install --save-dev eslint-plugin-react-hooks eslint-plugin-react-refresh

# Verify installation
npm list eslint-plugin-react-hooks
```

### Issue: TypeScript Errors

**Symptoms:**
```
error TS2307: Cannot find module '@tauri-apps/api'
```

**Solution:**
```bash
# Reinstall Tauri API
npm install @tauri-apps/api@latest

# Verify
npm list @tauri-apps/api
```

### Issue: Vite Build Errors

**Symptoms:**
```
Error: Failed to resolve import "@tauri-apps/api"
```

**Solution:**
```bash
# Check Vite configuration
cat vite.config.ts

# Verify paths are correct
# Reinstall dependencies
rm -rf node_modules package-lock.json
npm install
```

### Issue: Three.js Version Conflicts

**Symptoms:**
```
Error: Multiple versions of Three.js detected
```

**Solution:**
```bash
# Check for duplicate Three.js
npm ls three

# Deduplicate
npm dedupe

# If issues persist, clear cache
rm -rf node_modules/.vite
```

## ✅ Verification Checklist

After completing all tests, verify:

### Installation
- [ ] `npm install` completes without errors
- [ ] No deprecation warnings
- [ ] No peer dependency conflicts
- [ ] All packages are latest versions

### Linting
- [ ] `npm run lint` runs successfully
- [ ] ESLint 9 flat config works
- [ ] All plugins load correctly
- [ ] No linting errors in source files

### Type Checking
- [ ] `npm run type-check` passes
- [ ] No TypeScript errors
- [ ] All types resolve correctly
- [ ] TypeScript 5.6 features work

### Formatting
- [ ] `npm run format:check` passes
- [ ] Prettier formats correctly
- [ ] All files are consistent
- [ ] No formatting errors

### Building
- [ ] `npm run build` completes
- [ ] No build errors
- [ ] dist/ folder created
- [ ] Assets are generated

### Development
- [ ] `npm run dev` starts successfully
- [ ] HMR works correctly
- [ ] No console errors
- [ ] Hot reload works

### Production
- [ ] `npm run preview` works
- [ ] Production build runs
- [ ] No runtime errors
- [ ] Application performs well

## 🚀 Performance Benchmarks

Expected performance metrics:

| Metric | Target | Acceptable |
|--------|--------|------------|
| Install Time | < 60s | < 120s |
| Build Time | < 30s | < 60s |
| Dev Server Start | < 3s | < 5s |
| HMR Update | < 500ms | < 1s |
| Bundle Size | < 2 MB | < 3 MB |
| Page Load | < 2s | < 3s |

## 📝 Test Results Template

Use this template to document your test results:

```
## Test Results

Date: [Date]
Tester: [Name]
Node Version: [Version]
npm Version: [Version]

### Installation
- npm install: ✅ / ❌
- Warnings: [None / List warnings]

### Linting
- npm run lint: ✅ / ❌
- Errors: [None / List errors]

### Type Checking
- npm run type-check: ✅ / ❌
- Errors: [None / List errors]

### Building
- npm run build: ✅ / ❌
- Build Time: [Time]
- Bundle Size: [Size]

### Development
- npm run dev: ✅ / ❌
- HMR: ✅ / ❌

### Production
- npm run preview: ✅ / ❌

### Issues Found:
- [List any issues]

### Notes:
- [Additional notes]
```

## 🔄 Continuous Testing

For ongoing development, create a test script:

```bash
#!/bin/bash
# test-all.sh - Run all tests

echo "🧪 Running all tests..."

echo "📦 Installing dependencies..."
npm install || exit 1

echo "🔍 Type checking..."
npm run type-check || exit 1

echo "📏 Linting..."
npm run lint || exit 1

echo "🎨 Formatting check..."
npm run format:check || exit 1

echo "🏗️  Building..."
npm run build || exit 1

echo "✅ All tests passed!"
```

Make it executable:
```bash
chmod +x test-all.sh
./test-all.sh
```

## 📚 Resources

- [ESLint 9 Documentation](https://eslint.org/docs/latest/)
- [TypeScript ESLint v8](https://typescript-eslint.io/)
- [Vite 6 Guide](https://vitejs.dev/guide/)
- [React 18 Documentation](https://react.dev/)
- [Three.js Documentation](https://threejs.org/docs/)

## 💡 Best Practices

1. **Always run tests before committing**
2. **Keep dependencies updated**
3. **Use `npm ci` for CI/CD**
4. **Lock dependency versions for production**
5. **Monitor bundle size**
6. **Test on multiple Node.js versions**
7. **Use TypeScript strict mode**
8. **Enable ESLint auto-fix**
9. **Format code before committing**
10. **Document any custom configurations**

---

**Last Updated:** February 15, 2025
**Version:** 1.0.0
**Status:** All dependencies updated and verified