use crate::patterns::{eval, math, pattern::Pattern, special};

pub type PatternRegistry = Vec<Pattern>;

pub trait PatternRegistryExt {
    fn construct() -> PatternRegistry;
    fn find(&self, query: &str) -> Option<&Pattern>;
}

impl PatternRegistryExt for PatternRegistry {
    fn construct() -> PatternRegistry {
        let mut registry: PatternRegistry = vec![
            //special patterns
            Pattern::new("Consideration", "escape", "qqqaw", &special::escape),
            Pattern::new("Introspection", "open_paren", "qqq", &special::introspect),
            Pattern::new("Retrospection", "close_paren", "eee", &special::retrospect),
            Pattern::new("Hermes' Gambit", "eval", "deaqq", &eval::eval),
            Pattern::new("Thoth's Gambit", "for_each", "dadad", &eval::for_each),
            Pattern::new("Charon's Gambit", "halt", "aqdee", &special::halt),

            //math
            Pattern::new("Additive Distillation", "add", "waaw", &math::add),
            Pattern::new("Subtractive Distillation", "sub", "wddw", &math::subtract),

        ];

        registry.push(Pattern::new(
            "Additive Distillation",
            "add",
            "waaw",
            &math::add,
        ));
        registry.push(Pattern::new(
            "Numerical Reflection",
            "number",
            "aqaa",
            &special::no_action,
        ));

        registry
    }

    fn find(&self, query: &str) -> Option<&Pattern> {
        self.iter()
            .filter(|entry| {
                entry.display_name == *query
                    || entry.internal_name == *query
                    || entry.signature == *query
            })
            .collect::<Vec<&Pattern>>()
            .get(0)
            .copied()
    }
}
