<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import MusicForm from '$lib/components/MusicForm.svelte';
  import AudioPlayer from '$lib/components/AudioPlayer.svelte';
  import { t } from '$lib/i18n';

  let loading = false;
  let generatedMusic: any = null;
  let error: string | null = null;
  let autoCoverEnabled = false;
  let coverBlobUrl = '';
  let coverLoading = false;
  let coverError: string | null = null;

  // Read autoCover setting from localStorage
  try {
    const saved = localStorage.getItem('music-studio-settings');
    if (saved) {
      const settings = JSON.parse(saved);
      autoCoverEnabled = settings.autoCover ?? false;
    }
  } catch {}

  // Listen for storage changes (when user toggles in Settings page)
  window.addEventListener('storage', () => {
    try {
      const saved = localStorage.getItem('music-studio-settings');
      if (saved) {
        const settings = JSON.parse(saved);
        autoCoverEnabled = settings.autoCover ?? false;
      }
    } catch {}
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
    coverBlobUrl = '';
    coverError = null;

    try {
      generatedMusic = await invoke('generate_music', { params: data });
      // Auto-generate cover if enabled
      if (generatedMusic && autoCoverEnabled) {
        handleGenerateCover();
      }
    } catch (e: any) {
      error = e.toString();
    } finally {
      loading = false;
    }
  }

  async function handleGenerateCover() {
    if (!generatedMusic) return;
    coverLoading = true;
    coverError = null;
    try {
      // Step 1: Generate a cover image prompt via LLM
      const coverPrompt = await invoke<string>('generate_cover_prompt', {
        params: {
          title: generatedMusic.title,
          music_prompt: generatedMusic.prompt,
          lyrics: generatedMusic.lyrics || null,
        }
      });

      // Step 2: Generate cover image using the AI-generated prompt
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

<div class="generator-page">
  <div class="generator-header">
    <h1>{$t.generator.title}</h1>
    {#if autoCoverEnabled}
      <span class="auto-badge">{$t.generator.autoCoverOn}</span>
    {/if}
  </div>

  <div class="generator-layout">
    <div class="result-area">
      {#if error}
        <div class="error-box">
          <h3>{$t.generator.error}</h3>
          <p>{error}</p>
        </div>
      {/if}

      {#if coverError}
        <div class="error-box">
          <h3>{$t.generator.coverError}</h3>
          <p>{coverError}</p>
        </div>
      {/if}

      {#if generatedMusic}
        <div class="result-card">
          <div class="result-header">
            <h2>{generatedMusic.title}</h2>
            <span class="model-tag">{generatedMusic.model}</span>
          </div>

          {#if coverBlobUrl}
            <div class="cover-preview">
              <img src={coverBlobUrl} alt="Cover" />
            </div>
          {:else if generatedMusic.cover_image_path}
            <div class="cover-preview">
              <span>{$t.generator.loadingCover}</span>
            </div>
          {/if}

          <AudioPlayer src={generatedMusic.audio_path} />

          <div class="result-meta">
            {#if generatedMusic.duration_ms}
              <span>{$t.generator.duration}: {Math.floor(generatedMusic.duration_ms / 1000)}s</span>
            {/if}
            {#if generatedMusic.sample_rate}
              <span>{$t.generator.sampleRate}: {generatedMusic.sample_rate} Hz</span>
            {/if}
          </div>

          <div class="result-prompt">
            <h4>{$t.generator.prompt}</h4>
            <p>{generatedMusic.prompt}</p>
          </div>

          {#if generatedMusic.lyrics && !generatedMusic.is_instrumental}
            <div class="result-lyrics">
              <h4>{$t.generator.lyrics}</h4>
              <pre>{generatedMusic.lyrics}</pre>
            </div>
          {/if}

          <div class="result-actions">
            {#if !generatedMusic.cover_image_path}
              <button class="secondary" on:click={handleGenerateCover} disabled={coverLoading}>
                {#if coverLoading}
                  {$t.generator.generatingCover}
                {:else}
                  {$t.generator.generateCover}
                {/if}
              </button>
            {/if}
          </div>
        </div>
      {:else if loading}
        <div class="loading-state">
          <div class="spinner"></div>
          <p>{$t.generator.generatingMusic}</p>
        </div>
      {:else}
        <div class="empty-state">
          <span class="empty-icon">🎵</span>
          <p>{$t.generator.emptyPrompt}</p>
        </div>
      {/if}
    </div>

    <div class="form-area">
      <MusicForm onSubmit={handleGenerate} {loading} />
    </div>
  </div>
</div>

<style>
  .generator-page {
    padding: 20px;
    height: calc(100vh - 52px);
    overflow-y: auto;
  }

  .generator-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  .generator-header h1 {
    font-size: 22px;
    font-weight: 700;
  }

  .auto-badge {
    font-size: 11px;
    background: rgba(99, 102, 241, 0.2);
    color: var(--color-primary);
    padding: 4px 10px;
    border-radius: 10px;
  }

  .generator-layout {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
    align-items: start;
  }

  .result-area {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .form-area {
    position: sticky;
    top: 0;
    max-height: calc(100vh - 92px);
    overflow-y: auto;
  }

  .error-box {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid #ef4444;
    border-radius: 10px;
    padding: 16px;
  }

  .error-box h3 {
    color: #ef4444;
    font-size: 14px;
    margin-bottom: 6px;
  }

  .error-box p {
    font-size: 13px;
    color: var(--color-text-light);
  }

  .result-card {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: 12px;
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .result-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .result-header h2 {
    font-size: 18px;
    font-weight: 600;
  }

  .model-tag {
    font-size: 11px;
    background: var(--color-border);
    padding: 4px 8px;
    border-radius: 4px;
  }

  .cover-preview {
    width: 200px;
    height: 200px;
    border-radius: 8px;
    overflow: hidden;
    flex-shrink: 0;
  }

  .cover-preview img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .result-meta {
    display: flex;
    gap: 16px;
    font-size: 12px;
    color: var(--color-text-light);
  }

  .result-prompt, .result-lyrics {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .result-prompt h4, .result-lyrics h4 {
    font-size: 12px;
    font-weight: 600;
    color: var(--color-text-light);
  }

  .result-prompt p {
    font-size: 13px;
    color: var(--color-text);
    line-height: 1.5;
  }

  .result-lyrics pre {
    font-size: 12px;
    color: var(--color-text);
    background: var(--color-bg);
    padding: 12px;
    border-radius: 6px;
    white-space: pre-wrap;
    font-family: inherit;
    line-height: 1.6;
  }

  .result-actions {
    display: flex;
    gap: 10px;
    margin-top: 8px;
  }

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 80px 20px;
    color: var(--color-text-light);
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid var(--color-border);
    border-top-color: var(--color-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-bottom: 16px;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 80px 20px;
    color: var(--color-text-light);
  }

  .empty-icon {
    font-size: 48px;
    margin-bottom: 12px;
    opacity: 0.5;
  }

  .empty-state p {
    font-size: 14px;
  }
</style>
