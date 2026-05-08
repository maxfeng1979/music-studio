<script lang="ts">
  import { t } from '$lib/i18n';

  export let music: {
    title: string;
    prompt: string;
    lyrics: string | null;
    is_instrumental: boolean;
    ai_description: string | null;
  };

  export let onClose: () => void;

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') onClose();
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="modal-overlay" on:click={onClose} role="dialog" aria-modal="true">
  <div class="modal" on:click|stopPropagation role="document">
    <div class="modal-header">
      <h2>{music.title} — {$t.detailModal.title}</h2>
      <button class="close-btn" on:click={onClose}>×</button>
    </div>

    <div class="modal-body">
      {#if music.ai_description}
        <div class="field">
          <label>{$t.detailModal.aiDescription}</label>
          <div class="readonly-content">
            {music.ai_description}
          </div>
        </div>
      {/if}

      <div class="field">
        <label>{$t.detailModal.musicPrompt}</label>
        <div class="readonly-content">
          {music.prompt || $t.detailModal.noPrompt}
        </div>
      </div>

      <div class="field">
        <label>{$t.detailModal.lyrics}</label>
        {#if music.is_instrumental}
          <div class="readonly-content placeholder">{$t.detailModal.noLyrics}</div>
        {:else}
          <pre class="readonly-content lyrics">{music.lyrics || ''}</pre>
        {/if}
      </div>
    </div>

    <div class="modal-footer">
      <button class="secondary" on:click={onClose}>{$t.detailModal.close}</button>
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
    width: 480px;
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

  .field label {
    font-size: 13px;
    font-weight: 600;
    color: var(--color-text);
  }

  .readonly-content {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    padding: 12px;
    font-size: 13px;
    line-height: 1.6;
    color: var(--color-text);
    white-space: pre-wrap;
    word-break: break-word;
  }

  .readonly-content.placeholder {
    color: var(--color-text-light);
    font-style: italic;
  }

  .readonly-content.lyrics {
    max-height: 300px;
    overflow-y: auto;
    font-family: inherit;
    margin: 0;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    padding: 14px 20px;
    border-top: 1px solid var(--color-border);
  }
</style>
