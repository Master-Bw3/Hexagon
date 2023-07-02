use std::f32::consts::{PI, TAU, E};

use crate::patterns::lists;
use crate::patterns::{constructors, eval, math, pattern::Pattern, special};
use crate::iota::{Iota, VectorIota, NullIota};
use crate::interpreter::state::{StackExt, Stack};

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

            //lists
            Pattern::new("Integration Distillation", "append", "edqde", Box::new(lists::append)),
            Pattern::new("Combination Distillation", "concat", "qaeaq", Box::new(lists::concat)),
            Pattern::new("Selection Distillation", "index", "deeed", Box::new(lists::index)),
            Pattern::new("Abacus Purification", "list_size", "aqaeaq", Box::new(lists::list_size)),
            Pattern::new("Single's Purification", "singleton", "adeeed", Box::new(lists::singleton)),
            Pattern::new("Retrograde Purification", "reverse_list", "qqqaede", Box::new(lists::reverse_list)),
            Pattern::new("Flock's Gambit", "last_n_list", "ewdqdwe", Box::new(lists::last_n_list)),
            Pattern::new("Flock's Disintegration", "splat", "qwaeawq", Box::new(lists::splat)),
            Pattern::new("Locator's Distillation", "index_of", "dedqde", Box::new(lists::index_of)),
            Pattern::new("Excisor's Distillation", "list_remove", "edqdewaqa", Box::new(lists::list_remove)),
            Pattern::new("Selection Exaltation", "slice", "qaeaqwded", Box::new(lists::slice)),
            Pattern::new("Surgeon's Exaltation", "modify_in_place", "wqaeaqw", Box::new(lists::modify_in_place)),
            Pattern::new("Speaker's Distillation", "construct", "ddewedd", Box::new(lists::construct)),
            Pattern::new("Speaker's Decomposition", "deconstruct", "aaqwqaa", Box::new(lists::deconstruct)),

            //consts
            Pattern::new("Vacant Reflection", "empty_list", "qqaeaae", constructors::push_const(Iota::List(vec![]))),
            Pattern::new("Vector Reflection +X", "const/vec/px", "qqqqqea", constructors::push_const(Iota::Vector(VectorIota::new(1.0, 0.0, 0.0)))),
            Pattern::new("Vector Reflection +Y", "const/vec/py", "qqqqqew", constructors::push_const(Iota::Vector(VectorIota::new(0.0, 1.0, 0.0)))),
            Pattern::new("Vector Reflection +Z", "const/vec/pz", "qqqqqed", constructors::push_const(Iota::Vector(VectorIota::new(0.0, 0.0, 1.0)))),
            Pattern::new("Vector Reflection -X", "const/vec/nx", "eeeeeqa", constructors::push_const(Iota::Vector(VectorIota::new(-1.0, 0.0, 0.0)))),
            Pattern::new("Vector Reflection -Y", "const/vec/ny", "eeeeeqw", constructors::push_const(Iota::Vector(VectorIota::new(0.0, -1.0, 0.0)))),
            Pattern::new("Vector Reflection -Z", "const/vec/nz", "eeeeeqd", constructors::push_const(Iota::Vector(VectorIota::new(0.0, 0.0, -1.0)))),
            Pattern::new("Vector Reflection Zero", "const/vec/0", "qqqqq", constructors::push_const(Iota::Vector(VectorIota::new(0.0, 0.0, 0.0)))),
            Pattern::new("Arc's Reflection", "const/double/pi", "qdwdq", constructors::push_const(Iota::Number(PI))),
            Pattern::new("Circle's Reflection", "const/double/tau", "eawae", constructors::push_const(Iota::Number(TAU))),
            Pattern::new("Euler's Reflection", "const/double/e", "aaq", constructors::push_const(Iota::Number(E))),
            Pattern::new("d", "const/null", "Nullary Reflection", constructors::push_const(Iota::Null(NullIota::Null))),
            Pattern::new("aqae", "const/true", "True Reflection", constructors::push_const(Iota::Bool(true))),
            Pattern::new("dedq", "const/false", "False Reflection", constructors::push_const(Iota::Bool(false))),




            //requires value to be set
            Pattern::new("Numerical Reflection", "number", "", Box::new(special::no_action)),

            Pattern::new("Entity Purification",  "get_entity", "qqqqqdaqa", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Entity Purification: Animal",  "get_entity/animal", "qqqqqdaqaawa", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Entity Purification: Monster",  "get_entity/monster", "qqqqqdaqaawq", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Entity Purification: Item",  "get_entity/item", "qqqqqdaqaaww", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Entity Purification: Player",  "get_entity/player", "qqqqqdaqaawe", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Entity Purification: Living",  "get_entity/living", "qqqqqdaqaawd", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Zone Distillation: Any",  "zone_entity", "qqqqqwded", constructors::spell_2(Stack::get_vector, Stack::get_number)),
            Pattern::new("Zone Distillation: Animal",  "zone_entity/animal", "qqqqqwdeddwa", constructors::spell_2(Stack::get_vector, Stack::get_number)),
            Pattern::new("Zone Distillation: Non-Animal",  "zone_entity/not_animal", "eeeeewaqaawa", constructors::spell_2(Stack::get_vector, Stack::get_number)),
            Pattern::new("Zone Distillation: Monster",  "zone_entity/monster", "qqqqqwdeddwq", constructors::spell_2(Stack::get_vector, Stack::get_number)),
            Pattern::new("Zone Distillation: Non-Monster",  "zone_entity/not_monster", "eeeeewaqaawq", constructors::spell_2(Stack::get_vector, Stack::get_number)),
            Pattern::new("Zone Distillation: Item",  "zone_entity/item", "qqqqqwdeddww", constructors::spell_2(Stack::get_vector, Stack::get_number)),
            Pattern::new("Zone Distillation: Non-Item",  "zone_entity/not_item", "eeeeewaqaaww", constructors::spell_2(Stack::get_vector, Stack::get_number)),
            Pattern::new("Zone Distillation: Player",  "zone_entity/player", "qqqqqwdeddwe", constructors::spell_2(Stack::get_vector, Stack::get_number)),
            Pattern::new("Zone Distillation: Non-Player",  "zone_entity/not_player", "eeeeewaqaawe", constructors::spell_2(Stack::get_vector, Stack::get_number)),
            Pattern::new("Zone Distillation: Living",  "zone_entity/living", "qqqqqwdeddwd", constructors::spell_2(Stack::get_vector, Stack::get_number)),
            Pattern::new("Zone Distillation: Non-Living",  "zone_entity/not_living", "eeeeewaqaawd", constructors::spell_2(Stack::get_vector, Stack::get_number)),
            




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
