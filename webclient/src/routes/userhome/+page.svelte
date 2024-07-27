<script lang="ts">
	import NewCharacter from '$lib/components/NewCharacter.svelte';
	export let data;
</script>

Authorized page
<div class="flex flex-row h-full">
	<div
		class="relative top-0 left-0 z-40 w-64 h-screen transition-transform -translate-x-full sm:translate-x-0"
		aria-label="sidebar"
	>
		Character List:
		<ul>
			{#each data.character_list as c}
				{c}
			{/each}
		</ul>
	</div>
	<NewCharacter />
</div>
