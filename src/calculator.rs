// Thank you ChatGPT but also why must it take you so long to fix the bug I had :(

pub mod calculator {

    #[derive(Debug, Clone, Copy)]
    enum MathValue {
        Variable(char),
        Constant(i32),
    }

    #[derive(Debug, Clone, Copy)]
    enum MathOperator {
        Add,
        Subtract,
        Multiply,
        Divide,
        Exponentiate,
    }

    #[derive(Debug, Clone)]
    enum EquationElement {
        Value(MathValue),
        Operator(MathOperator),
        Group(Vec<EquationElement>),
    }

    pub fn calculate_eq(equation: &String, ending_x: usize) -> Vec<i32> {

        let eq = build_eq(&equation);


        let mut y_range: Vec<i32> = vec![];
        for x_integer in 0..ending_x {
            let value = evaluate_eq(&eq, x_integer as i32);
            y_range.push(value);
        }
        
        y_range
    }

    fn evaluate_eq(elements: &[EquationElement], x_value: i32) -> i32 {
        let mut output = Vec::new();
        let mut operators = Vec::new();
    
        for element in elements {
            match element {
                EquationElement::Value(value) => {
                    match value {
                        MathValue::Variable('x') => output.push(x_value),
                        MathValue::Constant(c) => output.push(*c),
                        _ => todo!(), // Handle other cases for MathValue::Variable
                    }
                }
                EquationElement::Operator(operator) => {
                    while let Some(&top_op) = operators.last() {
                        if should_apply_before_top_operator(*operator, top_op) {
                            apply_top_operator(&mut output, operators.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    operators.push(*operator);
                }
                EquationElement::Group(group_elements) => {
                    let group_result = evaluate_eq(group_elements, x_value);
                    output.push(group_result);
                }
            }
        }
    
        while let Some(operator) = operators.pop() {
            apply_top_operator(&mut output, operator);
        }
    
        if let Some(result) = output.pop() {
            result
        } else {
            panic!("Invalid expression: empty output stack");
        }
    }

    fn should_apply_before_top_operator(new_op: MathOperator, top_op: MathOperator) -> bool {
        // Define the order of operations here
        match (new_op, top_op) {
            (MathOperator::Exponentiate, _) => false,
            (_, MathOperator::Exponentiate) => true,
            (MathOperator::Multiply, MathOperator::Add)
            | (MathOperator::Multiply, MathOperator::Subtract)
            | (MathOperator::Divide, MathOperator::Add)
            | (MathOperator::Divide, MathOperator::Subtract) => false,
            _ => true,
        }
    }

    fn apply_top_operator(output: &mut Vec<i32>, operator: MathOperator) {
        if let (Some(b), Some(a)) = (output.pop(), output.pop()) {
            let result = match operator {
                MathOperator::Add => a + b,
                MathOperator::Subtract => a - b,
                MathOperator::Multiply => a * b,
                MathOperator::Divide => a / b,
                MathOperator::Exponentiate => a.pow(b as u32),
            };
            output.push(result);
        } else {
            panic!("Invalid expression: not enough values for operator");
        }
    }

    fn parse_element(chars: &mut std::iter::Peekable<std::str::Chars>) -> EquationElement {
        if let Some(&c) = chars.peek() {
            if c == '(' {
                EquationElement::Group(parse_group(chars))
            } else if c.is_digit(10) {
                let value = parse_number(chars);
                EquationElement::Value(MathValue::Constant(value))
            } else if c.is_alphabetic() {
                let variable = parse_variable(chars);
                EquationElement::Value(MathValue::Variable(variable))
            } else if "+-*/^".contains(c) {
                let op = parse_operator(c);
                chars.next();
                EquationElement::Operator(op)
            } else if c.is_whitespace() {
                chars.next();
                parse_element(chars) // Skip whitespaces and continue parsing
            } else {
                // Handle unexpected characters
                panic!("Unexpected character: {}", c);
            }
        } else {
            // Handle end of input
            panic!("Unexpected end of input");
        }
    }
    
    fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> i32 {
        let mut value = 0;

        while let Some(&next_char) = chars.peek() {
            if let Some(digit) = next_char.to_digit(10) {
                value = value * 10 + digit as i32;
                chars.next();
            } else {
                break;
            }
        }

        value
    }
    
    fn parse_variable(chars: &mut std::iter::Peekable<std::str::Chars>) -> char {
        if let Some(&_next_char) = chars.peek() {
            chars.next().unwrap()
        } else {
            unreachable!(); // Shouldn't be called without checking for characters
        }
    }
    
    fn parse_operator(c: char) -> MathOperator {
        match c {
            '+' => MathOperator::Add,
            '-' => MathOperator::Subtract,
            '*' => MathOperator::Multiply,
            '/' => MathOperator::Divide,
            '^' => MathOperator::Exponentiate,
            _ => unreachable!(),
        }
    }
    
    fn parse_group(chars: &mut std::iter::Peekable<std::str::Chars>) -> Vec<EquationElement> {
        let mut group_elements = Vec::new();
    
        // Skip the opening parenthesis
        chars.next();
    
        while let Some(&c) = chars.peek() {
            if c == ')' {
                // Skip the closing parenthesis
                chars.next();
                break;
            } else {
                let element = parse_element(chars);
                group_elements.push(element);
            }
        }
    
        group_elements
    }
    
    fn build_eq(input: &String) -> Vec<EquationElement> {
        let mut elements = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            let element = parse_element(&mut chars);
            elements.push(element);

            // Check if the current character is a number, and if the next character is a variable
            if c.is_digit(10) {
                if let Some(&next_char) = chars.peek() {
                    if next_char.is_alphabetic() {
                        // Insert a multiplication operator after the number
                        elements.push(EquationElement::Operator(MathOperator::Multiply));
                    }
                }
            }
        }

        elements
    }

}