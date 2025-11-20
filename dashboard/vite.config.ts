import tailwindcss from '@tailwindcss/vite';
import toastyrabbit from 'toastyrabbit/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit(), toastyrabbit()],
});
