use crate::patterns::{constructors, eval, math, pattern::Pattern, special};

use crate::iota::Iota;

pub type PatternRegistry = Vec<Pattern>;

pub trait PatternRegistryExt {
    fn construct() -> PatternRegistry;
    fn find(&self, query: &str) -> Option<&Pattern>;
}

impl PatternRegistryExt for PatternRegistry {
    #[rustfmt::skip]
    fn construct() -> PatternRegistry {
        let registry: PatternRegistry = vec![
            //special patterns
            Pattern::new("Consideration", "escape", "qqqaw", Box::new(special::escape)),
            Pattern::new("Introspection", "open_paren", "qqq", Box::new(special::introspect)),
            Pattern::new("Retrospection", "close_paren", "eee", Box::new(special::retrospect)),
            Pattern::new("Hermes' Gambit", "eval", "deaqq", Box::new(eval::eval)),
            Pattern::new("Thoth's Gambit", "for_each", "dadad", Box::new(eval::for_each)),
            Pattern::new("Charon's Gambit", "halt", "aqdee", Box::new(special::halt)),

            //math
            Pattern::new("Additive Distillation", "add", "waaw", Box::new(math::add)),
            Pattern::new("Subtractive Distillation", "sub", "wddw", Box::new(math::subtract)),
            Pattern::new("Multiplicative Distillation", "mul_dot", "waqaw", Box::new(math::mul_dot)),
            Pattern::new("Division Distillation", "div_cross", "wdedw", Box::new(math::div_cross)),
            Pattern::new("Length Purification", "abs_len", "wqaqw", Box::new(math::abs_len)),
            Pattern::new("Power Distillation", "pow_proj", "wedew", Box::new(math::pow_proj)),
            Pattern::new("Floor Purification", "floor", "ewq", Box::new(math::floor)),
            Pattern::new("Ceiling Purification", "ceil", "qwe", Box::new(math::ceil)),
            Pattern::new("Vector Exaltation", "construct_vec", "eqqqqq", Box::new(math::construct_vec)),
            Pattern::new("Vector Disintegration", "deconstruct_vec", "qeeeee", Box::new(math::deconstruct_vec)),
            Pattern::new("Axial Purification", "coerce_axial", "qqqqqaww", Box::new(math::coerce_axial)),
            Pattern::new("Conjunction Distillation", "and", "wdw", Box::new(math::and)),
            Pattern::new("Disjunction Distillation", "or", "waw", Box::new(math::or)),
            Pattern::new("Exclusion Distillation", "xor", "dwa", Box::new(math::xor)),
            Pattern::new("Maximus Distillation", "greater", "e", Box::new(math::greater)),
            Pattern::new("Minimus Distillation", "less", "q", Box::new(math::less)),
            Pattern::new("Maximus Distillation", "greater_eq", "ee", Box::new(math::greater_eq)),
            Pattern::new("Minimus Distillation", "less_eq", "qq", Box::new(math::less_eq)),
            Pattern::new("Equality Distillation", "equals", "ad", Box::new(math::equals)),
            Pattern::new("Inequality Distillation", "not_equals", "da", Box::new(math::not_equals)),
            Pattern::new("Negation Purification", "not", "dw", Box::new(math::not)),
            Pattern::new("Augur's Purification", "bool_coerce", "aw", Box::new(math::bool_coerce)),
            Pattern::new("Sine Purification", "sin", "qqqqqaa", Box::new(math::sin)),
            Pattern::new("Cosine Purification", "cos", "qqqqqad", Box::new(math::cos)),
            Pattern::new("Tangent Purification", "tan", "wqqqqqadq", Box::new(math::tan)),
            Pattern::new("Inverse Sine Purification", "arcsin", "ddeeeee", Box::new(math::arcsin)),
            Pattern::new("Inverse Cosine Purification", "arccos", "adeeeee", Box::new(math::arccos)),
            Pattern::new("Inverse Tangent Purification", "arctan", "eadeeeeew", Box::new(math::arctan)),
            Pattern::new("Logarithmic Distillation", "logarithm", "eqaqe", Box::new(math::logarithm)),
            Pattern::new("Modulus Distillation", "modulo", "addwaad", Box::new(math::modulo)),
            Pattern::new("Intersection Distillation", "and_bit", "wdweaqa", Box::new(math::and_bit)),
            Pattern::new("Unifying Distillation", "or_bit", "waweaqa", Box::new(math::or_bit)),
            Pattern::new("Exclusionary Distillation", "xor_bit", "dwaeaqa", Box::new(math::xor_bit)),
            Pattern::new("Inversion Purification", "not_bit", "dweaqa", Box::new(math::not_bit)),
            Pattern::new("Uniqueness Purification", "to_set", "aweaqa", Box::new(math::to_set)),

            //list)s
            // Pattern::new("Integration Distillation", "append", "edqde", Box::new(lists::append)),
            // Pattern::new("Combination Distillation", "concat", "qaeaq", Box::new(lists::concat)),
            // Pattern::new("Selection Distillation", "index", "deeed", Box::new(lists::index)),
            // Pattern::new("Abacus Purification", "list_size", "aqaeaq", Box::new(lists::list_size)),
            // Pattern::new("Single's Purification", "singleton", "adeeed", Box::new(lists::singleton)),
            Pattern::new("Vacant Reflection", "empty_list", "qqaeaae", constructors::push_const(Iota::List(vec![]))),
            // Pattern::new("Retrograde Purification", "reverse_list", "qqqaede", Box::new(lists::reverse_list)),
            // Pattern::new("Flock's Gambit", "last_n_list", "ewdqdwe", Box::new(lists::last_n_list)),
            // Pattern::new("Flock's Disintegration", "splat", "qwaeawq", Box::new(lists::splat)),
            // Pattern::new("Locator's Distillation", "index_of", "dedqde", Box::new(lists::index_of)),
            // Pattern::new("Excisor's Distillation", "list_remove", "edqdewaqa", Box::new(lists::list_remove)),
            // Pattern::new("Selection Exaltation", "slice", "qaeaqwded", Box::new(lists::slice)),
            // Pattern::new("Surgeon's Exaltation", "modify_in_place", "wqaeaqw", Box::new(lists::modify_in_place)),
            // Pattern::new("Speaker's Distillation", "construct", "ddewedd", Box::new(lists::construct)),
            // Pattern::new("Speaker's Decomposition", "deconstruct", "aaqwqaa", Box::new(lists::deconstruct)),



        ];

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
