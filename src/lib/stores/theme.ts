import { Store } from 'tauri-plugin-store-api';
import type { Theme } from '../../types';
import { writable } from 'svelte/store';

const store = new Store('.settings.dat');

const createThemeStore = () => {
  const { subscribe, update, set } = writable('');

  return {
    subscribe,
    toggleTheme: () => {
      update(currentTheme => {
        const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
        store.set('theme', newTheme);
        return newTheme;
      });
    },
    load: async () => {
      let storedTheme = (await store.get('theme')) as Theme | null;
      if (!storedTheme) {
        storedTheme = 'dark';
        store.set('theme', storedTheme);
      }
      set(storedTheme);
    }
  };
};
export const theme = createThemeStore();
