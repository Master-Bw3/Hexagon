use pest::{error::Error, iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct HexParser;

pub fn run() {
    let parse_result = parse(
        "
        Bookkeeper's Gambit: [{\"bob\", minecraft:cow}, 1] \n
        Gemini Decomposition \n
        Pace Purification \n
        Augur's Purification \n
        Jester's Gambit \n
        Nullary Reflection \n
        Augur's Exaltation \n
        Numerical Reflection: (1, 2, 3)
        ",
    );

    println!("{:?}", parse_result.unwrap())
}

fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = HexParser::parse(Rule::File, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::Action => {
                let mut pair = pair.into_inner();
                let left = pair.next().unwrap();
                let right = pair.next();

                ast.push(parse_action(left, right));
            }
            Rule::Op => {
                let mut pair = pair.into_inner();
                let left = pair.next().unwrap();
                let right = pair.next();

                ast.push(parse_action(left, right));
            }
            _ => {}
        }
    }

    Ok(ast)
}

fn parse_op(left: Pair<'_, Rule>, right: Option<Pair<'_, Rule>>) -> AstNode {
    AstNode::Op {
        name: {
            match left.as_str() {
                "Store" => OpName::Store,
                "Copy" => OpName::Copy,
                _ => unreachable!()
            }
        },
        value: {
            right.map(|pair| match pair.as_rule() {
                Rule::Iota => OpValue::Iota(parse_iota(pair.into_inner().next().unwrap())),
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
                Rule::Iota => ActionValue::Iota(parse_iota(pair.into_inner().next().unwrap())),
                Rule::BookkeeperValue => ActionValue::Bookkeeper(parse_bookkeeper(pair)),

                _ => unreachable!(),
            })
        },
    }
}

fn parse_iota(pair: Pair<'_, Rule>) -> Iota {
    match dbg!(pair.as_rule()) {
        Rule::Number => Iota::Number(pair.as_str().parse().unwrap()),
        Rule::Pattern => Iota::Pattern(pair.as_str().to_string()),
        Rule::Vector => {
            let mut inner = pair.into_inner();
            Iota::Vector(
                inner.next().unwrap().as_str().parse().unwrap(),
                inner.next().unwrap().as_str().parse().unwrap(),
                inner.next().unwrap().as_str().parse().unwrap(),
            )
        }
        Rule::Bool => match pair.as_str() {
            "True" => Iota::Bool(true),
            "False" => Iota::Bool(false),
            _ => unreachable!(),
        },
        Rule::Influence => match pair.as_str() {
            "Garbage" => Iota::Garbage,
            "Null" => Iota::Null,
            _ => unreachable!(),
        },
        Rule::Entity => {
            let mut inner = pair.into_inner();
            let name = parse_string(inner.next().unwrap());
            let entity_type = inner.next().unwrap().as_str().to_string();

            Iota::Entity { name, entity_type }
        }
        Rule::List => {
            let inner = pair.into_inner();
            Iota::List(
                inner
                    .map(|inner_pair| parse_iota(inner_pair.into_inner().next().unwrap()))
                    .collect(),
            )
        }
        _ => unreachable!(),
    }
}

fn parse_bookkeeper(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

fn parse_string(pair: Pair<'_, Rule>) -> String {
    pair.as_str()
        .trim_start_matches("\"")
        .trim_end_matches("\"")
        .to_string()
}

fn build_ast(program: Pair<'_, Rule>) {
    for token in program.into_inner() {
        //valid tokens inside Hex token: Keyword | Embed | Op | Var | Term | Action | IntroRetro
        match token.as_rule() {
            Rule::Keyword => todo!(),
            Rule::Embed => todo!(),
            Rule::Op => todo!(),
            Rule::Var => todo!(),
            Rule::Term => todo!(),
            Rule::Action => todo!(),
            Rule::IntroRetro => todo!(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum AstNode {
    Action {
        name: String,
        value: Option<ActionValue>,
    },
    Hex(Box<AstNode>),
    Op {
        name: OpName,
        value: Option<OpValue>,
    },
}

#[derive(Debug)]
enum OpName {
    Store,
    Copy,
}

#[derive(Debug)]
enum OpValue {
    Iota(Iota),
    Var(String),
}

#[derive(Debug)]
enum ActionValue {
    Iota(Iota),
    Bookkeeper(String),
}

#[derive(Debug)]

enum Iota {
    Number(f32),
    Vector(f32, f32, f32),
    Pattern(String),
    Bool(bool),
    Garbage,
    Null,
    Entity { name: String, entity_type: String },
    List(std::vec::Vec<Iota>),
}
