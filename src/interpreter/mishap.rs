use crate::iota::GarbageIota::Garbage;
use crate::iota::{PatternIota, VectorIota};
use crate::{interpreter::state::Stack, iota::Iota};

#[derive(Debug)]
pub enum Mishap {
    NotEnoughIotas(usize, usize),
    IncorrectIota(usize, String, Iota),
    MathematicalError(),
    HastyRetrospection,
    InvalidPattern,
    ExpectedPattern(Iota),
    ExpectedValue(String, String),
    InvalidValue(String, String),
    OpCannotBeConsidered,
    OpNotEnoughArgs(i32),
    OpExpectedVar(Iota),
    OpExpectedIota,
    VariableNotAssigned(String),
    NoIotaAtIndex(usize),
    NoAkashicRecord(VectorIota),
    HoldingIncorrectItem,
    EvalMishap(Vec<Iota>, usize, Box<Mishap>)
}

impl Mishap {
    pub fn apply_to_stack(self, stack: Stack) -> Stack {
        match self {
            Mishap::NotEnoughIotas(_, num) => {
                let mut new_stack = stack;
                new_stack.append(&mut vec![Iota::Garbage(Garbage); num]);
                new_stack
            }
            Mishap::IncorrectIota(index, _, _) => {
                let mut new_stack = stack;
                new_stack[index] = Iota::Garbage(Garbage);
                new_stack
            }
            Mishap::MathematicalError() => todo!(),
            Mishap::HastyRetrospection => {
                let retro_sig: &str = "eee";
                let mut new_stack = stack;
                new_stack.push(Iota::Pattern(PatternIota::from_sig(retro_sig, None)));
                new_stack
            }
            Mishap::InvalidPattern => {
                let mut new_stack = stack;
                new_stack.push(Iota::Garbage(Garbage));
                new_stack
            }
            Mishap::ExpectedPattern(_) => todo!(),

            Mishap::OpCannotBeConsidered => todo!(),
            Mishap::OpNotEnoughArgs(_) => todo!(),
            Mishap::OpExpectedVar(_) => todo!(),
            Mishap::VariableNotAssigned(_) => todo!(),
            Mishap::OpExpectedIota => todo!(),
            Mishap::NoIotaAtIndex(_) => todo!(),
            Mishap::NoAkashicRecord(_) => todo!(),
            Mishap::HoldingIncorrectItem => todo!(),
            Mishap::ExpectedValue(_, _) => todo!(),
            Mishap::InvalidValue(_, _) => todo!(),
            Mishap::EvalMishap(_, _, _) => todo!(),
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            Mishap::NotEnoughIotas(arg_count, stack_height) => format!("Expected {arg_count} or more arguments but the stack was only {stack_height} tall"),
            Mishap::IncorrectIota(index, expected, recieved) => format!(
                "expected {} at index {index} of the stack, but got {}",
                expected, recieved.display()
            ),
            Mishap::MathematicalError() => todo!(),
            Mishap::HastyRetrospection => "Expected preceding Introspection".to_string(),
            Mishap::InvalidPattern => "That pattern isn't associated with any action".to_string(),
            Mishap::ExpectedPattern(iota) => format!("Expected Pattern but got {}", iota.display()),
            Mishap::OpCannotBeConsidered => "Ops cannot be considered".to_string(),
            Mishap::OpNotEnoughArgs(arg_count) => format!("Expected {arg_count} arguments"),
            Mishap::OpExpectedVar(iota) => format!("Expected argument to be a variable but got iota {}", iota.display()),
            Mishap::OpExpectedIota => "Expected argument to be an iota".to_string(),
            Mishap::VariableNotAssigned(_) => "Variable never assigned".to_string(),
            Mishap::NoIotaAtIndex(_) => "No iota found at pointed location".to_string(),
            Mishap::NoAkashicRecord(location) => format!("No akashic record found at {location}"),
            Mishap::HoldingIncorrectItem => "Entity is not holding the right item".to_string(),
            Mishap::ExpectedValue(_, expected) => format!("Expected {expected} value to be supplied but got Nothing"),
            Mishap::InvalidValue(expected, recieved) =>  format!("Expected {expected} value to be supplied but got {recieved}"),
            Mishap::EvalMishap(_, _, mishap) => mishap.error_message(),
        }
    }

    pub fn error_hint(&self) -> Option<String> {
        match self {
            Mishap::NotEnoughIotas(arg_count, stack_height) => None,
            Mishap::IncorrectIota(index, expected, recieved) => None,
            Mishap::MathematicalError() => None,
            Mishap::HastyRetrospection => None,
            Mishap::InvalidPattern => None,
            Mishap::ExpectedPattern(iota) => None,
            Mishap::OpCannotBeConsidered => None,
            Mishap::OpNotEnoughArgs(arg_count) => Some("Provide arguments inside the parentheses: Op(arg)".to_string()),
            Mishap::OpExpectedVar(iota) => Some("Use a variable as the argument: Op($var)".to_string()),
            Mishap::OpExpectedIota => Some("Use an Iota as the argument: Op(1), Op([1, 1, 1]), ect.".to_string()),
            Mishap::VariableNotAssigned(varname) => Some(format!("Assign the variable using Store({varname}) or Copy({varname})")),
            Mishap::NoIotaAtIndex(_) => Some("This is typically caused by the Ravenmind being overwritten via Huginn's Gambit".to_string()),
            Mishap::NoAkashicRecord(location) => Some("Define an akashic record in a 'config.toml' file".to_string()),
            Mishap::HoldingIncorrectItem => Some("Define held items in a 'config.toml' file".to_string()),
            Mishap::ExpectedValue(action_name, expected) => Some(format!("Set a value for this action: {action_name}: {expected}")),
            Mishap::InvalidValue(expected, recieved) => None,
            Mishap::EvalMishap(_, _, mishap) => mishap.error_hint(),
        }
    }
}