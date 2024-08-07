<script lang="ts">
	import { onMount } from 'svelte';

	export let cid: number;

	export const load_character = async () => {
		let res = await fetch(`userhome/character/${cid}`, {
			method: 'GET'
		});

		let character = await res.json();
		console.log(character);
		return character;
	};

	let data;
	onMount(async () => {
		data = await load_character();
	});
</script>

<div>
	{data}
</div>
