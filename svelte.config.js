import sveltePreprocess from 'svelte-preprocess';

export default {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [
    sveltePreprocess({
      postcss: true,
      scss: {
        prependData: '@use "src/variables.scss" as *;'
      },
      sourceMap: !!process.env.TAURI_DEBUG
    })
  ]
};
