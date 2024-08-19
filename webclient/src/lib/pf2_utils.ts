import { type FullCharacterData } from './pf2_services_types';

export function is_apex_relevant(ability: string, data: FullCharacterData): boolean {
	return data
		? data.active_apex_item_bonus != undefined && data.active_apex_item_bonus === ability
		: false;
}

export function get_str_score(data: FullCharacterData | undefined): number {
	if (!data) return 0;

	const base_bonus = data.str_base + data.str_bonus;

	if (is_apex_relevant('Strength', data)) {
		if (base_bonus < 18) {
			return 18;
		} else {
			return base_bonus + 2;
		}
	}
	return base_bonus;
}
export function get_dex_score(data: FullCharacterData | undefined): number {
	if (!data) return 0;

	const base_bonus = data.dex_base + data.dex_bonus;

	if (is_apex_relevant('Dexterity', data)) {
		if (base_bonus < 18) {
			return 18;
		} else {
			return base_bonus + 2;
		}
	}
	return base_bonus;
}
export function get_con_score(data: FullCharacterData | undefined): number {
	if (!data) return 0;

	const base_bonus = data.con_base + data.con_bonus;

	if (is_apex_relevant('Constitution', data)) {
		if (base_bonus < 18) {
			return 18;
		} else {
			return base_bonus + 2;
		}
	}
	return base_bonus;
}
export function get_int_score(data: FullCharacterData | undefined): number {
	if (!data) return 0;

	const base_bonus = data.int_base + data.int_bonus;

	if (is_apex_relevant('Intelligence', data)) {
		if (base_bonus < 18) {
			return 18;
		} else {
			return base_bonus + 2;
		}
	}
	return base_bonus;
}
export function get_wis_score(data: FullCharacterData | undefined): number {
	if (!data) return 0;

	const base_bonus = data.wis_base + data.wis_bonus;

	if (is_apex_relevant('Wisdom', data)) {
		if (base_bonus < 18) {
			return 18;
		} else {
			return base_bonus + 2;
		}
	}
	return base_bonus;
}
export function get_cha_score(data: FullCharacterData | undefined): number {
	if (!data) return 0;

	const base_bonus = data.cha_base + data.cha_bonus;

	if (is_apex_relevant('Charisma', data)) {
		if (base_bonus < 18) {
			return 18;
		} else {
			return base_bonus + 2;
		}
	}
	return base_bonus;
}
