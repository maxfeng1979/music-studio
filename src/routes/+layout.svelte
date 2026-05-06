<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';

  const navItems = [
    { href: '/generator', label: 'Generator' },
    { href: '/library', label: 'Library' },
    { href: '/settings', label: 'Settings' },
  ];

  $: showNav = $page.url.pathname !== '/onboarding';
</script>

{#if showNav}
<nav>
  {#each navItems as item}
    <a href={item.href} class:active={$page.url.pathname === item.href}>
      {item.label}
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