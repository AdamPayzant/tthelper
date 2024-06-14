CREATE TYPE pf2_ability AS ENUM (
    'Strength',
    'Dexterity',
    'Constitution',
    'Intelligence',
    'Wisdom',
    'Charisma'
);

CREATE TYPE pf2_proficiency AS ENUM (
    'Untrained',
    'Trained',
    'Expert',
    'Master',
    'Legendary'
);

CREATE TYPE pf2_weapon_spec AS ENUM (
    'None',
    'WS',
    'GWS'
);

CREATE TYPE pf2_damage_type_modifier AS ENUM (
    'Weakness',
    'Resistance',
    'Immunity'
);

CREATE TYPE pf2_armor_type AS ENUM (
    'Unarmored',
    'Light',
    'Medium',
    'Heavy'
);

CREATE TYPE pf2_spell_tradition AS ENUM (
    'Arcane',
    'Divine',
    'Elemental',
    'Occult',
    'Primal',
    'Focus'
);

CREATE TYPE pf2_action AS ENUM (
    'Free',
    'Reaction',
    'One',
    'Two',
    'Three',
    'OneToThree',
    'TwoToThree'
);

CREATE TYPE pf2_weapon_type AS ENUM (
    'Melee',
    'Ranged'
);

CREATE TYPE pf2_weapon_category AS ENUM (
    'Unarmed',
    'Simple',
    'Martial',
    'Advanced'
);

CREATE TYPE pf2_attack_type AS ENUM (
    'StrStr',
    'DexStr',
    'DexDex',
    'RangedDexHalfStr',
    'RangedDexStr',
    'Athletics'
);

-- System data

CREATE TABLE IF NOT EXISTS pf2_traits (
    id serial PRIMARY KEY,
    trait_name text,
    description text
);

CREATE TABLE IF NOT EXISTS pf2_skills (
    id serial PRIMARY KEY,
    title text,
    ability pf2_ability
);

CREATE TABLE IF NOT EXISTS pf2_weapon_group (
    id serial PRIMARY KEY,
    group_name text,
    crit_spec text
);

CREATE TABLE IF NOT EXISTS pf2_armor_group (
    id serial PRIMARY KEY,
    group_name text,
    armor_spec text
);

CREATE TABLE IF NOT EXISTS pf2_items (
    id serial PRIMARY KEY,
    item_name text,
    item_description text,
    bulk integer,
    price integer,
    lvl integer,
    invested boolean DEFAULT false
);

CREATE TABLE IF NOT EXISTS pf2_item_traits (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_items,
    trait_id integer REFERENCES pf2_traits
);

CREATE TABLE IF NOT EXISTS pf2_weapon (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_items(id),
    weapon_type pf2_weapon_type,
    weapon_cat pf2_weapon_category,
    group_id integer REFERENCES pf2_weapon_group,
    damage_die integer,
    hands text,
    weapon_range integer DEFAULT null -- Optional
);

CREATE TABLE IF NOT EXISTS pf2_armor (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_items(id),
    ac_bonus integer,
    max_dex integer,
    check_penalty integer,
    speed_penalty integer,
    str_requirement integer,
    armor_type pf2_armor_type,
    armor_group integer REFERENCES pf2_armor_group(id)
);

CREATE TABLE IF NOT EXISTS pf2_shield (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_items(id),
    ac_bonus integer,
    hardness integer,
    hp integer,
    bp integer,
    speed_penalty integer
);

-- Character Data

CREATE TABLE IF NOT EXISTS pf2_characters (
    id serial PRIMARY KEY,
    character_name text,
    alignment text,
    ancestry text,
    background text,
    character_class text,
    key_ability text,
    lvl integer,

    hero_points integer,

    -- Abilities
        str_bonus integer,
        dex_bonus integer,
        con_bonus integer,
        int_bonus integer,
        wis_bonus integer,
        cha_bonus integer,

        active_apex_item text,
        active_apex_item_bonus pf2_ability,

    -- Health
        temp_hp integer,
        damage integer,
        dying integer,
        wound integer,
        doom integer,

    -- Saves
        fort_prof pf2_proficiency,
        fort_misc_bonus integer,
        refl_prof pf2_proficiency,
        refl_misc_bonus integer,
        will_prof pf2_proficiency,
        will_misc_bonus integer,

        perception_prof pf2_proficiency,
        perception_misc_bonus integer,

    -- Speeds
        base_land_speed integer DEFAULT 0,
        base_fly_speed integer DEFAULT 0,
        base_swim_speed integer DEFAULT 0,
        base_burrow_speed integer DEFAULT 0,
        base_climb_speed integer DEFAULT 0,


    -- Focus
        max_focus_points integer DEFAULT 0,
        current_focus_points integer DEFAULT 0,
    

    -- proficiencies
        simple_weapon_prof pf2_proficiency DEFAULT 'Untrained',
        martial_weapon_prof pf2_proficiency DEFAULT 'Untrained',
        weapon_spec pf2_weapon_spec DEFAULT 'None',

        unarmored_prof pf2_proficiency DEFAULT 'Untrained',
        light_armor_prof pf2_proficiency DEFAULT 'Untrained',
        med_armor_prof pf2_proficiency DEFAULT 'Untrained',
        heavy_armor_prof pf2_proficiency DEFAULT 'Untrained',

        class_prof pf2_proficiency DEFAULT 'Trained'
);

CREATE TABLE IF NOT EXISTS pf2_character_background_ability_bonus (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    ability pf2_ability
);

CREATE TABLE IF NOT EXISTS pf2_character_ancestry_ability_modifier (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    ability pf2_ability,
    positive_boost boolean DEFAULT true
);

CREATE TABLE IF NOT EXISTS pf2_character_ancestry_features (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    title text,
    description text
);

CREATE TABLE IF NOT EXISTS pf2_character_damage_type_modifier (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    modifier  pf2_damage_type_modifier,
    val integer -- NULL if immunity
);

CREATE TABLE IF NOT EXISTS pf2_character_senses (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    sense_name text,
    sense_description text
);

CREATE TABLE IF NOT EXISTS pf2_character_skills (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    skill_id integer REFERENCES pf2_skills(id),
    proficiency pf2_proficiency DEFAULT 'Untrained',
    bonuses integer,

    assurance boolean DEFAULT false
);

CREATE TABLE IF NOT EXISTS pf2_character_languages (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    title text
);

CREATE TABLE IF NOT EXISTS pf2_class_features (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    title text,
    description text
);

CREATE TABLE IF NOT EXISTS pf2_character_feats (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    title text,
    description text
);

CREATE TABLE IF NOT EXISTS pf2_character_armor_traits (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    trait_name text,
    description text
);

CREATE TABLE IF NOT EXISTS pf2_character_spellcasting_tables (
    -- Does not include rituals, those are their own thing

    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    tradition pf2_spell_tradition,
    ability pf2_ability,
    proficiency pf2_proficiency,
    spontaneous boolean DEFAULT false,

    casts_per_day integer[10],
    max_spells_known integer[10],

    item_bonus integer DEFAULT 0,
    misc_bonus integer DEFAULT 0
);

CREATE TABLE IF NOT EXISTS pf2_character_spell_known (
    id serial PRIMARY KEY,
    table_id integer REFERENCES pf2_character_spellcasting_tables,
    spell_name text,
    action_length pf2_action,
    base_level integer,
    duration text,
    spell_range text,
    area text,
    description text,

    heightening text
);

CREATE TABLE IF NOT EXISTS pf2_character_spells_prepared (
    id serial PRIMARY KEY,
    spell_id integer REFERENCES pf2_character_spell_known(id),
    level_prepared integer
);

CREATE TABLE IF NOT EXISTS pf2_character_statuses (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    status_name text,
    status_description text
);

CREATE TABLE IF NOT EXISTS pf2_character_formula_books (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    item_id integer REFERENCES pf2_items(id)
);

-- Inventory details

CREATE TABLE IF NOT EXISTS pf2_character_items (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    item_id integer REFERENCES pf2_items(id),
    quantity integer
);

CREATE TABLE IF NOT EXISTS pf2_character_stored_items (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_character_items(id)
);

CREATE TABLE IF NOT EXISTS pf2_character_containers (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_character_items(id),
    bulk_reduction integer,
    max_bulk integer
);

CREATE TABLE IF NOT EXISTS pf2_items_in_containers (
    id serial PRIMARY KEY,
    bag_id integer REFERENCES pf2_character_items(id),
    item_id integer REFERENCES pf2_character_items(id)
);

CREATE TABLE IF NOT EXISTS pf2_character_readied_items (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_items(id)
);

CREATE TABLE IF NOT EXISTS pf2_character_worn_items (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_items(id),
    invested boolean DEFAULT false
);

CREATE TABLE IF NOT EXISTS pf2_character_item_attached_runes (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_character_items(id),
    rune_id integer REFERENCES pf2_items(id)
);

CREATE TABLE IF NOT EXISTS pf2_character_attacks (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id),
    item_id integer REFERENCES pf2_character_items(id), -- Optional

    proficiency pf2_proficiency,
    matk integer DEFAULT 0,
    mdmg integer DEFAULT 0,
    attack_type pf2_attack_type,

    damage_die integer DEFAULT null -- Only used to override
);
