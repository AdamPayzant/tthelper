export interface NewCharacterAPI {
	character_name: string;
	alignment: string;
	ancestry: string;
	background: string;
	character_class: string;
	key_ability: string;
}

enum Pf2Ability {}
enum Pf2Proficiency {}
enum Pf2WeaponSpec {}
enum Pf2DamageTypeModifier {}
enum Pf2SpellTradition {}
enum Pf2Action {}
enum Pf2ArmorType {}
enum Pf2WeaponType {}
enum Pf2WeaponCategory {}
enum Pf2AttackType {}

export interface AbilityModifier {
	id: number;
	ability: Pf2Ability;
	is_positive_boost: boolean;
}

export interface AncestryFeature {
	id: number;
	name: string;
	description: string;
}

export interface ClassFeature {
	id: number;
	name: string;
	description: string;
}

export interface DamageRecievedModifier {
	id: number;
	modifier_type: Pf2DamageTypeModifier;
	value: number | undefined;
}

export interface Sense {
	id: number;
	name: string;
	description: string;
}

export interface Skill {
	id: number;
	skill: string;
	ability: Pf2Ability;

	proficiency: Pf2Proficiency;
	bonuses: number;
	assurance: boolean;
}

export interface Feat {
	id: number;
	name: string;
	description: string;
}

export interface Spell {
	id: number;
	name: string;
	description: string;
	heightening: string | undefined;

	action_length: Pf2Action;
	base_level: number;
	duration: string | undefined;
	spell_range: string | undefined;
	area: string | undefined;
}

export interface PreparedSpell {
	id: number;
	spell_id: number;
	level_prepared: number;
}

export interface SpellcastingTable {
	id: number;
	tradition: Pf2SpellTradition;
	ability: Pf2Ability;
	prof: Pf2Proficiency;
	spontaneous: boolean;

	casts_per_day: Array<number | undefined>;
	num_spells_known: Array<number | undefined>;

	item_bonus: number;
	misc_bonus: number;

	spells_known: Array<Spell>;
	spells_prepared: Array<PreparedSpell> | undefined;
}

export interface StoredItem {}

export interface InventoryItem {
	id: number; // The id from pf2_character_items
	name: string;
	description: string;
	bulk: number;
	price: number;
	lvl: number;
	invested: boolean;
	traits: Array<Trait>;
	quantity: number;

	runes: Array<InventoryItem>;
}

export interface ItemContainer {
	id: number;
	item: InventoryItem;
	bulk_reduction: number;
	max_bulk: number;
	contents: Array<InventoryItem>;
}

export interface Armor {
	id: number;
	item_details: InventoryItem;
	ac_bonus: number;
	max_dex: number | undefined;
	check_penalty: number;
	speed_penalty: number;
	str_requirement: number;
	armor_type: Pf2ArmorType;
	armor_group_name: string;
	armor_spec: string;
}

export interface Shield {
	id: number;
	item_details: InventoryItem;
	ac_bonus: number;
	hardness: number;
	hp: number;
	bp: number;
	speed_penalty: number;
}
export interface Weapon {
	id: number;
	item_details: InventoryItem;
	weapon_type: Pf2WeaponType;
	weapon_cat: Pf2WeaponCategory;
	group_name: string;
	crit_spec: string;
	damage_die: number;
	hands: string;
	weapon_range: number | undefined;
}

export interface Attack {
	id: number;
	weapon: Weapon | undefined;
	prof: Pf2Proficiency;
	matk: number;
	mdmg: number;
	attack_type: Pf2AttackType;
	damage_die: number | undefined;
}

export interface Status {
	id: number;
	name: string;
	description: string;
}

export interface Formula {
	id: number;
	item_id: number;
	item_name: string;
	level: number;
	item_value: number;
}

export interface Trait {
	id: number;
	name: string;
	description: string;
}

export interface FullCharacterData {
	id: number;
	name: string;
	alignment: string;
	ancestry: string;
	background: string;
	class: string;
	key_ability: string;
	lvl: number;
	hero_points: number;

	str_bonus: number;
	dex_bonus: number;
	con_bonus: number;
	int_bonus: number;
	wis_bonus: number;
	cha_bonus: number;

	active_apex_item: string | undefined;
	active_apex_item_bonus: Pf2Ability | undefined;

	temp_hp: number;
	damage: number;
	dying: number;
	wound: number;
	doom: number;

	fort_prof: Pf2Proficiency;
	fort_misc_bonus: number;
	refl_prof: Pf2Proficiency;
	refl_misc_bonus: number;
	will_prof: Pf2Proficiency;
	will_misc_bonus: number;
	perception_prof: Pf2Proficiency;
	perception_misc_bonus: number;

	base_land_speed: number | undefined;
	base_fly_speed: number | undefined;
	base_swim_speed: number | undefined;
	base_burrow_speed: number | undefined;
	base_climb_speed: number | undefined;

	max_focus_points: number | undefined;
	current_focus_points: number | undefined;

	simple_weapon_prof: Pf2Proficiency;
	martial_weapon_prof: Pf2Proficiency;
	weapon_spec: Pf2WeaponSpec;

	unarmored_prof: Pf2Proficiency;
	light_armor_prof: Pf2Proficiency;
	med_armor_prof: Pf2Proficiency;
	heavy_armor_prof: Pf2Proficiency;

	class_prof: Pf2Proficiency;

	background_ability_boosts: Array<AbilityModifier>;
	ancestry_ability_boosts: Array<AbilityModifier>;

	ancestry_features: Array<AncestryFeature>;
	class_features: Array<ClassFeature>;
	damage_recieved_mods: Array<DamageRecievedModifier>;

	senses: Array<Sense>;
	skills: Array<Skill>;
	languages: Array<string>;

	feats: Array<Feat>;

	casting_tables: Array<SpellcastingTable>;

	stored_items: Array<StoredItem>;
	readied_items: Array<InventoryItem>;
	worn_items: Array<InventoryItem>;

	armor: Armor | undefined;
	shield: Shield | undefined;

	weapons: Array<Weapon>;
	attacks: Array<Attack>;

	statuses: Array<Status>;

	formula_book: Array<Formula>;
}
