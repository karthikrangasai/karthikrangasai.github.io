<script>
	import CommandOuput from "./CommandOutput.svelte";

	export let command_outputs = [];
	export let machine_name;
</script>

<section>
	{#each command_outputs as { user, path, command, output }, i}
		<CommandOuput {machine_name} {user} {path} {command} {output} />
	{/each}
</section>

<style>
</style>
