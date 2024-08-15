<script lang="ts">
	import type { Writable } from 'svelte/store';
	import { type FullCharacterData, type AbilityUpdate } from '$lib/pf2_services_types';

	export let data: Writable<FullCharacterData | undefined>;

	$: is_apex_relevant = (ability: string): boolean => {
		return $data
			? $data.active_apex_item_bonus != undefined && $data.active_apex_item_bonus === ability
			: false;
	};

	// In retrospect, not putting the abilities in an array was a mistake, maybe fix that down the line
	$: get_str_score = (): number => {
		if (!$data) throw Error;
		let base_bonus = $data.str_base + $data.str_bonus;
		if (is_apex_relevant('Strength')) {
			if (base_bonus < 18) {
				return 18;
			} else {
				return base_bonus + 2;
			}
		}
		return base_bonus;
	};
	$: get_dex_score = (): number => {
		if (!$data) throw Error;
		let base_bonus = $data.dex_base + $data.dex_bonus;

		if (is_apex_relevant('Dexterity')) {
			if (base_bonus < 18) {
				return 18;
			} else {
				return base_bonus + 2;
			}
		}
		return base_bonus;
	};
	$: get_con_score = (): number => {
		if (!$data) throw Error;
		let base_bonus = $data.con_base + $data.con_bonus;

		if (is_apex_relevant('Constitution')) {
			if (base_bonus < 18) {
				return 18;
			} else {
				return base_bonus + 2;
			}
		}
		return base_bonus;
	};
	$: get_int_score = (): number => {
		if (!$data) throw Error;
		let base_bonus = $data.int_base + $data.int_bonus;

		if (is_apex_relevant('Intelligence')) {
			if (base_bonus < 18) {
				return 18;
			} else {
				return base_bonus + 2;
			}
		}
		return base_bonus;
	};
	$: get_wis_score = (): number => {
		if (!$data) throw Error;
		let base_bonus = $data.wis_base + $data.wis_bonus;

		if (is_apex_relevant('Wisdom')) {
			if (base_bonus < 18) {
				return 18;
			} else {
				return base_bonus + 2;
			}
		}
		return base_bonus;
	};
	$: get_cha_score = (): number => {
		if (!$data) throw Error;
		let base_bonus = $data.cha_base + $data.cha_bonus;

		if (is_apex_relevant('Charisma')) {
			if (base_bonus < 18) {
				return 18;
			} else {
				return base_bonus + 2;
			}
		}
		return base_bonus;
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

		let res = await fetch(`userhome/character/${cid}/abilities`, {
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
					{Math.floor((get_str_score() - 10) / 2)}
				</td>
				<td> {get_str_score()} </td>
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
					{#if is_apex_relevant('Strength')}
						2
					{/if}
				</td>
			</tr>
			<tr>
				<td> Dexterity </td>
				<td>
					{Math.floor((get_dex_score() - 10) / 2)}
				</td>
				<td> {get_dex_score()} </td>
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
					{#if is_apex_relevant('Dexterity')}
						2
					{/if}
				</td>
			</tr>
			<tr>
				<td> Constitution </td>
				<td>
					{Math.floor((get_con_score() - 10) / 2)}
				</td>
				<td> {get_con_score()} </td>
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
					{#if is_apex_relevant('Constitution')}
						2
					{/if}
				</td>
			</tr>
			<tr>
				<td> Intelligence </td>
				<td>
					{Math.floor((get_int_score() - 10) / 2)}
				</td>
				<td> {get_int_score()} </td>
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
					{#if is_apex_relevant('Intelligence')}
						2
					{/if}
				</td>
			</tr>
			<tr>
				<td> Wisdom </td>
				<td>
					{Math.floor((get_wis_score() - 10) / 2)}
				</td>
				<td> {get_wis_score()} </td>
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
					{#if is_apex_relevant('Wisdom')}
						2
					{/if}
				</td>
			</tr>
			<tr>
				<td> Charisma </td>
				<td>
					{Math.floor((get_cha_score() - 10) / 2)}
				</td>
				<td> {get_cha_score()} </td>
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
					{#if is_apex_relevant('Charisma')}
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
