import fs from 'node:fs'
import toml from '@iarna/toml'

import nodePkg from './package.json'
import tauriPkg from './src-tauri/tauri.conf.json'
import path from 'node:path'
import { spawnSync } from 'node:child_process'

const tauriPath = new URL(
  path.dirname(import.meta.url) + '/src-tauri/tauri.conf.json'
).pathname
tauriPkg.package.version = nodePkg.version
fs.writeFileSync(tauriPath, JSON.stringify(tauriPkg, null, 2))

const cargoPath = new URL(
  path.dirname(import.meta.url) + '/src-tauri/Cargo.toml'
).pathname
const cargoPkg = toml.parse(fs.readFileSync(cargoPath, 'utf-8'))
cargoPkg['package']['version'] = nodePkg.version
fs.writeFileSync(cargoPath, toml.stringify(cargoPkg))

const child = spawnSync('pnpm', ['prettier', '--write', 'src-tauri/Cargo.toml'])
