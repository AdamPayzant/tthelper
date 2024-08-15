// =========================================== //
//                API Structures               //
// =========================================== //

export interface NewCharacterAPI {
	character_name: string;
	alignment: string;
	ancestry: string;
	background: string;
	character_class: string;
	key_ability: string;
}

export interface AbilityUpdate {
	character_id: number;
	ability: string;
	base: number;
	bonus: number;
}

// =========================================== //
//            Full Character data              //
// =========================================== //

// Defining enums
// For now these are unused
export enum Pf2Ability {
	Strength,
	Dexterity,
	Constitution,
	Intelligence,
	Wisdom,
	Charisma
}

export enum Pf2Proficiency {
	Untrained,
	Trained,
	Expert,
	Master,
	Legendary
}

export enum Pf2WeaponSpec {
	None,
	WS,
	GWS
}

export enum Pf2DamageTypeModifier {
	Weakness,
	Resistance,
	Immunity
}

export enum Pf2SpellTradition {
	Arcane,
	Divine,
	Elemental,
	Occult,
	Primal,
	Focus
}

export enum Pf2Action {
	Free,
	Reaction,
	One,
	Two,
	Three,
	OneToThree,
	TwoToThree
}

export enum Pf2ArmorType {
	Unarmored,
	Light,
	Medium,
	Heavy
}

export enum Pf2WeaponType {
	Melee,
	Ranged
}

export enum Pf2WeaponCategory {
	Unarmed,
	Simple,
	Martial,
	Advanced
}

export enum Pf2AttackType {
	StrStr,
	DexStr,
	DexDex,
	RangedDexHalfStr,
	RangedDexStr,
	Athletics
}

// Actual structs
export interface AbilityModifier {
	id: number;
	ability: string;
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
	modifier_type: string;
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
	ability: string;

	proficiency: string;
	bonuses: number;
	assurance: boolean;
	extra_name: string | null;
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

	action_length: string;
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
	tradition: string;
	ability: string;
	prof: string;
	spontaneous: boolean;

	casts_per_day: Array<number | undefined>;
	num_spells_known: Array<number | undefined>;

	item_bonus: number;
	misc_bonus: number;

	spells_known: Array<Spell>;
	spells_prepared: Array<PreparedSpell> | undefined;
}

export interface StoredItemItem {
	Item: InventoryItem;
}

export interface StoredItemContainer {
	Container: ItemContainer;
}

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
	armor_type: string;
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
	weapon_type: string;
	weapon_cat: string;
	group_name: string;
	crit_spec: string;
	damage_die: number;
	hands: string;
	weapon_range: number | undefined;
}

export interface Attack {
	id: number;
	weapon: Weapon | undefined;
	prof: string;
	matk: number;
	mdmg: number;
	attack_type: string;
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
	exp: number;
	hero_points: number;

	str_base: number;
	str_bonus: number;
	dex_base: number;
	dex_bonus: number;
	con_base: number;
	con_bonus: number;
	int_base: number;
	int_bonus: number;
	wis_base: number;
	wis_bonus: number;
	cha_base: number;
	cha_bonus: number;

	active_apex_item: string | undefined;
	active_apex_item_bonus: string | undefined;

	temp_hp: number;
	damage: number;
	dying: number;
	wound: number;
	doom: number;

	misc_armor_bonus: number;

	fort_prof: string;
	fort_misc_bonus: number;
	refl_prof: string;
	refl_misc_bonus: number;
	will_prof: string;
	will_misc_bonus: number;
	perception_prof: string;
	perception_misc_bonus: number;

	base_land_speed: number | undefined;
	base_fly_speed: number | undefined;
	base_swim_speed: number | undefined;
	base_burrow_speed: number | undefined;
	base_climb_speed: number | undefined;

	max_focus_points: number | undefined;
	current_focus_points: number | undefined;

	simple_weapon_prof: string;
	martial_weapon_prof: string;
	weapon_spec: string;

	unarmored_prof: string;
	light_armor_prof: string;
	med_armor_prof: string;
	heavy_armor_prof: string;

	class_prof: string;

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

	stored_items: Array<StoredItemContainer | StoredItemItem>;
	readied_items: Array<InventoryItem>;
	worn_items: Array<InventoryItem>;

	armor: Armor | undefined;
	shield: Shield | undefined;

	weapons: Array<Weapon>;
	attacks: Array<Attack>;

	statuses: Array<Status>;

	formula_book: Array<Formula>;
}
