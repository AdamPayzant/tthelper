CREATE TABLE IF NOT EXISTS pf2_item (
    item_id integer REFERENCES items(id) unique,
    copper_value integer,
    bulk integer,
    rarity pf2_rarity
);

CREATE TABLE IF NOT EXISTS pf2_item_has_trait (
    item_id integer REFERENCES items(id),
    trait_id integer REFERENCES pf2_trait(id)
);

CREATE TABLE IF NOT EXISTS pf2_attached_rune (
    rune_id integer REFERENCES items(id),
    item_id integer REFERENCES items(id)
);

-- Weapon data

CREATE TABLE IF NOT EXISTS pf2_weapon_group (
    id serial PRIMARY KEY,
    url text REFERENCES pf2_url(url),
    crit_spec text
);

CREATE TABLE IF NOT EXISTS pf2_weapon_details (
    url text REFERENCES pf2_url(url),
    last_updated timestamp,
    item_id integer REFERENCES items(id) unique,
    damage_die varchar(8),
    weapon_group integer REFERENCES pf2_weapon_group(id),
    ranged boolean,
    category pf2_weapon_category
);

-- Armor data

CREATE TABLE IF NOT EXISTS pf2_armor_group (
    id serial PRIMARY KEY,
    url text REFERENCES pf2_url(url),
    last_updated timestamp,
    specialization text
);

CREATE TABLE IF NOT EXISTS pf2_armor_details (
    url text REFERENCES pf2_url(url),
    last_updated timestamp,
    item_id integer REFERENCES items(id) unique,
    ac_bonus integer,
    dex_cap integer,
    check_penalty integer,
    speed_penalty integer,
    strength integer,
    armor_category pf2_armor_category,
    armor_group_id integer REFERENCES pf2_armor_group(id)
);

-- Containers

CREATE TABLE IF NOT EXISTS pf2_container (
    item_id integer REFERENCES items(id),
    bulk_capacity integer, -- 0 = negligable, 1 = light, 10+ = real bulk
    bulk_reduction integer DEFAULT null -- null = all bulk negated
);
