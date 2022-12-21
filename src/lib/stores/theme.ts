import { writable } from 'svelte/store';
import type { Theme } from '$types';

const createThemeStore = () => {
  let theme = localStorage.getItem('theme') as Theme | null;
  if (!theme) {
    theme = window.matchMedia('(prefers-color-scheme: dark)').matches
      ? 'dark'
      : 'light';
    localStorage.setItem('theme', theme);
  }

  const { subscribe, update } = writable<Theme>(theme);

  return {
    subscribe,
    toggleTheme: () => {
      update(currentTheme => {
        const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
        localStorage.setItem('theme', newTheme);
        return newTheme;
      });
    }
  };
};

export const theme = createThemeStore();
