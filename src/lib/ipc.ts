import { commands } from '$commands'
import { events } from '$events'

const api = { ...commands, ...events }

export { api }
