use diesel::prelude::*;

use crate::db::db_enums;
use crate::db::schema;

use super::schema::sql_types::Pf2Proficiency;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_weapon_group)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WeaponGroup {
    pub id: i32,
    pub group_name: String,
    pub crit_spec: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_weapon)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Weapon {
    pub id: i32,
    pub item_id: i32,
    pub weapon_type: db_enums::Pf2WeaponType,
    pub weapon_cat: db_enums::Pf2WeaponCategory,
    pub group_id: Option<i32>,
    pub damage_die: i32,
    pub hands: String,
    pub weapon_range: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_traits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Trait {
    pub id: i32,
    pub trait_name: String,
    pub description: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_skills)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Skill {
    pub id: i32,
    pub title: String,
    pub ability: db_enums::Pf2Ability,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_shield)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Shield {
    pub id: i32,
    pub item_id: i32,
    pub ac_bonus: i32,
    pub hardness: i32,
    pub hp: i32,
    pub bp: i32,
    pub speed_penalty: i32,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_items_in_containers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ContainedItem {
    pub id: i32,
    pub bag_id: i32,
    pub item_id: i32,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item {
    pub id: i32,
    pub item_name: String,
    pub item_description: String,
    pub bulk: i32,
    pub price: i32,
    pub lvl: i32,
    pub invested: bool,
}

#[derive(Queryable, Selectable, Associations, Identifiable)]
#[diesel(belongs_to(Item))]
#[diesel(table_name = schema::pf2_item_traits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ItemTrait {
    pub id: i32,
    pub item_id: i32,
    pub trait_id: i32,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_class_features)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClassFeature {
    pub id: i32,
    pub character_id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_characters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Character {
    pub id: i32,
    pub owner: i32,
    pub character_name: String,
    pub alignment: String,
    pub ancestry: String,
    pub background: String,
    pub character_class: String,
    pub key_ability: String,
    pub lvl: i32,
    pub hero_points: i32,

    pub str_bonus: i32,
    pub dex_bonus: i32,
    pub con_bonus: i32,
    pub int_bonus: i32,
    pub wis_bonus: i32,
    pub cha_bonus: i32,
    pub active_apex_item: Option<String>,
    pub active_apex_item_bonus: Option<db_enums::Pf2Ability>,

    pub temp_hp: i32,
    pub damage: i32,
    pub dying: i32,
    pub wound: i32,
    pub doom: i32,

    pub fort_prof: db_enums::Pf2Proficiency,
    pub fort_misc_bonus: i32,
    pub refl_prof: db_enums::Pf2Proficiency,
    pub refl_misc_bonus: i32,
    pub will_prof: db_enums::Pf2Proficiency,
    pub will_misc_bonus: i32,
    pub perception_prof: db_enums::Pf2Proficiency,
    pub perception_misc_bonus: i32,

    pub base_land_speed: Option<i32>,
    pub base_fly_speed: Option<i32>,
    pub base_swim_speed: Option<i32>,
    pub base_burrow_speed: Option<i32>,
    pub base_climb_speed: Option<i32>,

    pub max_focus_points: Option<i32>,
    pub current_focus_points: Option<i32>,

    pub simple_weapon_prof: db_enums::Pf2Proficiency,
    pub martial_weapon_prof: db_enums::Pf2Proficiency,
    pub weapon_spec: db_enums::Pf2WeaponSpec,
    pub unarmored_prof: db_enums::Pf2Proficiency,
    pub light_armor_prof: db_enums::Pf2Proficiency,
    pub med_armor_prof: db_enums::Pf2Proficiency,
    pub heavy_armor_prof: db_enums::Pf2Proficiency,
    pub class_prof: db_enums::Pf2Proficiency,
}

#[derive(Insertable)]
#[diesel(table_name = schema::pf2_characters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCharacter<'a> {
    pub character_name: &'a str,
    pub owner: i32,
    pub alignment: &'a str,
    pub ancestry: &'a str,
    pub background: &'a str,
    pub character_class: &'a str,
    pub key_ability: &'a str,
}

#[derive(Queryable)]
#[diesel(table_name = schema::pf2_characters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterIdName<'a> {
    pub id: i32,
    pub character_name: &'a str,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_worn_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterWornItem {
    pub id: i32,
    pub item_id: i32,
    pub invested: bool,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_stored_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterStoredItem {
    pub id: i32,
    pub item_id: i32,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_statuses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterStatus {
    pub id: i32,
    pub character_id: i32,
    pub status_name: String,
    pub status_description: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_spells_prepared)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpellsPrepared {
    pub id: i32,
    pub spell_id: i32,
    pub level_prepared: i32,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_spellcasting_tables)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpellcastingTable {
    pub id: i32,
    pub character_id: i32,
    pub tradition: db_enums::Pf2SpellTradition,
    pub ability: db_enums::Pf2Ability,
    pub proficiency: db_enums::Pf2Proficiency,
    pub spontaneous: bool,
    pub casts_per_day: Vec<Option<i32>>,
    pub spells_known: Vec<Option<i32>>,
    pub item_bonus: i32,
    pub misc_bonus: i32,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_spell_known)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpellKnown {
    pub id: i32,
    pub table_id: i32,
    pub spell_name: String,
    pub action_length: db_enums::Pf2Action,
    pub base_level: i32,
    pub duration: Option<String>,
    pub spell_range: Option<String>,
    pub area: Option<String>,
    pub spell_description: String,
    pub heightening: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_skills)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterSkill {
    pub id: i32,
    pub character_id: i32,
    pub skill_id: i32,
    pub proficiency: db_enums::Pf2Proficiency,
    pub bonuses: i32,
    pub assurance: bool,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_senses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Sense {
    pub id: i32,
    pub character_id: i32,
    pub sense_name: String,
    pub sense_description: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_readied_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReadiedItem {
    pub id: i32,
    pub item_id: i32,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_languages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterLanguage {
    pub id: i32,
    pub character_id: i32,
    pub title: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterItem {
    pub id: i32,
    pub character_id: i32,
    pub item_id: i32,
    pub quantity: i32,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = schema::pf2_character_item_attached_runes)]
#[diesel(belongs_to(CharacterItem))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AttachedRune {
    pub id: i32,
    pub character_item_id: i32,
    pub rune_id: i32,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_formula_books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FormulaBook {
    pub id: i32,
    pub character_id: i32,
    pub item_id: i32,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_feats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterFeat {
    pub id: i32,
    pub character_id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_damage_type_modifier)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DamageTypeModifier {
    pub id: i32,
    pub character_id: i32,
    pub modifier: db_enums::Pf2DamageTypeModifier,
    pub val: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_containers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterContainers {
    pub id: i32,
    pub item_id: i32,
    pub bulk_reduction: i32,
    pub max_bulk: i32,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_background_ability_bonus)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BackgroundAbilityBonus {
    pub id: i32,
    pub character_id: i32,
    pub ability: db_enums::Pf2Ability,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_attacks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterAttack {
    pub id: i32,
    pub character_id: i32,
    pub item_id: Option<i32>,
    pub proficiency: db_enums::Pf2Proficiency,
    pub matk: i32,
    pub mdmg: i32,
    pub attack_type: db_enums::Pf2AttackType,
    pub damage_die: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_armor_traits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ArmorTrait {
    pub id: i32,
    pub character_id: i32,
    pub trait_name: String,
    pub description: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_ancestry_features)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AncestryFeature {
    pub id: i32,
    pub character_id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_character_ancestry_ability_modifier)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AncestryModifier {
    pub id: i32,
    pub character_id: i32,
    pub ability: db_enums::Pf2Ability,
    pub positive_boost: bool,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::pf2_armor_group)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ArmorGroup {
    pub id: i32,
    pub group_name: String,
    pub armor_spec: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(belongs_to(Item))]
#[diesel(table_name = schema::pf2_armor)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Armor {
    pub id: i32,
    pub item_id: i32,
    pub ac_bonus: i32,
    pub max_dex: Option<i32>,
    pub check_penalty: i32,
    pub speed_penalty: i32,
    pub str_requirement: i32,
    pub armor_type: db_enums::Pf2ArmorType,
    pub armor_group: i32,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
