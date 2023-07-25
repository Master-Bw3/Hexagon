use pest::Parser;
use std::{collections::HashMap, rc::Rc};
use toml::{map::Map, Table, Value};

use crate::{
    interpreter::state::{Holding, Library, Entity, EntityType},
    parser::{parse_iota, HexParser, Rule},
    pattern_registry::{PatternRegistry, PatternRegistryExt}, iota::{hex_casting::pattern::{Signature, SignatureExt}, Iota},
};

#[derive(Debug)]
pub struct Config {
    pub libraries: HashMap<[i32; 3], Library>,
    pub entities: HashMap<String, Entity>,
    pub great_spell_sigs: HashMap<String, String>,
}

pub fn parse_config(source: String) -> Config {
    let parsed = source.parse::<Table>().unwrap();

    let mut config = Config {
        libraries: HashMap::new(),
        entities: HashMap::new(),
        great_spell_sigs: PatternRegistry::gen_default_great_sigs(),
    };

    if let Some(Value::Table(sigs)) = &parsed.get("Great_Spells") {
        for (k, v) in sigs {
            config
                .great_spell_sigs
                .insert(k.clone(), parse_str(v).to_string());
        }
    };

    for (key, val) in &parsed {
        match &key[..] {
            "libraries" => parse_libraries(val, &mut config),
            "entities" => parse_entities(val, &mut config),
            _ => (),
        }
    }

    config
}

fn parse_libraries(libraries: &Value, config: &mut Config) {
    let libraries = match libraries {
        Value::Array(arr) => arr,
        _ => unreachable!(),
    };

    for val in libraries {
        match val {
            Value::Table(library) => parse_library(&mut library.clone(), config),
            _ => unreachable!(),
        }
    }
}

fn parse_library(library: &mut Map<String, Value>, config: &mut Config) {
    let mut contents: HashMap<_, Rc<dyn Iota>> = HashMap::new();

    let location_value = library.get("location").unwrap().clone();
    library.remove("location");

    for (key, val) in library {
        let iota = parse_iota(
            HexParser::parse(Rule::Iota, parse_str(val))
                .unwrap()
                .next()
                .unwrap(),
            &PatternRegistry::construct(&config.great_spell_sigs),
            &mut config.entities,
            &HashMap::new()
        );
        contents.insert(Signature::from_sig(key), iota);
    }

    let location = {
        match location_value {
            Value::Array(arr) => arr
                .iter()
                .map(parse_int)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            _ => unreachable!(),
        }
    };
    config.libraries.insert(location, contents);
}

fn parse_entities(entities: &Value, config: &mut Config) {
    let entities = match entities {
        Value::Array(arr) => arr,
        _ => unreachable!(),
    };

    for val in entities {
        match val {
            Value::Table(entity) => parse_entity(entity, config),
            _ => unreachable!(),
        }
    }
}

fn parse_entity(entity: &Map<String, Value>, config: &mut Config) {
    let name_value = entity.get("name").unwrap().clone();
    let name = parse_str(&name_value).to_string();

    let entity_type_value = entity.get("type").unwrap().clone();
    let entity_type_pair = HexParser::parse(Rule::EntityType, parse_str(&entity_type_value))
        .unwrap()
        .next()
        .unwrap();
    let entity_type = parse_entity_type(entity_type_pair.as_str().to_string());

    let held_item = entity.get("item");
    let held_item = held_item.map(|i| &parse_str(i)[..]);

    let held_item_contents_value = entity.get("iota");
    let held_item_contents_pair = held_item_contents_value.map(|value| {
        HexParser::parse(Rule::Iota, parse_str(value))
            .unwrap()
            .next()
            .unwrap()
    });
    let held_item_contents = held_item_contents_pair.map(|pair| {
        parse_iota(
            pair,
            &PatternRegistry::construct(&config.great_spell_sigs),
            &mut config.entities,
            &HashMap::new()
        )
    });

    let holding = match held_item {
        Some("Focus") => Holding::Focus(held_item_contents),
        Some("Trinket") => Holding::Trinket(held_item_contents),
        Some("Artifact") => Holding::Artifact(held_item_contents),
        Some("Cypher") => Holding::Cypher(held_item_contents),
        None => Holding::None,
        _ => unreachable!(),
    };

    config.entities.insert(
        name.clone(),
        Entity {
            name,
            entity_type,
            holding: Box::new(holding),
        },
    );
}

pub fn parse_entity_type(string: String) -> EntityType {
    match &string[..] {
        "Animal" => EntityType::Animal,
        "Monster" => EntityType::Monster,
        "Living" => EntityType::Living,
        "Item" => EntityType::Item,
        "Player" => EntityType::Player,
        "Misc" => EntityType::Misc,
        _ => unreachable!(),
    }
}


fn parse_int(value: &Value) -> i32 {
    match value {
        Value::Integer(int) => *int as i32,
        _ => unreachable!(),
    }
}

fn parse_str(value: &Value) -> &String {
    match value {
        Value::String(str) => str,
        _ => unreachable!(),
    }
}

