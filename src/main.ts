import './styles.postcss';
import App from './App.svelte';

const app = new App({
  target: document.getElementById('app'),
  intro: true
});

export default app;
