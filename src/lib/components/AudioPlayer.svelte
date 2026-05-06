<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { activeAudioSrc, playAudio } from '$lib/stores/audioStore';
  import { t } from '$lib/i18n';

  export let src: string;
  export let compact = false;

  let audio: HTMLAudioElement | undefined = undefined;
  let playing = false;
  let currentTime = 0;
  let duration = 0;
  let blobUrl = '';
  let audioLoaded = false;
  let loading = false;
  let errorMsg = '';

  async function loadAudio(filePath: string) {
    if (blobUrl) {
      URL.revokeObjectURL(blobUrl);
      blobUrl = '';
    }
    audioLoaded = false;
    loading = true;
    errorMsg = '';

    try {
      const result = await invoke<{ data: string; mime_type: string }>('read_file_as_data_url', { path: filePath });
      const binaryStr = atob(result.data);
      const bytes = new Uint8Array(binaryStr.length);
      for (let i = 0; i < binaryStr.length; i++) {
        bytes[i] = binaryStr.charCodeAt(i);
      }
      const blob = new Blob([bytes], { type: result.mime_type });
      blobUrl = URL.createObjectURL(blob);
    } catch (e) {
      console.error('Failed to load audio:', e);
      errorMsg = String(e);
      loading = false;
    }
  }

  $: if (src) {
    loadAudio(src);
  }

  // Stop this player when another one starts
  let unsubscribe: (() => void) | undefined;
  onMount(() => {
    unsubscribe = activeAudioSrc.subscribe((activeSrc) => {
      if (activeSrc !== src && audio && playing) {
        audio.pause();
        playing = false;
      }
    });
  });

  function togglePlay() {
    if (!audio) return;
    if (playing) {
      audio.pause();
    } else {
      playAudio(src);
      audio.play().catch((e) => {
        console.error('Play failed:', e);
      });
    }
  }

  function handleTimeUpdate() {
    if (audio) currentTime = audio.currentTime;
  }

  function handleLoadedMetadata() {
    if (audio) {
      duration = audio.duration;
      audioLoaded = true;
      loading = false;
    }
  }

  function handleEnded() {
    playing = false;
    currentTime = 0;
    activeAudioSrc.set(null);
  }

  function handlePlay() {
    playing = true;
    playAudio(src);
  }

  function handlePause() {
    playing = false;
  }

  function seek(e: MouseEvent) {
    if (!audio || !duration) return;
    const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
    const ratio = (e.clientX - rect.left) / rect.width;
    audio.currentTime = ratio * duration;
  }

  function formatTime(s: number): string {
    const m = Math.floor(s / 60);
    const sec = Math.floor(s % 60);
    return `${m}:${sec.toString().padStart(2, '0')}`;
  }

  onDestroy(() => {
    if (audio) {
      audio.pause();
    }
    if (blobUrl) {
      URL.revokeObjectURL(blobUrl);
    }
    if (unsubscribe) {
      unsubscribe();
    }
  });
</script>

<div class="audio-player" class:compact>
  <audio
    bind:this={audio}
    src={blobUrl}
    on:timeupdate={handleTimeUpdate}
    on:loadedmetadata={handleLoadedMetadata}
    on:ended={handleEnded}
    on:play={handlePlay}
    on:pause={handlePause}
  />

  <button class="play-btn" on:click={togglePlay} disabled={loading || errorMsg}>
    {#if loading}
      ...
    {:else if errorMsg}
      !
    {:else}
      {playing ? '⏸' : '▶'}
    {/if}
  </button>

  <div class="progress-container" on:click={seek} role="slider" tabindex="0" aria-label={$t.audioPlayer.seek}>
    <div class="progress-bar" style="width: {(currentTime / duration) * 100}%"></div>
  </div>
  <span class="time">{formatTime(currentTime)} / {formatTime(duration)}</span>
</div>

<style>
  .audio-player {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 14px;
    background: var(--color-surface);
    border-radius: 8px;
    border: 1px solid var(--color-border);
  }

  .audio-player.compact {
    padding: 6px 10px;
    gap: 8px;
  }

  .play-btn {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: var(--color-primary);
    color: white;
    font-size: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    flex-shrink: 0;
  }

  .play-btn:disabled {
    opacity: 0.5;
  }

  .compact .play-btn {
    width: 28px;
    height: 28px;
    font-size: 11px;
  }

  .progress-container {
    flex: 1;
    height: 6px;
    background: var(--color-border);
    border-radius: 3px;
    cursor: pointer;
    overflow: hidden;
    min-width: 60px;
  }

  .progress-bar {
    height: 100%;
    background: var(--color-primary);
    border-radius: 3px;
    transition: width 0.1s linear;
  }

  .time {
    font-size: 12px;
    color: var(--color-text-light);
    white-space: nowrap;
    min-width: 85px;
  }

  .compact .time {
    font-size: 11px;
    min-width: 70px;
  }
</style>
