-- types

CREATE TYPE pf2_expertise AS ENUM (
    'untrained',
    'trained',
    'expert',
    'master',
    'legendary'
);

CREATE TYPE pf2_weapon_category AS ENUM (
    'unarmed',
    'simple',
    'martial',
    'advanced'
);

CREATE TYPE pf2_armor_category as ENUM (
    'unarmored',
    'light',
    'medium',
    'heavy'
);

CREATE TYPE pf2_action_length as ENUM (
    'free',
    'reaction',
    'single',
    'two',
    'three'
);

CREATE TYPE pf2_rarity as ENUM (
    'common',
    'uncommon',
    'rare'
);

CREATE TYPE pf2_size_category as ENUM (
    'tiny',
    'small',
    'medium',
    'large',
    'huge',
    'gargantuan'
);

CREATE TYPE pf2_vision as ENUM (
    'normal',
    'low-light',
    'darkvision'
);

-- System tables

CREATE TABLE IF NOT EXISTS pf2_ability (
    id serial PRIMARY KEY,
    title text
);

CREATE TABLE IF NOT EXISTS pf2_skill (
    id serial PRIMARY KEY,
    title text,
    url text,
    last_updated timestamp,
    attr integer REFERENCES pf2_attribute(id)
);

CREATE TABLE IF NOT EXISTS pf2_skill_lore (
    skill_id integer REFERENCES pf2_skill(id),
    lore_name text
);

CREATE TABLE IF NOT EXISTS pf2_action (
    id serial PRIMARY KEY,
    title text,
    description text,
    duration pf2_action_length
);

CREATE TABLE IF NOT EXISTS pf2_url (
    url text PRIMARY KEY,
    last_updated timestamp
);

CREATE TABLE IF NOT EXISTS pf2_trait (
    id serial PRIMARY KEY,
    title text,
    description text,
    url text REFERENCES pf2_url(url)
);

CREATE TABLE IF NOT EXISTS pf2_languages (
    title text PRIMARY KEY
);

-- Feat Setup

CREATE TABLE IF NOT EXISTS pf2_feat (
    id serial PRIMARY KEY,
    title text,
    description text,
    url text REFERENCES pf2_url(url)
);

CREATE TABLE IF NOT EXISTS pf2_feat_has_trait (
    feat_id integer REFERENCES pf2_feat(id),
    trait_id integer REFERENCES pf2_trait(id)
);

-- Damage

CREATE TABLE IF NOT EXISTS pf2_damage_type (
    damage_id serial PRIMARY KEY,
    type_name text
);
