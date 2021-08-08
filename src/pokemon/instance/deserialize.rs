use core::{
    fmt::{self, Formatter},
    marker::PhantomData,
};
use serde::{
    __private::de::missing_field,
    de::{
        Deserialize, Deserializer, Error as SerdeError, IgnoredAny, MapAccess, SeqAccess, Visitor,
    },
};

use crate::{
    moves::MoveInstanceSet,
    pokemon::{
        data::Gender,
        default_friendship,
        stat::{Stats, default_iv},
        Experience, Friendship, Health, Level, Nickname, PokemonInstance,
    },
    status::StatusEffectInstance,
};

type ItemField = crate::item::ItemId;
type PokemonField = crate::pokemon::PokemonId;

impl<'de> Deserialize<'de> for PokemonInstance {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[allow(non_camel_case_types)]
        enum Field {
            Pokemon,
            Nickname,
            Level,
            Gender,
            IVs,
            EVs,
            Experience,
            Friendship,
            Moves,
            Effect,
            Item,
            CurrentHp,
            __ignore,
        }
        struct FieldVisitor;
        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Field;
            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("field identifier")
            }
            fn visit_u64<E: SerdeError>(self, value: u64) -> Result<Self::Value, E> {
                match value {
                    0u64 => Ok(Field::Pokemon),
                    1u64 => Ok(Field::Nickname),
                    2u64 => Ok(Field::Level),
                    3u64 => Ok(Field::Gender),
                    4u64 => Ok(Field::IVs),
                    5u64 => Ok(Field::EVs),
                    6u64 => Ok(Field::Experience),
                    7u64 => Ok(Field::Friendship),
                    8u64 => Ok(Field::Moves),
                    9u64 => Ok(Field::Effect),
                    10u64 => Ok(Field::Item),
                    11u64 => Ok(Field::CurrentHp),
                    _ => Ok(Field::__ignore),
                }
            }
            fn visit_str<E: SerdeError>(self, value: &str) -> Result<Self::Value, E> {
                match value {
                    "id" => Ok(Field::Pokemon),
                    "nickname" => Ok(Field::Nickname),
                    "level" => Ok(Field::Level),
                    "gender" => Ok(Field::Gender),
                    "ivs" => Ok(Field::IVs),
                    "evs" => Ok(Field::EVs),
                    "experience" => Ok(Field::Experience),
                    "friendship" => Ok(Field::Friendship),
                    "moves" => Ok(Field::Moves),
                    "effect" => Ok(Field::Effect),
                    "item" => Ok(Field::Item),
                    "current_hp" => Ok(Field::CurrentHp),
                    _ => Ok(Field::__ignore),
                }
            }
            fn visit_bytes<E: SerdeError>(self, value: &[u8]) -> Result<Self::Value, E> {
                match value {
                    b"id" => Ok(Field::Pokemon),
                    b"nickname" => Ok(Field::Nickname),
                    b"level" => Ok(Field::Level),
                    b"gender" => Ok(Field::Gender),
                    b"ivs" => Ok(Field::IVs),
                    b"evs" => Ok(Field::EVs),
                    b"experience" => Ok(Field::Experience),
                    b"friendship" => Ok(Field::Friendship),
                    b"moves" => Ok(Field::Moves),
                    b"effect" => Ok(Field::Effect),
                    b"item" => Ok(Field::Item),
                    b"current_hp" => Ok(Field::CurrentHp),
                    _ => Ok(Field::__ignore),
                }
            }
        }
        impl<'de> Deserialize<'de> for Field {
            #[inline]
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct PokemonInstanceVisitor<'de> {
            marker: PhantomData<PokemonInstance>,
            lifetime: PhantomData<&'de ()>,
        }
        impl<'de> Visitor<'de> for PokemonInstanceVisitor<'de> {
            type Value = PokemonInstance;
            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("struct PokemonInstance")
            }
            #[inline]
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let pokemon = match seq.next_element::<PokemonField>()? {
                    Some(__value) => __value,
                    None => {
                        return Err(serde::de::Error::invalid_length(
                            0usize,
                            &"struct PokemonInstance with 12 elements",
                        ));
                    }
                };
                let nickname = seq.next_element::<Nickname>()?.unwrap_or_default();
                let level = match seq.next_element::<Level>()? {
                    Some(__value) => __value,
                    None => {
                        return Err(serde::de::Error::invalid_length(
                            2usize,
                            &"struct PokemonInstance with 12 elements",
                        ));
                    }
                };
                let gender = match seq.next_element::<Gender>()? {
                    Some(__value) => __value,
                    None => {
                        return Err(serde::de::Error::invalid_length(
                            3usize,
                            &"struct PokemonInstance with 12 elements",
                        ));
                    }
                };
                let ivs = seq.next_element::<Stats>()?.unwrap_or_else(default_iv);
                let evs = seq.next_element::<Stats>()?.unwrap_or_default();
                let experience = seq.next_element::<Experience>()?.unwrap_or_default();
                let friendship = seq
                    .next_element::<Friendship>()?
                    .unwrap_or_else(default_friendship);
                let moves = seq
                    .next_element::<MoveInstanceSet>()?
                    .unwrap_or_else(|| default_moves(pokemon, level));
                let __field9 = seq
                    .next_element::<Option<StatusEffectInstance>>()?
                    .unwrap_or_default();
                let item = seq.next_element::<Option<ItemField>>()?.unwrap_or_default();
                let base = Default::default();
                let current_hp = seq
                    .next_element::<Health>()?
                    .unwrap_or_else(|| default_current_hp(pokemon));
                Ok(PokemonInstance {
                    id: pokemon,
                    nickname,
                    level,
                    gender,
                    ivs,
                    evs,
                    experience,
                    friendship,
                    moves,
                    effect: __field9,
                    item,
                    stages: base,
                    current_hp,
                })
            }
            #[inline]
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut pokemon: Option<PokemonField> = None;
                let mut nickname: Option<Nickname> = None;
                let mut level: Option<Level> = None;
                let mut gender: Option<Gender> = None;
                let mut ivs: Option<Stats> = None;
                let mut evs: Option<Stats> = None;
                let mut experience: Option<Experience> = None;
                let mut friendship: Option<Friendship> = None;
                let mut moves: Option<MoveInstanceSet> = None;
                let mut effect: Option<Option<StatusEffectInstance>> = None;
                let mut item: Option<Option<ItemField>> = None;
                let mut current_hp: Option<Health> = None;
                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        Field::Pokemon => {
                            if pokemon.is_some() {
                                return Err(SerdeError::duplicate_field("id"));
                            }
                            pokemon = Some(map.next_value::<PokemonField>()?);
                        }
                        Field::Nickname => {
                            if nickname.is_some() {
                                return Err(SerdeError::duplicate_field("nickname"));
                            }
                            nickname = Some(map.next_value::<Nickname>()?);
                        }
                        Field::Level => {
                            if level.is_some() {
                                return Err(SerdeError::duplicate_field("level"));
                            }
                            level = Some(map.next_value::<Level>()?);
                        }
                        Field::Gender => {
                            if gender.is_some() {
                                return Err(SerdeError::duplicate_field("gender"));
                            }
                            gender = Some(map.next_value::<Gender>()?);
                        }
                        Field::IVs => {
                            if ivs.is_some() {
                                return Err(SerdeError::duplicate_field("ivs"));
                            }
                            ivs = Some(map.next_value::<Stats>()?);
                        }
                        Field::EVs => {
                            if evs.is_some() {
                                return Err(SerdeError::duplicate_field("evs"));
                            }
                            evs = Some(map.next_value::<Stats>()?);
                        }
                        Field::Experience => {
                            if experience.is_some() {
                                return Err(SerdeError::duplicate_field("experience"));
                            }
                            experience = Some(map.next_value::<Experience>()?);
                        }
                        Field::Friendship => {
                            if friendship.is_some() {
                                return Err(SerdeError::duplicate_field("friendship"));
                            }
                            friendship = Some(map.next_value::<Friendship>()?);
                        }
                        Field::Moves => {
                            if moves.is_some() {
                                return Err(SerdeError::duplicate_field("moves"));
                            }
                            moves = Some(map.next_value::<MoveInstanceSet>()?);
                        }
                        Field::Effect => {
                            if effect.is_some() {
                                return Err(SerdeError::duplicate_field("effect"));
                            }
                            effect = Some(map.next_value::<Option<StatusEffectInstance>>()?);
                        }
                        Field::Item => {
                            if item.is_some() {
                                return Err(SerdeError::duplicate_field("item"));
                            }
                            item = Some(map.next_value::<Option<ItemField>>()?);
                        }
                        Field::CurrentHp => {
                            if current_hp.is_some() {
                                return Err(SerdeError::duplicate_field("current_hp"));
                            }
                            current_hp = Some(map.next_value::<Health>()?);
                        }
                        _ => {
                            let _ = map.next_value::<IgnoredAny>()?;
                        }
                    }
                }
                let pokemon = match pokemon {
                    Some(__field0) => __field0,
                    None => missing_field("id")?,
                };
                let nickname = nickname.unwrap_or_default();
                let level = match level {
                    Some(__field2) => __field2,
                    None => missing_field("level")?,
                };
                let gender = match gender {
                    Some(__field2) => __field2,
                    None => missing_field("gender")?,
                };
                let ivs = ivs.unwrap_or_else(default_iv);
                let evs = evs.unwrap_or_default();
                let experience = experience.unwrap_or_default();
                let friendship = friendship.unwrap_or_else(default_friendship);
                let moves = moves.unwrap_or_else(|| default_moves(pokemon, level));
                let effect = effect.unwrap_or_default();
                let item = item.unwrap_or_default();

                let current_hp = current_hp.unwrap_or_else(|| default_current_hp(pokemon));

                Ok(PokemonInstance {
                    id: pokemon,
                    nickname,
                    level,
                    gender,
                    ivs,
                    evs,
                    experience,
                    friendship,
                    moves,
                    effect,
                    item,
                    stages: Default::default(),
                    current_hp,
                })
            }
        }
        const FIELDS: &[&str] = &[
            "id",
            "nickname",
            "level",
            "gender",
            "ivs",
            "evs",
            "experience",
            "friendship",
            "moves",
            "effect",
            "item",
            "current_hp",
        ];
        deserializer.deserialize_struct(
            "PokemonInstance",
            FIELDS,
            PokemonInstanceVisitor {
                marker: PhantomData::<PokemonInstance>,
                lifetime: PhantomData,
            },
        )
    }
}

#[deprecated]
#[inline]
fn default_moves(pokemon: PokemonField, level: Level) -> MoveInstanceSet {
    // pokemon.generate_moves(level)
    log::error!("cannot deserialize default moves");
    Default::default()
}

// #[deprecated]
// #[inline]
// fn default_base(pokemon: PokemonField, ivs: &Stats, evs: &Stats, level: Level) -> BaseStats {
//     log::error!("cannot deserialize base stats");
//     // BaseStats::new(&pokemon, ivs, evs, level)
//     Default::default()
// }

#[inline]
fn default_current_hp(pokemon: PokemonField) -> Health {
    log::error!("cannot deserialize current hp");
    100
}
