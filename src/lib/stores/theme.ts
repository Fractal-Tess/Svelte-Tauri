import { Store } from 'tauri-plugin-store-api';
import { writable } from 'svelte/store';
import type { Theme } from '$types';

const store = new Store('.settings.dat');

const createThemeStore = () => {
  const { subscribe, update, set } = writable<Theme>();

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
