CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY,
    username text UNIQUE NOT NULL,
    password text NOT NULL
);

CREATE TYPE pf2_ability AS ENUM (
    'strength',
    'dexterity',
    'constitution',
    'intelligence',
    'wisdom',
    'charisma'
);

CREATE TYPE pf2_proficiency AS ENUM (
    'untrained',
    'trained',
    'expert',
    'master',
    'legendary'
);

CREATE TYPE pf2_weapon_spec AS ENUM (
    'none',
    'ws',
    'gws'
);

CREATE TYPE pf2_damage_type_modifier AS ENUM (
    'weakness',
    'resistance',
    'immunity'
);

CREATE TYPE pf2_armor_type AS ENUM (
    'unarmored',
    'light',
    'medium',
    'heavy'
);

CREATE TYPE pf2_spell_tradition AS ENUM (
    'arcane',
    'divine',
    'elemental',
    'occult',
    'primal',
    'focus'
);

CREATE TYPE pf2_action AS ENUM (
    'free',
    'reaction',
    'one',
    'two',
    'three',
    'one_to_three',
    'two_to_three'
);

CREATE TYPE pf2_weapon_type AS ENUM (
    'melee',
    'ranged'
);

CREATE TYPE pf2_weapon_category AS ENUM (
    'unarmed',
    'simple',
    'martial',
    'advanced'
);

CREATE TYPE pf2_attack_type AS ENUM (
    'str_str',
    'dex_str',
    'dex_dex',
    'ranged_dex_half_str',
    'ranged_dex_str',
    'athletics'
);

-- System data

CREATE TABLE IF NOT EXISTS pf2_traits (
    id serial PRIMARY KEY,
    trait_name text NOT NULL,
    description text DEFAULT '' NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_skills (
    id serial PRIMARY KEY,
    title text NOT NULL,
    ability pf2_ability NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_weapon_group (
    id serial PRIMARY KEY,
    group_name text NOT NULL,
    crit_spec text DEFAULT '' NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_armor_group (
    id serial PRIMARY KEY,
    group_name text NOT NULL,
    armor_spec text DEFAULT '' NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_items (
    id serial PRIMARY KEY,
    item_name text NOT NULL,
    item_description text DEFAULT '' NOT NULL,
    bulk integer DEFAULT 0 NOT NULL,
    price integer DEFAULT 0 NOT NULL,
    lvl integer DEFAULT 0 NOT NULL,
    invested boolean DEFAULT false NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_item_traits (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_items NOT NULL,
    trait_id integer REFERENCES pf2_traits NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_weapon (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_items(id) NOT NULL,
    weapon_type pf2_weapon_type NOT NULL,
    weapon_cat pf2_weapon_category NOT NULL,
    group_id integer REFERENCES pf2_weapon_group,
    damage_die integer NOT NULL,
    hands text NOT NULL,
    weapon_range integer DEFAULT null -- null if melee
);

CREATE TABLE IF NOT EXISTS pf2_armor (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_items(id) NOT NULL,
    ac_bonus integer DEFAULT 0 NOT NULL,
    max_dex integer DEFAULT null,
    check_penalty integer DEFAULT 0 NOT NULL,
    speed_penalty integer DEFAULT 0 NOT NULL,
    str_requirement integer DEFAULT 0 NOT NULL,
    armor_type pf2_armor_type NOT NULL,
    armor_group integer REFERENCES pf2_armor_group(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_shield (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_items(id) NOT NULL,
    ac_bonus integer DEFAULT 0 NOT NULL,
    hardness integer NOT NULL,
    hp integer NOT NULL,
    bp integer NOT NULL,
    speed_penalty integer DEFAULT 0 NOT NULL
);

-- Character Data

CREATE TABLE IF NOT EXISTS pf2_characters (
    id serial PRIMARY KEY,
    owner integer REFERENCES users(id) ON DELETE CASCADE NOT NULL,

    character_name text NOT NULL,
    alignment text NOT NULL,
    ancestry text NOT NULL,
    background text NOT NULL,
    character_class text NOT NULL,
    key_ability text NOT NULL,
    lvl integer DEFAULT 1 NOT NULL,
    exp integer DEFAULT 0 NOT NULL,

    hero_points integer DEFAULT 1 NOT NULL,

    -- Abilities
        str_base integer DEFAULT 10 NOT NULL,
        str_bonus integer DEFAULT 0 NOT NULL,
        dex_base integer DEFAULT 10 NOT NULL,
        dex_bonus integer DEFAULT 0 NOT NULL,
        con_base integer DEFAULT 10 NOT NULL,
        con_bonus integer DEFAULT 0 NOT NULL,
        int_base integer DEFAULT 10 NOT NULL,
        int_bonus integer DEFAULT 0 NOT NULL,
        wis_base integer DEFAULT 10 NOT NULL,
        wis_bonus integer DEFAULT 0 NOT NULL,
        cha_base integer DEFAULT 10 NOT NULL,
        cha_bonus integer DEFAULT 0 NOT NULL,

        active_apex_item text,
        active_apex_item_bonus pf2_ability,

    -- Health
        temp_hp integer DEFAULT 0 NOT NULL,
        damage integer DEFAULT 0 NOT NULL,
        dying integer DEFAULT 0 NOT NULL,
        wound integer DEFAULT 0 NOT NULL,
        doom integer DEFAULT 0 NOT NULL,

        misc_armor_bonus integer DEFAULT 0 NOT NULL,

    -- Saves
        fort_prof pf2_proficiency DEFAULT 'trained' NOT NULL,
        fort_misc_bonus integer DEFAULT 0 NOT NULL,
        refl_prof pf2_proficiency DEFAULT 'trained' NOT NULL,
        refl_misc_bonus integer DEFAULT 0 NOT NULL,
        will_prof pf2_proficiency DEFAULT 'trained' NOT NULL,
        will_misc_bonus integer DEFAULT 0 NOT NULL,

        perception_prof pf2_proficiency DEFAULT 'trained' NOT NULL,
        perception_misc_bonus integer DEFAULT 0 NOT NULL,

    -- Speeds
        base_land_speed integer,
        base_fly_speed integer,
        base_swim_speed integer,
        base_burrow_speed integer,
        base_climb_speed integer,


    -- Focus
        max_focus_points integer,
        current_focus_points integer,


    -- proficiencies
        simple_weapon_prof pf2_proficiency DEFAULT 'untrained' NOT NULL,
        martial_weapon_prof pf2_proficiency DEFAULT 'untrained' NOT NULL,
        weapon_spec pf2_weapon_spec DEFAULT 'none' NOT NULL,

        unarmored_prof pf2_proficiency DEFAULT 'untrained' NOT NULL,
        light_armor_prof pf2_proficiency DEFAULT 'untrained' NOT NULL,
        med_armor_prof pf2_proficiency DEFAULT 'untrained' NOT NULL,
        heavy_armor_prof pf2_proficiency DEFAULT 'untrained' NOT NULL,

        class_prof pf2_proficiency DEFAULT 'trained' NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_background_ability_bonus (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT NULL,
    ability pf2_ability NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_ancestry_ability_modifier (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT NULL,
    ability pf2_ability NOT NULL,
    positive_boost boolean DEFAULT true NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_ancestry_features (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT NULL,
    title text NOT NULL,
    description text DEFAULT '' NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_damage_type_modifier (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT NULL,
    modifier  pf2_damage_type_modifier NOT NULL,
    val integer -- NULL if immunity
);

CREATE TABLE IF NOT EXISTS pf2_character_senses (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT NULL,
    sense_name text NOT NULL,
    sense_description text DEFAULT '' NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_skills (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT NULL,
    skill_id integer REFERENCES pf2_skills(id) NOT NULL,
    proficiency pf2_proficiency DEFAULT 'untrained' NOT NULL,
    bonuses integer DEFAULT 0 NOT NULL,

    assurance boolean DEFAULT false NOT NULL,
    extra_name text -- For lores
);

CREATE TABLE IF NOT EXISTS pf2_character_languages (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT NULL,
    title text NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_class_features (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT NULL,
    title text NOT NULL,
    description text DEFAULT '' NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_feats (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT NULL,
    title text NOT NULL,
    description text DEFAULT '' NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_armor_traits (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT  NULL,
    trait_name text NOT NULL,
    description text DEFAULT '' NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_spellcasting_tables (
    -- Does not include rituals, those are their own thing

    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT NULL,
    tradition pf2_spell_tradition NOT NULL,
    ability pf2_ability NOT NULL,
    proficiency pf2_proficiency NOT NULL,
    spontaneous boolean DEFAULT false NOT NULL,

    casts_per_day integer[10] NOT NULL,
    spells_known integer[10] NOT NULL,

    item_bonus integer DEFAULT 0 NOT NULL,
    misc_bonus integer DEFAULT 0 NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_spell_known (
    id serial PRIMARY KEY,
    spellcasting_table_id integer REFERENCES pf2_character_spellcasting_tables ON DELETE CASCADE NOT NULL,
    spell_name text NOT NULL,
    action_length pf2_action NOT NULL,
    base_level integer NOT NULL,
    duration text,
    spell_range text,
    area text,
    spell_description text DEFAULT '' NOT NULL,

    heightening text
);

CREATE TABLE IF NOT EXISTS pf2_character_spells_prepared (
    id serial PRIMARY KEY,
    spellcasting_table_id integer REFERENCES pf2_character_spellcasting_tables(id) ON DELETE CASCADE NOT NULL,
    spell_id integer REFERENCES pf2_character_spell_known(id) ON DELETE CASCADE NOT NULL,
    level_prepared integer NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_statuses (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT  NULL,
    status_name text NOT NULL,
    status_description text DEFAULT '' NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_formula_books (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT NULL,
    item_id integer REFERENCES pf2_items(id) NOT NULL
);

-- Inventory details

CREATE TABLE IF NOT EXISTS pf2_character_items (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT NULL,
    item_id integer REFERENCES pf2_items(id) NOT NULL,
    quantity integer NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_stored_items (
    id serial PRIMARY KEY NOT NULL,
    item_id integer REFERENCES pf2_character_items(id) ON DELETE CASCADE NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_containers (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_character_items(id) ON DELETE CASCADE NOT NULL,
    bulk_reduction integer NOT NULL,
    max_bulk integer  NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_items_in_containers (
    id serial PRIMARY KEY,
    character_containers_id integer REFERENCES pf2_character_containers(id)  NOT NULL,
    item_id integer REFERENCES pf2_character_items(id) ON DELETE CASCADE NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_readied_items (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_character_items(id) ON DELETE CASCADE NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_worn_items (
    id serial PRIMARY KEY,
    item_id integer REFERENCES pf2_character_items(id) ON DELETE CASCADE NOT NULL,
    invested boolean DEFAULT false NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_item_attached_runes (
    id serial PRIMARY KEY,
    character_item_id integer REFERENCES pf2_character_items(id) ON DELETE CASCADE NOT NULL,
    rune_id integer REFERENCES pf2_items(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS pf2_character_attacks (
    id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_characters(id) ON DELETE CASCADE NOT NULL,
    item_id integer REFERENCES pf2_character_items(id) ON DELETE CASCADE, -- Optional

    proficiency pf2_proficiency NOT NULL,
    matk integer DEFAULT 0 NOT NULL,
    mdmg integer DEFAULT 0 NOT NULL,
    attack_type pf2_attack_type NOT NULL,

    damage_die integer DEFAULT null -- Only used to override
);

-- Functions

CREATE OR REPLACE FUNCTION init_pf2_data()
    RETURNS void LANGUAGE plpgsql
    AS $func$
    BEGIN
        PERFORM 1 FROM pg_tables WHERE schemaname = 'public' AND tablename = 'pf2_initialization_log';

        IF NOT FOUND THEN
            -- Create a log table to track initialization
            CREATE TABLE pf2_initialization_log (
                id serial PRIMARY KEY,
                initialized_at TIMESTAMP DEFAULT current_timestamp
            );

            INSERT INTO pf2_skills (title, ability) VALUES
                ('Acrobatics', 'dexterity'),
                ('Arcana', 'intelligence'),
                ('Athletics', 'strength'),
                ('Crafting', 'intelligence'),
                ('Deception', 'charisma'),
                ('Diplomacy', 'charisma'),
                ('Intimidation', 'charisma'),
                ('Medicine', 'wisdom'),
                ('Nature', 'wisdom'),
                ('Occultism', 'intelligence'),
                ('Performance', 'charisma'),
                ('Religion', 'wisdom'),
                ('Society', 'intelligence'),
                ('Stealth', 'dexterity'),
                ('Survival', 'wisdom'),
                ('Thievery', 'dexterity'),
                ('Lore', 'intelligence');

            -- Log the initialization
            INSERT INTO pf2_initialization_log (initialized_at) VALUES (current_timestamp);

            RAISE NOTICE 'PF2 database initialization completed successfully.';
        ELSE
            RAISE NOTICE 'PF2 database has already been initialized.';
        END IF;
    END;
    $func$;

CREATE OR REPLACE FUNCTION add_new_pf2_character_defaults()
    RETURNS TRIGGER AS $$
    BEGIN

        INSERT INTO pf2_character_skills (character_id, skill_id)
            SELECT NEW.id, s.id FROM pf2_skills s;

        RETURN NEW;
    END;
    $$ LANGUAGE plpgsql;

-- Set up triggers and run init

CREATE TRIGGER new_character AFTER INSERT ON pf2_characters
    FOR EACH ROW
    EXECUTE FUNCTION add_new_pf2_character_defaults();

SELECT "init_pf2_data"();


-- Test character command
-- INSERT INTO pf2_characters (character_name, owner, alignment, ancestry, background, character_class, key_ability) VALUES ('Grog', 1, 't' , 't', 't', 't', 't');
