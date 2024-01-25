<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let name = $state('');
	let greetMsg = $state('');

	async function greet() {
		const data = await invoke('find_subset', { data: [3, 2, -2, -1] });
		console.log(data);
	}
</script>

<div>
	<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
	<button on:click={greet}>Greet</button>
	<p>{greetMsg}</p>
</div>
