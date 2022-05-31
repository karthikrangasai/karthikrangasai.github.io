<script>
	import CommandOuputs from "./CommandOutputs.svelte";

	export let instantiate_terminal;

	const terminal = instantiate_terminal();

	let machine_name = terminal.machine_name;
	let current_command = "";

	let command_outputs = [terminal.run_command("help")];

	const TAB = 9;
	const ENTER = 13;

	const handleKeyDown = (event) => {
		switch (event.keyCode) {
			case TAB: {
				event.preventDefault();
				break;
			}
			default: {
				break;
			}
		}
	};
	const handleKeyUp = (event) => {
		switch (event.keyCode) {
			case TAB: {
				event.preventDefault();
				break;
			}
			default: {
				break;
			}
		}
	};
	const handleKeyPress = (event) => {
		if (event.keyCode === ENTER) {
			if (current_command === "") {
				console.log("Empty line `Beep` Sound.");
			} else {
				let command_output = terminal.run_command(current_command);
				console.log(command_output);
				console.log(`Output:\n${command_output.output}`);
				if (command_output.command === "clear") {
					command_outputs = [];
				} else {
					command_outputs = [...command_outputs, command_output];
				}
			}
			current_command = "";
		}
	};
	const handleKeyBoardInput = (event) => {
		switch (event.type) {
			case "keydown": {
				handleKeyDown(event);
				break;
			}
			case "keyup": {
				handleKeyUp(event);
				break;
			}
			default: {
				console.log(`Event: ${event.type}, Key: ${event.key}, Code: ${event.code}, Key Code: ${event.keyCode}`);
				handleKeyPress(event);
				break;
			}
		}
	};
</script>

<main>
	<CommandOuputs bind:command_outputs {machine_name} />
	<div class="terminal-input">
		<label id="location" for="command">{terminal.user}@{machine_name}:{terminal.current_path}$ </label>
		<input
			type="text"
			name="command"
			id="command"
			bind:value={current_command}
			on:keypress={handleKeyBoardInput}
			on:keydown={handleKeyBoardInput}
			on:keyup={handleKeyBoardInput}
		/>
	</div>
</main>

<style>
	main {
		font-family: "Source Code Pro", monospace;
		padding: 0.5%;
		margin: 0 auto;
		max-width: none;
		width: 98%;
		height: 98%;
		display: flex;
		flex-direction: column;
		overflow-y: scroll;
		border: 2px solid orange;
	}
	::-webkit-scrollbar {
		display: none;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}

	.terminal-input {
		width: 100%;
		display: flex;
		flex-flow: column;
		justify-content: flex-start;
		align-content: center;
		align-items: baseline;
		flex-wrap: wrap;
		flex-direction: row;
	}

	#location {
		height: 100%;
	}

	input {
		font-family: inherit;
		font-size: inherit;
		margin: 0 0 0 0.5em;
		padding: 0;
		box-sizing: none;
		border: none;
		border-radius: 0;
		caret-color: black;
	}

	input:focus {
		outline: none;
	}
</style>
