# Music Studio: API Key 管理 + 中文化 + 分发优化

**日期**: 2026-05-06
**状态**: 已确认

## 目标

将 Music Studio 从"需要设置系统环境变量"改为"App 内管理 API Key + 首次引导向导"，使其能够分发给不会编程的普通中国用户使用。同时完成全站中文化（支持中英双语，默认中文）。

## 目标用户

混合人群（完全小白到有一定 AI 工具经验的用户），主要面向中国用户。先只做 Windows 分发。

## 设计方案

### 1. API Key 存储层

**存储位置**: `{app_data_dir}/config.json`（即 `%APPDATA%/com.minimax.music-studio/config.json`）

**存储格式**:
```json
{
  "minimax_api_key": "user_entered_key_here"
}
```

**读取优先级**:
1. 先读 config.json 中的 `minimax_api_key`
2. 如果没有，fallback 读环境变量 `MINIMAX_API_KEY`（兼容老用户）
3. 都没有，返回错误

**Rust 端改动**:

`src-tauri/src/config.rs` 扩展:
- `get_config_path()` — 返回 config.json 路径
- `read_config()` — 读取并解析 config.json，返回 `Config { minimax_api_key: Option<String> }`
- `save_api_key(key: &str)` — 保存 Key 到 config.json
- `get_api_key()` — 按优先级（config → env）获取 Key，返回 `Result<String, String>`

`MinimaxClient` 和 `AnthropicClient` 改造:
- `new()` 改为 `new(api_key: &str)`，不再自己读环境变量
- 调用方（commands）通过 `get_api_key()` 获取 Key 后传入

**新增 Tauri Commands**:
- `save_api_key(key: String)` — 保存 Key 到 config.json
- `get_api_key_status()` — 返回 `{ configured: bool }`
- `test_api_connection(key: String)` — 用提供的 Key 调用 Anthropic 兼容端点发一个最小请求（如 "hi"，max_tokens=1），仅验证 Key 有效性，返回成功/失败 + 错误信息

### 2. 首次启动引导向导（Onboarding）

**触发条件**: App 启动时检查 config.json 中是否有 `minimax_api_key`，没有则显示引导页。

**向导流程**:

**Step 1 — 欢迎页**
- 标题：「欢迎使用 Music Studio」
- 简短介绍：AI 驱动的音乐创作工具
- 「开始设置」按钮

**Step 2 — API Key 配置**
- 标题：「连接你的 AI 服务」
- 说明文字：「Music Studio 使用 MiniMax 的 AI 服务来生成音乐。你需要一个 API Key 来开始。」
- 「如何获取 API Key」链接 → 打开 MiniMax 官网注册页
- API Key 输入框（密码遮罩 + 显示/隐藏切换）
- 「测试连接」按钮 → 调用 `test_api_connection`
  - 成功：绿色提示「连接成功！」
  - 失败：红色提示具体错误
- 连接成功后才能点「下一步」

**Step 3 — 完成**
- 标题：「一切就绪！」
- 提示：「你可以在设置中随时修改 API Key」
- 「开始创作」按钮 → 进入主界面

**实现方式**:
- 新增 `src/routes/onboarding/+page.svelte`
- `+layout.svelte` 中检测 Key 状态，未配置时重定向到 `/onboarding`
- 向导完成后存储 Key，重定向到 `/generator`
- Onboarding 页面不显示顶部导航栏

### 3. Settings 页面改造

**新增「API 配置」区域**（页面顶部）:
- API Key 输入框（密码遮罩 + 显示/隐藏切换）
- 「测试连接」按钮（状态反馈：成功/失败）
- 「保存」按钮
- 已配置状态显示：绿色指示器 + 「当前已连接 MiniMax 服务」

**新增「语言 / Language」区域**:
- 下拉选择：中文（默认）/ English
- 选择存 localStorage，实时切换

**Settings 页面布局（从上到下）**:
1. **API 配置** — Key 输入、测试、保存
2. **音乐生成** — 自动生成封面开关
3. **语言 / Language** — 中英文切换
4. **数据存储** — 存储路径
5. **关于** — 版本信息

### 4. 中英文双语 i18n

**文件结构**:
```
src/lib/i18n/
  zh.ts       — 中文文案
  en.ts       — 英文文案
  index.ts    — 导出 locale store + 切换逻辑
```

**实现方式**:
- 每个 `zh.ts` / `en.ts` 导出同结构的 key-value map
- `index.ts` 导出一个 Svelte writable store `locale`，根据当前语言返回对应文案
- 默认语言检测：`navigator.language.startsWith('zh') ? 'zh' : 'en'`
- 用户可在 Settings 手动切换，选择存 localStorage key `music-studio-locale`

**中文化范围**:
- 顶部导航：生成器 / 音乐库 / 设置
- Generator 页面所有表单标签和按钮
- Library 页面所有文案（空状态、排序选项等）
- Settings 页面所有文案
- Onboarding 向导所有文案
- 所有错误提示、成功提示、Loading 状态文案
- API Key 相关错误提示：
  - Key 为空 → 「请输入 API Key」
  - 测试失败 → 「连接失败：[具体错误信息]」
  - Key 无效 → 「API Key 无效，请检查后重试」
  - 网络错误 → 「网络连接失败，请检查网络设置」

### 5. README 使用文档

新增 `music-studio/README.md`，面向普通用户（非开发者），中英双语。

**内容结构**:
1. **应用简介** — 一句话说明 Music Studio 是什么
2. **系统要求** — Windows 10/11
3. **安装步骤** — 下载 `.exe` → 双击安装 → 完成
4. **首次使用** — 启动 → 引导向导 → 输入 API Key → 测试连接 → 开始创作
5. **如何获取 API Key** — MiniMax 注册链接 + 步骤说明
6. **功能说明** — 音乐生成、AI 助手写词、封面生成、音乐库
7. **常见问题** — 连接失败怎么办、如何更换 API Key、数据存在哪里
8. **截图** — 关键页面截图（引导向导、生成器、音乐库）

README 也会作为安装包内的说明文件一起分发。

### 6. Windows 打包分发

- 使用 `npm run tauri build` 生成安装包
- 输出格式：`.msi` + `.exe`（NSIS）
- 当前阶段直接提供 `.exe` 下载（GitHub Release 或网盘）
- Tauri bundle 配置已就绪，无需改动

## 文件改动清单

### 新增文件
- `music-studio/README.md` — 用户使用文档（中英双语）
- `src/routes/onboarding/+page.svelte` — 引导向导页面
- `src/lib/i18n/zh.ts` — 中文文案
- `src/lib/i18n/en.ts` — 英文文案
- `src/lib/i18n/index.ts` — i18n 逻辑

### 修改文件
- `src-tauri/src/config.rs` — 新增 config.json 读写、API Key 管理
- `src-tauri/src/api/minimax.rs` — `new()` 接受 Key 参数
- `src-tauri/src/api/anthropic.rs` — `new()` 接受 Key 参数
- `src-tauri/src/commands/music.rs` — 从 config 获取 Key
- `src-tauri/src/commands/image.rs` — 从 config 获取 Key
- `src-tauri/src/commands/ai_assist.rs` — 从 config 获取 Key
- `src-tauri/src/commands/settings.rs` — 新增 API Key 相关 commands
- `src-tauri/src/lib.rs` — 注册新 commands
- `src/routes/+layout.svelte` — 添加 Key 检测 + 重定向逻辑
- `src/routes/generator/+page.svelte` — 中文化
- `src/routes/library/+page.svelte` — 中文化
- `src/routes/settings/+page.svelte` — 改造 + 中文化
- `src/lib/components/MusicForm.svelte` — 中文化
- `src/lib/components/AudioPlayer.svelte` — 中文化
- `src/lib/components/MusicCard.svelte` — 中文化
- `src/lib/components/CoverModal.svelte` — 中文化
- `src/lib/components/MetadataModal.svelte` — 中文化
