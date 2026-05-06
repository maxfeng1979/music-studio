<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { t } from '$lib/i18n';

  export interface FormData {
    title: string;
    model: string;
    prompt: string;
    lyrics: string;
    is_instrumental: boolean;
    lyrics_optimizer: boolean;
    output_format: string;
    audio_setting: {
      sample_rate: number;
      bitrate: number;
      format: string;
    };
    aigc_watermark: boolean;
  }

  export let onSubmit: (data: FormData) => void;
  export let disabled = false;
  export let loading = false;

  let aiDescription = '';
  let aiLoading = false;
  let aiError: string | null = null;

  let formData: FormData = {
    title: '',
    model: 'music-2.6',
    prompt: '',
    lyrics: '',
    is_instrumental: false,
    lyrics_optimizer: false,
    output_format: 'hex',
    audio_setting: {
      sample_rate: 44100,
      bitrate: 256000,
      format: 'mp3',
    },
    aigc_watermark: false,
  };

  async function handleAiGenerate() {
    if (!aiDescription.trim()) return;
    aiLoading = true;
    aiError = null;
    try {
      const result = await invoke<{ title: string; prompt: string; lyrics: string }>('generate_music_ideas', {
        params: { description: aiDescription }
      });
      formData.title = result.title;
      formData.prompt = result.prompt;
      formData.lyrics = result.lyrics;
    } catch (e: any) {
      aiError = e.toString();
    } finally {
      aiLoading = false;
    }
  }
</script>

<div class="music-form">
  <div class="form-section ai-section">
    <h3>{$t.musicForm.aiAssistant}</h3>
    <p class="section-hint">{$t.musicForm.aiHint}</p>
    <textarea
      bind:value={aiDescription}
      placeholder={$t.musicForm.aiPlaceholder}
      rows="3"
      {disabled}
      class="ai-input"
    />
    <button class="secondary ai-btn" on:click={handleAiGenerate} disabled={disabled || aiLoading || !aiDescription.trim()}>
      {#if aiLoading}
        {$t.musicForm.aiGenerating}
      {:else}
        {$t.musicForm.aiGenerate}
      {/if}
    </button>
    {#if aiError}
      <p class="ai-error">{aiError}</p>
    {/if}
  </div>

  <div class="form-section">
    <h3>{$t.musicForm.model}</h3>
    <select bind:value={formData.model} {disabled}>
      <option value="music-2.6">{$t.musicForm.modelToken}</option>
      <option value="music-2.6-free">{$t.musicForm.modelFree}</option>
    </select>
  </div>

  <div class="form-section">
    <h3>{$t.musicForm.title}</h3>
    <input
      type="text"
      bind:value={formData.title}
      placeholder={$t.musicForm.titlePlaceholder}
      {disabled}
    />
  </div>

  <div class="form-section">
    <h3>{$t.musicForm.musicPrompt}</h3>
    <textarea
      bind:value={formData.prompt}
      placeholder={$t.musicForm.promptPlaceholder}
      rows="3"
      maxlength="2000"
      {disabled}
    />
    <span class="char-count">{formData.prompt.length}/2000</span>
  </div>

  <div class="form-section">
    <h3>{$t.musicForm.lyrics}</h3>
    <textarea
      bind:value={formData.lyrics}
      placeholder={$t.musicForm.lyricsPlaceholder}
      rows="8"
      maxlength="3500"
      {disabled}
    />
    <span class="char-count">{formData.lyrics.length}/3500</span>
  </div>

  <div class="form-row">
    <label class="checkbox-label">
      <input type="checkbox" bind:checked={formData.is_instrumental} {disabled} />
      {$t.musicForm.instrumental}
    </label>
    <label class="checkbox-label">
      <input type="checkbox" bind:checked={formData.lyrics_optimizer} {disabled} />
      {$t.musicForm.autoLyrics}
    </label>
  </div>

  <details class="advanced-section">
    <summary>{$t.musicForm.audioSettings}</summary>
    <div class="advanced-content">
      <div class="audio-settings">
        <div class="setting-field">
          <label>{$t.musicForm.sampleRate}</label>
          <select bind:value={formData.audio_setting.sample_rate} {disabled}>
            <option value={16000}>16000 Hz</option>
            <option value={24000}>24000 Hz</option>
            <option value={32000}>32000 Hz</option>
            <option value={44100}>44100 Hz</option>
          </select>
        </div>
        <div class="setting-field">
          <label>{$t.musicForm.bitrate}</label>
          <select bind:value={formData.audio_setting.bitrate} {disabled}>
            <option value={32000}>32 kbps</option>
            <option value={64000}>64 kbps</option>
            <option value={128000}>128 kbps</option>
            <option value={256000}>256 kbps</option>
          </select>
        </div>
        <div class="setting-field">
          <label>{$t.musicForm.format}</label>
          <select bind:value={formData.audio_setting.format} {disabled}>
            <option value="mp3">MP3</option>
            <option value="wav">WAV</option>
            <option value="pcm">PCM</option>
          </select>
        </div>
      </div>

      <div class="form-section" style="margin-top: 12px;">
        <h3>{$t.musicForm.outputFormat}</h3>
        <div class="form-row">
          <label class="radio-label">
            <input type="radio" bind:group={formData.output_format} value="hex" disabled={disabled} />
            {$t.musicForm.hex}
          </label>
          <label class="radio-label">
            <input type="radio" bind:group={formData.output_format} value="url" disabled={disabled} />
            {$t.musicForm.url}
          </label>
        </div>
      </div>

      <label class="checkbox-label full-width" style="margin-top: 8px;">
        <input type="checkbox" bind:checked={formData.aigc_watermark} {disabled} />
        {$t.musicForm.aigcWatermark}
      </label>
    </div>
  </details>

  <button class="primary generate-btn" on:click={() => onSubmit(formData)} disabled={disabled || loading}>
    {#if loading}
      {$t.musicForm.generating}
    {:else}
      {$t.musicForm.generateMusic}
    {/if}
  </button>
</div>

<style>
  .music-form {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 20px;
    background: var(--color-surface);
    border-radius: 12px;
    border: 1px solid var(--color-border);
  }

  .form-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .form-section h3 {
    font-size: 13px;
    font-weight: 600;
    color: var(--color-text);
  }

  .section-hint {
    font-size: 11px;
    color: var(--color-text-light);
    margin: 0;
  }

  .ai-section {
    background: rgba(99, 102, 241, 0.08);
    padding: 14px;
    border-radius: 10px;
    border: 1px solid rgba(99, 102, 241, 0.2);
  }

  .ai-input {
    border-color: rgba(99, 102, 241, 0.3) !important;
  }

  .ai-input:focus {
    border-color: var(--color-primary) !important;
    box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.15);
  }

  .ai-btn {
    width: 100%;
    margin-top: 4px;
    background: var(--color-primary);
    color: white;
    border: none;
  }

  .ai-btn:hover:not(:disabled) {
    opacity: 0.9;
  }

  .ai-btn:disabled {
    opacity: 0.5;
  }

  .ai-error {
    font-size: 12px;
    color: #ef4444;
    margin: 4px 0 0;
  }

  .char-count {
    font-size: 11px;
    color: var(--color-text-light);
    text-align: right;
  }

  .form-row {
    display: flex;
    gap: 16px;
    flex-wrap: wrap;
  }

  .checkbox-label, .radio-label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    color: var(--color-text);
    cursor: pointer;
  }

  .full-width {
    width: 100%;
  }

  .advanced-section {
    border: 1px solid var(--color-border);
    border-radius: 8px;
    padding: 0;
  }

  .advanced-section summary {
    padding: 10px 14px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    color: var(--color-text-light);
    list-style: none;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .advanced-section summary::before {
    content: '\25B6';
    font-size: 10px;
    transition: transform 0.2s;
  }

  .advanced-section[open] summary::before {
    transform: rotate(90deg);
  }

  .advanced-content {
    padding: 0 14px 14px;
  }

  .audio-settings {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
  }

  .setting-field {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .generate-btn {
    width: 100%;
    padding: 12px;
    font-size: 15px;
    margin-top: 8px;
  }

  textarea {
    resize: vertical;
    min-height: 80px;
    font-family: inherit;
    line-height: 1.6;
  }
</style>
