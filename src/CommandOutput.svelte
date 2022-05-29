<script>
	import About from "./About.svelte";

	export let user;
	export let path;
	export let command;
	export let output;
	export let machine_name;
</script>

<div class="output-container">
	<span class="metadata"> {user}@{machine_name}:{path}$ {command}</span>
	<div class="output">
		{#if command == "about"}
			<About />
		{:else}
			<pre class="output-value">{output}</pre>
		{/if}
	</div>
</div>

<style>
	.output-container {
		flex-direction: column;
		display: flex;
		flex-flow: column;
		justify-content: center;
	}
	.output-value {
		width: 100%;
		margin: 0.2em 0px;
		font-size: 16px;
		line-height: 1.2em;
		font-family: "Source Code Pro", monospace;
	}
</style>
