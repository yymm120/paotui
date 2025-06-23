# 🚚 Paotui Delivery Mobile App

A professional delivery service mobile application built with Tauri, React, TypeScript, and Tailwind CSS for Android platforms.

## 📱 Features

### Core Delivery Features

- **Real-time Order Management** - Accept, track, and manage delivery orders
- **Multi-tab Interface** - New tasks, pickup, and delivery statuses
- **Smart Filtering** - Sort by distance, earnings, priority, or time
- **Work Status Toggle** - Start/stop receiving orders

### Mobile-Specific Features

- **📍 GPS Location Tracking** - Real-time location with high accuracy
- **📷 Camera Integration** - Take delivery proof photos
- **🔔 Push Notifications** - Order alerts and status updates
- **💾 Offline Storage** - Local data persistence
- **⚙️ Settings Management** - Configurable delivery preferences
- **📊 Data Export** - Backup and export functionality

## 🛠 Technology Stack

- **Frontend**: React 19 + TypeScript + Tailwind CSS
- **Mobile Framework**: Tauri 2.0 (Rust backend)
- **UI Components**: Radix UI + Custom components
- **State Management**: React Hooks + Tauri APIs
- **Build Tool**: Vite
- **Target Platform**: Android (iOS support planned)

## 📋 Prerequisites

Before setting up the project, ensure you have:

### Required Software

- **Node.js** (v18 or higher)
- **Rust** (latest stable version)
- **Android Studio** with:
  - Android SDK (API level 24+)
  - Android NDK
  - Build Tools
- **Java Development Kit (JDK)** 11 or higher

### Environment Setup

```bash
# Android environment variables (add to ~/.bashrc or ~/.zshrc)
export ANDROID_HOME=$HOME/Android/Sdk
export NDK_HOME=$ANDROID_HOME/ndk/25.1.8937393
export PATH=$PATH:$ANDROID_HOME/tools:$ANDROID_HOME/platform-tools
```

## 🚀 Quick Start

### 1. Clone and Setup

```bash
git clone <repository-url>
cd paotui-mobile
chmod +x setup-mobile.sh
./setup-mobile.sh
```

### 2. Development Commands

```bash
# Web development (testing UI)
npm run dev

# Android development (live reload on device/emulator)
npm run android

# Build Android APK
npm run android:build

# Build web version
npm run build
```

### 3. First Run

1. Connect Android device (enable USB debugging) or start emulator
2. Run `npm run android`
3. App will install and launch on device

## 📂 Project Structure

```
src/
├── components/
│   ├── delivery/           # Main delivery UI components
│   ├── mobile/            # Mobile-specific components
│   │   ├── CameraComponent.tsx
│   │   ├── LocationComponent.tsx
│   │   ├── NotificationComponent.tsx
│   │   └── SettingsComponent.tsx
│   └── ui/                # Reusable UI components
├── hooks/
│   ├── useTauriMobile.ts  # Mobile API hooks
│   └── useDeliveryApp.ts  # Web fallback hooks
├── utils/
│   └── mobile.ts          # Mobile utilities
├── types/
│   └── delivery.ts        # TypeScript interfaces
└── data/
    └── mockOrders.ts      # Sample data

src-tauri/
├── src/
│   └── main.rs           # Rust backend
├── capabilities/
│   └── mobile.json       # Security permissions
├── Cargo.toml           # Rust dependencies
└── tauri.conf.json      # Tauri configuration
```

## 🔧 Configuration

### Tauri Configuration (`src-tauri/tauri.conf.json`)

- App metadata and identifiers
- Build configuration
- Mobile permissions
- Security settings

### Android Permissions

The app requests these permissions:

- **Location**: GPS tracking for delivery routes
- **Camera**: Taking delivery proof photos
- **Notifications**: Order alerts
- **Storage**: Saving photos and data
- **Network**: API communication

### Development Environment

Create `.env.development`:

```env
TAURI_DEV_HOST=0.0.0.0
VITE_TAURI_MOBILE=true
ANDROID_PLATFORM_TOOLS=$ANDROID_HOME/platform-tools
```

## 📱 Mobile Features Usage

### Location Tracking

```typescript
import { useTauriMobile } from "./hooks/useTauriMobile";

const { getCurrentLocationData, startLocationTracking } = useTauriMobile();

// Get current location
const location = await getCurrentLocationData();

// Start continuous tracking
await startLocationTracking();
```

### Camera Integration

```typescript
const { takeDeliveryPhoto } = useTauriMobile();

// Take delivery proof photo
const photoPath = await takeDeliveryPhoto(orderId);
```

### Push Notifications

```typescript
const { sendMobileNotification } = useTauriMobile();

// Send notification
await sendMobileNotification("Title", "Message body");
```

## 🔨 Build and Distribution

### Debug Build

```bash
npm run android:build
```

Output: `src-tauri/gen/android/app/build/outputs/apk/debug/`

### Release Build

```bash
npm run android:build -- --release
```

Output: `src-tauri/gen/android/app/build/outputs/apk/release/`

### Signing APK (for production)

1. Generate keystore:

```bash
keytool -genkey -v -keystore paotui-release-key.keystore -alias paotui -keyalg RSA -keysize 2048 -validity 10000
```

2. Configure in `src-tauri/gen/android/app/build.gradle`

## 🐛 Troubleshooting

### Common Issues

#### Android Build Fails

- Check Android SDK/NDK paths
- Verify ANDROID_HOME environment variable
- Ensure Java 11+ is installed

#### Device Connection Issues

- Enable USB debugging on device
- Check `adb devices` shows your device
- Try different USB cable/port

#### App Crashes on Start

- Check device logs: `adb logcat`
- Verify all permissions are granted
- Clear app data and reinstall

#### Location Not Working

- Grant location permissions in app settings
- Enable GPS on device
- Test with location mock apps

### Debug Commands

```bash
# Check connected devices
adb devices

# View app logs
adb logcat | grep "paotui"

# Clear app data
adb shell pm clear com.paotui.delivery

# Install APK manually
adb install path/to/app.apk
```

## 🎯 Development Tips

### Testing on Device

1. Always test on real device for location/camera features
2. Use Android Studio's device manager for emulation
3. Monitor performance with Android profiler

### Code Organization

- Keep mobile-specific code in `components/mobile/`
- Use `useTauriMobile` hook for Tauri APIs
- Implement web fallbacks for testing

### Performance

- Optimize images and assets for mobile
- Use React.memo for expensive components
- Implement proper loading states

## 📄 License

MIT License - see LICENSE file for details

## 🤝 Contributing

1. Fork the repository
2. Create feature branch
3. Test on Android device
4. Submit pull request

## 🆘 Support

For issues and questions:

- Check troubleshooting section
- Review Tauri mobile docs
- Create GitHub issue with device info and logs

---

**Happy Delivery! 🚚📱**
