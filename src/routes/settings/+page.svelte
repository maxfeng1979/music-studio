<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import { t, locale, setLocale } from '$lib/i18n';

  let autoCoverEnabled = false;
  let showSettingsModal = false;

  // API Key state
  let apiKeyInput = '';
  let showApiKey = false;
  let apiKeyStatus: { configured: boolean } | null = null;
  let testStatus: { testing: boolean; result: { ok: boolean; msg: string } | null } = { testing: false, result: null };
  let saveStatus: { saving: boolean; msg: string | null } = { saving: false, msg: null };

  // Data path state
  let currentDataPath = '';
  let newPath = '';
  let pathLoading = false;
  let migrating = false;
  let pathError: string | null = null;
  let pathSuccess: string | null = null;

  // Language
  let selectedLocale = 'zh';

  onMount(async () => {
    // Load settings
    const saved = localStorage.getItem('music-studio-settings');
    if (saved) {
      try {
        const settings = JSON.parse(saved);
        autoCoverEnabled = settings.autoCover ?? false;
      } catch {}
    }

    // Load API key status
    try {
      const configured: boolean = await invoke('get_api_key_status');
      apiKeyStatus = { configured };
    } catch {}

    // Load data path
    await loadDataPath();

    // Load locale
    const savedLocale = localStorage.getItem('music-studio-locale');
    if (savedLocale) selectedLocale = savedLocale;
  });

  function handleToggleAutoCover() {
    localStorage.setItem('music-studio-settings', JSON.stringify({
      autoCover: autoCoverEnabled,
    }));
  }

  async function handleBrowse() {
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === 'string') {
      newPath = selected;
      pathError = null;
      pathSuccess = null;
    }
  }

  async function handleApplyPath() {
    if (!newPath) return;
    migrating = true;
    pathError = null;
    pathSuccess = null;
    try {
      const result = await invoke<string>('set_data_path', { newPath });
      currentDataPath = result;
      newPath = '';
      pathSuccess = 'Data migrated successfully!';
    } catch (e: any) {
      pathError = e.toString();
    } finally {
      migrating = false;
    }
  }

  async function loadDataPath() {
    pathLoading = true;
    try {
      currentDataPath = await invoke<string>('get_data_path');
    } catch (e: any) {
      pathError = e.toString();
    } finally {
      pathLoading = false;
    }
  }

  async function handleTestConnection() {
    if (!apiKeyInput.trim()) return;
    testStatus = { testing: true, result: null };
    try {
      const msg: string = await invoke('test_api_connection', { key: apiKeyInput.trim() });
      testStatus = { testing: false, result: { ok: true, msg } };
    } catch (e: any) {
      testStatus = { testing: false, result: { ok: false, msg: e.toString() } };
    }
  }

  async function handleSaveApiKey() {
    if (!apiKeyInput.trim()) return;
    saveStatus = { saving: true, msg: null };
    try {
      await invoke('save_api_key', { key: apiKeyInput.trim() });
      apiKeyStatus = { configured: true };
      saveStatus = { saving: false, msg: '保存成功！' };
    } catch (e: any) {
      saveStatus = { saving: false, msg: e.toString() };
    }
  }

  function handleLocaleChange(e: Event) {
    const val = (e.target as HTMLSelectElement).value;
    selectedLocale = val;
    setLocale(val);
  }
</script>

<div class="settings-page">
  <div class="settings-header">
    <h1>{$t.settings.title}</h1>
  </div>

  <div class="settings-content">
    <!-- Section 1: API Configuration -->
    <div class="settings-section">
      <h2>{$t.settings.apiConfig}</h2>

      <div class="api-status-row">
        <span class="status-dot" class:configured={apiKeyStatus?.configured}></span>
        <span class="status-text">
          {apiKeyStatus?.configured ? $t.settings.connected : $t.settings.notConfigured}
        </span>
      </div>

      <div class="api-key-input-row">
        <input
          type={showApiKey ? 'text' : 'password'}
          bind:value={apiKeyInput}
          placeholder={$t.settings.apiKeyPlaceholder}
        />
        <button class="toggle-visibility-btn" on:click={() => showApiKey = !showApiKey}>
          {showApiKey ? '🙈' : '👁'}
        </button>
      </div>

      <div class="api-actions-row">
        <button
          class="secondary"
          on:click={handleTestConnection}
          disabled={!apiKeyInput.trim() || testStatus.testing}
        >
          {$t.settings.testConnection}
        </button>
        <button
          class="primary"
          on:click={handleSaveApiKey}
          disabled={!apiKeyInput.trim() || saveStatus.saving}
        >
          {$t.settings.save}
        </button>
      </div>

      {#if testStatus.result}
        <div class="test-result" class:success={testStatus.result.ok} class:error={!testStatus.result.ok}>
          {testStatus.result.msg}
        </div>
      {/if}

      {#if saveStatus.msg}
        <div class="save-result" class:success={saveStatus.msg.includes('成功')}>
          {saveStatus.msg}
        </div>
      {/if}
    </div>

    <!-- Section 2: Music Generation -->
    <div class="settings-section">
      <h2>{$t.settings.musicGeneration}</h2>

      <div class="setting-item">
        <div class="setting-info">
          <h3>{$t.settings.autoCover}</h3>
          <p>{$t.settings.autoCoverHint}</p>
        </div>
        <label class="toggle">
          <input type="checkbox" bind:checked={autoCoverEnabled} on:change={handleToggleAutoCover} />
          <span class="toggle-slider"></span>
        </label>
      </div>
    </div>

    <!-- Section 3: Language -->
    <div class="settings-section">
      <h2>{$t.settings.language}</h2>

      <div class="setting-item">
        <select bind:value={selectedLocale} on:change={handleLocaleChange}>
          <option value="zh">中文</option>
          <option value="en">English</option>
        </select>
      </div>
    </div>

    <!-- Section 4: Data Storage -->
    <div class="settings-section">
      <h2>{$t.settings.dataStorage}</h2>

      <div class="setting-item vertical">
        <div class="setting-info">
          <h3>{$t.settings.storagePath}</h3>
          <p>{$t.settings.currentLocation}</p>
        </div>
        <div class="path-display">
          {#if pathLoading}
            <span class="path-text">Loading...</span>
          {:else}
            <span class="path-text">{currentDataPath}</span>
          {/if}
        </div>

        <div class="path-actions">
          <div class="path-input-row">
            <input
              type="text"
              class="path-input"
              placeholder={$t.settings.selectNewFolder}
              bind:value={newPath}
              readonly
            />
            <button class="browse-btn" on:click={handleBrowse}>Browse</button>
          </div>
          <button
            class="apply-btn"
            on:click={handleApplyPath}
            disabled={!newPath || migrating}
          >
            {#if migrating}
              Migrating...
            {:else}
              {$t.settings.apply}
            {/if}
          </button>
        </div>

        {#if pathError}
          <p class="path-error">{pathError}</p>
        {/if}
        {#if pathSuccess}
          <p class="path-success">{pathSuccess}</p>
        {/if}
      </div>
    </div>

    <!-- Section 5: About -->
    <div class="settings-section">
      <h2>{$t.settings.about}</h2>
      <div class="about-info">
        <p><strong>Music Studio</strong> {$t.settings.version}: v0.1.0</p>
        <p>{$t.settings.desktopMusicTool}</p>
      </div>
    </div>
  </div>
</div>

<style>
  .settings-page {
    padding: 20px;
    height: calc(100vh - 52px);
    overflow-y: auto;
  }

  .settings-header {
    margin-bottom: 24px;
  }

  .settings-header h1 {
    font-size: 22px;
    font-weight: 700;
  }

  .settings-content {
    display: flex;
    flex-direction: column;
    gap: 24px;
    max-width: 600px;
  }

  .settings-section {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: 12px;
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .settings-section h2 {
    font-size: 14px;
    font-weight: 600;
    color: var(--color-text-light);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
  }

  .setting-item.vertical {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
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
    flex-shrink: 0;
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

  .path-display {
    width: 100%;
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    padding: 8px 12px;
  }

  .path-text {
    font-size: 12px;
    color: var(--color-text-light);
    font-family: monospace;
    word-break: break-all;
  }

  .path-actions {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .path-input-row {
    display: flex;
    gap: 8px;
  }

  .path-input {
    flex: 1;
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    padding: 8px 12px;
    font-size: 12px;
    color: var(--color-text);
    font-family: monospace;
    cursor: default;
  }

  .path-input::placeholder {
    color: var(--color-text-light);
    opacity: 0.5;
  }

  .browse-btn {
    padding: 8px 16px;
    border-radius: 6px;
    border: 1px solid var(--color-border);
    background: var(--color-border);
    color: var(--color-text);
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    white-space: nowrap;
  }

  .browse-btn:hover {
    background: var(--color-primary);
    color: white;
    border-color: var(--color-primary);
  }

  .apply-btn {
    align-self: flex-end;
    padding: 8px 24px;
    border-radius: 6px;
    border: none;
    background: var(--color-primary);
    color: white;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    opacity: 1;
  }

  .apply-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .apply-btn:hover:not(:disabled) {
    opacity: 0.9;
  }

  .path-error {
    font-size: 12px;
    color: #ef4444;
    margin: 0;
  }

  .path-success {
    font-size: 12px;
    color: #22c55e;
    margin: 0;
  }

  .about-info {
    display: flex;
    flex-direction: column;
    gap: 8px;
    font-size: 13px;
    color: var(--color-text);
  }

  /* API Configuration styles */
  .api-status-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #ef4444;
  }

  .status-dot.configured {
    background: #22c55e;
  }

  .status-text {
    font-size: 13px;
    color: var(--color-text);
  }

  .api-key-input-row {
    display: flex;
    gap: 8px;
  }

  .api-key-input-row input {
    flex: 1;
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    padding: 8px 12px;
    font-size: 12px;
    color: var(--color-text);
  }

  .api-key-input-row input::placeholder {
    color: var(--color-text-light);
    opacity: 0.5;
  }

  .toggle-visibility-btn {
    padding: 8px 12px;
    border-radius: 6px;
    border: 1px solid var(--color-border);
    background: var(--color-border);
    font-size: 14px;
    cursor: pointer;
  }

  .api-actions-row {
    display: flex;
    gap: 8px;
  }

  .api-actions-row button {
    padding: 8px 16px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
  }

  .api-actions-row button.secondary {
    border: 1px solid var(--color-border);
    background: var(--color-border);
    color: var(--color-text);
  }

  .api-actions-row button.secondary:hover:not(:disabled) {
    background: var(--color-text);
    color: var(--color-bg);
  }

  .api-actions-row button.primary {
    border: none;
    background: var(--color-primary);
    color: white;
  }

  .api-actions-row button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .test-result, .save-result {
    padding: 8px 12px;
    border-radius: 6px;
    font-size: 12px;
  }

  .test-result.success, .save-result.success {
    background: rgba(34, 197, 94, 0.1);
    color: #22c55e;
  }

  .test-result.error {
    background: rgba(239, 68, 68, 0.1);
    color: #ef4444;
  }

  .settings-section select {
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    padding: 8px 12px;
    font-size: 12px;
    color: var(--color-text);
    cursor: pointer;
  }
</style>
