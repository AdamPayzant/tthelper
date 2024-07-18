use actix_session::Session;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::{r2d2, PgConnection, QueryDsl, SelectableHelper};

use log::{error, warn};

use crate::auth;
use crate::db::{db_enums, models, schema, user_mgmt};

#[derive(Debug, Serialize, Deserialize)]
struct AbilityModifier {
    id: i32,
    ability: db_enums::Pf2Ability,
    is_positive_boost: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct DamageRecievedModifier {
    id: i32,
    modifier_type: db_enums::Pf2DamageTypeModifier,
    value: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Description {
    id: i32,
    name: String,
    description: String,
}

type Trait = Description;
type Feat = Description;
type Sense = Description;
type ClassFeature = Description;
type AncestryFeature = Description;

#[derive(Debug, Serialize, Deserialize)]
struct Skill {
    id: i32,
    skill: String,
    ability: db_enums::Pf2Ability,

    proficiency: db_enums::Pf2Proficiency,
    bonuses: i32,
    assurance: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Spell {
    id: i32,
    name: String,
    description: String,
    heightening: Option<String>,

    action_length: db_enums::Pf2Action,
    base_level: i32,
    duration: Option<String>,
    spell_range: Option<String>,
    area: Option<String>,
}

impl Spell {
    pub fn get_list_of_spells(
        table: &models::SpellcastingTable,
        conn: &mut PgConnection,
    ) -> Vec<Spell> {
        match models::SpellKnown::belonging_to(table)
            .select(models::SpellKnown::as_select())
            .load(conn)
        {
            Ok(entries) => entries
                .into_iter()
                .map(|e: models::SpellKnown| Spell {
                    id: e.id,
                    name: e.spell_name,
                    description: e.spell_description,
                    heightening: e.heightening,
                    action_length: e.action_length,
                    base_level: e.base_level,
                    duration: e.duration,
                    spell_range: e.spell_range,
                    area: e.area,
                })
                .collect(),
            Err(e) => {
                if e != diesel::result::Error::NotFound {
                    warn!("DB error getting spells known for list {}", table.id);
                }
                return Vec::new();
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PreparedSpell {
    id: i32,
    spell_id: i32,
    level_prepared: i32,
}

impl PreparedSpell {
    pub fn get_list_of_prepped_spells(
        table: &models::SpellcastingTable,
        conn: &mut PgConnection,
    ) -> Option<Vec<PreparedSpell>> {
        if table.spontaneous {
            return None;
        }

        match models::SpellsPrepared::belonging_to(table)
            .select(models::SpellsPrepared::as_select())
            .load(conn)
        {
            Ok(entries) => Some(
                entries
                    .into_iter()
                    .map(|e: models::SpellsPrepared| PreparedSpell {
                        id: e.id,
                        spell_id: e.spell_id,
                        level_prepared: e.level_prepared,
                    })
                    .collect(),
            ),
            Err(e) => {
                if e != diesel::result::Error::NotFound {
                    warn!("DB error getting spells prepared for list {}", table.id);
                }
                return Some(Vec::new());
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SpellcastingTable {
    id: i32,
    tradition: db_enums::Pf2SpellTradition,
    ability: db_enums::Pf2Ability,
    prof: db_enums::Pf2Proficiency,
    spontaneous: bool,

    casts_per_day: Vec<Option<i32>>,
    num_spells_known: Vec<Option<i32>>,

    item_bonus: i32,
    misc_bonus: i32,

    spells_known: Vec<Spell>,
    spells_prepared: Option<Vec<PreparedSpell>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct InventoryItem {
    id: i32, // The id from pf2_character_items
    name: String,
    description: String,
    bulk: i32,
    price: i32,
    lvl: i32,
    invested: bool,
    traits: Vec<Trait>,
    quantity: i32,

    runes: Vec<InventoryItem>,
}

impl InventoryItem {
    fn gen_rune(
        attachment_id: i32,
        details: models::Item,
        conn: &mut PgConnection,
    ) -> InventoryItem {
        use schema::pf2_traits::dsl::pf2_traits;

        let traits: Vec<(models::ItemTrait, models::Trait)> =
            models::ItemTrait::belonging_to(&details)
                .inner_join(pf2_traits)
                .select((models::ItemTrait::as_select(), models::Trait::as_select()))
                .load(conn)
                .unwrap();

        InventoryItem {
            id: attachment_id,
            name: details.item_name,
            description: details.item_description,
            bulk: details.bulk,
            price: details.price,
            invested: details.invested,
            lvl: details.lvl,
            traits: traits
                .into_iter()
                .map(|t: (models::ItemTrait, models::Trait)| {
                    let (attachment, trait_details) = t;
                    Trait {
                        id: attachment.id,
                        name: trait_details.trait_name,
                        description: trait_details.description,
                    }
                })
                .collect(),
            quantity: 1,
            runes: Vec::new(),
        }
    }

    pub fn gen_item(
        inventory_details: models::CharacterItem,
        item_details: &models::Item,
        conn: &mut PgConnection,
    ) -> Option<InventoryItem> {
        use schema::{pf2_items::dsl::pf2_items, pf2_traits::dsl::pf2_traits};

        let traits = match models::ItemTrait::belonging_to(&item_details)
            .inner_join(pf2_traits)
            .select((models::ItemTrait::as_select(), models::Trait::as_select()))
            .load(conn)
        {
            Ok(t) => t,
            Err(_) => {
                return None;
            }
        };

        let rune_list: Vec<InventoryItem> =
            match models::AttachedRune::belonging_to(&inventory_details)
                .inner_join(
                    pf2_items.on(schema::pf2_items::dsl::id
                        .eq(schema::pf2_character_item_attached_runes::dsl::rune_id)),
                )
                .select((models::AttachedRune::as_select(), models::Item::as_select()))
                .load(conn)
            {
                Ok(r) => r,
                Err(_) => {
                    return None;
                }
            }
            .into_iter()
            .map(|r| {
                let (attachment, details) = r;
                InventoryItem::gen_rune(attachment.id, details, conn)
            })
            .collect();

        let item = InventoryItem {
            id: inventory_details.id,
            name: item_details.item_name.clone(),
            description: item_details.item_description.clone(),
            bulk: item_details.bulk,
            price: item_details.price,
            invested: item_details.invested,
            lvl: item_details.lvl,
            traits: traits
                .into_iter()
                .map(|t: (models::ItemTrait, models::Trait)| {
                    let (attachment, trait_details) = t;
                    Trait {
                        id: attachment.id,
                        name: trait_details.trait_name,
                        description: trait_details.description,
                    }
                })
                .collect(),
            quantity: inventory_details.quantity,
            runes: rune_list,
        };

        Some(item)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ItemContainer {
    id: i32,
    item: InventoryItem,
    bulk_reduction: i32,
    max_bulk: i32,
    contents: Vec<InventoryItem>,
}

impl ItemContainer {
    pub fn gen_with_contents(
        container: models::CharacterContainers,
        item_info: InventoryItem,
        conn: &mut PgConnection,
    ) -> ItemContainer {
        use schema::{pf2_character_items::dsl::pf2_character_items, pf2_items::dsl::pf2_items};

        let contained_items: Vec<(models::ContainedItem, (models::CharacterItem, models::Item))> =
            match models::ContainedItem::belonging_to(&container)
                .inner_join(pf2_character_items.inner_join(pf2_items))
                .select((
                    models::ContainedItem::as_select(),
                    (
                        models::CharacterItem::as_select(),
                        models::Item::as_select(),
                    ),
                ))
                .load(conn)
            {
                Ok(items) => items,
                Err(_) => {
                    return ItemContainer {
                        id: container.id,
                        item: item_info,
                        bulk_reduction: container.bulk_reduction,
                        max_bulk: container.max_bulk,
                        contents: Vec::new(),
                    };
                }
            };

        let stored_items: Vec<InventoryItem> = contained_items
            .into_iter()
            .filter_map(|(_, (inventory_details, item_details))| {
                // Skip entries we can't load
                InventoryItem::gen_item(inventory_details, &item_details, conn)
            })
            .collect();

        ItemContainer {
            id: container.id,
            item: item_info,
            bulk_reduction: container.bulk_reduction,
            max_bulk: container.max_bulk,
            contents: stored_items,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum StoredItem {
    Container(ItemContainer),
    Item(InventoryItem),
}

#[derive(Debug, Serialize, Deserialize)]
struct Armor {
    id: i32,
    item_details: InventoryItem,
    ac_bonus: i32,
    max_dex: Option<i32>,
    check_penalty: i32,
    speed_penalty: i32,
    str_requirement: i32,
    armor_type: db_enums::Pf2ArmorType,
    armor_group_name: String,
    armor_spec: String,
}

impl Armor {
    pub fn is_armor(item_id: i32, item: &InventoryItem, conn: &mut PgConnection) -> Option<Armor> {
        use schema::{pf2_armor::dsl::pf2_armor, pf2_armor_group::dsl::pf2_armor_group};

        match pf2_armor
            .inner_join(pf2_armor_group)
            .filter(schema::pf2_armor::dsl::id.eq(item_id))
            .select((models::Armor::as_select(), models::ArmorGroup::as_select()))
            .load(conn)
        {
            Ok(res) => {
                let (entry, group) = &res[0];
                Some(Armor {
                    id: entry.id,
                    item_details: item.clone(),
                    ac_bonus: entry.ac_bonus,
                    max_dex: entry.max_dex,
                    check_penalty: entry.check_penalty,
                    speed_penalty: entry.speed_penalty,
                    str_requirement: entry.str_requirement,
                    armor_type: entry.armor_type.clone(),
                    armor_group_name: group.group_name.clone(),
                    armor_spec: group.armor_spec.clone(),
                })
            }
            Err(_) => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Shield {
    id: i32,
    item_details: InventoryItem,
    ac_bonus: i32,
    hardness: i32,
    hp: i32,
    bp: i32,
    speed_penalty: i32,
}

impl Shield {
    pub fn is_shield(
        item_id: i32,
        item: &InventoryItem,
        conn: &mut PgConnection,
    ) -> Option<Shield> {
        use schema::pf2_shield::dsl::pf2_shield;

        match pf2_shield
            .filter(schema::pf2_shield::dsl::item_id.eq(item_id))
            .select(models::Shield::as_select())
            .load(conn)
        {
            Ok(e) => {
                let s = &e[0];
                Some(Shield {
                    id: s.id,
                    item_details: item.clone(),
                    ac_bonus: s.ac_bonus,
                    hardness: s.hardness,
                    hp: s.hp,
                    bp: s.bp,
                    speed_penalty: s.speed_penalty,
                })
            }
            Err(_) => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Weapon {
    id: i32,
    item_details: InventoryItem,
    weapon_type: db_enums::Pf2WeaponType,
    weapon_cat: db_enums::Pf2WeaponCategory,
    group_name: String,
    crit_spec: String,
    damage_die: i32,
    hands: String,
    weapon_range: Option<i32>,
}

impl Weapon {
    pub fn is_weapon(
        item_id: i32,
        item: &InventoryItem,
        conn: &mut PgConnection,
    ) -> Option<Weapon> {
        use schema::{pf2_weapon::dsl::pf2_weapon, pf2_weapon_group::dsl::pf2_weapon_group};
        match pf2_weapon
            .inner_join(pf2_weapon_group)
            .filter(schema::pf2_weapon::dsl::id.eq(item_id))
            .select((
                models::Weapon::as_select(),
                models::WeaponGroup::as_select(),
            ))
            .load(conn)
        {
            Ok(res) => {
                let (weapon, group) = &res[0];
                Some(Weapon {
                    id: weapon.id,
                    item_details: item.clone(),
                    weapon_type: weapon.weapon_type,
                    weapon_cat: weapon.weapon_cat,
                    group_name: group.group_name.clone(),
                    crit_spec: group.crit_spec.clone(),
                    damage_die: weapon.damage_die,
                    hands: weapon.hands.clone(),
                    weapon_range: weapon.weapon_range,
                })
            }
            Err(_) => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Attack {
    id: i32,
    weapon: Option<Weapon>,
    prof: db_enums::Pf2Proficiency,
    matk: i32,
    mdmg: i32,
    attack_type: db_enums::Pf2AttackType,
    damage_die: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    id: i32,
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Formula {
    id: i32,
    item_id: i32,
    item_name: String,
    level: i32,
    item_value: i32,
}

// A large structure to send full character details to a client.
// Ideally this should only be sent once when the character is requested
#[derive(Debug, Serialize, Deserialize)]
struct FullCharacterInfo {
    id: i32,
    name: String,
    alignment: String,
    ancestry: String,
    background: String,
    class: String,
    key_ability: String,
    lvl: i32,
    hero_points: i32,

    str_bonus: i32,
    dex_bonus: i32,
    con_bonus: i32,
    int_bonus: i32,
    wis_bonus: i32,
    cha_bonus: i32,

    active_apex_item: Option<String>,
    active_apex_item_bonus: Option<db_enums::Pf2Ability>,

    temp_hp: i32,
    damage: i32,
    dying: i32,
    wound: i32,
    doom: i32,

    fort_prof: db_enums::Pf2Proficiency,
    fort_misc_bonus: i32,
    refl_prof: db_enums::Pf2Proficiency,
    refl_misc_bonus: i32,
    will_prof: db_enums::Pf2Proficiency,
    will_misc_bonus: i32,
    perception_prof: db_enums::Pf2Proficiency,
    perception_misc_bonus: i32,

    base_land_speed: Option<i32>,
    base_fly_speed: Option<i32>,
    base_swim_speed: Option<i32>,
    base_burrow_speed: Option<i32>,
    base_climb_speed: Option<i32>,

    max_focus_points: Option<i32>,
    current_focus_points: Option<i32>,

    simple_weapon_prof: db_enums::Pf2Proficiency,
    martial_weapon_prof: db_enums::Pf2Proficiency,
    weapon_spec: db_enums::Pf2WeaponSpec,

    unarmored_prof: db_enums::Pf2Proficiency,
    light_armor_prof: db_enums::Pf2Proficiency,
    med_armor_prof: db_enums::Pf2Proficiency,
    heavy_armor_prof: db_enums::Pf2Proficiency,

    class_prof: db_enums::Pf2Proficiency,

    background_ability_boosts: Vec<AbilityModifier>,
    ancestry_ability_boosts: Vec<AbilityModifier>,

    ancestry_features: Vec<AncestryFeature>,
    class_features: Vec<ClassFeature>,
    damage_recieved_mods: Vec<DamageRecievedModifier>,

    senses: Vec<Sense>,
    skills: Vec<Skill>,
    languages: Vec<String>,

    feats: Vec<Feat>,

    casting_tables: Vec<SpellcastingTable>,

    stored_items: Vec<StoredItem>,
    readied_items: Vec<InventoryItem>,
    worn_items: Vec<InventoryItem>, // Excludes Armor/shield

    armor: Option<Armor>,
    shield: Option<Shield>,

    weapons: Vec<Weapon>,
    attacks: Vec<Attack>,

    statuses: Vec<Status>,

    formula_book: Vec<Formula>,
}

impl FullCharacterInfo {
    pub fn load_character(conn: &mut PgConnection, id: i32) -> Result<FullCharacterInfo, String> {
        use schema::{
            pf2_character_containers::dsl::pf2_character_containers,
            pf2_character_items::dsl::pf2_character_items,
            pf2_character_readied_items::dsl::pf2_character_readied_items,
            pf2_character_stored_items::dsl::pf2_character_stored_items,
            pf2_character_worn_items::dsl::pf2_character_worn_items, pf2_characters,
            pf2_items::dsl::pf2_items, pf2_skills::dsl::pf2_skills,
        };

        // This gets the core data for the character
        let res = match pf2_characters::dsl::pf2_characters
            .filter(pf2_characters::dsl::id.eq(id))
            .load::<models::Character>(conn)
        {
            Ok(entry) => entry,
            Err(_) => return Err("Failed to access entry in database".to_owned()),
        };

        if res.len() < 1 {
            return Err("Character not found".to_owned());
        }
        if res.len() > 1 {
            error!("Multiple characters found with same id");
            return Err("Internal db error".to_owned());
        }

        let character: &models::Character = &res[0];

        let background_boosts: Vec<AbilityModifier> =
            match models::BackgroundAbilityBonus::belonging_to(character)
                .select(models::BackgroundAbilityBonus::as_select())
                .load(conn)
            {
                Ok(f) => f,
                Err(e) => {
                    if e != diesel::result::Error::NotFound {
                        warn!("DB error getting feats for character {}", character.id);
                    }
                    Vec::new()
                }
            }
            .into_iter()
            .map(|modifier| AbilityModifier {
                id: modifier.id,
                ability: modifier.ability,
                is_positive_boost: true,
            })
            .collect();

        let ancestry_modifiers: Vec<AbilityModifier> =
            match models::AncestryModifier::belonging_to(character)
                .select(models::AncestryModifier::as_select())
                .load(conn)
            {
                Ok(f) => f,
                Err(e) => {
                    if e != diesel::result::Error::NotFound {
                        warn!("DB error getting feats for character {}", character.id);
                    }
                    Vec::new()
                }
            }
            .into_iter()
            .map(|modifier| AbilityModifier {
                id: modifier.id,
                ability: modifier.ability,
                is_positive_boost: modifier.positive_boost,
            })
            .collect();

        let ancestry_features: Vec<AncestryFeature> =
            match models::AncestryFeature::belonging_to(character)
                .select(models::AncestryFeature::as_select())
                .load(conn)
            {
                Ok(val) => val,
                Err(e) => {
                    if e != diesel::result::Error::NotFound {
                        warn!("DB error getting feats for character {}", character.id);
                    }
                    Vec::new()
                }
            }
            .into_iter()
            .map(|feature| AncestryFeature {
                id: feature.id,
                name: feature.title,
                description: feature.description,
            })
            .collect();

        let class_features: Vec<ClassFeature> = match models::ClassFeature::belonging_to(character)
            .select(models::ClassFeature::as_select())
            .load(conn)
        {
            Ok(f) => f,
            Err(e) => {
                if e != diesel::result::Error::NotFound {
                    warn!("DB error getting feats for character {}", character.id);
                }
                Vec::new()
            }
        }
        .into_iter()
        .map(|feature| Feat {
            id: feature.id,
            name: feature.title,
            description: feature.description,
        })
        .collect();

        let damage_modifiers: Vec<DamageRecievedModifier> =
            match models::DamageTypeModifier::belonging_to(character)
                .select(models::DamageTypeModifier::as_select())
                .load(conn)
            {
                Ok(f) => f,
                Err(e) => {
                    if e != diesel::result::Error::NotFound {
                        warn!("DB error getting feats for character {}", character.id);
                    }
                    Vec::new()
                }
            }
            .into_iter()
            .map(|modifier| DamageRecievedModifier {
                id: modifier.id,
                modifier_type: modifier.modifier,
                value: modifier.val,
            })
            .collect();

        let senses: Vec<Sense> = match models::Sense::belonging_to(character)
            .select(models::Sense::as_select())
            .load(conn)
        {
            Ok(f) => f,
            Err(e) => {
                if e != diesel::result::Error::NotFound {
                    warn!("DB error getting feats for character {}", character.id);
                }
                Vec::new()
            }
        }
        .into_iter()
        .map(|sense| Sense {
            id: sense.id,
            name: sense.sense_name,
            description: sense.sense_description,
        })
        .collect();

        let skills: Vec<Skill> = match models::CharacterSkill::belonging_to(character)
            .inner_join(pf2_skills)
            .select((
                models::CharacterSkill::as_select(),
                models::Skill::as_select(),
            ))
            .load(conn)
        {
            Ok(f) => f,
            Err(e) => {
                if e != diesel::result::Error::NotFound {
                    warn!("DB error getting feats for character {}", character.id);
                }
                Vec::new()
            }
        }
        .into_iter()
        .map(|(character_details, skill_details)| Skill {
            id: character_details.id,
            skill: skill_details.title,
            ability: skill_details.ability,
            proficiency: character_details.proficiency,
            bonuses: character_details.bonuses,
            assurance: character_details.assurance,
        })
        .collect();

        let languages: Vec<String> = match models::CharacterLanguage::belonging_to(character)
            .select(models::CharacterLanguage::as_select())
            .load(conn)
        {
            Ok(f) => f,
            Err(e) => {
                if e != diesel::result::Error::NotFound {
                    warn!("DB error getting languages for character {}", character.id);
                }
                Vec::new()
            }
        }
        .into_iter()
        .map(|l| l.title)
        .collect();

        let feats: Vec<Feat> = match models::CharacterFeat::belonging_to(character)
            .select(models::CharacterFeat::as_select())
            .load(conn)
        {
            Ok(f) => f,
            Err(e) => {
                if e != diesel::result::Error::NotFound {
                    warn!("DB error getting feats for character {}", character.id);
                }
                Vec::new()
            }
        }
        .into_iter()
        .map(|f| Feat {
            id: f.id,
            name: f.title,
            description: f.description,
        })
        .collect();

        let statuses: Vec<Status> = match models::CharacterStatus::belonging_to(character)
            .select(models::CharacterStatus::as_select())
            .load(conn)
        {
            Ok(s) => s,
            Err(e) => {
                if e != diesel::result::Error::NotFound {
                    warn!("DB error getting statuses for character {}", character.id);
                }
                Vec::new()
            }
        }
        .into_iter()
        .map(|s| Status {
            id: s.id,
            name: s.status_name,
            description: s.status_description,
        })
        .collect();

        let formulas: Vec<Formula> = match models::FormulaBook::belonging_to(character)
            .inner_join(pf2_items)
            .select((models::FormulaBook::as_select(), models::Item::as_select()))
            .load(conn)
        {
            Ok(s) => s,
            Err(e) => {
                if e != diesel::result::Error::NotFound {
                    warn!("DB error getting formulas for character {}", character.id);
                }
                Vec::new()
            }
        }
        .into_iter()
        .map(|(form, item)| Formula {
            id: form.id,
            item_id: item.id,
            item_name: item.item_name,
            level: item.lvl,
            item_value: item.price,
        })
        .collect();

        // Get items
        let stored_items: Vec<StoredItem> = match pf2_character_stored_items
            .inner_join(pf2_character_items)
            .inner_join(
                pf2_items
                    .on(schema::pf2_character_items::dsl::item_id.eq(schema::pf2_items::dsl::id)),
            )
            .filter(schema::pf2_character_items::dsl::character_id.eq(id))
            .select((
                models::CharacterStoredItem::as_select(),
                models::CharacterItem::as_select(),
                models::Item::as_select(),
            ))
            .load(conn)
        {
            Ok(e) => e,
            Err(_) => Vec::new(),
        }
        .into_iter()
        .filter_map(|entry| {
            let (_, inventory_details, item_details) = entry;
            let item_id = item_details.id;
            let item = InventoryItem::gen_item(inventory_details, &item_details, conn)?;
            // Check if item is a container
            let container = match pf2_character_containers
                .filter(schema::pf2_character_containers::dsl::item_id.eq(item_id))
                .select(models::CharacterContainers::as_select())
                .load(conn)
            {
                Ok(c) => c,
                Err(e) => {
                    if e != diesel::result::Error::NotFound {
                        warn!("DB error {} while checking container", e);
                    }
                    // If we fail to get this info, just give the item. It's
                    return Some(StoredItem::Item(item));
                }
            };

            Some(StoredItem::Container(ItemContainer::gen_with_contents(
                container[0].clone(),
                item.clone(),
                conn,
            )))
        })
        .collect();

        let mut weapons: Vec<Weapon> = Vec::new();
        let readied_items: Vec<InventoryItem> = match pf2_character_readied_items
            .inner_join(pf2_character_items)
            .inner_join(
                pf2_items
                    .on(schema::pf2_character_items::dsl::item_id.eq(schema::pf2_items::dsl::id)),
            )
            .filter(schema::pf2_character_items::dsl::character_id.eq(id))
            .select((
                models::ReadiedItem::as_select(),
                models::CharacterItem::as_select(),
                models::Item::as_select(),
            ))
            .load(conn)
        {
            Ok(entries) => entries,
            Err(e) => {
                warn!("Failed loading readied items from database {}", e);
                Vec::new()
            }
        }
        .into_iter()
        .filter_map(|(_, inventory_details, item_details)| {
            match InventoryItem::gen_item(inventory_details, &item_details, conn) {
                Some(item) => match Weapon::is_weapon(item_details.id, &item, conn) {
                    Some(w) => {
                        weapons.push(w);
                        None
                    }
                    None => Some(item),
                },
                None => None,
            }
        })
        .collect();

        // Now that we have weapons, lets get the attacks
        let attacks: Vec<Attack> = match models::CharacterAttack::belonging_to(character)
            .select(models::CharacterAttack::as_select())
            .load(conn)
        {
            Ok(e) => e,
            Err(e) => {
                if e != diesel::result::Error::NotFound {
                    warn!("DB error getting attacks for character {}", character.id);
                }
                Vec::new()
            }
        }
        .into_iter()
        .map(|a: models::CharacterAttack| Attack {
            id: a.id,
            weapon: match a.item_id {
                Some(weapon_id) => match weapons.iter().find(|w| w.item_details.id == weapon_id) {
                    Some(w) => Some(w.clone()),
                    None => None,
                },
                None => None,
            },
            prof: a.proficiency,
            matk: a.matk,
            mdmg: a.mdmg,
            attack_type: a.attack_type,
            damage_die: a.damage_die,
        })
        .collect();

        let mut armor: Option<Armor> = None;
        let mut shield: Option<Shield> = None;
        let worn_items: Vec<InventoryItem> = match pf2_character_worn_items
            .inner_join(pf2_character_items)
            .inner_join(
                pf2_items
                    .on(schema::pf2_character_items::dsl::item_id.eq(schema::pf2_items::dsl::id)),
            )
            .filter(schema::pf2_character_items::dsl::character_id.eq(id))
            .select((
                models::CharacterWornItem::as_select(),
                models::CharacterItem::as_select(),
                models::Item::as_select(),
            ))
            .load(conn)
        {
            Ok(entries) => entries,
            Err(e) => {
                warn!("Failed loading worn items from database {}", e);
                Vec::new()
            }
        }
        .into_iter()
        .filter_map(|(_, inventory_details, item_details)| {
            match InventoryItem::gen_item(inventory_details, &item_details, conn) {
                Some(item) => match Armor::is_armor(item_details.id, &item, conn) {
                    Some(a) => {
                        armor = Some(a);
                        None
                    }
                    None => match Shield::is_shield(item_details.id, &item, conn) {
                        Some(s) => {
                            shield = Some(s);
                            None
                        }
                        None => Some(item),
                    },
                },
                None => None,
            }
        })
        .collect();

        // Get Spellcasting
        let casting_tables: Vec<SpellcastingTable> =
            match models::SpellcastingTable::belonging_to(character)
                .select(models::SpellcastingTable::as_select())
                .load(conn)
            {
                Ok(s) => s,
                Err(e) => {
                    if e != diesel::result::Error::NotFound {
                        warn!(
                            "DB error getting spellcasting tables for character {}",
                            character.id
                        );
                    }
                    Vec::new()
                }
            }
            .into_iter()
            .map(|t: models::SpellcastingTable| SpellcastingTable {
                id: t.id,
                tradition: t.tradition,
                ability: t.ability,
                prof: t.proficiency,
                spontaneous: t.spontaneous,
                casts_per_day: t.casts_per_day.clone(),
                num_spells_known: t.spells_known.clone(),
                item_bonus: t.item_bonus,
                misc_bonus: t.misc_bonus,
                spells_known: Spell::get_list_of_spells(&t, conn),
                spells_prepared: PreparedSpell::get_list_of_prepped_spells(&t, conn),
            })
            .collect();

        Ok(FullCharacterInfo {
            id: character.id,
            name: character.character_name.clone(),
            alignment: character.alignment.clone(),
            ancestry: character.ancestry.clone(),
            background: character.background.clone(),
            class: character.character_class.clone(),
            key_ability: character.key_ability.clone(),
            lvl: character.lvl,
            hero_points: character.hero_points,

            str_bonus: character.str_bonus,
            dex_bonus: character.dex_bonus,
            con_bonus: character.con_bonus,
            int_bonus: character.int_bonus,
            wis_bonus: character.wis_bonus,
            cha_bonus: character.cha_bonus,

            active_apex_item: character.active_apex_item.clone(),
            active_apex_item_bonus: character.active_apex_item_bonus,

            temp_hp: character.temp_hp,
            damage: character.damage,
            dying: character.dying,
            wound: character.wound,
            doom: character.doom,

            fort_prof: character.fort_prof,
            fort_misc_bonus: character.fort_misc_bonus,
            refl_prof: character.refl_prof,
            refl_misc_bonus: character.refl_misc_bonus,
            will_prof: character.will_prof,
            will_misc_bonus: character.will_misc_bonus,
            perception_prof: character.perception_prof,
            perception_misc_bonus: character.perception_misc_bonus,

            base_land_speed: character.base_land_speed,
            base_fly_speed: character.base_fly_speed,
            base_swim_speed: character.base_swim_speed,
            base_burrow_speed: character.base_burrow_speed,
            base_climb_speed: character.base_climb_speed,

            max_focus_points: character.max_focus_points,
            current_focus_points: character.current_focus_points,

            simple_weapon_prof: character.simple_weapon_prof,
            martial_weapon_prof: character.martial_weapon_prof,
            weapon_spec: character.weapon_spec,

            unarmored_prof: character.unarmored_prof,
            light_armor_prof: character.light_armor_prof,
            med_armor_prof: character.med_armor_prof,
            heavy_armor_prof: character.heavy_armor_prof,

            class_prof: character.class_prof,

            background_ability_boosts: background_boosts,
            ancestry_ability_boosts: ancestry_modifiers,

            ancestry_features,
            class_features,
            damage_recieved_mods: damage_modifiers,

            senses,
            skills,
            languages,

            feats,

            stored_items,
            readied_items,
            worn_items, // Excludes Armor/shield

            armor,
            shield,

            weapons,
            attacks,

            casting_tables,

            statuses,

            formula_book: formulas,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct NewCharacterAPI {
    character_name: String,
    alignment: String,
    ancestry: String,
    background: String,
    character_class: String,
    key_ability: String,
}

pub async fn create_new_character(
    req: HttpRequest,
    character_data: web::Json<NewCharacterAPI>,
    pool: web::Data<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>,
) -> actix_web::Result<impl Responder> {
    use schema::pf2_characters;
    use schema::users;

    if !auth::verify_header_token(req.headers()) {
        return Ok(HttpResponse::Unauthorized().json("User token not authenticated"));
    }

    // Get the user's id from their username, gives a chance
    let user = req
        .headers()
        .get("user")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let data = character_data.0;

    let mut conn = pool.get().unwrap();
    let user_entry = match users::dsl::users
        .filter(users::dsl::username.eq(user.clone()))
        .limit(2)
        .select(models::User::as_select())
        .load(&mut conn)
    {
        Ok(res) => res,
        Err(e) => {
            warn!("Error {} recieved while finding user {}", e, user);
            return Ok(HttpResponse::InternalServerError().body("User does not appear in database"));
        }
    };
    if user_entry.len() > 1 || user_entry.len() < 1 {
        error!("Multiple users found! Offending user: {user}\nPlease verify in database!");

        return Ok(
            HttpResponse::InternalServerError().body("Multiple users found with the same name")
        );
    }

    let new_entry = models::NewCharacter {
        character_name: &data.character_name,
        owner: user_entry[0].id,
        alignment: &data.alignment,
        ancestry: &data.ancestry,
        background: &data.background,
        character_class: &data.character_class,
        key_ability: &data.key_ability,
    };

    match diesel::insert_into(pf2_characters::dsl::pf2_characters)
        .values(new_entry)
        .execute(&mut conn)
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => {
            warn!(
                "Failed to add character {} (owned by {})",
                data.character_name, user_entry[0].username
            );
            Ok(HttpResponse::InternalServerError()
                .body("Failed to insert new character into internal DB"))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterDetails {
    character_id: i32,
    character_name: String,
}

#[get("/character")]
pub async fn get_character_list(
    req: HttpRequest,
    username: web::Json<String>,
    pool: web::Data<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>,
) -> actix_web::Result<impl Responder> {
    use schema::pf2_characters;

    if !auth::verify_header_token(req.headers()) {
        return Ok(HttpResponse::Unauthorized().json("User token not valid"));
    }
    let user = &username.0;

    let mut conn = pool.get().unwrap();
    let uid = user_mgmt::get_uid(&mut conn, user).unwrap();

    let list = match pf2_characters::dsl::pf2_characters
        .filter(pf2_characters::dsl::owner.eq(uid))
        .select(models::Character::as_select())
        .load(&mut conn)
    {
        Ok(vals) => vals,
        Err(e) => {
            warn!("Erroring loading characters for user {}", user);
            warn!("{:?}", e);
            return Ok(
                HttpResponse::InternalServerError().body("Failed to load characters for user")
            );
        }
    }
    .into_iter()
    .map(|e| CharacterDetails {
        character_id: e.id,
        character_name: e.character_name,
    })
    .collect::<Vec<CharacterDetails>>();

    Ok(HttpResponse::Ok().json(list))
}

#[get("/character/{character_id}")]
pub async fn get_full_character(
    req: HttpRequest,
    pool: web::Data<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>,
) -> actix_web::Result<impl Responder> {
    if !auth::verify_header_token(req.headers()) {
        return Ok(HttpResponse::Unauthorized().json("User token not authenticated"));
    }
    let mut conn = pool.get().unwrap();
    let character_id: i32 = req
        .match_info()
        .get("character_id")
        .unwrap()
        .parse()
        .unwrap();

    match FullCharacterInfo::load_character(&mut conn, character_id) {
        Ok(c) => Ok(HttpResponse::Ok().json(c)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(e)),
    }
}
