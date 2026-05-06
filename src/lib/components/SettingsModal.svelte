<script lang="ts">
  export let show = false;
  export let autoCover = false;
  export let onClose: () => void;
  export let onSave: (settings: { autoCover: boolean }) => void;

  let localAutoCover = autoCover;

  function handleSave() {
    onSave({ autoCover: localAutoCover });
    onClose();
  }
</script>

<div class="modal-overlay" on:click={onClose} role="dialog" aria-modal="true">
  <div class="modal" on:click|stopPropagation role="document">
    <div class="modal-header">
      <h2>Settings</h2>
      <button class="close-btn" on:click={onClose}>×</button>
    </div>

    <div class="modal-body">
      <div class="setting-item">
        <div class="setting-info">
          <h3>Auto-generate Cover</h3>
          <p>Automatically generate a cover image after music generation completes</p>
        </div>
        <label class="toggle">
          <input type="checkbox" bind:checked={localAutoCover} />
          <span class="toggle-slider"></span>
        </label>
      </div>

      <div class="info-box">
        <h4>About API</h4>
        <p>This app uses MiniMax API for music and image generation. Your API key is read from the <code>MINIMAX_API_KEY</code> environment variable.</p>
      </div>
    </div>

    <div class="modal-footer">
      <button class="secondary" on:click={onClose}>Cancel</button>
      <button class="primary" on:click={handleSave}>Save</button>
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
    gap: 20px;
    overflow-y: auto;
  }

  .setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
  }

  .setting-info h3 {
    font-size: 14px;
    font-weight: 600;
    margin-bottom: 4px;
  }

  .setting-info p {
    font-size: 12px;
    color: var(--color-text-light);
  }

  .toggle {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
    cursor: pointer;
  }

  .toggle input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .toggle-slider {
    position: absolute;
    inset: 0;
    background: var(--color-border);
    border-radius: 12px;
    transition: 0.2s;
  }

  .toggle-slider::before {
    content: '';
    position: absolute;
    left: 2px;
    top: 2px;
    width: 20px;
    height: 20px;
    background: white;
    border-radius: 50%;
    transition: 0.2s;
  }

  .toggle input:checked + .toggle-slider {
    background: var(--color-primary);
  }

  .toggle input:checked + .toggle-slider::before {
    transform: translateX(20px);
  }

  .info-box {
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    padding: 14px;
  }

  .info-box h4 {
    font-size: 12px;
    font-weight: 600;
    margin-bottom: 6px;
  }

  .info-box p {
    font-size: 12px;
    color: var(--color-text-light);
    line-height: 1.5;
  }

  .info-box code {
    background: var(--color-border);
    padding: 1px 5px;
    border-radius: 3px;
    font-size: 11px;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    padding: 14px 20px;
    border-top: 1px solid var(--color-border);
  }
</style>