CREATE TABLE IF NOT EXISTS pf2_ancestry (
    id integer PRIMARY KEY,
    title text,
    hit_points integer,
    size pf2_size_category,
    speed integer,
    vision pf2_vision DEFAULT 'normal'
    url text REFERENCES pf2_url(url)
);

CREATE TABLE IF NOT EXISTS pf2_ancestry_ability_boost (
    ancestry_id integer REFERENCES pf2_ancestry(id),
    ability_id integer REFERENCES pf2_ability(id)
);

CREATE TABLE IF NOT EXISTS pf2_ancestry_ability_flaw (
    ancestry_id integer REFERENCES pf2_ancestry(id),
    ability_id integer REFERENCES pf2_ability(id)
);

CREATE TABLE IF NOT EXISTS pf2_ancestry_has_trait (
    ancestry_id integer REFERENCES pf2_ancestry(id),
    trait_id integer REFERENCES pf2_trait(id)
);

CREATE TABLE IF NOT EXISTS pf2_ancestry_feature (
    ancestry_id integer REFERENCES pf2_ancestry(id),
    description text
);

CREATE TABLE IF NOT EXISTS pf2_ancestry_heritage (
    ancestry_id integer REFERENCES pf2_ancestry(id),
    description text
);
