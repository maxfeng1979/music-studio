<script lang="ts">
  import AudioPlayer from './AudioPlayer.svelte';
  import CoverModal from './CoverModal.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { t } from '$lib/i18n';

  export let music: {
    id: number;
    title: string;
    prompt: string;
    model: string;
    audio_path: string;
    cover_image_path: string | null;
    duration_ms: number | null;
    file_size: number | null;
    created_at: string;
    tags: string;
    is_instrumental: boolean;
  };

  export let onEdit: (music: typeof music) => void;
  export let onDelete: (id: number) => void;
  export let onCoverUpdated: (id: number, coverPath: string) => void;

  let coverBlobUrl = '';
  let showCoverModal = false;

  async function loadCover(filePath: string) {
    if (coverBlobUrl) {
      URL.revokeObjectURL(coverBlobUrl);
      coverBlobUrl = '';
    }
    try {
      const result = await invoke<{ data: string; mime_type: string }>('read_file_as_data_url', { path: filePath });
      const binaryStr = atob(result.data);
      const bytes = new Uint8Array(binaryStr.length);
      for (let i = 0; i < binaryStr.length; i++) {
        bytes[i] = binaryStr.charCodeAt(i);
      }
      const blob = new Blob([bytes], { type: result.mime_type });
      coverBlobUrl = URL.createObjectURL(blob);
    } catch (e) {
      console.error('Failed to load cover:', e);
    }
  }

  $: if (music.cover_image_path) {
    loadCover(music.cover_image_path);
  }

  function openCoverModal() {
    showCoverModal = true;
  }

  function closeCoverModal() {
    showCoverModal = false;
  }

  function handleCoverUpdated(id: number, coverPath: string) {
    if (coverBlobUrl) URL.revokeObjectURL(coverBlobUrl);
    coverBlobUrl = '';
    onCoverUpdated(id, coverPath);
    // Reload cover after DB update
    loadCover(coverPath);
  }

  function formatDuration(ms: number | null): string {
    if (!ms) return '--:--';
    const s = Math.floor(ms / 1000);
    const m = Math.floor(s / 60);
    const sec = s % 60;
    return `${m}:${sec.toString().padStart(2, '0')}`;
  }

  function formatSize(bytes: number | null): string {
    if (!bytes) return '';
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }

  function parseTags(tags: string): string[] {
    try {
      return JSON.parse(tags);
    } catch {
      return [];
    }
  }

  function formatDate(dateStr: string): string {
    const d = new Date(dateStr);
    return d.toLocaleDateString('zh-CN', { year: 'numeric', month: 'short', day: 'numeric' });
  }

  let tags: string[] = [];
  $: tags = parseTags(music.tags);
</script>

<div class="music-card">
  <div class="cover-area" on:click={openCoverModal} role="button" tabindex="0" aria-label={$t.library.viewCover}>
    {#if coverBlobUrl}
      <img src={coverBlobUrl} alt="Cover" class="cover-img" />
      <div class="cover-hover-hint">Click to view</div>
    {:else}
      <div class="cover-placeholder">
        <span>🎵</span>
      </div>
    {/if}
  </div>

  <div class="card-content">
    <div class="card-header">
      <h3 class="title">{music.title}</h3>
      <span class="date">{formatDate(music.created_at)}</span>
    </div>

    <div class="meta-row">
      <span class="duration">{formatDuration(music.duration_ms)}</span>
      {#if music.file_size}
        <span class="size">{formatSize(music.file_size)}</span>
      {/if}
      <span class="model-badge">{music.model}</span>
      {#if music.is_instrumental}
        <span class="instrumental-badge">Instrumental</span>
      {/if}
    </div>

    {#if tags.length > 0}
      <div class="tags">
        {#each tags as tag}
          <span class="tag">{tag}</span>
        {/each}
      </div>
    {/if}

    <AudioPlayer src={music.audio_path} compact />

    <div class="actions">
      <button class="action-btn" on:click={() => onEdit(music)}>{$t.library.edit}</button>
      <button class="action-btn danger" on:click={() => onDelete(music.id)}>{$t.library.delete}</button>
    </div>
  </div>
</div>

{#if showCoverModal}
  <CoverModal
    {music}
    {coverBlobUrl}
    onClose={closeCoverModal}
    onCoverUpdated={handleCoverUpdated}
  />
{/if}

<style>
  .music-card {
    display: flex;
    gap: 14px;
    padding: 14px;
    background: var(--color-surface);
    border-radius: 12px;
    border: 1px solid var(--color-border);
    transition: border-color 0.2s;
  }

  .music-card:hover {
    border-color: var(--color-primary);
  }

  .cover-area {
    width: 100px;
    height: 100px;
    flex-shrink: 0;
    border-radius: 8px;
    overflow: hidden;
    position: relative;
    cursor: pointer;
  }

  .cover-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .cover-hover-hint {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    background: rgba(0, 0, 0, 0.7);
    color: white;
    font-size: 10px;
    text-align: center;
    padding: 4px 0;
    opacity: 0;
    transition: opacity 0.2s;
  }

  .cover-area:hover .cover-hover-hint {
    opacity: 1;
  }

  .cover-placeholder {
    width: 100%;
    height: 100%;
    background: linear-gradient(135deg, #2d2d44, #1a1a2e);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 36px;
  }

  .card-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-width: 0;
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .title {
    font-size: 15px;
    font-weight: 600;
    color: var(--color-text);
    word-break: break-word;
  }

  .date {
    font-size: 11px;
    color: var(--color-text-light);
    white-space: nowrap;
  }

  .meta-row {
    display: flex;
    gap: 10px;
    font-size: 12px;
    color: var(--color-text-light);
  }

  .model-badge {
    background: var(--color-border);
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 10px;
  }

  .instrumental-badge {
    background: rgba(99, 102, 241, 0.2);
    color: var(--color-primary);
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 10px;
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
  }

  .tag {
    background: rgba(99, 102, 241, 0.2);
    color: var(--color-primary);
    padding: 2px 8px;
    border-radius: 10px;
    font-size: 11px;
  }

  .actions {
    display: flex;
    gap: 6px;
    margin-top: auto;
  }

  .action-btn {
    padding: 5px 10px;
    font-size: 11px;
    background: var(--color-border);
    color: var(--color-text);
    border-radius: 5px;
  }

  .action-btn:hover {
    background: var(--color-primary);
    color: white;
  }

  .action-btn.danger:hover {
    background: #ef4444;
  }
</style>
