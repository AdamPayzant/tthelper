<script lang="ts">
	import { onDestroy } from 'svelte';
	import type { Writable } from 'svelte/store';
	import type { FullCharacterData } from '$lib/pf2_services_types';

	export let cid: number;
	export let data: Writable<FullCharacterData | undefined>;

	async function load_data() {
		console.log('Loading');
		let res = await fetch(`userhome/character/${cid}`, {
			method: 'GET'
		});

		let character = await res.json();

		$data = character;
	}

	function start_load(): string {
		load_data();
		return 'Loading';
	}

	onDestroy(() => {
		console.log(`Destroying component for ${cid}`);
	});
</script>

<div class="w-full h-full p-10">
	{#if $data}
		<div class="flex border-4 w-full box-border border-slate-500">
			<div class="pr-4">
				<div class="flex border-2 box-border border-slate-500 font-bold text-4xl w-min">
					<p class="pt-2 pb-2 pr-4 pl-4">
						{$data.name}
					</p>
				</div>
				<div class="pl-2">
					<div>
						Level {$data.lvl}
					</div>
					<div class="text-sm">exp: {$data.exp}</div>
				</div>
			</div>
			<div class="flex max-h-48">
				<table class="border-2 box-border text-base">
					<tr>
						<td class="border-1 border-slate-400 border-solid p-4 min-h-40"> Alignment </td>
						<td class="border-1 border-slate-400 border-solid p-4 min-h-40"> {$data.alignment} </td>
					</tr>
					<tr>
						<td class="border-1 border-slate-400 border-solid p-4 min-h-40"> Ancestry </td>
						<td class="border-1 border-slate-400 border-solid p-4 min-h-40"> {$data.ancestry} </td>
					</tr>
					<tr>
						<td class="border-1 border-slate-400 border-solid p-4 min-h-40"> Background </td>
						<td class="border-1 border-slate-400 border-solid p-4 min-h-40">
							{$data.background}
						</td>
					</tr>
					<tr>
						<td class="border-1 border-slate-400 border-solid p-4 min-h-40"> Class </td>
						<td class="border-1 border-slate-400 border-solid p-4 min-h-40"> {$data.class} </td>
					</tr>
					<tr>
						<td class="border-1 border-slate-400 border-solid p-4 min-h-40"> Key Ability </td>
						<td class="border-1 border-slate-400 border-solid p-4 min-h-40">
							{$data.key_ability}
						</td>
					</tr>
				</table>
			</div>
			<div class="flex pl-4">
				<table class="border-2 box-border text-base">
					{#each $data.skills as s}
						<tr>
							<td> {s.skill} </td>
							<td> {s.ability.substring(0, 3)} </td>
							<td> {s.proficiency.substring(0, 1)} </td>
						</tr>
					{/each}
				</table>
			</div>
		</div>
	{:else}
		{start_load()}
	{/if}
</div>

<style>
	td {
		border: 1px solid #dddddd;
		text-align: left;
		padding: 4px;
		min-width: 40px;
	}
</style>
