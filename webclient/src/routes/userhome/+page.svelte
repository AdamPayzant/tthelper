<script lang="ts">
	import type { ComponentType } from 'svelte';

	import NewCharacter from '$lib/components/NewCharacter.svelte';
	import CharacterSheet from '$lib/components/CharacterSheet.svelte';

	export let data;

	export let pages: { name: string; cid: number | null; component: ComponentType }[] = [];
	export let active_page_idx = 0;
	// I don't like using this, but checking the len of pages doesn't update after first add
	export let display_component: boolean = false;

	export const handle_character_click = (id: number, name: string) => {
		console.log(id);
		const found = pages.findIndex(
			(el: { cid: number | null; component: ComponentType }) => el.cid === id
		);
		if (found !== -1) {
			active_page_idx = found;
		} else {
			active_page_idx =
				pages.push({
					name: name,
					cid: id,
					component: CharacterSheet
				}) - 1;
		}
		display_component = true;
		console.log(pages.length);
		console.log(active_page_idx);
	};
	export const handle_new_character_click = () => {
		console.log('Adding new character component');
		active_page_idx =
			pages.push({
				name: 'New Character',
				cid: null,
				component: NewCharacter
			}) - 1;
		display_component = true;
	};
</script>

<div class="flex flex-row">
	<div
		class="relative top-0 left-0 z-40 w-64 max-w-64 h-screen transition-transform -translate-x-full sm:translate-x-0"
		aria-label="sidebar"
	>
		Characters
		<button class="justify-end" on:click={() => handle_new_character_click()}> + </button>
		<ul class="border-2 box-border border-slate-500 h-full">
			{#await data.character_list then list}
				{#each list as c}
					<li class="pl-2 pt-1">
						<button on:click={() => handle_character_click(c.character_id, c.character_name)}>
							{c.character_name}
						</button>
					</li>
				{/each}
			{/await}
		</ul>
	</div>
	<div class="pt-10 pl-4">
		{#if display_component}
			{#if pages[active_page_idx].cid !== null}
				<svelte:component
					this={pages[active_page_idx].component}
					cid={pages[active_page_idx].cid}
					data_server={data.data_server}
					data_server_header={data.data_server_header}
				/>
			{:else}
				<svelte:component this={pages[active_page_idx].component} />
			{/if}
		{/if}
	</div>
</div>
