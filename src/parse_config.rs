use pest::Parser;
use std::{
    collections::{hash_map, HashMap},
    vec,
};
use toml::{map::Map, Table, Value};

use crate::{
    iota::{EntityIota, EntityType, Iota, Signature, SignatureExt, VectorIota},
    parser::{parse_entity_type, parse_iota, HexParser, Rule},
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

#[derive(Debug)]
struct Config {
    libraries: HashMap<[i32; 3], Library>,
    entities: Vec<EntityIota>,
}

type Library = HashMap<Signature, Iota>;

pub fn parse_config(source: String) {
    let parsed = source.parse::<Table>().unwrap();

    let mut config = Config {
        libraries: HashMap::new(),
        entities: vec![],
    };

    for (key, val) in parsed {
        match &key[..] {
            "libraries" => parse_libraries(val, &mut config),
            "entities" => parse_entities(val, &mut config),
            _ => unreachable!(),
        }
    }

    println!("{:?}", config);
}

fn parse_libraries(libraries: Value, config: &mut Config) {
    let libraries = match libraries {
        Value::Array(arr) => arr,
        _ => unreachable!(),
    };

    for val in libraries {
        match val {
            Value::Table(library) => parse_library(library, config),
            _ => unreachable!(),
        }
    }
}

fn parse_library(mut library: Map<String, Value>, config: &mut Config) {
    let mut contents = HashMap::new();

    let location_value = library.get("location").unwrap().clone();
    library.remove("location");

    for (key, val) in &library {
        let iota = parse_iota(
            HexParser::parse(Rule::Iota, parse_str(val))
                .unwrap()
                .into_iter()
                .next()
                .unwrap(),
            &PatternRegistry::construct(),
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

fn parse_entities(entities: Value, config: &mut Config) {
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

fn parse_entity(mut entity: Map<String, Value>, config: &mut Config) {
    let name_value = entity.get("name").unwrap().clone();
    let name = parse_str(&name_value).to_string();

    let entity_type_value = entity.get("type").unwrap().clone();
    let entity_type_pair = HexParser::parse(Rule::EntityType, parse_str(&entity_type_value))
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let entity_type = parse_entity_type(entity_type_pair.as_str().to_string());

    config.entities.push(EntityIota { name, entity_type })
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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn config_test() {
        let path = "./test.toml";
        let source = fs::read_to_string(path).expect("Should have been able to read the file");

        //let expected = vec![Iota::List(vec![Iota::Number(2.0)])];

        let result = parse_config(source);
        //assert_eq!(result.stack, expected)
    }
}
