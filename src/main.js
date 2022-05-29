import App from './App.svelte';
import wasm from '../terminal/Cargo.toml';

const initialize = async () => {
	const exports = await wasm();
	const app = new App({
		target: document.body,
		props: { ...exports }
	});
};

initialize();