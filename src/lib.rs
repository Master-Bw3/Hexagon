use pest::{error::Error, iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct HexParser;

pub fn run() {
    let parse_result = parse(
        "
        Bookkeeper's Gambit: --v \n
        Gemini Decomposition \n
        Pace Purification \n
        Augur's Purification \n
        Jester's Gambit \n
        Nullary Reflection \n
        Augur's Exaltation \n
        Numerical Reflection: 12
        ",
    );

    println!("{:?}", parse_result.unwrap())
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
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
            _ => {}
        }
    }

    Ok(ast)
}

pub fn parse_action(left: Pair<'_, Rule>, right: Option<Pair<'_, Rule>>) -> AstNode {
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

pub fn parse_iota(pair: Pair<'_, Rule>) -> Iota {
    match pair.as_rule() {
        Rule::Number => Iota::Number(pair.as_str().parse().unwrap()),
        _ => unreachable!(),
    }
}

pub fn parse_bookkeeper(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

pub fn build_ast(program: Pair<'_, Rule>) {
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
pub enum AstNode {
    Action {
        name: String,
        value: Option<ActionValue>,
    },
    Hex(Box<AstNode>),
    N,
}

#[derive(Debug)]

pub enum ActionValue {
    Iota(Iota),
    Bookkeeper(String),
}

#[derive(Debug)]

pub enum Iota {
    Number(f32),
    Vector(f32, f32, f32),
}
