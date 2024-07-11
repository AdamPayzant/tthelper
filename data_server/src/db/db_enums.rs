use serde::Deserialize;

#[derive(diesel_derive_enum::DbEnum, Debug, Deserialize)]
#[ExistingTypePath = "crate::db::schema::sql_types::Pf2Ability"]
pub enum Pf2Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Deserialize)]
#[ExistingTypePath = "crate::db::schema::sql_types::Pf2Action"]
pub enum Pf2Action {
    Free,
    Reaction,
    One,
    Two,
    Three,
    OneToThree,
    TwoToThree,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Deserialize)]
#[ExistingTypePath = "crate::db::schema::sql_types::Pf2ArmorType"]
pub enum Pf2ArmorType {
    Unarmored,
    Light,
    Medium,
    Heavy,
}

#[derive(diesel_derive_enum::DbEnum, Clone, Debug, Deserialize)]
#[ExistingTypePath = "crate::db::schema::sql_types::Pf2AttackType"]
pub enum Pf2AttackType {
    StrStr,
    DexStr,
    DexDex,
    RangedDexHalfStr,
    RangedDexStr,
    Athletics,
}

#[derive(diesel_derive_enum::DbEnum, Clone, Debug, Deserialize)]
#[ExistingTypePath = "crate::db::schema::sql_types::Pf2DamageTypeModifier"]
pub enum Pf2DamageTypeModifier {
    Weakness,
    Resistance,
    Immunity,
}

#[derive(diesel_derive_enum::DbEnum, Clone, Debug, Deserialize)]
#[ExistingTypePath = "crate::db::schema::sql_types::Pf2Proficiency"]
pub enum Pf2Proficiency {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
}

#[derive(diesel_derive_enum::DbEnum, Clone, Debug, Deserialize)]
#[ExistingTypePath = "crate::db::schema::sql_types::Pf2SpellTradition"]
pub enum Pf2SpellTradition {
    Arcane,
    Divine,
    Elemental,
    Occult,
    Primal,
    Focus,
}

#[derive(diesel_derive_enum::DbEnum, Clone, Debug, Deserialize)]
#[ExistingTypePath = "crate::db::schema::sql_types::Pf2WeaponCategory"]
pub enum Pf2WeaponCategory {
    Unarmed,
    Simple,
    Martial,
    Advanced,
}

#[derive(diesel_derive_enum::DbEnum, Clone, Debug, Deserialize)]
#[ExistingTypePath = "crate::db::schema::sql_types::Pf2WeaponSpec"]
pub enum Pf2WeaponSpec {
    None,
    WS,
    GWS,
}

#[derive(diesel_derive_enum::DbEnum, Clone, Debug, Deserialize)]
#[ExistingTypePath = "crate::db::schema::sql_types::Pf2WeaponType"]
pub enum Pf2WeaponType {
    Melee,
    Ranged,
}
