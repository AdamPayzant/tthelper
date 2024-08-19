<script lang="ts">
	import type { Writable } from 'svelte/store';
	import { type FullCharacterData, type AbilityUpdate } from '$lib/pf2_services_types';
	import {
		get_str_score,
		get_dex_score,
		get_con_score,
		get_int_score,
		get_wis_score,
		get_cha_score,
		is_apex_relevant
	} from '$lib/pf2_utils';

	export let data: Writable<FullCharacterData | undefined>;

	$: apex = (ability: string): boolean => {
		if (!$data) return false;
		return is_apex_relevant(ability, $data);
	};

	// In retrospect, not putting the abilities in an array was a mistake, maybe fix that down the line
	$: str_score = (): number => {
		return get_str_score($data);
	};
	$: dex_score = (): number => {
		return get_dex_score($data);
	};
	$: con_score = (): number => {
		return get_con_score($data);
	};
	$: int_score = (): number => {
		return get_int_score($data);
	};
	$: wis_score = (): number => {
		return get_wis_score($data);
	};
	$: cha_score = (): number => {
		return get_cha_score($data);
	};

	async function ability_change(ability: string) {
		if (!$data) {
			console.error('Somehow in ability change before data has loaded');
			return;
		}

		let update: AbilityUpdate = {
			character_id: $data.id,
			ability: ability,
			base: 0,
			bonus: 0
		};

		switch (ability.substring(0, 2)) {
			case 'St':
				update.base = $data.str_base;
				update.bonus = $data.str_bonus;
				break;
			case 'De':
				update.base = $data.dex_base;
				update.bonus = $data.dex_bonus;
				break;
			case 'Co':
				update.base = $data.con_base;
				update.bonus = $data.con_bonus;
				break;
			case 'In':
				update.base = $data.int_base;
				update.bonus = $data.int_bonus;
				break;
			case 'Wi':
				update.base = $data.wis_base;
				update.bonus = $data.wis_bonus;
				break;
			case 'Ch':
				update.base = $data.cha_base;
				update.bonus = $data.cha_bonus;
				break;
			default:
				console.error('Invalid skill provided');
				return;
		}

		let res = await fetch(`userhome/character/${$data.id}/abilities`, {
			method: 'PUT',
			body: JSON.stringify(update)
		});
		console.log(res);
	}
</script>

<div id="Abilities">
	{#if $data}
		<table>
			<tr>
				<th> Name </th>
				<th> Mod </th>
				<th> Score </th>
				<p class="pl-2 pr-2">=</p>
				<th> Base </th>
				<th> Bonus </th>
				<th> Apex </th>
			</tr>
			<tr>
				<td> Strength </td>
				<td>
					{Math.floor((str_score() - 10) / 2)}
				</td>
				<td> {str_score()} </td>
				<p class="pl-2 pr-2">=</p>
				<td>
					<input
						class="max-w-12"
						type="number"
						bind:value={$data.str_base}
						on:change={() => {
							ability_change('Strength');
						}}
					/>
				</td>
				<td>
					<input
						class="max-w-12"
						type="number"
						bind:value={$data.str_bonus}
						on:change={() => {
							ability_change('Strength');
						}}
					/>
				</td>
				<td>
					{#if apex('Strength')}
						2
					{/if}
				</td>
			</tr>
			<tr>
				<td> Dexterity </td>
				<td>
					{Math.floor((dex_score() - 10) / 2)}
				</td>
				<td> {dex_score()} </td>
				<p class="pl-2 pr-2">=</p>
				<td>
					<input
						class="max-w-12"
						type="number"
						bind:value={$data.dex_base}
						on:change={() => {
							ability_change('Dexterity');
						}}
					/>
				</td>
				<td>
					<input
						class="max-w-12"
						type="number"
						bind:value={$data.dex_bonus}
						on:change={() => {
							ability_change('Dexterity');
						}}
					/>
				</td>
				<td>
					{#if apex('Dexterity')}
						2
					{/if}
				</td>
			</tr>
			<tr>
				<td> Constitution </td>
				<td>
					{Math.floor((con_score() - 10) / 2)}
				</td>
				<td> {con_score()} </td>
				<p class="pl-2 pr-2">=</p>
				<td>
					<input
						class="max-w-12"
						type="number"
						bind:value={$data.con_base}
						on:change={() => {
							ability_change('Constitution');
						}}
					/>
				</td>
				<td>
					<input
						class="max-w-12"
						type="number"
						bind:value={$data.con_bonus}
						on:change={() => {
							ability_change('Constitution');
						}}
					/>
				</td>
				<td>
					{#if apex('Constitution')}
						2
					{/if}
				</td>
			</tr>
			<tr>
				<td> Intelligence </td>
				<td>
					{Math.floor((int_score() - 10) / 2)}
				</td>
				<td> {int_score()} </td>
				<p class="pl-2 pr-2">=</p>
				<td>
					<input
						class="max-w-12"
						type="number"
						bind:value={$data.int_base}
						on:change={() => {
							ability_change('Intelligence');
						}}
					/>
				</td>
				<td>
					<input
						class="max-w-12"
						type="number"
						bind:value={$data.int_bonus}
						on:change={() => {
							ability_change('Intelligence');
						}}
					/>
				</td>
				<td>
					{#if apex('Intelligence')}
						2
					{/if}
				</td>
			</tr>
			<tr>
				<td> Wisdom </td>
				<td>
					{Math.floor((wis_score() - 10) / 2)}
				</td>
				<td> {wis_score()} </td>
				<p class="pl-2 pr-2">=</p>
				<td>
					<input
						class="max-w-12"
						type="number"
						bind:value={$data.wis_base}
						on:change={() => {
							ability_change('Wisdom');
						}}
					/>
				</td>
				<td>
					<input
						class="max-w-12"
						type="number"
						bind:value={$data.wis_bonus}
						on:change={() => {
							ability_change('Wisdom');
						}}
					/>
				</td>
				<td>
					{#if apex('Wisdom')}
						2
					{/if}
				</td>
			</tr>
			<tr>
				<td> Charisma </td>
				<td>
					{Math.floor((cha_score() - 10) / 2)}
				</td>
				<td> {cha_score()} </td>
				<p class="pl-2 pr-2">=</p>
				<td>
					<input
						class="max-w-12"
						type="number"
						bind:value={$data.cha_base}
						on:change={() => {
							ability_change('Charisma');
						}}
					/>
				</td>
				<td>
					<input
						class="max-w-12"
						type="number"
						bind:value={$data.cha_bonus}
						on:change={() => {
							ability_change('Charisma');
						}}
					/>
				</td>
				<td>
					{#if apex('Charisma')}
						2
					{/if}
				</td>
			</tr>
		</table>
	{/if}
</div>

<style>
	td {
		border: 1px solid #dddddd;
		text-align: left;
		padding: 4px;
		min-width: 40px;
	}
	th {
		font-size: 0.875rem;
	}
</style>
