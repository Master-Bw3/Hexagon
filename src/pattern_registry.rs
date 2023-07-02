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
            Pattern::new("Multiplicative Distillation", "mul_dot", "waqaw", &math::mul_dot),
            Pattern::new("Division Distillation", "div_cross", "wdedw", &math::div_cross),
            Pattern::new("Length Purification", "abs_len", "wqaqw", &math::abs_len),
            Pattern::new("Power Distillation", "pow_proj", "wedew", &math::pow_proj),
            Pattern::new("Floor Purification", "floor", "ewq", &math::floor),
            Pattern::new("Ceiling Purification", "ceil", "qwe", &math::ceil),
            Pattern::new("Vector Exaltation", "construct_vec", "eqqqqq", &math::construct_vec),
            Pattern::new("Vector Disintegration", "deconstruct_vec", "qeeeee", &math::deconstruct_vec),
            Pattern::new("Axial Purification", "coerce_axial", "qqqqqaww", &math::coerce_axial),
            Pattern::new("Conjunction Distillation", "and", "wdw", &math::and),
            Pattern::new("Disjunction Distillation", "or", "waw", &math::or),
            Pattern::new("Exclusion Distillation", "xor", "dwa", &math::xor),
            Pattern::new("Maximus Distillation", "greater", "e", &math::greater),
            Pattern::new("Minimus Distillation", "less", "q", &math::less),
            Pattern::new("Maximus Distillation", "greater_eq", "ee", &math::greater_eq),
            Pattern::new("Minimus Distillation", "less_eq", "qq", &math::less_eq),
            Pattern::new("Equality Distillation", "equals", "ad", &math::equals),
            Pattern::new("Inequality Distillation", "not_equals", "da", &math::not_equals),
            Pattern::new("Negation Purification", "not", "dw", &math::not),
            Pattern::new("Augur's Purification", "bool_coerce", "aw", &math::bool_coerce),
            Pattern::new("Sine Purification", "sin", "qqqqqaa", &math::sin),
            Pattern::new("Cosine Purification", "cos", "qqqqqad", &math::cos),
            Pattern::new("Tangent Purification", "tan", "wqqqqqadq", &math::tan),
            Pattern::new("Inverse Sine Purification", "arcsin", "ddeeeee", &math::arcsin),
            Pattern::new("Inverse Cosine Purification", "arccos", "adeeeee", &math::arccos),
            Pattern::new("Inverse Tangent Purification", "arctan", "eadeeeeew", &math::arctan),
            Pattern::new("Logarithmic Distillation", "logarithm", "eqaqe", &math::logarithm),
            Pattern::new("Modulus Distillation", "modulo", "addwaad", &math::modulo),
            Pattern::new("Intersection Distillation", "and_bit", "wdweaqa", &math::and_bit),
            Pattern::new("Unifying Distillation", "or_bit", "waweaqa", &math::or_bit),
            Pattern::new("Exclusionary Distillation", "xor_bit", "dwaeaqa", &math::xor_bit),
            Pattern::new("Inversion Purification", "not_bit", "dweaqa", &math::not_bit),
            Pattern::new("Uniqueness Purification", "to_set", "aweaqa", &math::to_set),
            // Pattern::new(, "", "", &math::),


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
