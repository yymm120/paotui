# Android Build Issue in WSL Environment

## Problem
当前WSL环境中的NDK工具链架构不匹配：
- 已安装: `windows-x86_64` (Windows版本)
- 需要: `linux-x86_64` (Linux版本)

## Error Details
```
Missing tool `prebuilt toolchain`; tried at 
"/mnt/c/Users/12084/AppData/Local/Android/Sdk/ndk/29.0.13599879/toolchains/llvm/prebuilt/linux-x86_64"
```

## Solutions

### Option 1: Install Linux NDK in WSL (Recommended)
```bash
# 1. Download Linux NDK
cd ~/Downloads
wget https://dl.google.com/android/repository/android-ndk-r25c-linux.zip

# 2. Extract and setup
unzip android-ndk-r25c-linux.zip
sudo mv android-ndk-r25c /opt/android-ndk

# 3. Update environment variables
echo 'export NDK_HOME="/opt/android-ndk"' >> ~/.bashrc
echo 'export ANDROID_NDK_ROOT="/opt/android-ndk"' >> ~/.bashrc
source ~/.bashrc

# 4. Rebuild
pnpm tauri android build --debug
```

### Option 2: Build in Windows Environment
```cmd
# 在Windows环境中构建 (C:\mydata\code\paotui)
cd C:\mydata\code\paotui
pnpm tauri android build --debug
```

### Option 3: Use Docker (Alternative)
```bash
# 使用Docker容器构建Android应用
docker run --rm -v $(pwd):/workspace -w /workspace \
  ghcr.io/tauri-apps/tauri-action:latest \
  tauri android build --debug
```

## Current Status
- ✅ Android项目初始化完成
- ✅ Tauri v2配置正确
- ✅ 所有依赖已安装
- ❌ NDK工具链架构不匹配 (Windows版本在WSL中使用)

## Recommendation
建议使用Option 1安装Linux版本的NDK，或者切换到Windows环境进行构建。