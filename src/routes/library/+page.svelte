<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import MusicCard from '$lib/components/MusicCard.svelte';
  import MetadataModal from '$lib/components/MetadataModal.svelte';
  import { t } from '$lib/i18n';

  let musicList: any[] = [];
  let loading = false;
  let sortBy = 'date';
  let filterTag = '';
  let filterInstrumental: 'all' | 'instrumental' | 'songs' = 'all';
  let showModal = false;
  let selectedMusic: any = null;

  onMount(() => {
    loadMusic();
  });

  async function loadMusic() {
    loading = true;
    try {
      const filterInst = filterInstrumental === 'all' ? null : filterInstrumental === 'instrumental';
      musicList = await invoke('get_all_music', {
        sortBy: sortBy === 'date' ? null : sortBy,
        filterTag: filterTag || null,
        filterInstrumental: filterInst,
      });
    } catch (e: any) {
      console.error('Failed to load music:', e);
    } finally {
      loading = false;
    }
  }

  function handleEdit(music: any) {
    selectedMusic = music;
    showModal = true;
  }

  async function handleDelete(id: number) {
    if (!confirm('确定要删除这首音乐吗？')) return;
    try {
      await invoke('delete_music', { id });
      await loadMusic();
    } catch (e: any) {
      console.error('Delete failed:', e);
    }
  }

  function handleCoverUpdated(id: number, coverPath: string) {
    musicList = musicList.map(m =>
      m.id === id ? { ...m, cover_image_path: coverPath } : m
    );
  }

  async function handleSaveMetadata(updated: any) {
    try {
      await invoke('update_music_metadata', {
        id: updated.id,
        title: updated.title,
        tags: updated.tags,
        notes: updated.notes,
      });
      showModal = false;
      await loadMusic();
    } catch (e: any) {
      console.error('Update failed:', e);
    }
  }

  function handleSortChange(e: Event) {
    sortBy = (e.target as HTMLSelectElement).value;
    loadMusic();
  }
</script>

<div class="library-page">
  <div class="library-header">
    <h1>{$t.library.title}</h1>
    <div class="controls">
      <div class="filter-buttons">
        <button class="filter-btn" class:active={filterInstrumental === 'all'} on:click={() => { filterInstrumental = 'all'; loadMusic(); }}>{$t.library.all}</button>
        <button class="filter-btn" class:active={filterInstrumental === 'songs'} on:click={() => { filterInstrumental = 'songs'; loadMusic(); }}>{$t.library.songs}</button>
        <button class="filter-btn" class:active={filterInstrumental === 'instrumental'} on:click={() => { filterInstrumental = 'instrumental'; loadMusic(); }}>{$t.library.instrumental}</button>
      </div>
      <select bind:value={sortBy} on:change={handleSortChange}>
        <option value="date">{$t.library.sortByDate}</option>
        <option value="title">{$t.library.sortByTitle}</option>
        <option value="duration">{$t.library.sortByDuration}</option>
      </select>
      <input
        type="text"
        placeholder={$t.library.filterByTag}
        bind:value={filterTag}
        on:keydown={(e) => e.key === 'Enter' && loadMusic()}
      />
      <button class="secondary" on:click={loadMusic}>{$t.library.refresh}</button>
    </div>
  </div>

  {#if loading}
    <div class="loading">Loading...</div>
  {:else if musicList.length === 0}
    <div class="empty-state">
      <span class="empty-icon">📁</span>
      <p>{$t.library.noMusic}</p>
      <p>{$t.library.goToGenerator}</p>
    </div>
  {:else}
    <div class="music-grid">
      {#each musicList as music (music.id)}
        <MusicCard
          {music}
          onEdit={handleEdit}
          onDelete={handleDelete}
          onCoverUpdated={handleCoverUpdated}
        />
      {/each}
    </div>
  {/if}
</div>

{#if showModal && selectedMusic}
  <MetadataModal
    music={selectedMusic}
    onClose={() => { showModal = false; selectedMusic = null; }}
    onSave={handleSaveMetadata}
  />
{/if}

<style>
  .library-page {
    padding: 20px;
    height: calc(100vh - 52px);
    overflow-y: auto;
  }

  .library-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
    flex-wrap: wrap;
    gap: 12px;
  }

  .library-header h1 {
    font-size: 22px;
    font-weight: 700;
  }

  .controls {
    display: flex;
    gap: 10px;
    align-items: center;
    flex-wrap: wrap;
  }

  .filter-buttons {
    display: flex;
    gap: 4px;
  }

  .filter-btn {
    padding: 5px 12px;
    font-size: 11px;
    background: var(--color-border);
    color: var(--color-text);
    border-radius: 5px;
  }

  .filter-btn:hover {
    background: var(--color-primary);
    color: white;
  }

  .filter-btn.active {
    background: var(--color-primary);
    color: white;
  }

  .controls select, .controls input {
    width: auto;
  }

  .controls input {
    width: 180px;
  }

  .loading {
    text-align: center;
    padding: 40px;
    color: var(--color-text-light);
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

  .music-grid {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
</style>
