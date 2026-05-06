import { writable, derived } from 'svelte/store';
import { zh } from './zh';
import { en } from './en';

type Translations = typeof zh;

const translations: Record<string, Translations> = { zh, en };

function getInitialLocale(): string {
  if (typeof window === 'undefined') return 'zh';
  const saved = localStorage.getItem('music-studio-locale');
  if (saved && (saved === 'zh' || saved === 'en')) return saved;
  return navigator.language.startsWith('zh') ? 'zh' : 'en';
}

export const locale = writable<string>(getInitialLocale());

locale.subscribe((val) => {
  if (typeof window !== 'undefined') {
    localStorage.setItem('music-studio-locale', val);
  }
});

export const t = derived(locale, ($locale) => translations[$locale] || translations.zh);

export function setLocale(lang: string) {
  locale.set(lang);
}
