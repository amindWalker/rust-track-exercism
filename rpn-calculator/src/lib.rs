use std::ops::Mul;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    use crate::CalculatorInput::*;
    for i in inputs {
        match i {
            Value(num) => stack.push(*num),
            Add => *stack.last_mut()? += stack.pop()?,
            Subtract => *stack.last_mut()? -= stack.pop()?,
            Multiply => *stack.last_mut()? *= stack.pop()?,
            Divide => *stack.last_mut()? /= stack.pop()?,
        }
    }

    match stack.len() {
        1 => stack.pop(),
        _ => None,
    }
}
