# Music Studio: API Key 管理 + 中文化 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 Music Studio 从环境变量读取 API Key 改为 App 内置管理 + 首次引导向导，同时完成全站中英双语支持。

**Architecture:**
- Rust 端：在 `config.rs` 扩展 API Key 读写（config.json 优先，回退 env），API Client 改为接收 Key 参数而非自读
- Svelte 前端：新增 onboarding 向导页，Settings 新增 Key 输入/测试/保存，新增 i18n locale store，中文化所有 UI
- README 中英双语

**Tech Stack:** Tauri v2 + Svelte 5 + SvelteKit + Rust + SQLite

---

## 文件结构

```
改动概述：
- src-tauri/src/config.rs         — 扩展：API Key 读写
- src-tauri/src/api/minimax.rs    — new(api_key) 不再读 env
- src-tauri/src/api/anthropic.rs  — new(api_key) 不再读 env
- src-tauri/src/commands/music.rs — get_key() 后传入 client
- src-tauri/src/commands/image.rs — get_key() 后传入 client
- src-tauri/src/commands/ai_assist.rs — get_key() 后传入 client
- src-tauri/src/commands/settings.rs — 新增 save_api_key / get_api_key_status / test_api_connection
- src-tauri/src/lib.rs — 注册新 commands
- src/routes/+layout.svelte — 检测 Key 状态，未配置重定向 /onboarding
- src/routes/onboarding/+page.svelte — 新增：首次引导向导（3步）
- src/routes/generator/+page.svelte — 中文化
- src/routes/library/+page.svelte — 中文化
- src/routes/settings/+page.svelte — 改造：API 配置区 + 语言切换 + 中文化
- src/lib/components/MusicForm.svelte — 中文化
- src/lib/components/AudioPlayer.svelte — 中文化
- src/lib/components/MusicCard.svelte — 中文化
- src/lib/components/CoverModal.svelte — 中文化
- src/lib/components/MetadataModal.svelte — 中文化
- src/lib/i18n/zh.ts — 新增：中文文案
- src/lib/i18n/en.ts — 新增：英文文案
- src/lib/i18n/index.ts — 新增：locale store + 切换逻辑
- README.md — 新增：中英双语用户文档
```

---

## Task 1: Rust — 扩展 config.rs，添加 API Key 管理

**Files:**
- Modify: `src-tauri/src/config.rs:1-149`

**Steps:**

- [ ] **Step 1: 修改 AppConfig 结构体，添加 api_key 字段**

在 `src-tauri/src/config.rs` 的 `AppConfig` 结构体中添加 `minimax_api_key: Option<String>` 字段（放在 `data_path` 旁边）：

```rust
#[derive(Debug, Serialize, Deserialize, Default)]
struct AppConfig {
    #[serde(rename = "data_path")]
    data_path: Option<String>,
    #[serde(rename = "minimax_api_key", skip_serializing_if = "Option::is_none")]
    minimax_api_key: Option<String>,
}
```

- [ ] **Step 2: 添加 get_api_key() 函数（优先级：config.json → env → error）**

在 `config.rs` 末尾添加：

```rust
/// Returns the API key by priority: config.json > MINIMAX_API_KEY env var
pub fn get_api_key() -> Result<String, String> {
    if let Ok(config) = read_config() {
        if let Some(ref key) = config.minimax_api_key {
            if !key.is_empty() {
                return Ok(key.clone());
            }
        }
    }
    std::env::var("MINIMAX_API_KEY")
        .map_err(|_| "MINIMAX_API_KEY not found. Please set it in Settings.".into())
}
```

- [ ] **Step 3: 添加 save_api_key() 函数**

```rust
/// Saves the API key to config.json
pub fn save_api_key(key: &str) -> Result<(), String> {
    let config_path = get_config_path()?;
    let mut config = read_config().unwrap_or_default();
    config.minimax_api_key = Some(key.to_string());
    let content = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    std::fs::write(&config_path, content).map_err(|e| e.to_string())
}
```

- [ ] **Step 4: 添加 get_api_key_status() 函数**

```rust
/// Returns whether an API key is configured (either in config.json or env)
pub fn get_api_key_configured() -> bool {
    get_api_key().is_ok()
}
```

- [ ] **Step 5: 修改 MinimaxClient::new()，改为接受 api_key 参数**

修改 `src-tauri/src/api/minimax.rs` 第 126 行：

```rust
// 之前：env::var("MINIMAX_API_KEY").map_err(...)
// 改为：
pub fn new(api_key: &str) -> Result<Self, String> {
    Ok(Self {
        api_key: api_key.to_string(),
        client: reqwest::Client::new(),
    })
}
```

- [ ] **Step 6: 修改 AnthropicClient::new()，改为接受 api_key 参数**

修改 `src-tauri/src/api/anthropic.rs` 第 46-53 行：

```rust
// 之前：env::var("MINIMAX_API_KEY").map_err(...)
// 改为：
pub fn new(api_key: &str) -> Result<Self, String> {
    Ok(Self {
        api_key: api_key.to_string(),
        client: reqwest::Client::new(),
    })
}
```

- [ ] **Step 7: 修改 music.rs 中的 client 创建，传入 key**

修改 `src-tauri/src/commands/music.rs` 第 28 行和第 88 行：

```rust
// 之前：let client = MinimaxClient::new()?;
// 改为：
let api_key = crate::config::get_api_key()?;
let client = MinimaxClient::new(&api_key)?;
```

- [ ] **Step 8: 修改 image.rs 中的 client 创建，传入 key**

修改 `src-tauri/src/commands/image.rs` 第 28 行：

```rust
// 之前：let client = MinimaxClient::new()?;
// 改为：
let api_key = crate::config::get_api_key()?;
let client = MinimaxClient::new(&api_key)?;
```

- [ ] **Step 9: 修改 ai_assist.rs 中的 client 创建，传入 key**

修改 `src-tauri/src/commands/ai_assist.rs` 第 25 行和第 65 行：

```rust
// 之前：let client = AnthropicClient::new()?;
// 改为：
let api_key = crate::config::get_api_key()?;
let client = AnthropicClient::new(&api_key)?;
```

- [ ] **Step 10: 修改 settings.rs，添加 3 个新 command**

修改 `src-tauri/src/commands/settings.rs`，在现有 `get_data_path` 和 `set_data_path` 之后添加：

```rust
#[tauri::command]
pub async fn save_api_key(key: String) -> Result<(), String> {
    crate::config::save_api_key(&key)
}

#[tauri::command]
pub fn get_api_key_status() -> Result<bool, String> {
    Ok(crate::config::get_api_key_configured())
}

#[tauri::command]
pub async fn test_api_connection(key: String) -> Result<String, String> {
    let client = crate::api::anthropic::AnthropicClient::new(&key)?;
    // Send a minimal request to verify the key is valid
    let resp = client.chat(
        "You are a connection test assistant. Respond with only the word OK if you receive this.",
        "hi"
    ).await?;
    if resp.trim().to_uppercase().contains("OK") {
        Ok("连接成功！".to_string())
    } else {
        Ok("连接正常".to_string())
    }
}
```

- [ ] **Step 11: 注册新 commands 到 lib.rs**

修改 `src-tauri/src/lib.rs` 第 18-31 行的 `invoke_handler`：

```rust
settings::save_api_key,
settings::get_api_key_status,
settings::test_api_connection,
```

- [ ] **Step 12: 验证 Rust 编译**

Run: `cd music-studio/src-tauri && cargo build`
Expected: 编译成功，无错误（仅有之前已存在的 warning）

---

## Task 2: Svelte i18n 基础设施

**Files:**
- Create: `src/lib/i18n/zh.ts`
- Create: `src/lib/i18n/en.ts`
- Create: `src/lib/i18n/index.ts`

**Steps:**

- [ ] **Step 1: 创建中文文案文件 zh.ts**

```ts
// src/lib/i18n/zh.ts
export const zh = {
  nav: {
    generator: '生成器',
    library: '音乐库',
    settings: '设置',
  },
  generator: {
    title: '音乐生成器',
    autoCoverOn: '自动封面：开启',
    error: '错误',
    coverError: '封面错误',
    generatingMusic: '正在生成音乐...',
    emptyPrompt: '填写参数后点击「生成音乐」开始创作',
    loadingCover: '封面加载中...',
    duration: '时长',
    sampleRate: '采样率',
    prompt: '风格描述',
    lyrics: '歌词',
    generateCover: '生成封面',
    generatingCover: '正在生成封面...',
  },
  musicForm: {
    aiAssistant: 'AI 音乐助手',
    aiHint: '用自然语言描述你想要的音乐，AI 会帮你生成标题、风格和歌词',
    aiPlaceholder: '例如：一首关于雨夜独自漫步的忧郁独立民谣，怀念失去的爱情...',
    aiGenerate: 'AI 自动生成',
    aiGenerating: 'AI 生成中...',
    model: '模型',
    modelToken: 'music-2.6 (积分计划)',
    modelFree: 'music-2.6-free (免费额度)',
    title: '标题',
    titlePlaceholder: '歌曲标题（可由 AI 自动生成或手动输入）',
    musicPrompt: '音乐风格描述',
    promptPlaceholder: '例如：独立民谣,忧郁,内省,渴望,独自漫步,咖啡馆',
    lyrics: '歌词（可选）',
    lyricsPlaceholder: '[verse]\n街灯微亮晚风轻抚\n影子拉长独自漫步\n\n[chorus]\n推开木门香气弥漫',
    instrumental: '器乐（无人声）',
    autoLyrics: '自动优化歌词',
    audioSettings: '音频设置',
    sampleRate: '采样率',
    bitrate: '比特率',
    format: '格式',
    outputFormat: '输出格式',
    hex: 'Hex（默认）',
    url: 'URL',
    aigcWatermark: '添加 AIGC 水印',
    generateMusic: '生成音乐',
    generating: '生成中...',
  },
  library: {
    title: '音乐库',
    all: '全部',
    songs: '歌曲',
    instrumental: '器乐',
    sortByDate: '按日期排序',
    sortByTitle: '按标题排序',
    sortByDuration: '按时长排序',
    filterByTag: '按标签筛选...',
    refresh: '刷新',
    noMusic: '暂无音乐',
    goToGenerator: '去生成器创建你的第一首音乐吧！',
    instrumental: '器乐',
    edit: '编辑',
    delete: '删除',
  },
  settings: {
    title: '设置',
    apiConfig: 'API 配置',
    apiKey: 'API Key',
    apiKeyPlaceholder: '输入你的 MiniMax API Key',
    testConnection: '测试连接',
    save: '保存',
    connected: '当前已连接 MiniMax 服务',
    notConfigured: '未配置 API Key',
    musicGeneration: '音乐生成',
    autoCover: '自动生成封面',
    autoCoverHint: '音乐生成完成后自动生成封面图片',
    language: '语言 / Language',
    dataStorage: '数据存储',
    storagePath: '存储路径',
    currentLocation: '当前位置',
    selectNewFolder: '选择新文件夹...',
    apply: '应用',
    about: '关于',
    version: '版本',
    desktopMusicTool: '由 MiniMax API 驱动的桌面音乐生成工具',
  },
  onboarding: {
    welcomeTitle: '欢迎使用 Music Studio',
    welcomeSubtitle: 'AI 驱动的音乐创作工具',
    getStarted: '开始设置',
    connectTitle: '连接你的 AI 服务',
    connectDescription: 'Music Studio 使用 MiniMax 的 AI 服务来生成音乐。你需要一个 API Key 来开始。',
    howToGetKey: '如何获取 API Key？',
    enterApiKey: '输入你的 API Key',
    apiKeyPlaceholder: '粘贴你的 MiniMax API Key',
    testConnection: '测试连接',
    testing: '测试中...',
    connectionSuccess: '连接成功！',
    connectionFailed: '连接失败',
    next: '下一步',
    readyTitle: '一切就绪！',
    readySubtitle: '你已经完成设置，可以开始创作音乐了',
    readyHint: '你可以在设置中随时修改 API Key',
    startCreating: '开始创作',
  },
  metadataModal: {
    editMetadata: '编辑信息',
    title: '标题',
    titlePlaceholder: '音乐标题',
    tags: '标签',
    addTag: '添加',
    notes: '备注',
    notesPlaceholder: '你的备注...',
    cancel: '取消',
    save: '保存',
  },
  coverModal: {
    coverImage: '封面图片',
    generatingCover: '正在生成封面...',
    noCover: '暂无封面',
    useThisCover: '使用此封面',
    discard: '放弃',
    regenerateCover: '重新生成封面',
  },
  audioPlayer: {
    seek: '拖动调整播放进度',
  },
  errors: {
    apiKeyEmpty: '请输入 API Key',
    connectionFailed: '连接失败：',
    invalidKey: 'API Key 无效，请检查后重试',
    networkError: '网络连接失败，请检查网络设置',
  },
};
```

- [ ] **Step 2: 创建英文文案文件 en.ts**

```ts
// src/lib/i18n/en.ts
export const en = {
  nav: {
    generator: 'Generator',
    library: 'Library',
    settings: 'Settings',
  },
  generator: {
    title: 'Music Generator',
    autoCoverOn: 'Auto Cover: ON',
    error: 'Error',
    coverError: 'Cover Error',
    generatingMusic: 'Generating music...',
    emptyPrompt: 'Fill in the parameters and click Generate to create music',
    loadingCover: 'Loading cover...',
    duration: 'Duration',
    sampleRate: 'Sample Rate',
    prompt: 'Prompt',
    lyrics: 'Lyrics',
    generateCover: 'Generate Cover',
    generatingCover: 'Generating cover...',
  },
  musicForm: {
    aiAssistant: 'AI Music Assistant',
    aiHint: 'Describe the music you want, AI will generate title, style and lyrics for you.',
    aiPlaceholder: 'e.g. A melancholic indie folk song about walking alone on a rainy night in the city, feeling nostalgic about a lost love...',
    aiGenerate: 'AI Auto-Generate',
    aiGenerating: 'AI Generating...',
    model: 'Model',
    modelToken: 'music-2.6 (Token Plan)',
    modelFree: 'music-2.6-free (Free tier)',
    title: 'Title',
    titlePlaceholder: 'Song title (auto-filled by AI or enter manually)',
    musicPrompt: 'Music Prompt',
    promptPlaceholder: 'e.g. indie folk, melancholic, introspective, longing, walking alone, cafe',
    lyrics: 'Lyrics (Optional)',
    lyricsPlaceholder: '[verse]\nStreet lights glow\nWandering alone at night\n\n[chorus]\nPushing open the wooden door',
    instrumental: 'Instrumental (No vocals)',
    autoLyrics: 'Auto-generate lyrics',
    audioSettings: 'Audio Settings',
    sampleRate: 'Sample Rate',
    bitrate: 'Bitrate',
    format: 'Format',
    outputFormat: 'Output Format',
    hex: 'Hex (default)',
    url: 'URL',
    aigcWatermark: 'Add AIGC watermark to audio',
    generateMusic: 'Generate Music',
    generating: 'Generating...',
  },
  library: {
    title: 'Music Library',
    all: 'All',
    songs: 'Songs',
    instrumental: 'Instrumental',
    sortByDate: 'Sort by Date',
    sortByTitle: 'Sort by Title',
    sortByDuration: 'Sort by Duration',
    filterByTag: 'Filter by tag...',
    refresh: 'Refresh',
    noMusic: 'No music yet',
    goToGenerator: 'Go to Generator to create some!',
    edit: 'Edit',
    delete: 'Delete',
  },
  settings: {
    title: 'Settings',
    apiConfig: 'API Configuration',
    apiKey: 'API Key',
    apiKeyPlaceholder: 'Enter your MiniMax API Key',
    testConnection: 'Test Connection',
    save: 'Save',
    connected: 'Connected to MiniMax',
    notConfigured: 'API Key not configured',
    musicGeneration: 'Music Generation',
    autoCover: 'Auto-generate Cover',
    autoCoverHint: 'Automatically generate a cover image after music generation completes',
    language: 'Language / 语言',
    dataStorage: 'Data Storage',
    storagePath: 'Storage Path',
    currentLocation: 'Current location',
    selectNewFolder: 'Select a new folder...',
    apply: 'Apply',
    about: 'About',
    version: 'Version',
    desktopMusicTool: 'Desktop music generation tool powered by MiniMax API',
  },
  onboarding: {
    welcomeTitle: 'Welcome to Music Studio',
    welcomeSubtitle: 'AI-powered music creation tool',
    getStarted: 'Get Started',
    connectTitle: 'Connect Your AI Service',
    connectDescription: 'Music Studio uses MiniMax AI service to generate music. You need an API Key to get started.',
    howToGetKey: 'How to get API Key?',
    enterApiKey: 'Enter your API Key',
    apiKeyPlaceholder: 'Paste your MiniMax API Key',
    testConnection: 'Test Connection',
    testing: 'Testing...',
    connectionSuccess: 'Connection successful!',
    connectionFailed: 'Connection failed',
    next: 'Next',
    readyTitle: 'All Set!',
    readySubtitle: 'You are ready to start creating music',
    readyHint: 'You can modify your API Key anytime in Settings',
    startCreating: 'Start Creating',
  },
  metadataModal: {
    editMetadata: 'Edit Metadata',
    title: 'Title',
    titlePlaceholder: 'Music title',
    tags: 'Tags',
    addTag: 'Add',
    notes: 'Notes',
    notesPlaceholder: 'Your notes...',
    cancel: 'Cancel',
    save: 'Save',
  },
  coverModal: {
    coverImage: 'Cover Image',
    generatingCover: 'Generating cover...',
    noCover: 'No cover yet',
    useThisCover: 'Use This Cover',
    discard: 'Discard',
    regenerateCover: 'Regenerate Cover',
  },
  audioPlayer: {
    seek: 'Drag to seek',
  },
  errors: {
    apiKeyEmpty: 'Please enter API Key',
    connectionFailed: 'Connection failed:',
    invalidKey: 'Invalid API Key, please check and try again',
    networkError: 'Network connection failed, please check your network',
  },
};
```

- [ ] **Step 3: 创建 i18n index.ts（locale store + 切换逻辑）**

```ts
// src/lib/i18n/index.ts
import { writable, derived } from 'svelte/store';
import { zh } from './zh';
import { en } from './en';

type Translations = typeof zh;

const translations: Record<string, Translations> = { zh, en };

function getInitialLocale(): string {
  if (typeof window === 'undefined') return 'zh';
  const saved = localStorage.getItem('music-studio-locale');
  if (saved && (saved === 'zh' || saved === 'en')) return saved;
  return navigator.language.startsWith('zh') ? 'zh' : 'en';
}

export const locale = writable<string>(getInitialLocale());

locale.subscribe((val) => {
  if (typeof window !== 'undefined') {
    localStorage.setItem('music-studio-locale', val);
  }
});

export const t = derived(locale, ($locale) => translations[$locale] || translations.zh);

export function setLocale(lang: string) {
  locale.set(lang);
}
```

---

## Task 3: Onboarding 向导页

**Files:**
- Create: `src/routes/onboarding/+page.svelte`

**Steps:**

- [ ] **Step 1: 创建 onboarding 页面**

创建 `src/routes/onboarding/+page.svelte`，包含 3 步流程：

```svelte
<script lang="ts">
  import { goto } from '$app/navigation';
  import { invoke } from '@tauri-apps/api/core';
  import { t } from '$lib/i18n';

  let step = 1; // 1=欢迎, 2=API Key, 3=完成
  let apiKey = '';
  let showKey = false;
  let testing = false;
  let testResult: { ok: boolean; msg: string } | null = null;

  async function checkKeyStatus() {
    try {
      const configured: boolean = await invoke('get_api_key_status');
      if (configured) {
        goto('/generator');
      }
    } catch {}
  }

  checkKeyStatus();

  async function handleTest() {
    if (!apiKey.trim()) return;
    testing = true;
    testResult = null;
    try {
      const msg: string = await invoke('test_api_connection', { key: apiKey.trim() });
      testResult = { ok: true, msg };
    } catch (e: any) {
      testResult = { ok: false, msg: e.toString() };
    } finally {
      testing = false;
    }
  }

  async function handleSaveAndNext() {
    if (!apiKey.trim()) return;
    testing = true;
    try {
      await invoke('save_api_key', { key: apiKey.trim() });
      step = 3;
    } catch (e: any) {
      testResult = { ok: false, msg: e.toString() };
    } finally {
      testing = false;
    }
  }

  function openMinimaxSite() {
    window.open('https://platform.minimaxi.com/', '_blank');
  }
</script>

<div class="onboarding">
  <div class="logo-area">
    <div class="logo">🎵</div>
    <h1>Music Studio</h1>
  </div>

  {#if step === 1}
    <div class="step-content">
      <h2>{$t.onboarding.welcomeTitle}</h2>
      <p class="subtitle">{$t.onboarding.welcomeSubtitle}</p>
      <button class="primary-btn" on:click={() => step = 2}>
        {$t.onboarding.getStarted}
      </button>
    </div>
  {:else if step === 2}
    <div class="step-content">
      <h2>{$t.onboarding.connectTitle}</h2>
      <p class="description">{$t.onboarding.connectDescription}</p>

      <button class="link-btn" on:click={openMinimaxSite}>
        {$t.onboarding.howToGetKey} →
      </button>

      <div class="input-group">
        <label>{$t.onboarding.enterApiKey}</label>
        <div class="key-input-row">
          <input
            type={showKey ? 'text' : 'password'}
            bind:value={apiKey}
            placeholder={$t.onboarding.apiKeyPlaceholder}
          />
          <button class="toggle-btn" on:click={() => showKey = !showKey}>
            {showKey ? '🙈' : '👁'}
          </button>
        </div>
      </div>

      <button class="secondary-btn" on:click={handleTest} disabled={!apiKey.trim() || testing}>
        {testing ? $t.onboarding.testing : $t.onboarding.testConnection}
      </button>

      {#if testResult}
        <div class="test-result" class:ok={testResult.ok} class:error={!testResult.ok}>
          {#if testResult.ok}
            ✅ {$t.onboarding.connectionSuccess}
          {:else}
            ❌ {$t.onboarding.connectionFailed}: {testResult.msg}
          {/if}
        </div>
      {/if}

      <button
        class="primary-btn"
        on:click={handleSaveAndNext}
        disabled={!apiKey.trim() || testing}
      >
        {$t.onboarding.next}
      </button>
    </div>
  {:else}
    <div class="step-content">
      <div class="success-icon">✅</div>
      <h2>{$t.onboarding.readyTitle}</h2>
      <p class="subtitle">{$t.onboarding.readySubtitle}</p>
      <p class="hint">{$t.onboarding.readyHint}</p>
      <button class="primary-btn" on:click={() => goto('/generator')}>
        {$t.onboarding.startCreating}
      </button>
    </div>
  {/if}
</div>

<style>
  .onboarding {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
    padding: 20px;
  }
  .logo-area {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 40px;
  }
  .logo {
    font-size: 40px;
  }
  .logo-area h1 {
    font-size: 28px;
    font-weight: 700;
    color: white;
  }
  .step-content {
    background: var(--color-surface, #1e1e32);
    border: 1px solid var(--color-border, #2d2d44);
    border-radius: 16px;
    padding: 40px;
    width: 420px;
    max-width: 90vw;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;
    text-align: center;
  }
  .step-content h2 {
    font-size: 22px;
    font-weight: 700;
    color: var(--color-text, #eee);
    margin: 0;
  }
  .subtitle {
    font-size: 14px;
    color: var(--color-text-light, #888);
    margin: 0;
  }
  .description {
    font-size: 14px;
    color: var(--color-text-light, #888);
    margin: 0;
    line-height: 1.6;
  }
  .hint {
    font-size: 12px;
    color: var(--color-text-light, #666);
    margin: 0;
  }
  .link-btn {
    background: none;
    border: none;
    color: var(--color-primary, #6366f1);
    font-size: 13px;
    cursor: pointer;
    text-decoration: underline;
  }
  .input-group {
    width: 100%;
    text-align: left;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .input-group label {
    font-size: 13px;
    font-weight: 600;
    color: var(--color-text, #ccc);
  }
  .key-input-row {
    display: flex;
    gap: 8px;
  }
  .key-input-row input {
    flex: 1;
    padding: 10px 12px;
    border-radius: 8px;
    border: 1px solid var(--color-border, #2d2d44);
    background: var(--color-bg, #0f0f1a);
    color: var(--color-text, #eee);
    font-size: 13px;
  }
  .toggle-btn {
    padding: 0 12px;
    border-radius: 8px;
    border: 1px solid var(--color-border, #2d2d44);
    background: var(--color-border, #2d2d44);
    color: #aaa;
    cursor: pointer;
  }
  .test-result {
    width: 100%;
    padding: 10px 14px;
    border-radius: 8px;
    font-size: 13px;
    text-align: left;
  }
  .test-result.ok {
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
    border: 1px solid rgba(34, 197, 94, 0.3);
  }
  .test-result.error {
    background: rgba(239, 68, 68, 0.15);
    color: #ef4444;
    border: 1px solid rgba(239, 68, 68, 0.3);
  }
  .success-icon {
    font-size: 56px;
  }
  .primary-btn {
    width: 100%;
    padding: 12px;
    border-radius: 10px;
    border: none;
    background: var(--color-primary, #6366f1);
    color: white;
    font-size: 15px;
    font-weight: 600;
    cursor: pointer;
  }
  .primary-btn:hover:not(:disabled) { opacity: 0.9; }
  .primary-btn:disabled { opacity: 0.4; cursor: not-allowed; }
  .secondary-btn {
    width: 100%;
    padding: 10px;
    border-radius: 10px;
    border: 1px solid var(--color-primary, #6366f1);
    background: transparent;
    color: var(--color-primary, #6366f1);
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
  }
  .secondary-btn:hover:not(:disabled) { background: rgba(99,102,241,0.1); }
  .secondary-btn:disabled { opacity: 0.4; cursor: not-allowed; }
</style>
```

---

## Task 4: 修改 +layout.svelte 添加 Key 检测重定向

**Files:**
- Modify: `src/routes/+layout.svelte`

**Steps:**

- [ ] **Step 1: 修改 layout，添加 API Key 检测逻辑**

替换 `src/routes/+layout.svelte` 的 `<script>` 部分，添加 Key 检测和导航重定向：

```svelte
<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { t } from '$lib/i18n';

  onMount(async () => {
    // Check API key status; if not configured and not on onboarding, go to onboarding
    try {
      const configured: boolean = await invoke('get_api_key_status');
      if (!configured && !$page.url.pathname.startsWith('/onboarding')) {
        goto('/onboarding');
      }
    } catch {}
  });

  $: isOnboarding = $page.url.pathname === '/onboarding';
</script>
```

替换导航部分，使用 `$t.nav.*`：

```svelte
{#if !isOnboarding}
<nav>
  <a href="/generator" class:active={$page.url.pathname === '/generator'}>
    {$t.nav.generator}
  </a>
  <a href="/library" class:active={$page.url.pathname === '/library'}>
    {$t.nav.library}
  </a>
  <a href="/settings" class:active={$page.url.pathname === '/settings'}>
    {$t.nav.settings}
  </a>
</nav>
{/if}
```

---

## Task 5: 中文化 — Generator、Library、Settings 页面

**Files:**
- Modify: `src/routes/generator/+page.svelte`
- Modify: `src/routes/library/+page.svelte`
- Modify: `src/routes/settings/+page.svelte`

**Steps:**

- [ ] **Step 1: 中文化 generator/+page.svelte**

在 `<script>` 开头添加 `import { t } from '$lib/i18n';`

替换所有英文标签：
- `<h1>Music Generator</h1>` → `<h1>{$t.generator.title}</h1>`
- `auto-badge: "Auto Cover: ON"` → `{$t.generator.autoCoverOn}`
- `error h3` → `{$t.generator.error}`
- `coverError h3` → `{$t.generator.coverError}`
- `Generating music...` → `{$t.generator.generatingMusic}`
- `Fill in the parameters...` → `{$t.generator.emptyPrompt}`
- `Loading cover...` → `{$t.generator.loadingCover}`
- `Duration:` → `{$t.generator.duration}:`
- `Sample Rate:` → `{$t.generator.sampleRate}:`
- `Prompt` → `{$t.generator.prompt}`
- `Lyrics` → `{$t.generator.lyrics}`
- `Generate Cover` / `Generating Cover...` → `{$t.generator.generateCover}` / `{$t.generator.generatingCover}`

- [ ] **Step 2: 中文化 library/+page.svelte**

在 `<script>` 开头添加 `import { t } from '$lib/i18n';`

替换所有英文标签：
- `<h1>Music Library</h1>` → `<h1>{$t.library.title}</h1>`
- `All` / `Songs` / `Instrumental` → `$t.library.all` / `$t.library.songs` / `$t.library.instrumental`
- Sort options → `$t.library.sortByDate` / `$t.library.sortByTitle` / `$t.library.sortByDuration`
- `Filter by tag...` → `$t.library.filterByTag`
- `Refresh` → `$t.library.refresh`
- Empty state text → `$t.library.noMusic` / `$t.library.goToGenerator`
- `Edit` / `Delete` → `$t.library.edit` / `$t.library.delete`

- [ ] **Step 3: 改造 settings/+page.svelte（API 配置区 + 语言切换 + 中文化）**

重写 `src/routes/settings/+page.svelte`：

新布局（从上到下）：
1. **API 配置** — Key 输入框 + 显示/隐藏切换 + 测试连接 + 保存 + 状态指示
2. **音乐生成** — 自动封面开关
3. **语言** — 中英文下拉选择
4. **数据存储** — 现有路径配置
5. **关于** — 版本信息

新增 `<script>` 逻辑：
- `apiKey` 变量保存当前 Key
- `showApiKey` 控制密码遮罩
- `testStatus` 保存测试结果 `{ testing, result }`
- `selectedLocale` 从 localStorage 读取，切换时调用 `setLocale()`
- `currentLocale` 订阅 locale store
- `apiKeyStatus` — 调用 `get_api_key_status()` 获取连接状态

关键 UI：
- API Key 输入行：输入框（type=showKey?'text':'password'）+ toggle 按钮 + 保存按钮
- 连接状态：绿色圆点 + "已连接" 或 灰色圆点 + "未连接"
- 语言选择：`<select>` 绑定 `selectedLocale`，切换调用 `setLocale()`

---

## Task 6: 中文化 — 组件

**Files:**
- Modify: `src/lib/components/MusicForm.svelte`
- Modify: `src/lib/components/AudioPlayer.svelte`
- Modify: `src/lib/components/MusicCard.svelte`
- Modify: `src/lib/components/CoverModal.svelte`
- Modify: `src/lib/components/MetadataModal.svelte`

**Steps:**

- [ ] **Step 1: 中文化 MusicForm.svelte**

在 `<script>` 开头添加 `import { t } from '$lib/i18n';`

替换所有英文标签为 `{$t.musicForm.xxx}`：
- `AI Music Assistant` / `AI Auto-Generate` / `AI Generating...`
- AI hint text
- textarea placeholder
- Model options
- Title / Music Prompt / Lyrics labels
- Placeholders
- `Instrumental (No vocals)` / `Auto-generate lyrics`
- Audio Settings summary / Sample Rate / Bitrate / Format labels
- `Hex (default)` / `URL` / `Add AIGC watermark`
- `Generate Music` / `Generating...`
- Char counts

- [ ] **Step 2: 中文化 AudioPlayer.svelte**

在 `<script>` 开头添加 `import { t } from '$lib/i18n';`

- 保留播放按钮图标逻辑不变
- aria-label 改为 `{$t.audioPlayer.seek}`

- [ ] **Step 3: 中文化 MusicCard.svelte**

在 `<script>` 开头添加 `import { t } from '$lib/i18n';`

- `Click to view cover` → `{$t.library.viewCover || 'Click to view cover'}`（MusicCard 在 library 中用）
- `Edit` / `Delete` → `$t.library.edit` / `$t.library.delete`

- [ ] **Step 4: 中文化 CoverModal.svelte**

在 `<script>` 开头添加 `import { t } from '$lib/i18n';`

- `Cover Image` → `{$t.coverModal.coverImage}`
- `Generating cover...` → `{$t.coverModal.generatingCover}`
- `No cover yet` → `{$t.coverModal.noCover}`
- `Use This Cover` → `{$t.coverModal.useThisCover}`
- `Discard` → `{$t.coverModal.discard}`
- `Regenerate Cover` / `Generating...` → `$t.coverModal.regenerateCover` / `$t.coverModal.generatingCover`

- [ ] **Step 5: 中文化 MetadataModal.svelte**

在 `<script>` 开头添加 `import { t } from '$lib/i18n';`

- `Edit Metadata` → `{$t.metadataModal.editMetadata}`
- `Title` / `Tags` / `Notes` labels
- Placeholders
- `Add` → `$t.metadataModal.addTag`
- `Cancel` / `Save` → `$t.metadataModal.cancel` / `$t.metadataModal.save`

---

## Task 7: 创建 README.md

**Files:**
- Create: `music-studio/README.md`

**Steps:**

- [ ] **Step 1: 编写中英双语 README**

```markdown
# Music Studio

🎵 AI-powered music generation desktop application

---

## 中文说明

### 应用简介

Music Studio 是一款由 AI 驱动的桌面音乐创作工具。通过 MiniMax API，你可以使用自然语言描述来生成音乐、AI 帮你写歌词、并自动生成专辑封面。

### 系统要求

- Windows 10 或 Windows 11
- WebView2 Runtime（Windows 11 自带，Windows 10 用户点击[这里下载](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)）

### 安装步骤

1. 下载最新版本的 `.exe` 安装包
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

1. 访问 [MiniMax 开放平台](https://platform.minimaxi.com/)
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

---

## English

### About

Music Studio is an AI-powered desktop music generation tool. Using the MiniMax API, you can describe the music you want in natural language, have AI write lyrics for you, and automatically generate album covers.

### System Requirements

- Windows 10 or Windows 11
- WebView2 Runtime (built into Windows 11; Windows 10 users download [here](https://developer.microsoft.com/en-us/microsoft-edge/webview2/))

### Installation

1. Download the latest `.exe` installer
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

1. Visit [MiniMax Open Platform](https://platform.minimaxi.com/)
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
```

---

## Task 8: 构建和测试

**Files:**
- Build: `npm run tauri build`

**Steps:**

- [ ] **Step 1: 确认 dev server 可以启动**

Run: `cd music-studio && npm run tauri dev`
Expected: Tauri 窗口正常启动，Onboarding 页面显示

- [ ] **Step 2: 测试引导向导流程**

- 首次启动应重定向到 `/onboarding`
- Step 1 欢迎页正常显示
- Step 2 输入 Key 并测试连接
- Step 3 完成并跳转到 Generator

- [ ] **Step 3: 测试 Settings API 配置**

- 输入正确 Key → 保存 → 显示"已连接"
- 输入错误 Key → 测试连接 → 显示失败错误
- 语言切换 → 界面实时切换为英文

- [ ] **Step 4: 构建生产包**

Run: `cd music-studio && npm run tauri build`
Expected: `src-tauri/target/release/bundle/` 下生成 `.exe` 和 `.msi` 文件

- [ ] **Step 5: 提交代码**

```bash
cd music-studio
git add -A
git commit -m "feat: add in-app API key management, onboarding wizard, and i18n support

- Add API key management via config.json with env fallback
- Add 3-step onboarding wizard for first-time setup
- Add bilingual i18n (Chinese/English) with locale switcher in Settings
- Localize all UI components and pages
- Add bilingual README for end users
- Add save_api_key, get_api_key_status, test_api_connection Tauri commands"
```

---

## 自审清单

- [ ] 所有 `{$t.xxx}` 引用在 `zh.ts` 和 `en.ts` 中都有对应 key
- [ ] `get_api_key_status` 在 layout mount 时调用，未配置时重定向 onboarding
- [ ] `MinimaxClient::new(&api_key)` 和 `AnthropicClient::new(&api_key)` 签名一致
- [ ] Rust 端 3 个新 command 都已注册到 `lib.rs` 的 `invoke_handler`
- [ ] onboarding 页面不显示顶部导航（`isOnboarding` 条件渲染）
- [ ] README 中英文内容完整
