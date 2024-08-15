// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pf2_ability"))]
    pub struct Pf2Ability;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pf2_action"))]
    pub struct Pf2Action;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pf2_armor_type"))]
    pub struct Pf2ArmorType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pf2_attack_type"))]
    pub struct Pf2AttackType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pf2_damage_type_modifier"))]
    pub struct Pf2DamageTypeModifier;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pf2_proficiency"))]
    pub struct Pf2Proficiency;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pf2_spell_tradition"))]
    pub struct Pf2SpellTradition;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pf2_weapon_category"))]
    pub struct Pf2WeaponCategory;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pf2_weapon_spec"))]
    pub struct Pf2WeaponSpec;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pf2_weapon_type"))]
    pub struct Pf2WeaponType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2ArmorType;

    pf2_armor (id) {
        id -> Int4,
        item_id -> Int4,
        ac_bonus -> Int4,
        max_dex -> Nullable<Int4>,
        check_penalty -> Int4,
        speed_penalty -> Int4,
        str_requirement -> Int4,
        armor_type -> Pf2ArmorType,
        armor_group -> Int4,
    }
}

diesel::table! {
    pf2_armor_group (id) {
        id -> Int4,
        group_name -> Text,
        armor_spec -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2Ability;

    pf2_character_ancestry_ability_modifier (id) {
        id -> Int4,
        character_id -> Int4,
        ability -> Pf2Ability,
        positive_boost -> Bool,
    }
}

diesel::table! {
    pf2_character_ancestry_features (id) {
        id -> Int4,
        character_id -> Int4,
        title -> Text,
        description -> Text,
    }
}

diesel::table! {
    pf2_character_armor_traits (id) {
        id -> Int4,
        character_id -> Int4,
        trait_name -> Text,
        description -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2Proficiency;
    use super::sql_types::Pf2AttackType;

    pf2_character_attacks (id) {
        id -> Int4,
        character_id -> Int4,
        item_id -> Nullable<Int4>,
        proficiency -> Pf2Proficiency,
        matk -> Int4,
        mdmg -> Int4,
        attack_type -> Pf2AttackType,
        damage_die -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2Ability;

    pf2_character_background_ability_bonus (id) {
        id -> Int4,
        character_id -> Int4,
        ability -> Pf2Ability,
    }
}

diesel::table! {
    pf2_character_containers (id) {
        id -> Int4,
        item_id -> Int4,
        bulk_reduction -> Int4,
        max_bulk -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2DamageTypeModifier;

    pf2_character_damage_type_modifier (id) {
        id -> Int4,
        character_id -> Int4,
        modifier -> Pf2DamageTypeModifier,
        val -> Nullable<Int4>,
    }
}

diesel::table! {
    pf2_character_feats (id) {
        id -> Int4,
        character_id -> Int4,
        title -> Text,
        description -> Text,
    }
}

diesel::table! {
    pf2_character_formula_books (id) {
        id -> Int4,
        character_id -> Int4,
        item_id -> Int4,
    }
}

diesel::table! {
    pf2_character_item_attached_runes (id) {
        id -> Int4,
        character_item_id -> Int4,
        rune_id -> Int4,
    }
}

diesel::table! {
    pf2_character_items (id) {
        id -> Int4,
        character_id -> Int4,
        item_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    pf2_character_languages (id) {
        id -> Int4,
        character_id -> Int4,
        title -> Text,
    }
}

diesel::table! {
    pf2_character_readied_items (id) {
        id -> Int4,
        item_id -> Int4,
    }
}

diesel::table! {
    pf2_character_senses (id) {
        id -> Int4,
        character_id -> Int4,
        sense_name -> Text,
        sense_description -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2Proficiency;

    pf2_character_skills (id) {
        id -> Int4,
        character_id -> Int4,
        skill_id -> Int4,
        proficiency -> Pf2Proficiency,
        bonuses -> Int4,
        assurance -> Bool,
        extra_name -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2Action;

    pf2_character_spell_known (id) {
        id -> Int4,
        spellcasting_table_id -> Int4,
        spell_name -> Text,
        action_length -> Pf2Action,
        base_level -> Int4,
        duration -> Nullable<Text>,
        spell_range -> Nullable<Text>,
        area -> Nullable<Text>,
        spell_description -> Text,
        heightening -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2SpellTradition;
    use super::sql_types::Pf2Ability;
    use super::sql_types::Pf2Proficiency;

    pf2_character_spellcasting_tables (id) {
        id -> Int4,
        character_id -> Int4,
        tradition -> Pf2SpellTradition,
        ability -> Pf2Ability,
        proficiency -> Pf2Proficiency,
        spontaneous -> Bool,
        casts_per_day -> Array<Nullable<Int4>>,
        spells_known -> Array<Nullable<Int4>>,
        item_bonus -> Int4,
        misc_bonus -> Int4,
    }
}

diesel::table! {
    pf2_character_spells_prepared (id) {
        id -> Int4,
        spellcasting_table_id -> Int4,
        spell_id -> Int4,
        level_prepared -> Int4,
    }
}

diesel::table! {
    pf2_character_statuses (id) {
        id -> Int4,
        character_id -> Int4,
        status_name -> Text,
        status_description -> Text,
    }
}

diesel::table! {
    pf2_character_stored_items (id) {
        id -> Int4,
        item_id -> Int4,
    }
}

diesel::table! {
    pf2_character_worn_items (id) {
        id -> Int4,
        item_id -> Int4,
        invested -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2Ability;
    use super::sql_types::Pf2Proficiency;
    use super::sql_types::Pf2WeaponSpec;

    pf2_characters (id) {
        id -> Int4,
        owner -> Int4,
        character_name -> Text,
        alignment -> Text,
        ancestry -> Text,
        background -> Text,
        character_class -> Text,
        key_ability -> Text,
        lvl -> Int4,
        exp -> Int4,
        hero_points -> Int4,
        str_base -> Int4,
        str_bonus -> Int4,
        dex_base -> Int4,
        dex_bonus -> Int4,
        con_base -> Int4,
        con_bonus -> Int4,
        int_base -> Int4,
        int_bonus -> Int4,
        wis_base -> Int4,
        wis_bonus -> Int4,
        cha_base -> Int4,
        cha_bonus -> Int4,
        active_apex_item -> Nullable<Text>,
        active_apex_item_bonus -> Nullable<Pf2Ability>,
        temp_hp -> Int4,
        damage -> Int4,
        dying -> Int4,
        wound -> Int4,
        doom -> Int4,
        misc_armor_bonus -> Int4,
        fort_prof -> Pf2Proficiency,
        fort_misc_bonus -> Int4,
        refl_prof -> Pf2Proficiency,
        refl_misc_bonus -> Int4,
        will_prof -> Pf2Proficiency,
        will_misc_bonus -> Int4,
        perception_prof -> Pf2Proficiency,
        perception_misc_bonus -> Int4,
        base_land_speed -> Nullable<Int4>,
        base_fly_speed -> Nullable<Int4>,
        base_swim_speed -> Nullable<Int4>,
        base_burrow_speed -> Nullable<Int4>,
        base_climb_speed -> Nullable<Int4>,
        max_focus_points -> Nullable<Int4>,
        current_focus_points -> Nullable<Int4>,
        simple_weapon_prof -> Pf2Proficiency,
        martial_weapon_prof -> Pf2Proficiency,
        weapon_spec -> Pf2WeaponSpec,
        unarmored_prof -> Pf2Proficiency,
        light_armor_prof -> Pf2Proficiency,
        med_armor_prof -> Pf2Proficiency,
        heavy_armor_prof -> Pf2Proficiency,
        class_prof -> Pf2Proficiency,
    }
}

diesel::table! {
    pf2_class_features (id) {
        id -> Int4,
        character_id -> Int4,
        title -> Text,
        description -> Text,
    }
}

diesel::table! {
    pf2_initialization_log (id) {
        id -> Int4,
        initialized_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pf2_item_traits (id) {
        id -> Int4,
        item_id -> Int4,
        trait_id -> Int4,
    }
}

diesel::table! {
    pf2_items (id) {
        id -> Int4,
        item_name -> Text,
        item_description -> Text,
        bulk -> Int4,
        price -> Int4,
        lvl -> Int4,
        invested -> Bool,
    }
}

diesel::table! {
    pf2_items_in_containers (id) {
        id -> Int4,
        character_containers_id -> Int4,
        item_id -> Int4,
    }
}

diesel::table! {
    pf2_shield (id) {
        id -> Int4,
        item_id -> Int4,
        ac_bonus -> Int4,
        hardness -> Int4,
        hp -> Int4,
        bp -> Int4,
        speed_penalty -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2Ability;

    pf2_skills (id) {
        id -> Int4,
        title -> Text,
        ability -> Pf2Ability,
    }
}

diesel::table! {
    pf2_traits (id) {
        id -> Int4,
        trait_name -> Text,
        description -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2WeaponType;
    use super::sql_types::Pf2WeaponCategory;

    pf2_weapon (id) {
        id -> Int4,
        item_id -> Int4,
        weapon_type -> Pf2WeaponType,
        weapon_cat -> Pf2WeaponCategory,
        group_id -> Nullable<Int4>,
        damage_die -> Int4,
        hands -> Text,
        weapon_range -> Nullable<Int4>,
    }
}

diesel::table! {
    pf2_weapon_group (id) {
        id -> Int4,
        group_name -> Text,
        crit_spec -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
    }
}

diesel::joinable!(pf2_armor -> pf2_armor_group (armor_group));
diesel::joinable!(pf2_armor -> pf2_items (item_id));
diesel::joinable!(pf2_character_ancestry_ability_modifier -> pf2_characters (character_id));
diesel::joinable!(pf2_character_ancestry_features -> pf2_characters (character_id));
diesel::joinable!(pf2_character_armor_traits -> pf2_characters (character_id));
diesel::joinable!(pf2_character_attacks -> pf2_character_items (item_id));
diesel::joinable!(pf2_character_attacks -> pf2_characters (character_id));
diesel::joinable!(pf2_character_background_ability_bonus -> pf2_characters (character_id));
diesel::joinable!(pf2_character_containers -> pf2_character_items (item_id));
diesel::joinable!(pf2_character_damage_type_modifier -> pf2_characters (character_id));
diesel::joinable!(pf2_character_feats -> pf2_characters (character_id));
diesel::joinable!(pf2_character_formula_books -> pf2_characters (character_id));
diesel::joinable!(pf2_character_formula_books -> pf2_items (item_id));
diesel::joinable!(pf2_character_item_attached_runes -> pf2_character_items (character_item_id));
diesel::joinable!(pf2_character_item_attached_runes -> pf2_items (rune_id));
diesel::joinable!(pf2_character_items -> pf2_characters (character_id));
diesel::joinable!(pf2_character_items -> pf2_items (item_id));
diesel::joinable!(pf2_character_languages -> pf2_characters (character_id));
diesel::joinable!(pf2_character_readied_items -> pf2_character_items (item_id));
diesel::joinable!(pf2_character_senses -> pf2_characters (character_id));
diesel::joinable!(pf2_character_skills -> pf2_characters (character_id));
diesel::joinable!(pf2_character_skills -> pf2_skills (skill_id));
diesel::joinable!(pf2_character_spell_known -> pf2_character_spellcasting_tables (spellcasting_table_id));
diesel::joinable!(pf2_character_spellcasting_tables -> pf2_characters (character_id));
diesel::joinable!(pf2_character_spells_prepared -> pf2_character_spell_known (spell_id));
diesel::joinable!(pf2_character_spells_prepared -> pf2_character_spellcasting_tables (spellcasting_table_id));
diesel::joinable!(pf2_character_statuses -> pf2_characters (character_id));
diesel::joinable!(pf2_character_stored_items -> pf2_character_items (item_id));
diesel::joinable!(pf2_character_worn_items -> pf2_character_items (item_id));
diesel::joinable!(pf2_characters -> users (owner));
diesel::joinable!(pf2_class_features -> pf2_characters (character_id));
diesel::joinable!(pf2_item_traits -> pf2_items (item_id));
diesel::joinable!(pf2_item_traits -> pf2_traits (trait_id));
diesel::joinable!(pf2_items_in_containers -> pf2_character_containers (character_containers_id));
diesel::joinable!(pf2_items_in_containers -> pf2_character_items (item_id));
diesel::joinable!(pf2_shield -> pf2_items (item_id));
diesel::joinable!(pf2_weapon -> pf2_items (item_id));
diesel::joinable!(pf2_weapon -> pf2_weapon_group (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    pf2_armor,
    pf2_armor_group,
    pf2_character_ancestry_ability_modifier,
    pf2_character_ancestry_features,
    pf2_character_armor_traits,
    pf2_character_attacks,
    pf2_character_background_ability_bonus,
    pf2_character_containers,
    pf2_character_damage_type_modifier,
    pf2_character_feats,
    pf2_character_formula_books,
    pf2_character_item_attached_runes,
    pf2_character_items,
    pf2_character_languages,
    pf2_character_readied_items,
    pf2_character_senses,
    pf2_character_skills,
    pf2_character_spell_known,
    pf2_character_spellcasting_tables,
    pf2_character_spells_prepared,
    pf2_character_statuses,
    pf2_character_stored_items,
    pf2_character_worn_items,
    pf2_characters,
    pf2_class_features,
    pf2_initialization_log,
    pf2_item_traits,
    pf2_items,
    pf2_items_in_containers,
    pf2_shield,
    pf2_skills,
    pf2_traits,
    pf2_weapon,
    pf2_weapon_group,
    users,
);
