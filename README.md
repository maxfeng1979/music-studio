# Music Studio

**中文** | [English](./README_EN.md)

🎵 AI 驱动的桌面音乐创作工具

[![Windows最新下载](https://img.shields.io/github/v/release/maxfeng1979/music-studio?label=%E4%B8%8B%E8%BD%BD%E5%AE%89%E8%A3%85%E5%8C%85)](https://github.com/maxfeng1979/music-studio/releases/latest/download/MusicStudio_0.1.0_x64-setup.exe)

---

### 应用简介

Music Studio 是一款由 AI 驱动的桌面音乐创作工具。通过 MiniMax API，你可以使用自然语言描述来生成音乐、AI 帮你写歌词、并自动生成专辑封面。

### 系统要求

- Windows 10 或 Windows 11
- WebView2 Runtime（Windows 11 自带，Windows 10 用户[点击这里下载](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)）

### 安装步骤

1. 点击上方按钮下载最新版本的 `.exe` 安装包
2. 双击运行安装包
3. 按照提示完成安装
4. 启动 Music Studio

### 首次使用

1. 首次启动会显示引导向导
2. 点击「如何获取 API Key？」访问 MiniMax 平台注册
3. 复制你的 API Key，粘贴到输入框
4. 点击「测试连接」，确认连接成功后点击「下一步」
5. 开始创作你的第一首音乐！

### 如何获取 API Key

1. 访问 MiniMax 开放平台：https://platform.minimaxi.com/
2. 注册账号并登录
3. 进入「开发者中心」→「API Key」，创建新 Key
4. 复制 Key 并在应用中使用

> 注意：API Key 由你自行保管，使用服务产生的费用由你自行承担。

### 功能说明

- **音乐生成**：输入风格描述和歌词，AI 生成完整音乐
- **AI 助手**：用自然语言描述你想要的音乐，AI 自动帮你生成标题、风格描述和结构化歌词
- **封面生成**：AI 根据歌曲内容自动生成专辑封面
- **音乐库**：管理你生成的所有音乐，编辑标题、标签、备注

### 常见问题

**连接失败怎么办？**
- 确认 API Key 填写正确，没有多余空格
- 确认你的网络可以访问 api.minimaxi.com
- 确认 MiniMax 账号有足够的 API 调用额度

**如何更换 API Key？**
- 打开「设置」→「API 配置」，输入新的 Key 并点击「保存」

**音乐文件存在哪里？**
- 默认存储在 `%APPDATA%/com.minimax.music-studio/`
- 你可以在「设置」→「数据存储」中更改存储位置
