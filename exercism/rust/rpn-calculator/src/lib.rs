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

    for i in inputs {
        if let CalculatorInput::Value(n) = i {
            stack.push(*n);
            continue;
        }

        if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
            match i {
                CalculatorInput::Add => stack.push(a + b),
                CalculatorInput::Subtract => stack.push(a - b),
                CalculatorInput::Multiply => stack.push(a * b),
                CalculatorInput::Divide => stack.push(a / b),
                _ => {}
            }
        } else {
            return None;
        }
    }

    match stack.len() {
        1 => stack.pop(),
        _ => None,
    }
}
