use std::collections::HashMap;

use crate::{
    interpreter::state::Holding,
    iota::{EntityIota, EntityType, GarbageIota, Iota, NullIota, PatternIota},
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};
use nalgebra::matrix;
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct HexParser;
pub fn parse(
    source: &str,
    great_spell_sigs: &HashMap<String, String>,
) -> Result<AstNode, Box<Error<Rule>>> {

    let mut ast = vec![];
    let pattern_registry = PatternRegistry::construct(great_spell_sigs);

    let pairs = HexParser::parse(Rule::File, source)?;
    for pair in pairs {
        if let Some(node) = construct_ast_node(pair, &pattern_registry) {
            ast.push(node);
        }
    }

    Ok(AstNode::File(ast))
}

fn construct_ast_node(pair: Pair<'_, Rule>, pattern_registry: &PatternRegistry) -> Option<AstNode> {
    match pair.as_rule() {
        Rule::Action => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next();
            let righter = pair.next();

            Some(parse_action(left, right, righter, pattern_registry))
        }
        Rule::Op => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let arg = pair.next();

            Some(parse_op(name, arg, pattern_registry))
        }
        Rule::EscapedIntroRetro => Some(parse_intro_retro(pair)),
        Rule::Var => Some(parse_var(pair)),
        Rule::Embed => Some(parse_embed(pair, pattern_registry)),
        Rule::IfBlock => Some(parse_if_block(pair, pattern_registry)),
        Rule::Term => Some(AstNode::Hex(
            pair.into_inner()
                .filter_map(|node| construct_ast_node(node, pattern_registry))
                .collect(),
        )),
        _ => None,
    }
}

fn parse_op(
    name: Pair<'_, Rule>,
    arg: Option<Pair<'_, Rule>>,
    pattern_registry: &PatternRegistry,
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
                Rule::Iota => OpValue::Iota(parse_iota(pair, pattern_registry)),
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
) -> AstNode {
    right
        .clone()
        .map(|pair| match pair.as_rule() {
            Rule::Iota => AstNode::Action {
                name: left.as_str().to_string(),
                value: Some(ActionValue::Iota(parse_iota(
                    pair.clone(),
                    pattern_registry,
                ))),
                line: pair.line_col(),
            },
            Rule::EntityType => AstNode::Action {
                name: format!("{}: {}", left.as_str(), right.unwrap().as_str()),
                value: righter.map(|p| ActionValue::Iota(parse_iota(p, pattern_registry))),
                line: pair.line_col(),
            },
            Rule::BookkeeperValue => AstNode::Action {
                name: left.as_str().to_string(),
                value: Some(ActionValue::Bookkeeper(parse_bookkeeper(pair.clone()))),
                line: pair.line_col(),
            },
            _ => unreachable!(),
        })
        .unwrap_or(AstNode::Action {
            name: left.as_str().to_string(),
            value: None,
            line: left.line_col(),
        })
}

fn parse_intro_retro(pair: Pair<'_, Rule>) -> AstNode {
    let line = pair.line_col();
    let inner = pair.into_inner().next().unwrap();
    AstNode::Action {
        name: {
            match inner.as_str() {
                "{" => "open_paren".to_string(),
                "}" => "close_paren".to_string(),
                _ => unreachable!(),
            }
        },
        value: None,
        line,
    }
}

fn parse_var(pair: Pair<'_, Rule>) -> AstNode {
    AstNode::Op {
        name: OpName::Push,
        arg: { Some(OpValue::Var(pair.as_str().to_string())) },
        line: pair.line_col(),
    }
}

fn parse_embed(pair: Pair<'_, Rule>, pattern_registry: &PatternRegistry) -> AstNode {
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
            .map(|iota| OpValue::Iota(parse_iota(iota, pattern_registry)))),
        line: pair.line_col(),
    }
}

fn parse_if_block(pair: Pair<'_, Rule>, pattern_registry: &PatternRegistry) -> AstNode {
    fn parse_inner(
        line: (usize, usize),
        mut inner: Pairs<'_, Rule>,
        pattern_registry: &PatternRegistry,
    ) -> AstNode {
        AstNode::IfBlock {
            condition: {
                let mut condition = inner.next().unwrap().into_inner();
                Box::new(construct_ast_node(condition.next().unwrap(), pattern_registry).unwrap())
            },
            succeed: {
                let mut succeed = inner.next().unwrap().into_inner();
                Box::new(construct_ast_node(succeed.next().unwrap(), pattern_registry).unwrap())
            },
            fail: {
                inner.clone().next().map(|branch| match branch.as_rule() {
                    Rule::Else => Box::new(
                        construct_ast_node(branch.into_inner().next().unwrap(), pattern_registry)
                            .unwrap(),
                    ),
                    Rule::ElseIf => Box::new(parse_inner(line, inner, pattern_registry)),
                    _ => unreachable!(),
                })
            },
            line,
        }
    }
    parse_inner(pair.line_col(), pair.into_inner(), pattern_registry)
}

pub fn parse_iota(pair: Pair<'_, Rule>, pattern_registry: &PatternRegistry) -> Iota {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::Number => Iota::Number(inner_pair.as_str().parse().unwrap()),
        Rule::Pattern => match inner_pair.clone().into_inner().next() {
            Some(inner_inner_pair) => match inner_inner_pair.as_str() {
                "{" => Iota::Pattern(PatternIota::from_name(pattern_registry, "open_paren", None)),
                "}" => Iota::Pattern(PatternIota::from_name(
                    pattern_registry,
                    "close_paren",
                    None,
                )),
                _ => match inner_inner_pair.as_rule() {
                    Rule::ActionName => Iota::Pattern(PatternIota::from_name(
                        pattern_registry,
                        inner_pair.as_str(),
                        None,
                    )),
                    Rule::PatternName => Iota::Pattern(PatternIota::from_sig(
                        inner_inner_pair.into_inner().last().unwrap().as_str(),
                        None,
                    )),
                    _ => unreachable!(),
                },
            },
            None => unreachable!(),
        },
        Rule::Vector => {
            let mut inner = inner_pair.into_inner();
            Iota::Vector(matrix![
                inner.next().unwrap().as_str().parse().unwrap(),
                inner.next().unwrap().as_str().parse().unwrap(),
                inner.next().unwrap().as_str().parse().unwrap();
            ])
        }
        Rule::Bool => match inner_pair.as_str() {
            "True" => Iota::Bool(true),
            "False" => Iota::Bool(false),
            _ => unreachable!(),
        },
        Rule::Influence => match inner_pair.as_str() {
            "Garbage" => Iota::Garbage(GarbageIota::Garbage),
            "Null" => Iota::Null(NullIota::Null),
            _ => unreachable!(),
        },
        Rule::Entity => {
            let mut inner = inner_pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let entity_type = parse_entity_type(inner.next().unwrap().as_str().to_string());

            Iota::Entity(EntityIota {
                name,
                entity_type,
                holding: Box::new(Holding::None),
            })
        }
        Rule::List => {
            let inner = inner_pair.into_inner();
            Iota::List(inner.map(|x| parse_iota(x, pattern_registry)).collect())
        }
        _ => unreachable!(),
    }
}

pub fn parse_entity_type(string: String) -> EntityType {
    match &string[..] {
        "Animal" => EntityType::Animal,
        "Monster" => EntityType::Monster,
        "Living" => EntityType::Living,
        "Item" => EntityType::Item,
        "Player" => EntityType::Player,
        "Misc" => EntityType::Misc,
        _ => unreachable!(),
    }
}

fn parse_bookkeeper(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

fn parse_string(pair: Pair<'_, Rule>) -> String {
    pair.as_str()
        .trim_start_matches('\"')
        .trim_end_matches('\"')
        .to_string()
}

#[derive(Debug)]
pub enum AstNode {
    File(Vec<AstNode>),
    Action {
        line: (usize, usize),
        name: String,
        value: Option<ActionValue>,
    },
    Hex(Vec<AstNode>),
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

#[derive(Debug)]
pub enum OpName {
    Store,
    Copy,
    Push,
    Embed,
    SmartEmbed,
    ConsiderEmbed,
    IntroEmbed,
}

#[derive(Debug)]
pub enum OpValue {
    Iota(Iota),
    Var(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActionValue {
    Iota(Iota),
    Bookkeeper(String),
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
        )
        .unwrap();
    }
}
