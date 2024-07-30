<script lang="ts">
	import NewCharacter from '$lib/components/NewCharacter.svelte';
	export let data;
</script>

<div class="flex flex-row">
	<div
		class="relative top-0 left-0 z-40 w-64 max-w-64 h-screen transition-transform -translate-x-full sm:translate-x-0"
		aria-label="sidebar"
	>
		Character List:
		<ul class="border-2 box-border border-slate-500 h-full">
			{#await data.character_list then list}
				{#each list as c}
					{c.character_name}
				{/each}
			{/await}
		</ul>
	</div>
	<div class="pt-10 pl-4">
		<NewCharacter />
	</div>
</div>
