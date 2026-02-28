import './styles.css'
import App from './App.svelte'
import { mount } from 'svelte'

const target = document.getElementById('app')
if (!target)
  throw new Error(
    "The element with id of 'app' wasn't found on the base html file."
  )

const app = mount(App, {
  target,
  intro: true
})

export default app
