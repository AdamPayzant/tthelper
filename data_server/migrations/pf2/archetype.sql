-- Archetype Setup
CREATE TABLE IF NOT EXISTS pf2_archetype (
    id serial PRIMARY KEY,
    title text,
    description text,
    url text REFERENCES pf2_url(url),
    rarity pf2_rarity DEFAULT 'common'
);

CREATE TABLE IF NOT EXISTS pf2_archetype_feat (
    archetype_id integer REFERENCES pf2_archetype(id),
    feat_id integer REFERENCES pf2_feat(feat_id)
);
