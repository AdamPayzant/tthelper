<script lang="ts">
	import type { Writable } from 'svelte/store';
	import { type FullCharacterData, type Attack } from '$lib/pf2_services_types';

	export let data: Writable<FullCharacterData | undefined>;

	function get_damage_die_amt(attack: Attack): number {
		// TODO: Check if attacking item has runes
		return 1;
	}

	function handle_new_attack_click() {
		console.log('Adding attack');
	}

	function handle_attack_prof_change(a: Attack) {
		console.log(a);
	}
</script>

{#if $data}
	<table class="max-h-1">
		<tr>
			<th>
				<p>Attacks</p>
			</th>
			<th>
				<button on:click={() => handle_new_attack_click()}> + </button>
			</th>
		</tr>
		{#each $data.attacks as attack}
			<tr>
				<td>
					<div id="Skills" class="flex pl-4">
						<select
							bind:value={attack.prof}
							on:change={() => {
								handle_attack_prof_change(attack);
							}}
						>
							<option value="Untrained"> U </option>
							<option value="Trained"> T </option>
							<option value="Expert"> E </option>
							<option value="Master"> M </option>
							<option value="Legendary"> L </option>
						</select>
					</div>
				</td>
				<td> {attack.name} </td>
				<td> {attack.matk} </td>
				<td> {attack.mdmg} </td>
				<td>
					{#if attack.damage_die}
						{get_damage_die_amt(attack)}d{attack.damage_die}
					{/if}
				</td>
			</tr>
		{/each}
	</table>
{/if}

<style>
	td {
		border: 1px solid #dddddd;
		text-align: left;
		padding: 4px;
		min-width: 40px;
	}
</style>
