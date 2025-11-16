import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	// Set the base to './' for relative paths, crucial for Tauri bundling
	base: './' 
});