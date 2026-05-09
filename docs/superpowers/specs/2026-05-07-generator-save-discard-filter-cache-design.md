# Generator Save/Discard + Filter Fix + Cover Cache

## Overview

Three independent improvements to the music-studio app:
1. Generator page: preview-then-save flow with discard confirmation and leave-page guard
2. Library page: fix broken song/instrumental filter (SQL bug)
3. Library page: cache cover images to avoid repeated disk reads

## 1. Generator Preview-then-Save Flow

### Current behavior
- `generate_music` command generates audio, saves file to disk, inserts into DB, returns record.
- No way to discard generated music. No explicit save action.

### New behavior

**Backend changes:**
- New command `preview_music`: generates audio, saves file to disk (same directory), does NOT insert into DB. Returns music data without `id`.
- New command `save_music_to_library`: accepts music data (title, prompt, lyrics, model, audio_path, duration, sample_rate, bitrate, is_instrumental), inserts into DB, returns full `MusicRecord`.
- New command `discard_preview`: accepts audio file path, deletes the file from disk.

**Frontend flow:**
1. User fills form, clicks Generate.
2. Frontend calls `preview_music` → shows result with audio player, metadata, prompt, lyrics.
3. Two buttons appear: "Save to Library" and "Discard".
4. "Save to Library" → calls `save_music_to_library` → gets back record with `id` → now cover generation becomes available. Button changes to show saved state or redirects.
5. "Discard" → confirmation dialog (browser `confirm()` or custom modal) → confirmed: calls `discard_preview` with audio path → clears page state.
6. Cover generation: only available after saving (needs `music_id` for file naming). If auto-cover is enabled, it triggers after save.

**Leave-page guard:**
- Track `hasUnsavedPreview` state: `true` when preview exists and hasn't been saved/discarded.
- On `onDestroy` or route change: if `hasUnsavedPreview`, show confirmation dialog asking user to save or discard.
- Implementation: use Svelte's `beforeNavigate` lifecycle or `window.onbeforeunload`.

## 2. Song/Instrumental Filter Fix

### Root cause
`get_all_music` in `music_crud.rs` has a double-WHERE bug. Conditions are built with `WHERE` prefix per condition, then the whole string is wrapped in another `format!("WHERE {}", filter)`, producing invalid SQL like `WHERE WHERE is_instrumental = 1`.

### Fix
Rewrite condition building:
1. Collect condition fragments as `Vec<String>` (no WHERE keyword in each).
2. After all conditions collected, join with ` AND ` and prefix with `WHERE` once.
3. Change tag filter from string interpolation to parameterized query (`rusqlite::params!`) to prevent SQL injection.

## 3. Cover Image Cache

### Root cause
`MusicCard` reactive statement `$: if (music.cover_image_path) { loadCover(...) }` re-fires every time the parent `musicList` array is reassigned (sort, filter, refresh). Each re-fire reads the file from disk, base64-decodes, creates a Blob URL.

### Fix
- Module-level `Map<string, string>` cache in MusicCard or a shared store: maps `cover_image_path` → `blobUrl`.
- `loadCover()` checks cache first. On hit, reuse existing blobUrl. On miss, read from disk, cache the blobUrl.
- On component destroy: do NOT revoke cached blobUrls (they may be reused by other cards). Revoke only when the cache is explicitly cleared (page unmount).
- `CoverModal` cover updates: invalidate the cache entry for that music's cover path after regeneration.
