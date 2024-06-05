-- Character:
--     Alignment
--     Ancestry
--     Background
--     Class
--     Key Ability

--     Abilities
--         Str
--             base
--             level bonus
--         Dex
--             base
--             level bonus
--         Con
--             base
--             level bonus
--         Int
--             base
--             level bonus
--         Wis
--             base
--             level bonus
--         Cha
--             base
--             level bonus
--         Apex item
--     Health
--         Max HP
--         Temp HP
--         Damage
--         Dying value
--         Wound value
--         Doom value

--     Resistances

--     AC
--     Saves
--         Fort
--             Proficiency
--         Refl
--             Proficiency
--         Will
--             Proficiency
    
--     Strides
--         Land
--         Fly
--         Swim
--         Burrow
--         Climb
    
--     Senses
--         Perception
--             Proficiency
--         Abnormal Senses : List
--     Hero Points
--     Focus
--         Max Focus points
--         Current Focus points
    
--     Skills
--         <skill>
--             Proficiency
    
--     Attacks
--         Proficiency, matk, mdmg, fundamental runes, attack bonus map chart, [<damage die>|<damage bonus>]<damage type>..., traits
--     Weapon Proficiency
--         Simple
--         Martial
--         Weapon Spec
--     Armor Proficiency
--         Unarmored
--         Light
--         Medium
--         Heavy
--     Class DC

--     Ancestry Features
--         <List of ancestry features>
--     Class Features
--         <List of class features>
--     Feats
--         <List of feats>
    
--     Inventory
--         Worn/Invested
--             Armor
--             <Other worn items>
--         Readied Items
--             <List of readied items>
--         Stored
--             <List of stored items>

--     // Data columns for each tradition known
--     Spellcasting Data
--         Tradition
--         Ability
--         Spellcasting Proficiency
--         Spells Known/x per day
--         Attack
--         DC
--         Spells Known
--             <list of spells known by level>
--                 Prepared

--     Formula Book
--         <Formula>
    
--     Statuses
--         <List of Statuses>



CREATE TABLE IF NOT EXISTS pf2_character (
    id serial PRIMARY KEY,
    title text,

    ancestry_id integer REFERENCES pf2_ancestry(id),
    background_id integer REFERENCES pf2_background(id),
    class_id integer REFERENCES pf2_class(id),

    lvl integer DEFAULT 1,
    alignment text,

    damage integer DEFAULT 0,

    hero_points integer
);

-- Inventory

CREATE TABLE IF NOT EXISTS pf2_character_inventory_item (
    character_id integer REFERENCES pf2_character(id),
    item_id integer REFERENCES items(id)
);

CREATE TABLE IF NOT EXISTS pf2_character_container (
    bag_id serial PRIMARY KEY,
    character_id integer REFERENCES pf2_character(id),
    bag_item_id integer REFERENCES items(id)
);

CREATE TABLE IF NOT EXISTS pf2_item_in_container (
    item_id integer REFERENCES items(id),
    bag_id integer REFERENCES pf2_character_container(bag_id)
);

-- Weapons and Armor

CREATE TABLE IF NOT EXISTS pf2_worn_armor (
    character_id integer REFERENCES pf2_character,
    armor_id
);

-- Stats

CREATE TABLE IF NOT EXISTS pf2_character_ability (
    ability_id integer REFERENCES pf2_ability(id)
);

CREATE TABLE IF NOT EXISTS pf2_character_skill (
    character_id integer REFERENCES pf2_character(id),
    character_skill integer REFERENCES pf2_skill(id),
    expertise pf2_expertise DEFAULT 'untrained'
);
