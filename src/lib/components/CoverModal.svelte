<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { t } from '$lib/i18n';

  export let music: {
    id: number;
    title: string;
    prompt: string;
    lyrics?: string;
    cover_image_path: string | null;
  };
  export let coverBlobUrl: string;
  export let onClose: () => void;
  export let onCoverUpdated: (id: number, coverPath: string) => void;

  let generating = false;
  let error: string | null = null;

  // Preview state after generation
  let pendingCoverPath: string | null = null;
  let pendingCoverBlobUrl = '';

  async function handleRegenerate() {
    generating = true;
    error = null;
    try {
      // Generate a cover image prompt via LLM
      const coverPrompt = await invoke<string>('generate_cover_prompt', {
        params: {
          title: music.title,
          music_prompt: music.prompt,
          lyrics: music.lyrics || null,
        }
      });

      const coverPath = await invoke<string>('generate_cover_image', {
        params: {
          music_id: music.id,
          prompt: coverPrompt,
          aspect_ratio: '1:1',
          response_format: 'base64',
          n: 1,
        }
      });

      // Load the new cover as blob for preview
      const result = await invoke<{ data: string; mime_type: string }>('read_file_as_data_url', { path: coverPath });
      const binaryStr = atob(result.data);
      const bytes = new Uint8Array(binaryStr.length);
      for (let i = 0; i < binaryStr.length; i++) {
        bytes[i] = binaryStr.charCodeAt(i);
      }
      const blob = new Blob([bytes], { type: result.mime_type });
      pendingCoverBlobUrl = URL.createObjectURL(blob);
      pendingCoverPath = coverPath;
    } catch (e: any) {
      error = e.toString();
    } finally {
      generating = false;
    }
  }

  function confirmCover() {
    if (!pendingCoverPath) return;
    onCoverUpdated(music.id, pendingCoverPath);
    pendingCoverPath = null;
    pendingCoverBlobUrl = '';
    onClose();
  }

  function cancelCover() {
    if (pendingCoverBlobUrl) URL.revokeObjectURL(pendingCoverBlobUrl);
    pendingCoverBlobUrl = '';
    pendingCoverPath = null;
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      cancelCover();
      onClose();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      cancelCover();
      onClose();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="modal-backdrop" on:click={handleBackdropClick}>
  <div class="modal-content">
    <div class="modal-header">
      <h3>{$t.coverModal.coverImage}</h3>
      <button class="close-btn" on:click={() => { cancelCover(); onClose(); }}>x</button>
    </div>

    <div class="cover-display">
      {#if pendingCoverPath}
        <img src={pendingCoverBlobUrl} alt="New Cover Preview" class="cover-img preview" />
        <div class="confirm-bar">
          <button class="confirm-btn yes" on:click={confirmCover}>{$t.coverModal.useThisCover}</button>
          <button class="confirm-btn no" on:click={cancelCover}>{$t.coverModal.discard}</button>
        </div>
      {:else if generating}
        <div class="cover-placeholder">
          <div class="spinner"></div>
          <p>{$t.coverModal.generatingCover}</p>
        </div>
      {:else if coverBlobUrl}
        <img src={coverBlobUrl} alt="{music.title} Cover" class="cover-img" />
      {:else}
        <div class="cover-placeholder">
          <span>{$t.coverModal.noCover}</span>
        </div>
      {/if}
    </div>

    {#if error}
      <p class="error-msg">{error}</p>
    {/if}

    {#if !pendingCoverPath}
      <div class="modal-actions">
        <button class="regenerate-btn" on:click={handleRegenerate} disabled={generating}>
          {generating ? $t.coverModal.generatingCover : $t.coverModal.regenerateCover}
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    background: var(--color-surface, #1a1a2e);
    border-radius: 16px;
    border: 1px solid var(--color-border, #333);
    padding: 20px;
    width: 360px;
    max-width: 90vw;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  .modal-header h3 {
    font-size: 16px;
    font-weight: 600;
    color: var(--color-text, #eee);
  }

  .close-btn {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    border: none;
    background: var(--color-border, #333);
    color: var(--color-text-light, #999);
    font-size: 14px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-btn:hover {
    background: #ef4444;
    color: white;
  }

  .cover-display {
    width: 100%;
    aspect-ratio: 1;
    border-radius: 12px;
    overflow: hidden;
    background: linear-gradient(135deg, #2d2d44, #1a1a2e);
    position: relative;
  }

  .cover-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .cover-img.preview {
    border: 3px solid #22c55e;
    box-sizing: border-box;
  }

  .cover-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--color-text-light, #999);
    gap: 12px;
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid rgba(255, 255, 255, 0.2);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .confirm-bar {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    display: flex;
    gap: 8px;
    padding: 12px;
    background: rgba(0, 0, 0, 0.8);
    backdrop-filter: blur(4px);
  }

  .confirm-btn {
    flex: 1;
    padding: 10px;
    border-radius: 8px;
    border: none;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    color: white;
  }

  .confirm-btn.yes {
    background: #22c55e;
  }

  .confirm-btn.yes:hover {
    background: #16a34a;
  }

  .confirm-btn.no {
    background: rgba(255, 255, 255, 0.15);
  }

  .confirm-btn.no:hover {
    background: #ef4444;
  }

  .error-msg {
    font-size: 12px;
    color: #ef4444;
    margin-top: 12px;
    text-align: center;
  }

  .modal-actions {
    margin-top: 16px;
  }

  .regenerate-btn {
    width: 100%;
    padding: 12px;
    border-radius: 8px;
    border: 1px solid var(--color-border, #333);
    background: var(--color-border, #333);
    color: var(--color-text, #eee);
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
  }

  .regenerate-btn:hover:not(:disabled) {
    background: var(--color-primary, #6366f1);
    color: white;
    border-color: var(--color-primary, #6366f1);
  }

  .regenerate-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
