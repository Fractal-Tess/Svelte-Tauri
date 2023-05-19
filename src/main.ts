import './styles.pcss';
import App from './App.svelte';

const target = document.getElementById('app');
if (!target)
  throw new Error(
    "The element with id of 'app' wasn't found on the base html file."
  );

const app = new App({
  target,
  intro: true
});

export default app;
