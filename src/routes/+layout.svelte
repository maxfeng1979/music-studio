<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { t } from '$lib/i18n';

  let updateAvailable = false;
  let updateDownloading = false;
  let updateProgress = 0;
  let updateReady = false;
  let updateError = '';
  let pendingUpdate: any = null;

  onMount(async () => {
    let configured = false;
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      configured = await invoke('get_api_key_status');
    } catch (e) {
      console.error('Failed to check API key status:', e);
    }
    const path = $page.url.pathname;
    if (configured) {
      if (path === '/' || path === '') {
        goto('/generator', { replaceState: true });
      }
    } else if (!path.startsWith('/onboarding')) {
      goto('/onboarding', { replaceState: true });
    }

    checkForUpdate();
  });

  async function checkForUpdate() {
    try {
      const { check } = await import('@tauri-apps/plugin-updater');
      const update = await check();
      if (update) {
        pendingUpdate = update;
        updateAvailable = true;
      }
    } catch (e) {
      console.log('Update check skipped:', e);
    }
  }

  async function handleUpdate() {
    if (!pendingUpdate) return;
    updateDownloading = true;
    updateError = '';

    try {
      await pendingUpdate.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            break;
          case 'Progress':
            updateProgress = Math.round((event.data.chunkLength / event.data.contentLength) * 100);
            break;
          case 'Finished':
            updateReady = true;
            break;
        }
      });
    } catch (e) {
      updateError = String(e);
      updateDownloading = false;
    }
  }

  function dismissUpdate() {
    updateAvailable = false;
    pendingUpdate = null;
  }

  $: showNav = $page.url.pathname !== '/onboarding';
</script>

{#if updateAvailable && !updateReady}
  <div class="update-banner">
    <span>发现新版本 {pendingUpdate?.version || ''}</span>
    {#if updateDownloading}
      <span class="update-progress">下载中... {updateProgress}%</span>
    {:else}
      <button class="update-btn" on:click={handleUpdate}>立即更新</button>
      <button class="dismiss-btn" on:click={dismissUpdate}>稍后</button>
    {/if}
    {#if updateError}
      <span class="update-error">{updateError}</span>
    {/if}
  </div>
{/if}

{#if showNav}
<nav>
  {#each [
    { href: '/generator', key: 'generator' },
    { href: '/library', key: 'library' },
    { href: '/settings', key: 'settings' },
  ] as item}
    <a href={item.href} class:active={$page.url.pathname === item.href}>
      {$t.nav[item.key]}
    </a>
  {/each}
</nav>
{/if}

<slot />

<style>
  nav {
    display: flex;
    gap: 4px;
    padding: 8px 16px;
    background: #1a1a2e;
    border-bottom: 1px solid #2d2d44;
  }
  nav a {
    padding: 8px 16px;
    color: #a0a0b0;
    text-decoration: none;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 500;
    transition: all 0.2s;
  }
  nav a:hover {
    background: #2d2d44;
    color: #ffffff;
  }
  nav a.active {
    background: #4f46e5;
    color: #ffffff;
  }

  .update-banner {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 16px;
    background: #1e3a5f;
    border-bottom: 1px solid #2d5986;
    font-size: 13px;
    color: #e0e0e0;
  }

  .update-btn {
    padding: 4px 14px;
    background: #4f46e5;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
  }

  .update-btn:hover {
    background: #4338ca;
  }

  .dismiss-btn {
    padding: 4px 10px;
    background: transparent;
    color: #a0a0b0;
    border: 1px solid #2d2d44;
    border-radius: 4px;
    font-size: 12px;
    cursor: pointer;
  }

  .update-progress {
    color: #93c5fd;
  }

  .update-error {
    color: #ef4444;
    font-size: 12px;
  }
</style>