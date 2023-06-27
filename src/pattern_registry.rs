use crate::patterns::{pattern::Pattern, selectors::get_caster};

pub struct PatternRegistry (Vec<Pattern>);


impl PatternRegistry {
pub fn construct() -> PatternRegistry {
    let mut registry: Vec<Pattern> = vec![];
    // registry.push(Pattern { display_name: "Mind's Reflection".to_string(), internal_name: "get_caster".to_string(), signature: "qaq".to_string(), action: get_caster });


    PatternRegistry(registry)
}}