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
        item_id -> Nullable<Int4>,
        ac_bonus -> Nullable<Int4>,
        max_dex -> Nullable<Int4>,
        check_penalty -> Nullable<Int4>,
        speed_penalty -> Nullable<Int4>,
        str_requirement -> Nullable<Int4>,
        armor_type -> Nullable<Pf2ArmorType>,
        armor_group -> Nullable<Int4>,
    }
}

diesel::table! {
    pf2_armor_group (id) {
        id -> Int4,
        group_name -> Nullable<Text>,
        armor_spec -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2Ability;

    pf2_character_ancestry_ability_modifier (id) {
        id -> Int4,
        character_id -> Nullable<Int4>,
        ability -> Nullable<Pf2Ability>,
        positive_boost -> Nullable<Bool>,
    }
}

diesel::table! {
    pf2_character_ancestry_features (id) {
        id -> Int4,
        character_id -> Nullable<Int4>,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    pf2_character_armor_traits (id) {
        id -> Int4,
        character_id -> Nullable<Int4>,
        trait_name -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2Proficiency;
    use super::sql_types::Pf2AttackType;

    pf2_character_attacks (id) {
        id -> Int4,
        character_id -> Nullable<Int4>,
        item_id -> Nullable<Int4>,
        proficiency -> Nullable<Pf2Proficiency>,
        matk -> Nullable<Int4>,
        mdmg -> Nullable<Int4>,
        attack_type -> Nullable<Pf2AttackType>,
        damage_die -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2Ability;

    pf2_character_background_ability_bonus (id) {
        id -> Int4,
        character_id -> Nullable<Int4>,
        ability -> Nullable<Pf2Ability>,
    }
}

diesel::table! {
    pf2_character_containers (id) {
        id -> Int4,
        item_id -> Nullable<Int4>,
        bulk_reduction -> Nullable<Int4>,
        max_bulk -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2DamageTypeModifier;

    pf2_character_damage_type_modifier (id) {
        id -> Int4,
        character_id -> Nullable<Int4>,
        modifier -> Nullable<Pf2DamageTypeModifier>,
        val -> Nullable<Int4>,
    }
}

diesel::table! {
    pf2_character_feats (id) {
        id -> Int4,
        character_id -> Nullable<Int4>,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    pf2_character_formula_books (id) {
        id -> Int4,
        character_id -> Nullable<Int4>,
        item_id -> Nullable<Int4>,
    }
}

diesel::table! {
    pf2_character_item_attached_runes (id) {
        id -> Int4,
        item_id -> Nullable<Int4>,
        rune_id -> Nullable<Int4>,
    }
}

diesel::table! {
    pf2_character_items (id) {
        id -> Int4,
        character_id -> Nullable<Int4>,
        item_id -> Nullable<Int4>,
        quantity -> Nullable<Int4>,
    }
}

diesel::table! {
    pf2_character_languages (id) {
        id -> Int4,
        character_id -> Nullable<Int4>,
        title -> Nullable<Text>,
    }
}

diesel::table! {
    pf2_character_readied_items (id) {
        id -> Int4,
        item_id -> Nullable<Int4>,
    }
}

diesel::table! {
    pf2_character_senses (id) {
        id -> Int4,
        character_id -> Nullable<Int4>,
        sense_name -> Nullable<Text>,
        sense_description -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2Proficiency;

    pf2_character_skills (id) {
        id -> Int4,
        character_id -> Nullable<Int4>,
        skill_id -> Nullable<Int4>,
        proficiency -> Nullable<Pf2Proficiency>,
        bonuses -> Nullable<Int4>,
        assurance -> Nullable<Bool>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2Action;

    pf2_character_spell_known (id) {
        id -> Int4,
        table_id -> Nullable<Int4>,
        spell_name -> Nullable<Text>,
        action_length -> Nullable<Pf2Action>,
        base_level -> Nullable<Int4>,
        duration -> Nullable<Text>,
        spell_range -> Nullable<Text>,
        area -> Nullable<Text>,
        description -> Nullable<Text>,
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
        character_id -> Nullable<Int4>,
        tradition -> Nullable<Pf2SpellTradition>,
        ability -> Nullable<Pf2Ability>,
        proficiency -> Nullable<Pf2Proficiency>,
        spontaneous -> Nullable<Bool>,
        casts_per_day -> Nullable<Array<Nullable<Int4>>>,
        max_spells_known -> Nullable<Array<Nullable<Int4>>>,
        item_bonus -> Nullable<Int4>,
        misc_bonus -> Nullable<Int4>,
    }
}

diesel::table! {
    pf2_character_spells_prepared (id) {
        id -> Int4,
        spell_id -> Nullable<Int4>,
        level_prepared -> Nullable<Int4>,
    }
}

diesel::table! {
    pf2_character_statuses (id) {
        id -> Int4,
        character_id -> Nullable<Int4>,
        status_name -> Nullable<Text>,
        status_description -> Nullable<Text>,
    }
}

diesel::table! {
    pf2_character_stored_items (id) {
        id -> Int4,
        item_id -> Nullable<Int4>,
    }
}

diesel::table! {
    pf2_character_worn_items (id) {
        id -> Int4,
        item_id -> Nullable<Int4>,
        invested -> Nullable<Bool>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2Ability;
    use super::sql_types::Pf2Proficiency;
    use super::sql_types::Pf2WeaponSpec;

    pf2_characters (id) {
        id -> Int4,
        character_name -> Nullable<Text>,
        alignment -> Nullable<Text>,
        ancestry -> Nullable<Text>,
        background -> Nullable<Text>,
        character_class -> Nullable<Text>,
        key_ability -> Nullable<Text>,
        lvl -> Nullable<Int4>,
        hero_points -> Nullable<Int4>,
        str_bonus -> Nullable<Int4>,
        dex_bonus -> Nullable<Int4>,
        con_bonus -> Nullable<Int4>,
        int_bonus -> Nullable<Int4>,
        wis_bonus -> Nullable<Int4>,
        cha_bonus -> Nullable<Int4>,
        active_apex_item -> Nullable<Text>,
        active_apex_item_bonus -> Nullable<Pf2Ability>,
        temp_hp -> Nullable<Int4>,
        damage -> Nullable<Int4>,
        dying -> Nullable<Int4>,
        wound -> Nullable<Int4>,
        doom -> Nullable<Int4>,
        fort_prof -> Nullable<Pf2Proficiency>,
        fort_misc_bonus -> Nullable<Int4>,
        refl_prof -> Nullable<Pf2Proficiency>,
        refl_misc_bonus -> Nullable<Int4>,
        will_prof -> Nullable<Pf2Proficiency>,
        will_misc_bonus -> Nullable<Int4>,
        perception_prof -> Nullable<Pf2Proficiency>,
        perception_misc_bonus -> Nullable<Int4>,
        base_land_speed -> Nullable<Int4>,
        base_fly_speed -> Nullable<Int4>,
        base_swim_speed -> Nullable<Int4>,
        base_burrow_speed -> Nullable<Int4>,
        base_climb_speed -> Nullable<Int4>,
        max_focus_points -> Nullable<Int4>,
        current_focus_points -> Nullable<Int4>,
        simple_weapon_prof -> Nullable<Pf2Proficiency>,
        martial_weapon_prof -> Nullable<Pf2Proficiency>,
        weapon_spec -> Nullable<Pf2WeaponSpec>,
        unarmored_prof -> Nullable<Pf2Proficiency>,
        light_armor_prof -> Nullable<Pf2Proficiency>,
        med_armor_prof -> Nullable<Pf2Proficiency>,
        heavy_armor_prof -> Nullable<Pf2Proficiency>,
        class_prof -> Nullable<Pf2Proficiency>,
    }
}

diesel::table! {
    pf2_class_features (id) {
        id -> Int4,
        character_id -> Nullable<Int4>,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    pf2_item_traits (id) {
        id -> Int4,
        item_id -> Nullable<Int4>,
        trait_id -> Nullable<Int4>,
    }
}

diesel::table! {
    pf2_items (id) {
        id -> Int4,
        item_name -> Nullable<Text>,
        item_description -> Nullable<Text>,
        bulk -> Nullable<Int4>,
        price -> Nullable<Int4>,
        lvl -> Nullable<Int4>,
        invested -> Nullable<Bool>,
    }
}

diesel::table! {
    pf2_items_in_containers (id) {
        id -> Int4,
        bag_id -> Nullable<Int4>,
        item_id -> Nullable<Int4>,
    }
}

diesel::table! {
    pf2_shield (id) {
        id -> Int4,
        item_id -> Nullable<Int4>,
        ac_bonus -> Nullable<Int4>,
        hardness -> Nullable<Int4>,
        hp -> Nullable<Int4>,
        bp -> Nullable<Int4>,
        speed_penalty -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2Ability;

    pf2_skills (id) {
        id -> Int4,
        title -> Nullable<Text>,
        ability -> Nullable<Pf2Ability>,
    }
}

diesel::table! {
    pf2_traits (id) {
        id -> Int4,
        trait_name -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pf2WeaponType;
    use super::sql_types::Pf2WeaponCategory;

    pf2_weapon (id) {
        id -> Int4,
        item_id -> Nullable<Int4>,
        weapon_type -> Nullable<Pf2WeaponType>,
        weapon_cat -> Nullable<Pf2WeaponCategory>,
        group_id -> Nullable<Int4>,
        damage_die -> Nullable<Int4>,
        hands -> Nullable<Text>,
        weapon_range -> Nullable<Int4>,
    }
}

diesel::table! {
    pf2_weapon_group (id) {
        id -> Int4,
        group_name -> Nullable<Text>,
        crit_spec -> Nullable<Text>,
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
diesel::joinable!(pf2_character_item_attached_runes -> pf2_character_items (item_id));
diesel::joinable!(pf2_character_item_attached_runes -> pf2_items (rune_id));
diesel::joinable!(pf2_character_items -> pf2_characters (character_id));
diesel::joinable!(pf2_character_items -> pf2_items (item_id));
diesel::joinable!(pf2_character_languages -> pf2_characters (character_id));
diesel::joinable!(pf2_character_readied_items -> pf2_items (item_id));
diesel::joinable!(pf2_character_senses -> pf2_characters (character_id));
diesel::joinable!(pf2_character_skills -> pf2_characters (character_id));
diesel::joinable!(pf2_character_skills -> pf2_skills (skill_id));
diesel::joinable!(pf2_character_spell_known -> pf2_character_spellcasting_tables (table_id));
diesel::joinable!(pf2_character_spellcasting_tables -> pf2_characters (character_id));
diesel::joinable!(pf2_character_spells_prepared -> pf2_character_spell_known (spell_id));
diesel::joinable!(pf2_character_statuses -> pf2_characters (character_id));
diesel::joinable!(pf2_character_stored_items -> pf2_character_items (item_id));
diesel::joinable!(pf2_character_worn_items -> pf2_items (item_id));
diesel::joinable!(pf2_class_features -> pf2_characters (character_id));
diesel::joinable!(pf2_item_traits -> pf2_items (item_id));
diesel::joinable!(pf2_item_traits -> pf2_traits (trait_id));
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
    pf2_item_traits,
    pf2_items,
    pf2_items_in_containers,
    pf2_shield,
    pf2_skills,
    pf2_traits,
    pf2_weapon,
    pf2_weapon_group,
);
