<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { invoke } from '@tauri-apps/api/core';
  import { t } from '$lib/i18n';

  onMount(async () => {
    // Check API key status; if not configured and not on onboarding, redirect
    try {
      const configured: boolean = await invoke('get_api_key_status');
      if (!configured && !$page.url.pathname.startsWith('/onboarding')) {
        goto('/onboarding');
      }
    } catch {}
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