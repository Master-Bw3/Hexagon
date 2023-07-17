use std::collections::HashMap;
use std::f32::consts::{E, PI, TAU};

use crate::interpreter::state::{EntityType, Stack, StackExt};
use crate::iota::{Iota, NullIota, VectorIota};
use crate::parser::ActionValue;
use crate::patterns::hex_casting::{
    eval, lists, math, read_write, sentinel, special, stack, swizzle,
};
use crate::patterns::more_iotas::matrices;
use crate::patterns::{constructors, Pattern};

pub type PatternRegistry = Vec<Pattern>;

pub trait PatternRegistryExt {
    fn gen_default_great_sigs() -> HashMap<String, String>;

    fn construct(great_sigs: &HashMap<String, String>) -> PatternRegistry;
    fn find(&self, query: &str, value: &Option<ActionValue>) -> Option<Pattern>;
}

impl PatternRegistryExt for PatternRegistry {
    #[rustfmt::skip]
    fn gen_default_great_sigs() -> HashMap<String, String> {
        let mut hashmap = HashMap::new();
        hashmap.insert("craft/battery".to_string(), "aqqqaqwwaqqqqqeqaqqqawwqwqwqwqwqw".to_string());
        hashmap.insert("potion/regeneration".to_string(), "qqqqaawawaedd".to_string());
        hashmap.insert("potion/night_vision".to_string(), "qqqaawawaeqdd".to_string());
        hashmap.insert("potion/absorption".to_string(), "qqaawawaeqqdd".to_string());
        hashmap.insert("potion/haste".to_string(), "qaawawaeqqqdd".to_string());
        hashmap.insert("potion/strength".to_string(), "aawawaeqqqqdd".to_string());
        hashmap.insert("lightning".to_string(), "waadwawdaaweewq".to_string());
        hashmap.insert("flight".to_string(), "eawwaeawawaa".to_string());
        hashmap.insert("create_lava".to_string(), "eaqawqadaqd".to_string());
        hashmap.insert("teleport".to_string(), "wwwqqqwwwqqeqqwwwqqwqqdqqqqqdqq".to_string());
        hashmap.insert("sentinel/create/great".to_string(), "waeawaeqqqwqwqqwq".to_string());
        hashmap.insert("dispel_rain".to_string(), "eeewwweeewwaqqddqdqd".to_string());
        hashmap.insert("summon_rain".to_string(), "wwweeewwweewdawdwad".to_string());
        hashmap.insert("brainsweep".to_string(), "qeqwqwqwqwqeqaeqeaqeqaeqaqded".to_string());
        hashmap
    }

    #[rustfmt::skip]
    fn construct(great_sigs: &HashMap<String, String>) -> PatternRegistry {
        let registry: PatternRegistry = vec![
            //special patterns
            Pattern::new_with_val("Consideration", "escape", "qqqaw", Box::new(special::escape)),
            Pattern::new("Introspection", "open_paren", "qqq", Box::new(special::introspect)),
            Pattern::new("Retrospection", "close_paren", "eee", Box::new(special::retrospect)),
            Pattern::new("Hermes' Gambit", "eval", "deaqq", Box::new(eval::eval)),
            Pattern::new("Thoth's Gambit", "for_each", "dadad", Box::new(eval::for_each)),
            Pattern::new("Charon's Gambit", "halt", "aqdee", Box::new(special::halt)),
            Pattern::new("Reveal", "print", "de", Box::new(special::print)),
            Pattern::new("Iris' Gambit", "eval/cc", "qwaqde", Box::new(eval::eval_cc)),



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
            Pattern::new("Augur's Exaltation", "if", "awdd",  Box::new(math::bool_if)),
            Pattern::new("Entropy Reflection", "random", "eqqq",  Box::new(math::random)),


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

            //stack manipulation
            Pattern::new("Gemini Decomposition", "duplicate", "aadaa", Box::new(stack::duplicate)),
            Pattern::new("Dioscuri Gambit", "two_dup", "aadadaaw", Box::new(stack::two_dup)),
            Pattern::new("Gemini Gambit", "duplicate_n", "aadaadaa", Box::new(stack::duplicate_n)),
            Pattern::new("Jester's Gambit", "swap", "aawdd", Box::new(stack::swap)),
            Pattern::new("Rotation Gambit", "rotate", "aaeaa", Box::new(stack::rotate)),
            Pattern::new("Rotation Gambit II", "rotate_reverse", "ddqdd", Box::new(stack::rotate_reverse)),
            Pattern::new("Prospector's Gambit", "over", "aaedd", Box::new(stack::over)),
            Pattern::new("Undertaker's Gambit", "tuck", "ddqaa", Box::new(stack::tuck)),
            Pattern::new("Flock's Reflection", "stack_len", "qwaeawqaeaqa", Box::new(stack::stack_len)),
            Pattern::new("Fisherman's Gambit", "fisherman", "ddad", Box::new(stack::fisherman)),
            Pattern::new("Fisherman's Gambit II", "fisherman/copy", "aada", Box::new(stack::fisherman_copy)),
            Pattern::new("Swindler's Gambit", "swizzle", "qaawdde", Box::new(swizzle::swizzle)),


            //read/write
            Pattern::new("Muninn's Reflection", "read/local",  "qeewdweddw", Box::new(read_write::read_local)),
            Pattern::new("Huginn's Gambit", "write/local",  "eqqwawqaaw", Box::new(read_write::write_local)),
            Pattern::new("Erase Item", "erase", "qdqawwaww", Box::new(read_write::erase)),
            Pattern::new("Craft Trinket", "craft/trinket", "wwaqqqqqeaqeaeqqqeaeq", Box::new(read_write::craft_trinket)),
            Pattern::new("Craft Cypher", "craft/cypher", "waqqqqq", Box::new(read_write::craft_cypher)),
            Pattern::new("Craft Artifact", "craft/artifact", "wwaqqqqqeawqwqwqwqwqwwqqeadaeqqeqqeadaeqq", Box::new(read_write::craft_artifact)),
            Pattern::new("Scribe's Reflection", "read",  "aqqqqq", Box::new(read_write::read)),
            Pattern::new("Scribe's Gambit", "write",  "deeeee", Box::new(read_write::write)),
            Pattern::new("Auditor's Reflection", "readable",  "aqqqqqe", Box::new(read_write::readable)),
            Pattern::new("Assessor's Reflection", "writable",  "deeeeeq", Box::new(read_write::writable)),
            Pattern::new("Akasha's Distillation", "akashic/read",  "qqqwqqqqqaq", Box::new(read_write::akashic_read)),
            Pattern::new("Akasha's Gambit", "akashic/write",  "eeeweeeeede", Box::new(read_write::akashic_write)),
            Pattern::new("Chronicler's Purification", "read/entity",  "wawqwqwqwqwqw", Box::new(read_write::read_entity)),
            Pattern::new("Chronicler's Gambit", "write/entity",  "wdwewewewewew", Box::new(read_write::write_entity)),
            Pattern::new("Auditor's Purification", "readable/entity",  "wawqwqwqwqwqwew", Box::new(read_write::readable_entity)),
            Pattern::new("Assessor's Purification", "writable/entity",  "wdwewewewewewqw", Box::new(read_write::writeable_entity)),


            //sentinel
            Pattern::new( "sentinel/create", "Summon Sentinel", "waeawae", Box::new(sentinel::create)),
            Pattern::new( "sentinel/destroy", "Banish Sentinel","qdwdqdw", Box::new(sentinel::destroy)),
            Pattern::new( "sentinel/get_pos", "Locate Sentinel", "waeawaede", Box::new(sentinel::get_pos)),
            Pattern::new( "sentinel/wayfind", "Wayfind Sentinel","waeawaedwa", Box::new(sentinel::wayfind)),


            //consts
            Pattern::new("Mind's Reflection", "get_caster", "qaq", 
                constructors::push_const(Iota::Entity("Caster".to_string()))),
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
            Pattern::new("Nullary Reflection", "const/null", "d", constructors::push_const(Iota::Null(NullIota::Null))),
            Pattern::new("True Reflection", "const/true", "aqae",constructors::push_const(Iota::Bool(true))),
            Pattern::new("False Reflection", "const/false", "dedq",constructors::push_const(Iota::Bool(false))),

            //spells
            Pattern::new("Alter Gravity", "interop/gravity/set", "wdwdwaaqw", constructors::spell_2(Stack::get_entity, Stack::get_vector)),
            Pattern::new("Alter Scale", "interop/pehkui/set", "ddwdwwdwwd", constructors::spell_2(Stack::get_entity, Stack::get_number)),
            Pattern::new("Explosion", "explode", "aawaawaa", constructors::spell_2(Stack::get_vector, Stack::get_number)),
            Pattern::new("Fireball", "explode/fire", "ddwddwdd", constructors::spell_2(Stack::get_vector, Stack::get_number)),
            Pattern::new("Impulse", "add_motion", "awqqqwaqw", constructors::spell_2(Stack::get_entity, Stack::get_vector)),
            Pattern::new("Blink", "blink", "awqqqwaq", constructors::spell_2(Stack::get_entity, Stack::get_number)),
            Pattern::new("Break Block", "break_block", "qaqqqqq", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Place Block", "place_block", "eeeeede", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Internalize Pigment", "colorize", "awddwqawqwawq", Box::new(special::no_action)),
            Pattern::new("Create Water", "create_water", "aqawqadaq", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Destroy Liquid", "destroy_water", "dedwedade", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Ignite Block", "ignite", "aaqawawa", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Extinguish Area", "extinguish", "ddedwdwd", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Conjure Block", "conjure_block", "qqa", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Conjure Light", "conjure_light", "qqd", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Overgrow", "bonemeal", "wqaqwawqaqw", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Recharge Item", "recharge", "qqqqqwaeaeaeaeaea", constructors::spell_1(Stack::get_entity)),
            Pattern::new("Edify Sapling", "edify", "wqaqwd", constructors::spell_1(Stack::get_vector)),
            Pattern::new("Make Note", "beep", "adaa", Box::new(special::beep)),
            Pattern::new("White Sun's Nadir", "potion/weakness", "qqqqqaqwawaw", constructors::spell_3(Stack::get_entity, Stack::get_number, Stack::get_number)),
            Pattern::new("Blue Sun's Nadir", "potion/levitation", "qqqqqawwawawd", constructors::spell_2(Stack::get_entity, Stack::get_number)),
            Pattern::new("Black Sun's Nadir", "potion/wither", "qqqqqaewawawe", constructors::spell_3(Stack::get_entity, Stack::get_number, Stack::get_number)),
            Pattern::new("Red Sun's Nadir", "potion/poison", "qqqqqadwawaww", constructors::spell_3(Stack::get_entity, Stack::get_number, Stack::get_number)),
            Pattern::new("Green Sun's Nadir", "potion/slowness", "qqqqqadwawaw", constructors::spell_3(Stack::get_entity, Stack::get_number, Stack::get_number)),

            //great spells
            Pattern::new("Craft Phial", "craft/battery", great_sigs.get("craft/battery").unwrap(), 
                constructors::spell_1(Stack::get_entity)),

            Pattern::new("White Sun's Zenith", "potion/regeneration", great_sigs.get("potion/regeneration").unwrap(), 
                constructors::spell_3(Stack::get_entity, Stack::get_number, Stack::get_number)),

            Pattern::new("Blue Sun's Zenith", "potion/night_vision", great_sigs.get("potion/night_vision").unwrap(), 
                constructors::spell_2(Stack::get_entity, Stack::get_number)),

            Pattern::new("Black Sun's Zenith", "potion/absorption", great_sigs.get("potion/absorption").unwrap(), 
                constructors::spell_3(Stack::get_entity, Stack::get_number, Stack::get_number)),

            Pattern::new("Red Sun's Zenith", "potion/haste", great_sigs.get("potion/haste").unwrap(), 
                constructors::spell_3(Stack::get_entity, Stack::get_number, Stack::get_number)),

            Pattern::new("Green Sun's Zenith", "potion/strength", great_sigs.get("potion/strength").unwrap(), 
                constructors::spell_3(Stack::get_entity, Stack::get_number, Stack::get_number)),

            Pattern::new("Summon Lightning", "lightning", great_sigs.get("lightning").unwrap(), 
                constructors::spell_1(Stack::get_vector)),

            Pattern::new("Flight", "flight", great_sigs.get("flight").unwrap(), 
                constructors::spell_3(Stack::get_entity, Stack::get_number, Stack::get_number)),

            Pattern::new("Create Lava", "create_lava", great_sigs.get("create_lava").unwrap(), 
                constructors::spell_1(Stack::get_vector)),

            Pattern::new("Greater Teleport", "teleport", great_sigs.get("teleport").unwrap(), 
                constructors::spell_2(Stack::get_entity, Stack::get_vector)),

            Pattern::new("Summon Greater Sentinel", "sentinel/create/great", great_sigs.get("sentinel/create/great").unwrap(), 
                constructors::spell_1(Stack::get_vector)),

            Pattern::new("Dispel Rain", "dispel_rain", great_sigs.get("dispel_rain").unwrap(), Box::new(special::no_action)),
            Pattern::new("Summon Rain", "summon_rain", great_sigs.get("summon_rain").unwrap(), Box::new(special::no_action)),
            Pattern::new("Flay Mind", "brainsweep", great_sigs.get("brainsweep").unwrap(), Box::new(special::no_action)),


            //requires value to be set
            Pattern::new_with_val("Numerical Reflection", "number", "", 
                constructors::value_0(Stack::get_number, "Number", "Numerical Reflection")),

            Pattern::new_with_val("Entity Purification", "get_entity", "qqqqqdaqa",
                constructors::get_entity(None, "Entity Purification")),

            Pattern::new_with_val("Entity Purification: Animal", "get_entity/animal", "qqqqqdaqaawa",
                constructors::get_entity(Some(&EntityType::Animal), "Entity Purification: Animal")),


            Pattern::new_with_val("Entity Purification: Monster", "get_entity/monster", "qqqqqdaqaawq",
                constructors::get_entity(Some(&EntityType::Monster), "Entity Purification: Monster")),

            Pattern::new_with_val("Entity Purification: Item", "get_entity/item", "qqqqqdaqaaww",
                constructors::get_entity(Some(&EntityType::Item), "Entity Purification: Item")),

            Pattern::new_with_val("Entity Purification: Player", "get_entity/player", "qqqqqdaqaawe",
                constructors::get_entity(Some(&EntityType::Player), "Entity Purification: Player")),

            Pattern::new_with_val("Entity Purification: Living", "get_entity/living", "qqqqqdaqaawd",
                constructors::get_entity(Some(&EntityType::Living), "Entity Purification: Living")),

            Pattern::new_with_val("Zone Distillation: Any", "zone_entity", "qqqqqwded",
                constructors::zone_entity(None, &false, "Zone Distillation: Any")),

            Pattern::new_with_val("Zone Distillation: Animal", "zone_entity/animal", "qqqqqwdeddwa",
                constructors::zone_entity(Some(&EntityType::Animal), &false, "Zone Distillation: Animal")),

            Pattern::new_with_val("Zone Distillation: Non-Animal", "zone_entity/not_animal", "eeeeewaqaawa",
                constructors::zone_entity(Some(&EntityType::Animal), &true, "Zone Distillation: Non-Animal")),

            Pattern::new_with_val("Zone Distillation: Monster", "zone_entity/monster", "qqqqqwdeddwq",
                constructors::zone_entity(Some(&EntityType::Monster), &false, "Zone Distillation: Monster")),

            Pattern::new_with_val("Zone Distillation: Non-Monster", "zone_entity/not_monster", "eeeeewaqaawq",
                constructors::zone_entity(Some(&EntityType::Monster), &true, "Zone Distillation: Non-Monster")),

            Pattern::new_with_val("Zone Distillation: Item", "zone_entity/item", "qqqqqwdeddww",
                constructors::zone_entity(Some(&EntityType::Item), &false, "Zone Distillation: Item")),

            Pattern::new_with_val("Zone Distillation: Non-Item", "zone_entity/not_item", "eeeeewaqaaww",
                constructors::zone_entity(Some(&EntityType::Item), &true, "Zone Distillation: Non-Item")),

            Pattern::new_with_val("Zone Distillation: Player", "zone_entity/player", "qqqqqwdeddwe",
                constructors::zone_entity(Some(&EntityType::Player), &false, "Zone Distillation: Player")),

            Pattern::new_with_val("Zone Distillation: Non-Player", "zone_entity/not_player", "eeeeewaqaawe",
                constructors::zone_entity(Some(&EntityType::Player), &true, "Zone Distillation: Non-Player")),

            Pattern::new_with_val("Zone Distillation: Living", "zone_entity/living", "qqqqqwdeddwd",
                constructors::zone_entity(Some(&EntityType::Living), &false, "Zone Distillation: Living")),

            Pattern::new_with_val("Zone Distillation: Non-Living", "zone_entity/not_living", "eeeeewaqaawd",
                constructors::zone_entity(Some(&EntityType::Living), &true, "Zone Distillation: Non-Living")),

            Pattern::new_with_val("entity_pos/eye", "Compass' Purification", "aa",
                constructors::value_1(Stack::get_entity, Stack::get_vector, "Vector", "entity_pos/eye")),

            Pattern::new_with_val("entity_pos/foot", "Compass' Purification II", "dd",
                constructors::value_1(Stack::get_entity, Stack::get_vector, "Vector", "entity_pos/foot")),

            Pattern::new_with_val("Alidade's Purification", "get_entity_look", "wa",
                constructors::value_1(Stack::get_entity, Stack::get_vector, "Vector", "get_entity_look")),

            Pattern::new_with_val("Stadiometer's Purification", "get_entity_height", "awq",
                constructors::value_1(Stack::get_entity, Stack::get_number, "Number", "get_entity_height")),

            Pattern::new_with_val("Pace Purification", "get_entity_velocity", "wq",
                constructors::value_1(Stack::get_entity, Stack::get_vector, "Vector", "get_entity_velocity")),

            Pattern::new_with_val("Gravitational Purification", "interop/gravity/get", "wawawddew",
                constructors::value_1(Stack::get_entity, Stack::get_vector, "Vector", "interop/gravity/get")),

            Pattern::new_with_val("Gulliver's Purification", "interop/pehkui/get", "aawawwawwa",
                constructors::value_1(Stack::get_entity, Stack::get_number, "Number", "interop/pehkui/get")),

            Pattern::new_with_val("Archer's Distillation", "raycast", "wqaawdd",
                constructors::value_2(Stack::get_vector, Stack::get_vector, Stack::get_vector, "Vector", "Archer's Distillation")),

            Pattern::new_with_val("Architect's Distillation", "raycast/axis", "weddwaa",
                constructors::value_2(Stack::get_vector, Stack::get_vector, Stack::get_vector, "Vector", "Architect's Distillation")),

            Pattern::new_with_val("Scout's Distillation", "raycast/entity", "weaqa",
                constructors::value_2(Stack::get_vector, Stack::get_vector, Stack::get_entity, "Entity", "Scout's Distillation")),

            Pattern::new_with_val("Waystone Reflection", "circle/impetus_pos", "eaqwqae",
                constructors::value_1(Stack::get_entity, Stack::get_number, "Number", "Waystone Reflection")),

            Pattern::new_with_val("Lodestone Reflection", "circle/impetus_dir", "eaqwqaewede",
                constructors::value_1(Stack::get_entity, Stack::get_number, "Number", "Lodestone Reflection")),

            Pattern::new_with_val("Lesser Fold Reflection", "circle/bounds/min", "eaqwqaewdd",
                constructors::value_1(Stack::get_entity, Stack::get_number, "Number", "Lesser Fold Reflection")),

            Pattern::new_with_val("Greater Fold Reflection", "circle/bounds/max", "aqwqawaaqa",
                constructors::value_1(Stack::get_entity, Stack::get_number, "Number", "Greater Fold Reflection")),

            //MoreIotas
            Pattern::new("Transformation Purification", "matrix/make", "awwaeawwaadwa", Box::new(matrices::make)),

        ];

        registry
    }

    fn find(&self, query: &str, value: &Option<ActionValue>) -> Option<Pattern> {
        if let Some(ActionValue::Bookkeeper(code)) = value {
            let mut bookkeeper =
                Pattern::new_with_val("Bookkeeper's Gambit", "mask", "", Box::new(stack::mask));
            bookkeeper.signature = parse_bookkeeper_code(code);
            if query == bookkeeper.display_name
                || query == bookkeeper.internal_name
                || query == bookkeeper.signature
            {
                return Some(bookkeeper);
            } else {
                return None;
            }
        }

        self.iter()
            .filter(|entry| {
                entry.display_name == *query
                    || entry.internal_name == *query
                    || entry.signature == *query
            })
            .collect::<Vec<&Pattern>>()
            .get(0)
            .copied()
            .cloned()
    }
}

fn parse_bookkeeper_code(code: &str) -> String {
    code.chars()
        .fold(
            (' ', vec![]),
            |mut acc: (char, Vec<&str>), segment| match segment {
                '-' => {
                    if acc.0 == '-' {
                        acc.1.push("w");
                        (segment, acc.1)
                    } else if acc.0 == 'v' {
                        acc.1.push("e");
                        (segment, acc.1)
                    } else {
                        (segment, acc.1)
                    }
                }

                'v' => {
                    if acc.0 == '-' {
                        acc.1.push("ea");
                        (segment, acc.1)
                    } else if acc.0 == 'v' {
                        acc.1.push("da");
                        (segment, acc.1)
                    } else {
                        acc.1.push("a");
                        (segment, acc.1)
                    }
                }

                _ => acc,
            },
        )
        .1
        .concat()
}
