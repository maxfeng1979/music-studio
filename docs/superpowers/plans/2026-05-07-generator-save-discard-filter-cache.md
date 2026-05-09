# Generator Save/Discard + Filter Fix + Cover Cache Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add preview-then-save flow to the generator page, fix the broken song/instrumental filter, and add cover image caching for faster library loading.

**Architecture:** Three independent changes. Backend gets new `preview_music` / `save_music_to_library` / `discard_preview` commands; SQL filter logic is rewritten with parameterized queries; frontend gets a module-level cover cache map and leave-page guard.

**Tech Stack:** Rust (Tauri), SvelteKit 5, SQLite (rusqlite), TypeScript

---

## Task 1: Fix SQL filter bug in `get_all_music`

**Files:**
- Modify: `music-studio/src-tauri/src/db/music_crud.rs:89-135`

- [ ] **Step 1: Rewrite `get_all_music` condition building**

Replace lines 89-135 of `music_crud.rs` with:

```rust
pub fn get_all_music(&self, sort_by: Option<&str>, filter_tag: Option<&str>, filter_instrumental: Option<bool>) -> rusqlite::Result<Vec<MusicRecord>> {
    let conn = self.conn.lock().unwrap();
    let order = match sort_by {
        Some("title") => "title ASC",
        Some("duration") => "duration_ms DESC",
        _ => "created_at DESC",
    };

    let mut conditions = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(tag) = filter_tag {
        if !tag.is_empty() {
            conditions.push("tags LIKE ?".to_string());
            params.push(Box::new(format!("%{}%", tag)));
        }
    }
    if let Some(is_inst) = filter_instrumental {
        conditions.push(if is_inst { "is_instrumental = 1" } else { "is_instrumental = 0" }.to_string());
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let sql = format!(
        "SELECT id, title, prompt, lyrics, model, audio_path, cover_image_path, duration_ms, file_size, sample_rate, bitrate, created_at, tags, notes, is_instrumental FROM music {} ORDER BY {}",
        where_clause, order
    );

    let mut stmt = conn.prepare(&sql)?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(MusicRecord {
            id: row.get(0)?,
            title: row.get(1)?,
            prompt: row.get(2)?,
            lyrics: row.get(3)?,
            model: row.get(4)?,
            audio_path: row.get(5)?,
            cover_image_path: row.get(6)?,
            duration_ms: row.get(7)?,
            file_size: row.get(8)?,
            sample_rate: row.get(9)?,
            bitrate: row.get(10)?,
            created_at: row.get(11)?,
            tags: row.get(12)?,
            notes: row.get(13)?,
            is_instrumental: row.get(14)?,
        })
    })?;
    rows.collect()
}
```

- [ ] **Step 2: Verify compilation**

Run: `cd music-studio/src-tauri && cargo check`
Expected: compiles with no errors (warnings OK)

- [ ] **Step 3: Commit**

```bash
git add music-studio/src-tauri/src/db/music_crud.rs
git commit -m "fix: rewrite get_all_music SQL filter to fix double-WHERE bug and SQL injection"
```

---

## Task 2: Add backend commands for preview/save/discard

**Files:**
- Modify: `music-studio/src-tauri/src/commands/music.rs`
- Modify: `music-studio/src-tauri/src/lib.rs`

- [ ] **Step 1: Add `preview_music` command**

Append to `music.rs` (before the closing, after `generate_music_streaming`):

```rust
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct PreviewMusicResult {
    pub title: String,
    pub prompt: String,
    pub lyrics: Option<String>,
    pub model: String,
    pub audio_path: String,
    pub duration_ms: Option<i64>,
    pub file_size: Option<i64>,
    pub sample_rate: Option<i64>,
    pub bitrate: Option<i64>,
    pub is_instrumental: bool,
}

#[tauri::command]
pub async fn preview_music(params: GenerateMusicParams) -> Result<PreviewMusicResult, String> {
    let api_key = crate::config::get_api_key()?;
    let client = MinimaxClient::new(&api_key)?;

    let req = GenerateMusicRequest {
        model: params.model.clone(),
        prompt: params.prompt.clone(),
        lyrics: params.lyrics.clone(),
        is_instrumental: params.is_instrumental,
        lyrics_optimizer: params.lyrics_optimizer,
        output_format: params.output_format.clone(),
        stream: params.stream,
        audio_setting: params.audio_setting.clone(),
        aigc_watermark: params.aigc_watermark,
    };

    let resp = client.generate_music(req).await?;

    let audio_data = resp.data.audio.unwrap_or_default();
    let audio_path = storage::save_audio(&audio_data, &params.audio_setting.format)?;

    let file_size = std::fs::metadata(&audio_path)
        .map(|m| m.len() as i64)
        .ok();

    let extra = resp.extra_info;
    let duration_ms = extra.as_ref().and_then(|e| e.music_duration);
    let sample_rate = extra.as_ref().and_then(|e| e.music_sample_rate);
    let bitrate = extra.as_ref().and_then(|e| e.bitrate);

    Ok(PreviewMusicResult {
        title: params.title,
        prompt: params.prompt,
        lyrics: params.lyrics,
        model: params.model,
        audio_path: audio_path.to_string_lossy().to_string(),
        duration_ms,
        file_size,
        sample_rate: sample_rate.map(|s| s as i64),
        bitrate: bitrate.map(|b| b as i64),
        is_instrumental: params.is_instrumental,
    })
}
```

- [ ] **Step 2: Add `save_music_to_library` and `discard_preview` commands**

Append to `music.rs`:

```rust
#[derive(Debug, serde::Deserialize)]
pub struct SaveMusicParams {
    pub title: String,
    pub prompt: String,
    pub lyrics: Option<String>,
    pub model: String,
    pub audio_path: String,
    pub duration_ms: Option<i64>,
    pub file_size: Option<i64>,
    pub sample_rate: Option<i64>,
    pub bitrate: Option<i64>,
    pub is_instrumental: bool,
}

#[tauri::command]
pub async fn save_music_to_library(params: SaveMusicParams, db: State<'_, Database>) -> Result<DbMusicRecord, String> {
    let new_music = NewMusic {
        title: params.title,
        prompt: params.prompt,
        lyrics: params.lyrics,
        model: params.model,
        audio_path: params.audio_path,
        cover_image_path: None,
        duration_ms: params.duration_ms,
        file_size: params.file_size,
        sample_rate: params.sample_rate,
        bitrate: params.bitrate,
        is_instrumental: params.is_instrumental,
    };
    db.insert_music(&new_music).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn discard_preview(audio_path: String) -> Result<(), String> {
    storage::delete_audio(&audio_path)
}
```

- [ ] **Step 3: Register new commands in `lib.rs`**

In `music-studio/src-tauri/src/lib.rs`, update the `invoke_handler` macro to add the three new commands after the existing `music::generate_music_streaming` line:

```rust
music::preview_music,
music::save_music_to_library,
music::discard_preview,
```

- [ ] **Step 4: Verify compilation**

Run: `cd music-studio/src-tauri && cargo check`
Expected: compiles with no errors

- [ ] **Step 5: Commit**

```bash
git add music-studio/src-tauri/src/commands/music.rs music-studio/src-tauri/src/lib.rs
git commit -m "feat: add preview_music, save_music_to_library, discard_preview commands"
```

---

## Task 3: Update generator page frontend for preview/save/discard flow

**Files:**
- Modify: `music-studio/src/routes/generator/+page.svelte`
- Modify: `music-studio/src/lib/i18n/zh.ts`
- Modify: `music-studio/src/lib/i18n/en.ts`

- [ ] **Step 1: Add i18n keys**

In `zh.ts`, add these keys inside the `generator` object (after `generatingCover`):

```typescript
saveToLibrary: '保存到音乐库',
discard: '放弃',
discardConfirm: '确定要放弃这首音乐吗？音频文件将被删除。',
saved: '已保存',
leaveConfirm: '你有未保存的音乐，确定要离开吗？',
```

In `en.ts`, add matching keys inside the `generator` object:

```typescript
saveToLibrary: 'Save to Library',
discard: 'Discard',
discardConfirm: 'Are you sure you want to discard? The audio file will be deleted.',
saved: 'Saved',
leaveConfirm: 'You have unsaved music. Are you sure you want to leave?',
```

- [ ] **Step 2: Rewrite generator page script**

Replace the entire `<script>` section (lines 1-101) of `+page.svelte` with:

```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { beforeNavigate } from '$app/navigation';
  import { onMount } from 'svelte';
  import MusicForm from '$lib/components/MusicForm.svelte';
  import AudioPlayer from '$lib/components/AudioPlayer.svelte';
  import { t } from '$lib/i18n';

  let loading = false;
  let generatedMusic: any = null;
  let isSaved = false;
  let error: string | null = null;
  let autoCoverEnabled = false;
  let coverBlobUrl = '';
  let coverLoading = false;
  let coverError: string | null = null;

  $: hasUnsavedPreview = generatedMusic !== null && !isSaved;

  // Read autoCover setting from localStorage
  try {
    const saved = localStorage.getItem('music-studio-settings');
    if (saved) {
      const settings = JSON.parse(saved);
      autoCoverEnabled = settings.autoCover ?? false;
    }
  } catch {}

  window.addEventListener('storage', () => {
    try {
      const saved = localStorage.getItem('music-studio-settings');
      if (saved) {
        const settings = JSON.parse(saved);
        autoCoverEnabled = settings.autoCover ?? false;
      }
    } catch {}
  });

  beforeNavigate(() => {
    if (hasUnsavedPreview) {
      if (!confirm($t.generator.leaveConfirm)) {
        return false;
      }
      // User confirmed leaving — clean up preview audio
      if (generatedMusic?.audio_path) {
        invoke('discard_preview', { audioPath: generatedMusic.audio_path }).catch(() => {});
      }
    }
  });

  async function loadCoverAsBlob(filePath: string) {
    if (coverBlobUrl) URL.revokeObjectURL(coverBlobUrl);
    coverBlobUrl = '';
    try {
      const result = await invoke<{ data: string; mime_type: string }>('read_file_as_data_url', { path: filePath });
      const binaryStr = atob(result.data);
      const bytes = new Uint8Array(binaryStr.length);
      for (let i = 0; i < binaryStr.length; i++) bytes[i] = binaryStr.charCodeAt(i);
      const blob = new Blob([bytes], { type: result.mime_type });
      coverBlobUrl = URL.createObjectURL(blob);
    } catch (e) {
      console.error('Failed to load cover:', e);
    }
  }

  async function handleGenerate(data: any) {
    loading = true;
    error = null;
    generatedMusic = null;
    isSaved = false;
    coverBlobUrl = '';
    coverError = null;

    try {
      generatedMusic = await invoke('preview_music', { params: data });
    } catch (e: any) {
      error = e.toString();
    } finally {
      loading = false;
    }
  }

  async function handleSaveToLibrary() {
    if (!generatedMusic) return;
    try {
      const record = await invoke('save_music_to_library', { params: generatedMusic });
      generatedMusic = record;
      isSaved = true;

      // Auto-generate cover if enabled
      if (autoCoverEnabled) {
        handleGenerateCover();
      }
    } catch (e: any) {
      error = e.toString();
    }
  }

  async function handleDiscard() {
    if (!generatedMusic) return;
    if (!confirm($t.generator.discardConfirm)) return;

    try {
      if (generatedMusic.audio_path) {
        await invoke('discard_preview', { audioPath: generatedMusic.audio_path });
      }
    } catch (e) {
      console.error('Failed to discard preview:', e);
    }

    generatedMusic = null;
    isSaved = false;
    coverBlobUrl = '';
    coverError = null;
  }

  async function handleGenerateCover() {
    if (!generatedMusic || !generatedMusic.id) return;
    coverLoading = true;
    coverError = null;
    try {
      const coverPrompt = await invoke<string>('generate_cover_prompt', {
        params: {
          title: generatedMusic.title,
          music_prompt: generatedMusic.prompt,
          lyrics: generatedMusic.lyrics || null,
        }
      });

      const coverPath = await invoke('generate_cover_image', {
        params: {
          music_id: generatedMusic.id,
          prompt: coverPrompt,
          aspect_ratio: '1:1',
          response_format: 'base64',
          n: 1,
        }
      });
      generatedMusic = { ...generatedMusic, cover_image_path: coverPath };
      await loadCoverAsBlob(coverPath as string);
    } catch (e: any) {
      coverError = e.toString();
    } finally {
      coverLoading = false;
    }
  }
</script>
```

- [ ] **Step 3: Update the result-actions div in the template**

Replace the `<div class="result-actions">` block (lines 168-178) with:

```svelte
          <div class="result-actions">
            {#if !isSaved}
              <button class="primary" on:click={handleSaveToLibrary}>{$t.generator.saveToLibrary}</button>
              <button class="secondary danger-btn" on:click={handleDiscard}>{$t.generator.discard}</button>
            {:else}
              <span class="saved-badge">{$t.generator.saved}</span>
              {#if !generatedMusic.cover_image_path}
                <button class="secondary" on:click={handleGenerateCover} disabled={coverLoading}>
                  {#if coverLoading}
                    {$t.generator.generatingCover}
                  {:else}
                    {$t.generator.generateCover}
                  {/if}
                </button>
              {/if}
            {/if}
          </div>
```

- [ ] **Step 4: Add styles for new elements**

Add these CSS rules inside the `<style>` section:

```css
  .primary {
    background: var(--color-primary);
    color: white;
    padding: 8px 16px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 600;
  }

  .primary:hover {
    opacity: 0.9;
  }

  .danger-btn {
    color: #ef4444;
    border-color: #ef4444;
  }

  .danger-btn:hover {
    background: #ef4444;
    color: white;
  }

  .saved-badge {
    font-size: 12px;
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
    padding: 6px 12px;
    border-radius: 8px;
    font-weight: 600;
  }
```

- [ ] **Step 5: Commit**

```bash
git add music-studio/src/routes/generator/+page.svelte music-studio/src/lib/i18n/zh.ts music-studio/src/lib/i18n/en.ts
git commit -m "feat: add preview-then-save flow with discard and leave-page guard"
```

---

## Task 4: Add cover image cache

**Files:**
- Modify: `music-studio/src/lib/components/MusicCard.svelte`

- [ ] **Step 1: Add module-level cache and update loadCover**

Add at the top of the `<script>` section (after the imports, before `export let music`):

```typescript
  const coverCache = new Map<string, string>();
```

Replace the `loadCover` function (lines 28-45) with:

```typescript
  async function loadCover(filePath: string) {
    // Check cache first
    if (coverCache.has(filePath)) {
      coverBlobUrl = coverCache.get(filePath)!;
      return;
    }
    try {
      const result = await invoke<{ data: string; mime_type: string }>('read_file_as_data_url', { path: filePath });
      const binaryStr = atob(result.data);
      const bytes = new Uint8Array(binaryStr.length);
      for (let i = 0; i < binaryStr.length; i++) {
        bytes[i] = binaryStr.charCodeAt(i);
      }
      const blob = new Blob([bytes], { type: result.mime_type });
      const url = URL.createObjectURL(blob);
      coverCache.set(filePath, url);
      coverBlobUrl = url;
    } catch (e) {
      console.error('Failed to load cover:', e);
    }
  }
```

- [ ] **Step 2: Update the reactive statement to skip re-loading cached paths**

Replace the reactive statement (lines 47-49):

```typescript
  $: if (music.cover_image_path && !coverCache.has(music.cover_image_path)) {
    loadCover(music.cover_image_path);
  } else if (music.cover_image_path && coverCache.has(music.cover_image_path)) {
    coverBlobUrl = coverCache.get(music.cover_image_path)!;
  }
```

- [ ] **Step 3: Update handleCoverUpdated to invalidate cache**

Replace `handleCoverUpdated` (lines 59-65) with:

```typescript
  function handleCoverUpdated(id: number, coverPath: string) {
    coverCache.delete(coverPath);
    coverBlobUrl = '';
    onCoverUpdated(id, coverPath);
  }
```

- [ ] **Step 4: Commit**

```bash
git add music-studio/src/lib/components/MusicCard.svelte
git commit -m "feat: add cover image cache to avoid repeated disk reads"
```

---

## Task 5: Manual verification

- [ ] **Step 1: Start the dev server**

Run: `cd music-studio && npm run tauri dev`

- [ ] **Step 2: Test generator preview/save/discard flow**

1. Go to Generator page, fill form, click Generate.
2. Verify: result appears with "Save to Library" and "Discard" buttons, no cover button yet.
3. Click "Save to Library" — verify "Saved" badge appears, cover button now visible.
4. Generate another music, click "Discard" — verify confirmation dialog appears, confirm clears the page.
5. Generate music, then try clicking Library nav — verify leave-page confirmation appears.

- [ ] **Step 3: Test song/instrumental filter**

1. Go to Library page.
2. Click "Songs" — verify only songs (non-instrumental) appear.
3. Click "Instrumental" — verify only instrumental tracks appear.
4. Click "All" — verify all tracks appear.

- [ ] **Step 4: Test cover cache**

1. On Library page, wait for all covers to load.
2. Click "Refresh" — verify covers appear instantly (no spinner/delay) since they are cached.
3. Sort by title — verify covers still show immediately without re-loading.
