use crate::patterns::{math, pattern::Pattern, selectors};

pub type PatternRegistry = Vec<Pattern>;

pub trait PatternRegistryExt {
    fn construct() -> PatternRegistry;
    fn find(&self, query: String) -> Option<&Pattern>;
}

impl PatternRegistryExt for PatternRegistry {
    fn construct() -> PatternRegistry {
        let mut registry: PatternRegistry = vec![];
        // registry.push(Pattern::new("Mind's Reflection", "get_caster", "qaq", selectors::get_caster));
        registry.push(Pattern::new("Additive Distillation", "add", "waaw", math::add));

        registry
    }

    fn find(&self, query: String) -> Option<&Pattern> {
        self.into_iter()
            .filter(|entry| {
                entry.display_name == query
                    || entry.internal_name == query
                    || entry.signature == query
            })
            .collect::<Vec<&Pattern>>()
            .get(0).map(|x| *x)
            
    }
}
