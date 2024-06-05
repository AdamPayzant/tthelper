CREATE TABLE IF NOT EXISTS pf2_creature (
    id serial PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS pf2_creature_has_trait (
    creature_id integer REFERENCES pf2_creature(id),
    trait_id integer REFERENCES pf2_trait(id)
);