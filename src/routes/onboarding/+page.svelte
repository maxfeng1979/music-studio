<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { invoke } from '@tauri-apps/api/core';
  import { t } from '$lib/i18n';

  let step: 1 | 2 | 3 = 1;
  let showKey = false;
  let apiKey = '';
  let testing = false;
  let testResult: { ok: boolean; msg: string } | null = null;

  onMount(async () => {
    // Check if API key is already configured
    try {
      const configured: boolean = await invoke('get_api_key_status');
      if (configured) {
        goto('/generator');
        return;
      }
    } catch (e) {
      console.error('Failed to check API key status:', e);
    }
  });

  async function handleTestConnection() {
    if (!apiKey.trim()) return;
    testing = true;
    testResult = null;
    try {
      const msg = await invoke<string>('test_api_connection', { key: apiKey });
      testResult = { ok: true, msg };
    } catch (e: any) {
      testResult = { ok: false, msg: e.toString() };
    } finally {
      testing = false;
    }
  }

  async function handleNext() {
    try {
      await invoke('save_api_key', { key: apiKey });
      step = 3;
    } catch (e: any) {
      testResult = { ok: false, msg: e.toString() };
    }
  }

  function handleGetStarted() {
    step = 2;
  }

  function handleStartCreating() {
    goto('/generator');
  }
</script>

<div class="onboarding-page">
  <div class="card">
    {#if step === 1}
      <!-- Step 1: Welcome -->
      <div class="step-content">
        <div class="logo-area">
          <span class="logo-emoji">🎵</span>
          <h1 class="app-title">Music Studio</h1>
        </div>
        <h2>{$t.onboarding.welcomeTitle}</h2>
        <p class="subtitle">{$t.onboarding.welcomeSubtitle}</p>
        <button class="primary" on:click={handleGetStarted}>
          {$t.onboarding.getStarted}
        </button>
      </div>
    {:else if step === 2}
      <!-- Step 2: API Key Configuration -->
      <div class="step-content">
        <h2>{$t.onboarding.connectTitle}</h2>
        <p class="description">{$t.onboarding.connectDescription}</p>

        <a
          href="https://platform.minimaxi.com/docs/faq/about-apis"
          target="_blank"
          rel="noopener noreferrer"
          class="help-link"
        >
          {$t.onboarding.howToGetKey}
        </a>

        <div class="input-group">
          <label for="api-key">{$t.onboarding.enterApiKey}</label>
          <div class="input-row">
            <input
              id="api-key"
              type={showKey ? 'text' : 'password'}
              placeholder={$t.onboarding.apiKeyPlaceholder}
              bind:value={apiKey}
            />
            <button
              type="button"
              class="toggle-visibility"
              on:click={() => (showKey = !showKey)}
              aria-label={showKey ? 'Hide API key' : 'Show API key'}
            >
              {showKey ? '🙈' : '👁'}
            </button>
          </div>
        </div>

        <button
          class="secondary"
          on:click={handleTestConnection}
          disabled={testing || !apiKey.trim()}
        >
          {testing ? $t.onboarding.testing : $t.onboarding.testConnection}
        </button>

        {#if testResult}
          <div class="result-box" class:success={testResult.ok} class:error={!testResult.ok}>
            <span class="result-icon">{testResult.ok ? '✅' : '❌'}</span>
            <span class="result-text">
              {testResult.ok ? $t.onboarding.connectionSuccess : `${$t.onboarding.connectionFailed}: ${testResult.msg}`}
            </span>
          </div>
        {/if}

        <button
          class="primary"
          on:click={handleNext}
          disabled={!apiKey.trim()}
        >
          {$t.onboarding.next}
        </button>
      </div>
    {:else if step === 3}
      <!-- Step 3: Done -->
      <div class="step-content">
        <div class="success-emoji">✅</div>
        <h2>{$t.onboarding.readyTitle}</h2>
        <p class="subtitle">{$t.onboarding.readySubtitle}</p>
        <p class="hint">{$t.onboarding.readyHint}</p>
        <button class="primary" on:click={handleStartCreating}>
          {$t.onboarding.startCreating}
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  .onboarding-page {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 20px;
    background: linear-gradient(135deg, var(--color-bg) 0%, #1a1a2e 100%);
  }

  .card {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: 16px;
    padding: 40px;
    width: 100%;
    max-width: 420px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  }

  .step-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;
    text-align: center;
  }

  .logo-area {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 8px;
  }

  .logo-emoji {
    font-size: 48px;
    line-height: 1;
  }

  .app-title {
    font-size: 28px;
    font-weight: 700;
    color: var(--color-text);
    margin: 0;
  }

  h2 {
    font-size: 22px;
    font-weight: 600;
    color: var(--color-text);
    margin: 0;
  }

  .subtitle {
    font-size: 14px;
    color: var(--color-text-light);
    margin: 0;
    line-height: 1.5;
  }

  .description {
    font-size: 14px;
    color: var(--color-text-light);
    margin: 0;
    line-height: 1.6;
    text-align: left;
  }

  .hint {
    font-size: 12px;
    color: var(--color-text-light);
    margin: 0;
    opacity: 0.8;
  }

  .help-link {
    font-size: 13px;
    color: var(--color-primary);
    text-decoration: none;
    align-self: flex-start;
  }

  .help-link:hover {
    text-decoration: underline;
  }

  .input-group {
    width: 100%;
    text-align: left;
  }

  .input-group label {
    font-size: 12px;
    color: var(--color-text-light);
    margin-bottom: 6px;
    display: block;
  }

  .input-row {
    display: flex;
    gap: 8px;
  }

  .input-row input {
    flex: 1;
  }

  .toggle-visibility {
    padding: 8px 12px;
    background: var(--color-border);
    color: var(--color-text);
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 16px;
  }

  .toggle-visibility:hover {
    background: var(--color-primary);
  }

  .result-box {
    width: 100%;
    padding: 12px;
    border-radius: 8px;
    display: flex;
    align-items: flex-start;
    gap: 8px;
    text-align: left;
  }

  .result-box.success {
    background: rgba(34, 197, 94, 0.1);
    border: 1px solid var(--color-success);
  }

  .result-box.error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid #ef4444;
  }

  .result-icon {
    flex-shrink: 0;
    font-size: 16px;
  }

  .result-text {
    font-size: 13px;
    color: var(--color-text);
    word-break: break-word;
  }

  .success-emoji {
    font-size: 64px;
    line-height: 1;
  }

  button.primary {
    width: 100%;
    padding: 12px 24px;
    background: var(--color-primary);
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s;
  }

  button.primary:hover {
    background: var(--color-primary-hover);
  }

  button.primary:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  button.secondary {
    width: 100%;
    padding: 10px 24px;
    background: var(--color-surface);
    color: var(--color-text);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s;
  }

  button.secondary:hover {
    background: var(--color-border);
  }

  button.secondary:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
</style>
