#!/bin/bash

# Paotui Delivery Mobile App Setup Script
# This script sets up the Tauri mobile development environment

echo "🚚 Setting up Paotui Delivery Mobile App..."

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js first."
    exit 1
fi

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi

# Install Tauri CLI
echo "📦 Installing Tauri CLI..."
cargo install tauri-cli --version "^2.0"

# Install Tauri Android dependencies
echo "📱 Setting up Android development..."
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

# Install Node.js dependencies
echo "📦 Installing Node.js dependencies..."
npm install

# Initialize Tauri project
echo "🔧 Initializing Tauri mobile project..."
npm run tauri android init

# Set up Android development environment
echo "📱 Android Setup Instructions:"
echo "1. Install Android Studio from https://developer.android.com/studio"
echo "2. Install Android SDK and NDK through Android Studio"
echo "3. Set up environment variables:"
echo "   export ANDROID_HOME=\$HOME/Android/Sdk"
echo "   export NDK_HOME=\$ANDROID_HOME/ndk/<version>"
echo "   export PATH=\$PATH:\$ANDROID_HOME/tools:\$ANDROID_HOME/platform-tools"

# Create development environment file
cat > .env.development << EOF
# Tauri Mobile Development Environment
TAURI_DEV_HOST=0.0.0.0
VITE_TAURI_MOBILE=true
ANDROID_PLATFORM_TOOLS=\$ANDROID_HOME/platform-tools
EOF

echo "✅ Setup complete!"
echo ""
echo "🚀 Next steps:"
echo "1. Ensure Android Studio is installed and configured"
echo "2. Connect an Android device or start an emulator"
echo "3. Run 'npm run android' to start development"
echo "4. Run 'npm run android:build' to build APK"
echo ""
echo "📱 Available commands:"
echo "  npm run dev          - Web development"
echo "  npm run android      - Android development"
echo "  npm run android:build - Build Android APK"
echo "  npm run build        - Build web version"
echo ""
echo "🔧 Troubleshooting:"
echo "  - Make sure Android SDK and NDK are installed"
echo "  - Check that ANDROID_HOME is set correctly"
echo "  - Ensure USB debugging is enabled on your device"
