use crate::patterns::{eval, math, misc, pattern::Pattern, selectors, special};

pub type PatternRegistry = Vec<Pattern>;

pub trait PatternRegistryExt {
    fn construct() -> PatternRegistry;
    fn find(&self, query: &String) -> Option<&Pattern>;
}

#[rustfmt::skip]
impl PatternRegistryExt for PatternRegistry {
    fn construct() -> PatternRegistry {
        let mut registry: PatternRegistry = vec![];
        // registry.push(Pattern::new("Mind's Reflection", "get_caster", "qaq", selectors::get_caster));

        //special patterns
        registry.push(Pattern::new("Consideration", "escape", "qqqaw", special::escape));
        registry.push(Pattern::new("Introspection", "open_paren", "qqq", special::introspect));
        registry.push(Pattern::new("Retrospection", "close_paren", "eee", special::retrospect));
        registry.push(Pattern::new("Hermes' Gambit", "eval", "deaqq", eval::eval));
        registry.push(Pattern::new("Thoth's Gambit", "for_each", "dadad", eval::for_each));
        registry.push(Pattern::new("Charon's Gambit", "halt", "aqdee", special::halt));


        registry.push(Pattern::new("Additive Distillation", "add", "waaw", math::add));
        registry.push(Pattern::new("Numerical Reflection", "number", "aqaa", special::no_action));

        registry
    }

    fn find(&self, query: &String) -> Option<&Pattern> {
        self.into_iter()
            .filter(|entry| {
                entry.display_name == *query
                    || entry.internal_name == *query
                    || entry.signature == *query
            })
            .collect::<Vec<&Pattern>>()
            .get(0).map(|x| *x)
            
    }
}
