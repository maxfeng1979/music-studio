import { writable } from 'svelte/store';

// Track which audio source is currently playing. Only one at a time.
export const activeAudioSrc = writable<string | null>(null);

// Call this when an audio starts playing. Stops all others.
export function playAudio(src: string) {
  activeAudioSrc.set(src);
}
