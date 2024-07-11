use actix_session::Session;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use serde::Deserialize;

use diesel::prelude::*;
use diesel::{r2d2, PgConnection, QueryDsl, SelectableHelper};

use log::{error, warn};

use crate::auth::AuthorizedData;
use crate::db::{db_enums, models, schema};

#[derive(Debug, Deserialize)]
struct AbilityModifier {
    ability: db_enums::Pf2Ability,
    is_positive_boost: bool,
}

#[derive(Debug, Deserialize)]
struct DamageRecievedModifier {
    modifier_type: db_enums::Pf2DamageTypeModifier,
    value: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct Description {
    name: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct Skill {
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
    traits: Vec<Description>,

    runes: Vec<Description>,
}

#[derive(Debug, Deserialize)]
struct ItemContainer {
    item: InventoryItem,
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

    ancestry_features: Vec<Description>,
    class_features: Vec<Description>,
    damage_recieved_mods: Vec<DamageRecievedModifier>,

    senses: Vec<Description>,
    skills: Vec<Skill>,
    languages: Vec<String>,

    feats: Vec<Description>,

    casting_tables: Vec<SpellcastingTable>,

    stored_items: Vec<StoredItem>,
    readied_items: Vec<InventoryItem>,
    worn_items: Vec<InventoryItem>, // Excludes Armor/shield

    armor: Option<Armor>,
    shield: Option<Shield>,

    weapons: Vec<Weapon>,
    attacks: Vec<Attack>,

    statuses: Vec<Status>,
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
