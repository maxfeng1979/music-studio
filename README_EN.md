# Music Studio

**中文** | [English](./README_EN.md)

🎵 AI-powered music generation desktop application

[![Windows Download](https://img.shields.io/github/v/release/maxfeng1979/music-studio?label=Download%20Installer)](https://github.com/maxfeng1979/music-studio/releases/latest/download/MusicStudio_0.1.0_x64_en-US.msi)

---

### About

Music Studio is an AI-powered desktop music generation tool. Using the MiniMax API, you can describe the music you want in natural language, have AI write lyrics for you, and automatically generate album covers.

### System Requirements

- Windows 10 or Windows 11
- WebView2 Runtime (built into Windows 11; Windows 10 users [download here](https://developer.microsoft.com/en-us/microsoft-edge/webview2/))

### Installation

1. Click the button above to download the latest `.exe` installer
2. Double-click to run the installer
3. Follow the prompts to complete installation
4. Launch Music Studio

### Getting Started

1. The onboarding wizard will appear on first launch
2. Click "How to get API Key?" to visit the MiniMax platform and register
3. Copy your API Key and paste it into the input
4. Click "Test Connection", then "Next" on success
5. Start creating your first music!

### How to Get API Key

1. Visit MiniMax Open Platform: https://platform.minimaxi.com/
2. Register and log in
3. Go to "Developer Center" → "API Key" and create a new key
4. Copy and use it in the app

> Note: You are responsible for your API Key and any usage costs.

### Features

- **Music Generation**: Enter style description and lyrics, AI generates the complete track
- **AI Assistant**: Describe the music you want in natural language, AI generates title, style, and structured lyrics
- **Cover Generation**: AI automatically generates album covers based on song content
- **Music Library**: Manage all your generated music, edit titles, tags, and notes

### FAQ

**What if connection fails?**
- Verify the API Key is entered correctly with no extra spaces
- Check your network can reach api.minimaxi.com
- Check your MiniMax account has sufficient API quota

**How to change API Key?**
- Open "Settings" → "API Configuration", enter the new key and click "Save"

**Where are music files stored?**
- Default: `%APPDATA%/com.minimax.music-studio/`
- Change anytime in "Settings" → "Data Storage"
