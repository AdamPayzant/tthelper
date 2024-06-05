CREATE TABLE IF NOT EXISTS pf2_background (
    id serial PRIMARY KEY,
    title text,
    description text,
    free_ability_boosts integer,
    url text REFERENCES pf2_url(url)
);

CREATE TABLE IF NOT EXISTS pf2_background_feat (
    background_id integer REFERENCES pf2_background(id),
    feat_id integer REFERENCES pf2_feat(id)
);

CREATE TABLE IF NOT EXISTS pf2_background_ability_boost (
    background_id integer REFERENCES pf2_background(id),
    ability_id integer REFERENCES pf2_ability(id)
);

CREATE TABLE IF NOT EXISTS pf2_background_skills (
    background_id integer REFERENCES pf2_background(id),
    skill_id integer REFERENCES pf2_skill(id)
);
