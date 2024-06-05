-- Class Setup

CREATE TABLE IF NOT EXISTS pf2_class (
    id serial PRIMARY KEY,
    title text,
    url text REFERENCES pf2_url(url),

    description text,
    key_attribute integer REFERENCES pf2_ability(id),
    hp integer,

    perception pf2_expertise DEFAULT 'trained',

    fort pf2_expertise DEFAULT 'trained',
    reflex pf2_expertise DEFAULT 'trained',
    will pf2_expertise DEFAULT 'trained',
    
    additional_skills integer,

    weapon_unarmed_training pf2_expertise DEFAULT 'untrained',
    weapon_simple_training pf2_expertise DEFAULT 'untrained',
    weapon_martial_training pf2_expertise DEFAULT 'untrained',

    armor_unarmored_training pf2_expertise DEFAULT 'untrained',
    armor_light_training pf2_expertise DEFAULT 'untrained',
    armor_medium_training pf2_expertise DEFAULT 'untrained',
    armor_heavy_training pf2_expertise DEFAULT 'untrained'

    class_dc pf2_expertise
);

CREATE TABLE IF NOT EXISTS pf2_class_skill (
    class_id integer REFERENCES pf2_action(id),
    skill_id integer REFERENCES pf2_skill(id),
    training pf2_expertise DEFAULT 'trained'
);

CREATE TABLE IF NOT EXISTS pf2_class_feature (
    id serial PRIMARY KEY,
    class_id integer REFERENCES pf2_call(id),
    description text
);

CREATE TABLE IF NOT EXISTS pf2_feature_action (
    feature_id integer REFERENCES pf2_class_feature(id),
    action_id integer REFERENCES pf2_action(id)
);
