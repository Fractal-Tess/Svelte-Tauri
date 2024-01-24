import './styles.postcss'
import App from './App.svelte'

const target = document.getElementById('app')
if (!target) {
  throw new Error(
    "The element with id of 'app' was not found on the base html file. Create that element and try again"
  )
}

const app = new App({
  target,
  intro: true
})

export default app
