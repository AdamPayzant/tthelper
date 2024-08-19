<script lang="ts">
	import { onDestroy } from 'svelte';
	import type { Writable } from 'svelte/store';
	import {
		type FullCharacterData,
		type InventoryItem,
		Pf2ArmorType
	} from '$lib/pf2_services_types';
	import { get_str_score, get_dex_score, get_con_score, get_wis_score } from '$lib/pf2_utils';

	import Abilities from '$lib/components/pf2/sheet_subcomponents/Abilities.svelte';
	import SkillList from '$lib/components/pf2/sheet_subcomponents/SkillList.svelte';
	import Attacks from '$lib/components/pf2/sheet_subcomponents/Attacks.svelte';

	export let cid: number;
	export let data: Writable<FullCharacterData | undefined>;

	async function load_data() {
		console.log('Loading');
		let res = await fetch(`userhome/character/${cid}`, {
			method: 'GET'
		});

		let character = await res.json();

		$data = character;

		console.log($data);
	}

	function start_load(): string {
		load_data();
		return 'Loading';
	}

	// ============================== //
	//        Reactive Functions      //
	// ============================== //

	$: get_fort_bonus = (): number => {
		if (!$data) return 0;
		return (
			Math.floor((get_con_score($data) - 10) / 2) +
			(get_proficiency_bonus($data.fort_prof) + $data.lvl) +
			$data.fort_misc_bonus +
			get_resilience_bonus()
		);
	};

	$: get_refl_bonus = (): number => {
		if (!$data) return 0;
		// TODO: Check if Mighty Bulwark is taken and relevant
		return (
			Math.floor((get_dex_score($data) - 10) / 2) +
			(get_proficiency_bonus($data.refl_prof) + $data.lvl) +
			$data.refl_misc_bonus +
			get_resilience_bonus()
		);
	};

	$: get_will_bonus = (): number => {
		if (!$data) return 0;
		return (
			Math.floor((get_wis_score($data) - 10) / 2) +
			(get_proficiency_bonus($data.will_prof) + $data.lvl) +
			$data.will_misc_bonus +
			get_resilience_bonus()
		);
	};

	$: get_resilience_bonus = (): number => {
		if (!$data) return 0;
		if (!$data.armor) return 0;

		let found = $data.armor.item_details.runes.find((val: InventoryItem) => {
			return val.name === 'Resilient';
		});
		if (!found) return 0;

		switch (found.lvl) {
			case 8:
				return 1;
			case 14:
				return 2;
			case 20:
				return 3;
			default:
				console.warn('Unexpected resilient rune level found, defaultin to 1');
				return 1;
		}
	};

	$: get_armor_proficiency_bonus = (): number => {
		if (!$data) return 0;

		if (!$data.armor) {
			return get_proficiency_bonus($data.unarmored_prof) + $data.lvl;
		}
		switch ($data.armor.armor_type) {
			case Pf2ArmorType[Pf2ArmorType.Unarmored]:
				return get_proficiency_bonus($data.unarmored_prof) + $data.lvl;
			case Pf2ArmorType[Pf2ArmorType.Light]:
				return get_proficiency_bonus($data.light_armor_prof) + $data.lvl;
			case Pf2ArmorType[Pf2ArmorType.Medium]:
				return get_proficiency_bonus($data.med_armor_prof) + $data.lvl;
			case Pf2ArmorType[Pf2ArmorType.Heavy]:
				return get_proficiency_bonus($data.heavy_armor_prof) + $data.lvl;
			default:
				console.error('Unknown armor type, defaulting to light');
				return get_proficiency_bonus($data.light_armor_prof) + $data.lvl;
		}
	};

	// ============================== //
	//          Util Functions        //
	// ============================== //

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

	// ============================== //
	//         Update Functions       //
	// ============================== //

	async function level_change() {
		console.log('Not implemented');
	}

	async function exp_change() {
		console.log('Not implemented');
	}

	onDestroy(() => {
		console.log(`Destroying component for ${cid}`);
	});
</script>

<div class="w-full h-full p-10">
	{#if $data}
		<div class="flex border-4 w-full box-border border-slate-500">
			<div id="NameLvlAbilities" class="pr-4">
				<div class="flex border-2 box-border border-slate-500 font-bold text-4xl w-min">
					<p class="pt-2 pb-2 pr-4 pl-4">
						{$data.name}
					</p>
				</div>
				<div id="LvlDets" class="pl-2">
					<div>
						Level <input
							class="max-w-10"
							type="number"
							bind:value={$data.lvl}
							on:change={level_change}
						/>
					</div>
					<div class="text-sm">
						exp: <input
							class="max-w-10"
							step="10"
							type="number"
							bind:value={$data.exp}
							on:change={exp_change}
						/>
					</div>
				</div>
				<Abilities {data} />
				<div id="DefenseSavesAttacks" class="flex">
					<div id="Defense">
						<div id="Armor">
							<table>
								Armor
								<tr>
									<td>{0}</td>
									=
									<td>{Math.floor((get_str_score($data) - 10) / 2)}</td>
									<td>{get_armor_proficiency_bonus()}</td>
									<td>
										{#if $data.armor}
											{$data.armor.ac_bonus}
										{:else}
											{0}
										{/if}
									</td>
									<td>{$data.misc_armor_bonus}</td>
								</tr>
							</table>
						</div>
					</div>
					<div id="Saves">
						<table class="text-center">
							<tr>
								<th></th>
								<th>Total</th>
								<div class="pl-2 pr-2">DC</div>
								<th>Ability</th>
								<th>Prof</th>
								<th>Item</th>
								<th>Misc</th>
							</tr>
							<tr>
								<div class="pl-2 pr-2">Fort</div>
								<td>{get_fort_bonus()}</td>
								{get_fort_bonus() + 10}
								<td>{Math.floor((get_con_score($data) - 10) / 2)}</td>
								<td>{get_proficiency_bonus($data.fort_prof) + $data.lvl}</td>
								<td>{get_resilience_bonus()}</td>
								<td>{$data.fort_misc_bonus}</td>
							</tr>
							<tr>
								<div class="pl-2 pr-2">Refl</div>
								<td>{get_refl_bonus()}</td>
								{get_refl_bonus() + 10}
								<td>{Math.floor((get_dex_score($data) - 10) / 2)}</td>
								<td>{get_proficiency_bonus($data.refl_prof) + $data.lvl}</td>
								<td>{get_resilience_bonus()}</td>
								<td>{$data.refl_misc_bonus}</td>
							</tr>
							<tr>
								<div class="pl-2 pr-2">Will</div>
								<td>{get_will_bonus()}</td>
								{get_will_bonus() + 10}
								<td>{Math.floor((get_wis_score($data) - 10) / 2)}</td>
								<td>{get_proficiency_bonus($data.will_prof) + $data.lvl}</td>
								<td>{get_resilience_bonus()}</td>
								<td>{$data.will_misc_bonus}</td>
							</tr>
						</table>
					</div>
					<Attacks {data} />
				</div>
			</div>
			<div id="CoreDetails" class="flex max-h-48">
				<table class="border-2 box-border text-base" id="CharacterInfo">
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
			<SkillList {data} />
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
	th {
		font-size: 0.875rem;
	}
</style>
