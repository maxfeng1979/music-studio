<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { t } from '$lib/i18n';

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
  });

  $: showNav = $page.url.pathname !== '/onboarding';
</script>

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
</style>