use diesel::prelude::*;

use crate::db::db_enums;
use crate::db::schema;

use super::schema::sql_types::Pf2Proficiency;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_weapon_group)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WeaponGroup {
    id: i32,
    group_name: String,
    crit_spec: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_weapon)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Weapon {
    id: i32,
    item_id: i32,
    weapon_type: db_enums::Pf2WeaponType,
    weapon_cat: db_enums::Pf2WeaponCategory,
    group_id: Option<i32>,
    damage_die: i32,
    hands: String,
    weapon_range: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_traits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Trait {
    id: i32,
    trait_name: String,
    description: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_skills)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Skill {
    id: i32,
    title: String,
    ability: db_enums::Pf2Ability,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_shield)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Shield {
    id: i32,
    item_id: i32,
    ac_bonus: i32,
    hardness: i32,
    hp: i32,
    bp: i32,
    speed_penalty: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_items_in_containers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ContainedItem {
    id: i32,
    bag_id: i32,
    item_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item {
    id: i32,
    item_name: String,
    item_description: String,
    bulk: i32,
    price: i32,
    lvl: i32,
    invested: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_item_traits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ItemTrait {
    id: i32,
    item_id: i32,
    trait_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_class_features)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClassFeature {
    id: i32,
    character_id: i32,
    title: String,
    description: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_characters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Character {
    id: i32,
    character_name: String,
    alignment: String,
    ancestry: String,
    background: String,
    character_class: String,
    key_ability: String,
    lvl: i32,
    hero_points: i32,

    str_bonus: i32,
    dex_bonus: i32,
    con_bonus: i32,
    int_bonus: i32,
    wis_bonus: i32,
    cha_bonus: i32,
    active_apex_item: Option<String>,
    active_apex_item_bonus: Option<db_enums::Pf2Ability>,

    temp_hp: i32,
    damage: i32,
    dying: i32,
    wound: i32,
    doom: i32,

    fort_prof: db_enums::Pf2Proficiency,
    fort_misc_bonus: i32,
    refl_prof: db_enums::Pf2Proficiency,
    refl_misc_bonus: i32,
    will_prof: db_enums::Pf2Proficiency,
    will_misc_bonus: i32,
    perception_prof: db_enums::Pf2Proficiency,
    perception_misc_bonus: i32,

    base_land_speed: Option<i32>,
    base_fly_speed: Option<i32>,
    base_swim_speed: Option<i32>,
    base_burrow_speed: Option<i32>,
    base_climb_speed: Option<i32>,

    max_focus_points: Option<i32>,
    current_focus_points: Option<i32>,

    simple_weapon_prof: db_enums::Pf2Proficiency,
    martial_weapon_prof: db_enums::Pf2Proficiency,
    weapon_spec: db_enums::Pf2WeaponSpec,
    unarmored_prof: db_enums::Pf2Proficiency,
    light_armor_prof: db_enums::Pf2Proficiency,
    med_armor_prof: db_enums::Pf2Proficiency,
    heavy_armor_prof: db_enums::Pf2Proficiency,
    class_prof: db_enums::Pf2Proficiency,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_worn_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterWornItem {
    id: i32,
    item_id: i32,
    invested: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_stored_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterStoredItem {
    id: i32,
    item_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_statuses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterStatus {
    id: i32,
    character_id: i32,
    status_name: String,
    status_description: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_spells_prepared)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpellsPrepared {
    id: i32,
    spell_id: i32,
    level_prepared: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_spellcasting_tables)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpellcastingTable {
    id: i32,
    character_id: i32,
    tradition: db_enums::Pf2SpellTradition,
    ability: db_enums::Pf2Ability,
    proficiency: db_enums::Pf2Proficiency,
    spontaneous: bool,
    casts_per_day: Vec<Option<i32>>,
    spells_known: Vec<Option<i32>>,
    item_bonus: i32,
    misc_bonus: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_spell_known)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpellKnown {
    id: i32,
    table_id: i32,
    spell_name: String,
    action_length: db_enums::Pf2Action,
    base_level: i32,
    duration: Option<String>,
    spell_range: Option<String>,
    area: Option<String>,
    spell_description: String,
    heightening: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_skills)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterSkill {
    id: i32,
    character_id: i32,
    skill_id: i32,
    proficiency: db_enums::Pf2Proficiency,
    bonuses: i32,
    assurance: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_senses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Sense {
    id: i32,
    character_id: i32,
    sense_name: String,
    sense_description: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_readied_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReadiedItem {
    id: i32,
    item_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_languages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterLanguage {
    id: i32,
    character_id: i32,
    title: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterItem {
    id: i32,
    character_id: i32,
    item_id: i32,
    quantity: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_item_attached_runes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AttachedRune {
    id: i32,
    item_id: i32,
    rune_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_formula_books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FormulaBook {
    id: i32,
    character_id: i32,
    item_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_feats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterFeat {
    id: i32,
    character_id: i32,
    title: String,
    description: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_damage_type_modifier)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DamageTypeModifier {
    id: i32,
    character_id: i32,
    modifier: db_enums::Pf2DamageTypeModifier,
    val: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_containers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterContainers {
    id: i32,
    item_id: i32,
    bulk_reduction: i32,
    max_bulk: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_background_ability_bonus)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BackgroundAbilityBonus {
    id: i32,
    character_id: i32,
    ability: db_enums::Pf2Ability,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_attacks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CharacterAttack {
    id: i32,
    character_id: i32,
    item_id: Option<i32>,
    proficiency: db_enums::Pf2Proficiency,
    matk: i32,
    mdmg: i32,
    attack_type: db_enums::Pf2AttackType,
    damage_die: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_armor_traits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ArmorTrait {
    id: i32,
    character_id: i32,
    trait_name: String,
    description: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_ancestry_features)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AncestryFeature {
    id: i32,
    character_id: i32,
    title: String,
    description: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_character_ancestry_ability_modifier)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AncestryModifier {
    id: i32,
    character_id: i32,
    ability: db_enums::Pf2Ability,
    positive_boost: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_armor_group)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ArmorGroup {
    id: i32,
    group_name: String,
    armor_spec: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::pf2_armor)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Armor {
    id: i32,
    item_id: i32,
    ac_bonus: i32,
    max_dex: Option<i32>,
    check_penalty: i32,
    speed_penalty: i32,
    str_requirement: i32,
    armor_type: db_enums::Pf2ArmorType,
    armor_group: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub salt: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub salt: &'a str,
}
