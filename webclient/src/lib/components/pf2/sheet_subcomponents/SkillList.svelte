<script lang="ts">
	import type { Writable } from 'svelte/store';
	import { type FullCharacterData, type Skill } from '$lib/pf2_services_types';
	import {
		get_str_score,
		get_dex_score,
		get_con_score,
		get_int_score,
		get_wis_score,
		get_cha_score
	} from '$lib/pf2_utils';

	export let data: Writable<FullCharacterData | undefined>;

	$: get_skill_modifier = (ability: string, prof: string, misc_bonus: number): number => {
		if (!$data) {
			return 0;
		}

		let prof_bonus = get_proficiency_bonus(prof);
		return (
			misc_bonus + ability_to_modifier(ability) + (prof_bonus != 0 ? prof_bonus + $data.lvl : 0)
		);
	};

	function ability_to_modifier(ability: string): number {
		switch (ability.substring(0, 2)) {
			case 'St':
				return Math.floor((get_str_score($data) - 10) / 2);
			case 'De':
				return Math.floor((get_dex_score($data) - 10) / 2);
			case 'Co':
				return Math.floor((get_con_score($data) - 10) / 2);
			case 'In':
				return Math.floor((get_int_score($data) - 10) / 2);
			case 'Wi':
				return Math.floor((get_wis_score($data) - 10) / 2);
			case 'Ch':
				return Math.floor((get_cha_score($data) - 10) / 2);
			default:
				console.error('Invalid ability modifier lookup');
				return 0;
		}
	}

	function get_proficiency_bonus(prof: string): number {
		let bonus = 0;
		switch (prof.substring(0, 1)) {
			case 'U':
				bonus += 0;
				break;
			case 'T':
				bonus += 2;
				break;
			case 'E':
				bonus += 4;
				break;
			case 'M':
				bonus += 6;
				break;
			case 'L':
				bonus += 8;
				break;
			default:
				console.error('Modifier requested for invalid proficiency');
				return 0;
		}
		return bonus;
	}

	async function skill_change(skill: Skill) {
		if (!$data) return;

		let res = await fetch(`userhome/character/${$data.id}/skill/${skill.id}`, {
			method: 'PUT',
			body: JSON.stringify(skill)
		});
		console.log(res);
	}
</script>

<div id="Skills" class="flex pl-4">
	{#if $data}
		<table class="border-2 box-border text-base">
			<tr>
				<th> Name </th>
				<th> </th>
				<th> </th>
				<th> Total </th>
				<p class="pl-2 pr-2">=</p>
				<th> Ability </th>
				<th> Prof </th>
				<th> Bonus </th>
			</tr>
			{#each $data.skills as s}
				{#if s.skill != 'Lore' || s.extra_name}
					<tr>
						<td>
							{#if s.extra_name}
								{`${s.skill}: ${s.extra_name}`}
							{:else}
								{s.skill}
							{/if}
						</td>
						<td> {s.ability.substring(0, 3)} </td>
						<td>
							<div id="Skills" class="flex pl-4">
								<select
									bind:value={s.proficiency}
									on:change={() => {
										skill_change(s);
									}}
								>
									<option value="Untrained"> U </option>
									<option value="Trained"> T </option>
									<option value="Expert"> E </option>
									<option value="Master"> M </option>
									<option value="Legendary"> L </option>
								</select>
							</div></td
						>
						<td> {get_skill_modifier(s.ability, s.proficiency, s.bonuses)} </td>
						<p class="pl-2 pr-2">=</p>
						<td> {ability_to_modifier(s.ability)} </td>
						<td>
							{get_proficiency_bonus(s.proficiency) +
								(s.proficiency.substring(0, 1) === 'U' ? 0 : $data.lvl)}
						</td>
						<td>
							<input
								class="max-w-12"
								type="number"
								bind:value={s.bonuses}
								on:change={() => {
									skill_change(s);
								}}
							/>
						</td>
					</tr>
				{/if}
			{/each}
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
