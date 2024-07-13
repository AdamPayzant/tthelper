use actix_session::Session;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::{r2d2, PgConnection, QueryDsl, SelectableHelper};

use log::{error, warn};

use crate::auth::AuthorizedData;
use crate::db::schema::pf2_items::item_description;
use crate::db::schema::{pf2_character_feats, pf2_character_statuses, pf2_item_traits, pf2_items};
use crate::db::{db_enums, models, schema, user_mgmt};

#[derive(Debug, Deserialize)]
struct AbilityModifier {
    id: i32,
    ability: db_enums::Pf2Ability,
    is_positive_boost: bool,
}

#[derive(Debug, Deserialize)]
struct DamageRecievedModifier {
    id: i32,
    modifier_type: db_enums::Pf2DamageTypeModifier,
    value: Option<i32>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct Skill {
    id: i32,
    skill: String,
    ability: db_enums::Pf2Ability,

    proficiency: db_enums::Pf2Proficiency,
    bonuses: i32,
    assurance: bool,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct PreparedSpell {
    id: i32,
    spell: Spell,
    level_prepared: i32,
}

#[derive(Debug, Deserialize)]
struct SpellcastingTable {
    id: i32,
    tradition: db_enums::Pf2SpellTradition,
    ability: db_enums::Pf2Ability,
    prof: db_enums::Pf2Proficiency,
    spontaneous: bool,

    casts_per_day: [i32; 10],
    num_spells_known: [i32; 10],

    item_bonus: i32,
    misc_bonus: i32,

    spells_known: Vec<Spell>,
    spells_prepared: Vec<PreparedSpell>,
}

#[derive(Debug, Deserialize)]
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
        item_details: models::Item,
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
            name: item_details.item_name,
            description: item_details.item_description,
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

#[derive(Debug, Deserialize)]
struct ItemContainer {
    id: i32,
    item: InventoryItem,
    bulk_reduction: i32,
    max_bulk: i32,
    contents: Vec<StoredItem>,
}

#[derive(Debug, Deserialize)]
enum StoredItem {
    Container(ItemContainer),
    Item(InventoryItem),
}

#[derive(Debug, Deserialize)]
struct Armor {
    id: i32,
    item_details: InventoryItem,
    ac_bonus: i32,
    max_dex: i32,
    check_penalty: i32,
    speed_penalty: i32,
    str_requirement: i32,
    armor_type: db_enums::Pf2ArmorType,
    armor_group_name: String,
    armor_spec: String,
}

#[derive(Debug, Deserialize)]
struct Shield {
    id: i32,
    item_details: InventoryItem,
    ac_bonus: i32,
    hardness: i32,
    hp: i32,
    bp: i32,
    speed_penalty: i32,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct Attack {
    id: i32,
    weapon: Option<Weapon>,
    prof: db_enums::Pf2Proficiency,
    matck: i32,
    mdmg: i32,
    attack_type: db_enums::Pf2AttackType,
}

#[derive(Debug, Deserialize)]
struct Status {
    id: i32,
    name: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct Formula {
    item_id: i32,
    item_name: String,
    level: i32,
}

// A large structure to send full character details to a client.
// Ideally this should only be sent once when the character is requested
#[derive(Debug, Deserialize)]
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
    active_apex_item_bonus: db_enums::Pf2Ability,

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
            pf2_character_ancestry_ability_modifier::dsl::pf2_character_ancestry_ability_modifier,
            pf2_character_ancestry_features::dsl::pf2_character_ancestry_features,
            pf2_character_attacks::dsl::pf2_character_attacks,
            pf2_character_background_ability_bonus::dsl::pf2_character_background_ability_bonus,
            pf2_character_containers::dsl::pf2_character_containers,
            pf2_character_damage_type_modifier::dsl::pf2_character_damage_type_modifier,
            pf2_character_feats::dsl::pf2_character_feats,
            pf2_character_formula_books::dsl::pf2_character_formula_books,
            pf2_character_item_attached_runes::dsl::pf2_character_item_attached_runes,
            pf2_character_items::dsl::pf2_character_items,
            pf2_character_languages::dsl::pf2_character_languages,
            pf2_character_readied_items::dsl::pf2_character_readied_items,
            pf2_character_senses::dsl::pf2_character_senses,
            pf2_character_skills::dsl::pf2_character_skills,
            pf2_character_spellcasting_tables::dsl::pf2_character_spellcasting_tables,
            pf2_character_statuses::dsl::pf2_character_statuses,
            pf2_character_stored_items::dsl::pf2_character_stored_items,
            pf2_character_worn_items::dsl::pf2_character_worn_items, pf2_characters,
            pf2_class_features::dsl::pf2_class_features, pf2_item_traits::dsl::pf2_item_traits,
            pf2_items::dsl::pf2_items, pf2_skills::dsl::pf2_skills, pf2_traits::dsl::pf2_traits,
        };

        // This gets the core data for the character
        let res = match pf2_characters::dsl::pf2_characters
            .inner_join(pf2_character_background_ability_bonus)
            .inner_join(pf2_character_ancestry_ability_modifier)
            .inner_join(pf2_character_ancestry_features)
            .inner_join(pf2_class_features)
            .inner_join(pf2_character_damage_type_modifier)
            .inner_join(pf2_character_senses)
            .inner_join(pf2_character_skills.inner_join(pf2_skills))
            .inner_join(pf2_character_languages)
            .inner_join(pf2_character_feats)
            .inner_join(pf2_character_attacks)
            .inner_join(pf2_character_statuses)
            .inner_join(pf2_character_formula_books)
            .filter(pf2_characters::dsl::id.eq(id))
            .load::<(
                models::Character,
                models::BackgroundAbilityBonus,
                models::AncestryModifier,
                models::AncestryFeature,
                models::ClassFeature,
                models::DamageTypeModifier,
                models::Sense,
                (models::CharacterSkill, models::Skill),
                models::CharacterLanguage,
                models::CharacterFeat,
                models::CharacterAttack,
                models::CharacterStatus,
                models::FormulaBook,
            )>(conn)
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

        let (
            character,
            background_bonuses,
            ancestry_modifiers,
            ancestry_features,
            class_features,
            damage_modifiers,
            senses,
            (character_skills, skill_details),
            languages,
            feats,
            attacks,
            statuses,
            formulas,
        ) = &res[0];

        // Get items
        let stored_items_data = match pf2_character_stored_items
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
            Err(_) => {
                return Err("Failed getting inventory from DB".to_string());
            }
        };

        let stored_items: Vec<StoredItem> = stored_items_data
            .into_iter()
            .filter_map(|entry| {
                let (_, inventory_details, item_details) = entry;
                let item = InventoryItem::gen_item(inventory_details, item_details, conn)?;
                // Check if item is a container

                Some(StoredItem::Item(item))
            })
            .collect();

        // Get Spellcasting

        Err("Unreachable".to_owned())
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
    character_data: web::Json<AuthorizedData<NewCharacterAPI>>,
    session: Session,
    pool: web::Data<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>,
) -> actix_web::Result<impl Responder> {
    use schema::pf2_characters;
    use schema::users;

    // Get the user's id from their username, gives a chance
    let user = character_data.get_user();
    let data = match character_data.get_verified_request(&session) {
        Ok(d) => d,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

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

pub async fn get_character_list(
    username: web::Json<AuthorizedData<String>>,
    session: Session,
    pool: web::Data<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>,
) -> actix_web::Result<impl Responder> {
    use schema::pf2_characters;

    let user = match username.get_verified_request(&session) {
        Ok(s) => s,
        Err(()) => {
            return Ok(HttpResponse::Unauthorized().body("User not authenticated"));
        }
    };

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

#[derive(Debug, Deserialize)]
pub struct CharacterRequest {
    username: String,
    character_id: i32,
}

pub async fn get_full_character(
    request: web::Json<AuthorizedData<CharacterRequest>>,
    session: Session,
    pool: web::Data<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>,
) -> actix_web::Result<impl Responder> {
    Ok(HttpResponse::Ok())
}
