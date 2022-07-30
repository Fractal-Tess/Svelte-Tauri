<script lang="ts">
	import ThemeToggleIcon from '$lib/example/header/ThemeToggleIcon.svelte';
	import { theme } from '$lib/stores/theme';
	import { appWindow } from '@tauri-apps/api/window';
</script>

<div data-tauri-drag-region class="h-12 bg-base-100 shadow-lg items-center justify-between flex">
	<button on:click={theme.toggleTheme} class="hover:text-secondary ml-2" aria-live="polite">
		<ThemeToggleIcon cssClass={'w-8'} />
	</button>

	<div>
		<button
			on:click={appWindow.minimize}
			aria-live="polite"
			class="p-2 text-xl hover:text-secondary h-full"
		>
			<i class="fa-solid fa-window-minimize w-6" />
		</button>
		<button
			on:click={appWindow.close}
			aria-live="polite"
			class="p-2 text-2xl hover:text-secondary h-full"
		>
			<i class="fa-solid fa-xmark w-6" />
		</button>
	</div>
</div>
