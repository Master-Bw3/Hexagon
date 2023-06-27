use crate::iota::{EntityIota, GarbageIota, Iota, NullIota};
use nalgebra::matrix;
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct HexParser;
pub fn parse(source: &str) -> Result<AstNode, Error<Rule>> {
    let mut ast = vec![];

    let pairs = HexParser::parse(Rule::File, source)?;
    for pair in pairs {
        if let Some(node) = construct_ast_node(pair) {
            ast.push(node);
        }
    }

    Ok(AstNode::Hex(ast))
}

fn construct_ast_node(pair: Pair<'_, Rule>) -> Option<AstNode> {
    match pair.as_rule() {
        Rule::Action => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next();

            Some(parse_action(left, right))
        }
        Rule::Op => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let arg = pair.next();

            Some(parse_op(name, arg))
        }
        Rule::IntroRetro => Some(parse_intro_retro(pair)),
        Rule::Var => Some(parse_var(pair)),
        Rule::Embed => Some(parse_embed(pair)),
        Rule::IfBlock => Some(parse_if_block(pair)),
        Rule::Term => Some(AstNode::Hex(
            pair.into_inner().filter_map(construct_ast_node).collect(),
        )),
        _ => None,
    }
}

fn parse_op(name: Pair<'_, Rule>, arg: Option<Pair<'_, Rule>>) -> AstNode {
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
                Rule::Iota => OpValue::Iota(parse_iota(pair)),
                Rule::Var => OpValue::Var(pair.as_str().to_string()),
                _ => unreachable!(),
            })
        },
    }
}

fn parse_action(left: Pair<'_, Rule>, right: Option<Pair<'_, Rule>>) -> AstNode {
    AstNode::Action {
        name: { left.as_str().to_string() },
        value: {
            right.map(|pair| match pair.as_rule() {
                Rule::Iota => ActionValue::Iota(parse_iota(pair)),
                Rule::BookkeeperValue => ActionValue::Bookkeeper(parse_bookkeeper(pair)),

                _ => unreachable!(),
            })
        },
    }
}

fn parse_intro_retro(pair: Pair<'_, Rule>) -> AstNode {
    AstNode::Action {
        name: {
            match pair.as_str() {
                "{" => "Introspection".to_string(),
                "}" => "Retrospection".to_string(),
                _ => unreachable!(),
            }
        },
        value: None,
    }
}

fn parse_var(pair: Pair<'_, Rule>) -> AstNode {
    AstNode::Op {
        name: OpName::Push,
        arg: { Some(OpValue::Var(pair.as_str().to_string())) },
    }
}

fn parse_embed(pair: Pair<'_, Rule>) -> AstNode {
    let inner_pair = pair.into_inner().next().unwrap();
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
            .map(|iota| OpValue::Iota(parse_iota(iota)))),
    }
}

fn parse_if_block(pair: Pair<'_, Rule>) -> AstNode {
    fn parse_inner(mut inner: Pairs<'_, Rule>) -> AstNode {
        AstNode::IfBlock {
            condition: {
                let mut condition = inner.next().unwrap().into_inner();
                Box::new(construct_ast_node(condition.next().unwrap()).unwrap())
            },
            succeed: {
                let mut succeed = inner.next().unwrap().into_inner();
                Box::new(construct_ast_node(succeed.next().unwrap()).unwrap())
            },
            fail: {
                inner.clone().next().map(|branch| match branch.as_rule() {
                    Rule::Else => {
                        Box::new(construct_ast_node(branch.into_inner().next().unwrap()).unwrap())
                    }
                    Rule::ElseIf => Box::new(parse_inner(inner)),
                    _ => unreachable!(),
                })
            },
        }
    }
    parse_inner(pair.into_inner())
}

fn parse_iota(pair: Pair<'_, Rule>) -> Iota {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::Number => Iota::Number(inner_pair.as_str().parse().unwrap()),
        Rule::Pattern => Iota::Pattern(inner_pair.as_str().to_string()),
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
            let entity_type = inner.next().unwrap().as_str().to_string();

            Iota::Entity(EntityIota { name, entity_type })
        }
        Rule::List => {
            let inner = inner_pair.into_inner();
            Iota::List(inner.map(parse_iota).collect())
        }
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
    Action {
        name: String,
        value: Option<ActionValue>,
    },
    Hex(Vec<AstNode>),
    Op {
        name: OpName,
        arg: Option<OpValue>,
    },
    IfBlock {
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

#[derive(Debug)]
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
        )
        .unwrap();
    }
}
