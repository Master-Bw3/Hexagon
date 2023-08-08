use std::{collections::HashMap, ops::Deref, rc::Rc};

use crate::{
    interpreter::state::Entity,
    iota::{
        hex_casting::{
            entity::EntityIota, garbage::GarbageIota, list::ListIota, null::NullIota,
            number::NumberIota, pattern::PatternIota,
        },
        more_iotas::matrix::MatrixIota,
        Iota,
    },
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};
use nalgebra::matrix;
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

pub type Macros = HashMap<String, (PatternIota, AstNode)>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct HexParser;
pub fn parse(
    source: &str,
    great_spell_sigs: &HashMap<String, String>,
    conf_entities: &mut HashMap<String, Entity>,
) -> Result<(AstNode, Macros), Box<Error<Rule>>> {
    let mut ast = vec![];
    let pattern_registry = PatternRegistry::construct(great_spell_sigs);
    let mut macros: Macros = HashMap::new();

    let pairs = HexParser::parse(Rule::File, source)?;
    for pair in pairs.clone() {
        if Rule::Macro == pair.as_rule() {
            let hex_macro = parse_macro(pair, &pattern_registry, conf_entities);
            macros.insert(hex_macro.0, hex_macro.1);
        }
    }
    for pair in pairs {
        if let Some(node) = construct_ast_node(pair, &pattern_registry, conf_entities, &macros) {
            ast.push(node);
        }
    }

    Ok((AstNode::File(ast), macros))
}

fn parse_macro(
    pair: Pair<'_, Rule>,
    pattern_registry: &PatternRegistry,
    conf_entities: &mut HashMap<String, Entity>,
) -> (String, (PatternIota, AstNode)) {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let pattern = parse_pattern(
        inner.next().unwrap(),
        pattern_registry,
        conf_entities,
        &HashMap::new(),
    );
    let hex = construct_ast_node(
        inner.next().unwrap(),
        pattern_registry,
        conf_entities,
        &HashMap::new(),
    )
    .unwrap();

    (name, (pattern, hex))
}

fn construct_ast_node(
    pair: Pair<'_, Rule>,
    pattern_registry: &PatternRegistry,
    conf_entities: &mut HashMap<String, Entity>,
    macros: &Macros,
) -> Option<AstNode> {
    match pair.as_rule() {
        Rule::Action => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next();
            let righter = pair.next();

            Some(parse_action(
                left,
                right,
                righter,
                pattern_registry,
                conf_entities,
                macros,
            ))
        }
        Rule::Op => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let arg = pair.next();

            Some(parse_op(name, arg, pattern_registry, conf_entities, macros))
        }
        Rule::Var => Some(parse_var(pair)),
        Rule::Embed => Some(parse_embed(pair, pattern_registry, conf_entities, macros)),
        Rule::IfBlock => Some(parse_if_block(
            pair,
            pattern_registry,
            conf_entities,
            macros,
        )),
        Rule::Term => Some(AstNode::Hex {
            nodes: pair
                .into_inner()
                .filter_map(|node| {
                    construct_ast_node(node, pattern_registry, conf_entities, macros)
                })
                .collect(),
            external: false,
        }),

        Rule::ExternTerm => Some(AstNode::Hex {
            nodes: pair
                .into_inner()
                .filter_map(|node| {
                    construct_ast_node(node, pattern_registry, conf_entities, macros)
                })
                .collect(),
            external: true,
        }),
        _ => None,
    }
}

fn parse_op(
    name: Pair<'_, Rule>,
    arg: Option<Pair<'_, Rule>>,
    pattern_registry: &PatternRegistry,
    conf_entities: &mut HashMap<String, Entity>,
    macros: &Macros,
) -> AstNode {
    AstNode::Op {
        name: {
            match name.as_str() {
                "Store" => OpName::Store,
                "Copy" => OpName::Copy,
                "Push" => OpName::Push,
                _ => unreachable!(),
            }
        },
        arg: {
            arg.map(|pair| match pair.as_rule() {
                Rule::Iota => {
                    OpValue::Iota(parse_iota(pair, pattern_registry, conf_entities, macros))
                }
                Rule::Var => OpValue::Var(pair.as_str().to_string()),
                _ => unreachable!(),
            })
        },
        line: name.line_col(),
    }
}

fn parse_action(
    left: Pair<'_, Rule>,
    right: Option<Pair<'_, Rule>>,
    righter: Option<Pair<'_, Rule>>,
    pattern_registry: &PatternRegistry,
    conf_entities: &mut HashMap<String, Entity>,
    macros: &Macros,
) -> AstNode {
    right
        .clone()
        .map(|pair| match pair.as_rule() {
            Rule::Iota => AstNode::Action {
                name: left.as_str().to_string(),
                value: Some(ActionValue::Iota(parse_iota(
                    pair.clone(),
                    pattern_registry,
                    conf_entities,
                    macros,
                ))),
                line: pair.line_col(),
            },

            Rule::BookkeeperValue => AstNode::Action {
                name: left.as_str().to_string(),
                value: Some(ActionValue::Bookkeeper(parse_bookkeeper(pair.clone()))),
                line: pair.line_col(),
            },

            _ => AstNode::Action {
                name: format!("{}: {}", left.as_str(), right.unwrap().as_str()),
                value: righter.map(|p| {
                    ActionValue::Iota(parse_iota(p, pattern_registry, conf_entities, macros))
                }),
                line: pair.line_col(),
            },
        })
        .unwrap_or(AstNode::Action {
            name: left.as_str().to_string(),
            value: None,
            line: left.line_col(),
        })
}

fn parse_action_iota(
    left: Pair<'_, Rule>,
    right: Option<Pair<'_, Rule>>,
    righter: Option<Pair<'_, Rule>>,
    pattern_registry: &PatternRegistry,
    macros: &Macros,
    conf_entities: &mut HashMap<String, Entity>,
) -> PatternIota {
    right
        .clone()
        .map(|pair| match pair.as_rule() {
            Rule::Iota => PatternIota::from_name(
                pattern_registry,
                left.as_str(),
                Some(ActionValue::Iota(parse_iota(
                    pair.clone(),
                    pattern_registry,
                    conf_entities,
                    macros,
                ))),
                None,
            ),
            Rule::EntityType => PatternIota::from_name(
                pattern_registry,
                &format!("{}: {}", left.as_str(), right.unwrap().as_str()),
                righter.map(|p| {
                    ActionValue::Iota(parse_iota(p, pattern_registry, conf_entities, macros))
                }),
                None,
            ),
            Rule::BookkeeperValue => PatternIota::from_name(
                pattern_registry,
                left.as_str(),
                Some(ActionValue::Bookkeeper(parse_bookkeeper(pair.clone()))),
                None,
            ),
            _ => unreachable!(),
        })
        .unwrap_or(Ok(
            //check if macro
            macros
                .get(left.as_str())
                .map(|(pattern, _)| pattern.clone())
                .unwrap_or_else(
                    //check if pattern name
                    || PatternIota::from_name(pattern_registry, left.as_str(), None, None).unwrap(),
                ),
        ))
        .unwrap()
}

fn parse_var(pair: Pair<'_, Rule>) -> AstNode {
    AstNode::Op {
        name: OpName::Push,
        arg: { Some(OpValue::Var(pair.as_str().to_string())) },
        line: pair.line_col(),
    }
}

fn parse_embed(
    pair: Pair<'_, Rule>,
    pattern_registry: &PatternRegistry,
    conf_entities: &mut HashMap<String, Entity>,
    macros: &Macros,
) -> AstNode {
    let inner_pair = pair.clone().into_inner().next().unwrap();
    AstNode::Op {
        name: {
            match inner_pair.as_rule() {
                Rule::DirectEmbed => OpName::Embed,
                Rule::SmartEmbed => OpName::SmartEmbed,
                Rule::IntroEmbed => OpName::IntroEmbed,
                Rule::ConsiderEmbed => OpName::ConsiderEmbed,
                _ => unreachable!(),
            }
        },
        arg: (inner_pair
            .into_inner()
            .next()
            .map(|iota| OpValue::Iota(parse_iota(iota, pattern_registry, conf_entities, macros)))),
        line: pair.line_col(),
    }
}

fn parse_if_block(
    pair: Pair<'_, Rule>,
    pattern_registry: &PatternRegistry,
    conf_entities: &mut HashMap<String, Entity>,
    macros: &Macros,
) -> AstNode {
    fn parse_inner(
        line: (usize, usize),
        mut inner: Pairs<'_, Rule>,
        pattern_registry: &PatternRegistry,
        conf_entities: &mut HashMap<String, Entity>,
        macros: &Macros,
    ) -> AstNode {
        AstNode::IfBlock {
            condition: {
                let mut condition = inner.next().unwrap().into_inner();
                Box::new(
                    construct_ast_node(
                        condition.next().unwrap(),
                        pattern_registry,
                        conf_entities,
                        macros,
                    )
                    .unwrap(),
                )
            },
            succeed: {
                let mut succeed = inner.next().unwrap().into_inner();
                Box::new(
                    construct_ast_node(
                        succeed.next().unwrap(),
                        pattern_registry,
                        conf_entities,
                        macros,
                    )
                    .unwrap(),
                )
            },
            fail: {
                inner.clone().next().map(|branch| match branch.as_rule() {
                    Rule::Else => Box::new(
                        construct_ast_node(
                            branch.into_inner().next().unwrap(),
                            pattern_registry,
                            conf_entities,
                            macros,
                        )
                        .unwrap(),
                    ),
                    Rule::ElseIf => Box::new(parse_inner(
                        line,
                        inner,
                        pattern_registry,
                        conf_entities,
                        macros,
                    )),
                    _ => unreachable!(),
                })
            },
            line,
        }
    }
    parse_inner(
        pair.line_col(),
        pair.into_inner(),
        pattern_registry,
        conf_entities,
        macros,
    )
}

pub fn parse_iota(
    pair: Pair<'_, Rule>,
    pattern_registry: &PatternRegistry,
    conf_entities: &mut HashMap<String, Entity>,
    macros: &Macros,
) -> Rc<dyn Iota> {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::Number => Rc::new(inner_pair.as_str().parse::<NumberIota>().unwrap()),
        Rule::Pattern => Rc::new(parse_pattern(
            inner_pair.into_inner().next().unwrap(),
            pattern_registry,
            conf_entities,
            macros,
        )),
        Rule::Vector => {
            let mut inner = inner_pair.into_inner();
            Rc::new(matrix![
                inner.next().unwrap().as_str().parse().unwrap(),
                inner.next().unwrap().as_str().parse().unwrap(),
                inner.next().unwrap().as_str().parse().unwrap();
            ])
        }
        Rule::Bool => match inner_pair.as_str() {
            "True" => Rc::new(true),
            "False" => Rc::new(false),
            _ => unreachable!(),
        },
        Rule::Influence => match inner_pair.as_str() {
            "Garbage" => Rc::new(GarbageIota),
            "Null" => Rc::new(NullIota),
            _ => unreachable!(),
        },
        Rule::Entity => {
            let name = inner_pair.as_str()[1..].to_string();
            Rc::new(EntityIota {
                name: Rc::from(name.clone()),
                uuid: conf_entities
                    .get(&name)
                    .map(|entity| entity.uuid.clone())
                    .unwrap_or("[I;0,0,0,0]".to_string()),
            })
        }
        Rule::List => {
            let inner = inner_pair.into_inner();
            Rc::new(
                inner
                    .map(|x| parse_iota(x, pattern_registry, conf_entities, macros))
                    .collect::<ListIota>(),
            )
        }
        Rule::String => {
            let string = snailquote::unescape(&inner_pair.as_str())
                .unwrap()
                .to_string();
            Rc::new(string)
        }
        Rule::Matrix => {
            let mut inner = inner_pair.into_inner();
            let nrows = inner.next().unwrap().as_str().parse::<usize>().unwrap();
            let ncols = inner.next().unwrap().as_str().parse::<usize>().unwrap();
            let data = inner
                .map(|x| x.as_str().parse::<NumberIota>().unwrap())
                .collect::<Vec<_>>();

            Rc::new(MatrixIota::from_vec(nrows, ncols, data))
        }

        _ => unreachable!(),
    }
}

fn parse_pattern(
    pair: Pair<'_, Rule>,
    pattern_registry: &PatternRegistry,
    conf_entities: &mut HashMap<String, Entity>,
    macros: &Macros,
) -> PatternIota {
    match pair.as_str() {
        "{" => PatternIota::from_name(pattern_registry, "open_paren", None, None).unwrap(),

        "}" => PatternIota::from_name(pattern_registry, "close_paren", None, None).unwrap(),

        _ => match pair.as_rule() {
            Rule::Action => {
                let mut pairs = pair.into_inner();
                parse_action_iota(
                    pairs.next().unwrap(),
                    pairs.next(),
                    pairs.next(),
                    pattern_registry,
                    macros,
                    conf_entities,
                )
            }
            Rule::PatternRaw => {
                PatternIota::from_sig(pair.into_inner().last().unwrap().as_str(), None, None)
            }
            _ => unreachable!("{:?}", pair.as_rule()),
        },
    }
}

fn parse_bookkeeper(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

// fn parse_string(pair: Pair<'_, Rule>) -> String {
//     pair.as_str()
//         .trim_start_matches('\"')
//         .trim_end_matches('\"')
//         .to_string()
// }

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    File(Vec<AstNode>),
    Action {
        line: (usize, usize),
        name: String,
        value: Option<ActionValue>,
    },
    Hex {
        external: bool,
        nodes: Vec<AstNode>,
    },
    Op {
        line: (usize, usize),
        name: OpName,
        arg: Option<OpValue>,
    },
    IfBlock {
        line: (usize, usize),
        condition: Box<AstNode>,
        succeed: Box<AstNode>,
        fail: Option<Box<AstNode>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpName {
    Store,
    Copy,
    Push,
    Embed,
    SmartEmbed,
    ConsiderEmbed,
    IntroEmbed,
}

#[derive(Debug, Clone)]
pub enum OpValue {
    Iota(Rc<dyn Iota>),
    Var(String),
}

impl PartialEq for OpValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Iota(l0), Self::Iota(r0)) => l0.tolerates_other(r0.deref()),
            (Self::Var(l0), Self::Var(r0)) => l0 == r0,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ActionValue {
    Iota(Rc<dyn Iota>),
    Bookkeeper(String),
}

impl PartialEq for ActionValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Iota(l0), Self::Iota(r0)) => l0.tolerates_other(r0.deref()),
            (Self::Bookkeeper(l0), Self::Bookkeeper(r0)) => l0 == r0,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex() {
        parse(
            "
            {
            Bookkeeper's Gambit: [{\"bob\", minecraft:cow}, 1] \n
            Gemini Decomposition \n
            Pace Purification \n
            Augur's Purification \n
            Jester's Gambit \n
            Nullary Reflection \n
            Augur's Exaltation \n
            Numerical Reflection: (1, 2, 3)
            Store()
            }
            Consideration: Huginn's Gambit
            ",
            &PatternRegistry::gen_default_great_sigs(),
            &mut HashMap::new(),
        )
        .unwrap();
    }
}
