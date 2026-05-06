<script lang="ts">
  import { t } from '$lib/i18n';

  export let music: {
    id: number;
    title: string;
    tags: string;
    notes: string | null;
  };

  export let onClose: () => void;
  export let onSave: (updated: { id: number; title: string; tags: string; notes: string }) => void;

  let title = music.title;
  let tagsInput = music.tags;
  let notes = music.notes || '';

  let tagList: string[] = [];

  try {
    tagList = JSON.parse(music.tags);
  } catch {}

  let newTag = '';

  function addTag() {
    const t = newTag.trim();
    if (t && !tagList.includes(t)) {
      tagList = [...tagList, t];
      tagsInput = JSON.stringify(tagList);
    }
    newTag = '';
  }

  function removeTag(tag: string) {
    tagList = tagList.filter(t => t !== tag);
    tagsInput = JSON.stringify(tagList);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault();
      addTag();
    }
  }

  function handleSave() {
    onSave({
      id: music.id,
      title,
      tags: tagsInput,
      notes,
    });
  }
</script>

<div class="modal-overlay" on:click={onClose} role="dialog" aria-modal="true">
  <div class="modal" on:click|stopPropagation role="document">
    <div class="modal-header">
      <h2>{$t.metadataModal.editMetadata}</h2>
      <button class="close-btn" on:click={onClose}>×</button>
    </div>

    <div class="modal-body">
      <div class="field">
        <label>{$t.metadataModal.title}</label>
        <input type="text" bind:value={title} placeholder={$t.metadataModal.titlePlaceholder} />
      </div>

      <div class="field">
        <label>{$t.metadataModal.tags}</label>
        <div class="tags-input-area">
          <div class="tags-list">
            {#each tagList as tag}
              <span class="tag-item">
                {tag}
                <button on:click={() => removeTag(tag)}>×</button>
              </span>
            {/each}
          </div>
          <div class="tag-input-row">
            <input
              type="text"
              bind:value={newTag}
              placeholder="Add tag..."
              on:keydown={handleKeydown}
            />
            <button class="add-tag-btn" on:click={addTag}>{$t.metadataModal.addTag}</button>
          </div>
        </div>
      </div>

      <div class="field">
        <label>{$t.metadataModal.notes}</label>
        <textarea bind:value={notes} placeholder={$t.metadataModal.notesPlaceholder} rows="3"></textarea>
      </div>
    </div>

    <div class="modal-footer">
      <button class="secondary" on:click={onClose}>{$t.metadataModal.cancel}</button>
      <button class="primary" on:click={handleSave}>{$t.metadataModal.save}</button>
    </div>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(4px);
  }

  .modal {
    background: var(--color-surface);
    border-radius: 14px;
    border: 1px solid var(--color-border);
    width: 420px;
    max-width: 90vw;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid var(--color-border);
  }

  .modal-header h2 {
    font-size: 16px;
    font-weight: 600;
  }

  .close-btn {
    width: 28px;
    height: 28px;
    border-radius: 6px;
    background: none;
    color: var(--color-text-light);
    font-size: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
  }

  .modal-body {
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    overflow-y: auto;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .tags-input-area {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .tags-list {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .tag-item {
    display: flex;
    align-items: center;
    gap: 4px;
    background: rgba(99, 102, 241, 0.2);
    color: var(--color-primary);
    padding: 4px 8px;
    border-radius: 12px;
    font-size: 12px;
  }

  .tag-item button {
    width: 16px;
    height: 16px;
    padding: 0;
    background: none;
    color: inherit;
    font-size: 14px;
    line-height: 1;
    opacity: 0.7;
  }

  .tag-item button:hover {
    opacity: 1;
  }

  .tag-input-row {
    display: flex;
    gap: 8px;
  }

  .tag-input-row input {
    flex: 1;
  }

  .add-tag-btn {
    padding: 8px 14px;
    white-space: nowrap;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    padding: 14px 20px;
    border-top: 1px solid var(--color-border);
  }

  textarea {
    resize: vertical;
    font-family: inherit;
  }
</style>