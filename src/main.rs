use anyhow::{Error, Result};

enum Operation {
    Add,
    Mult,
}

// 2 pass
// 1. tokenize (vector of tokens, string->vec of str)
// 2. parse using shunting yard algorithm (format into Node enum)

// 1 + 2

enum Node {
    Int(u32),
    Expr {
        op: Operation,
        rhs: Box<Node>,
        lhs: Box<Node>,
    },
}

//// tokenize input str into a vec of str
/*fn tokenize<'a>(
    idx: usize,
    input: &'a str,
    output: &'a mut Vec<&'a str>,
    int_str: &'a str,
    curr_token: &'a char,
) -> &'a mut Vec<&'a str> {
    /*let mut tokens: Vec<&str> = Vec::new();
    for token in input.split_whitespace() {
        //// TODO: recursive
        tokens.push(token);
    }
    return tokens;*/

    if input.is_empty() {
        return output;
    }

    // let curr_token = input.chars().nth(idx).unwrap();
    if *curr_token == '(' {
        output.push("(");
        return tokenize(
            idx + 1,
            &input[idx..input.len()],
            output,
            int_str,
            &input.chars().nth(idx + 1).unwrap(),
        );
    } else if *curr_token == ')' {
        output.push(")");
        return tokenize(
            idx + 1,
            &input[idx..input.len()],
            output,
            int_str,
            &input.chars().nth(idx + 1).unwrap(),
        );
    } else if curr_token.is_whitespace() {
        output.push(int_str);
        return tokenize(
            idx + 1,
            &input[idx..input.len()],
            output,
            "",
            &input.chars().nth(idx + 1).unwrap(),
        );
    } else {
        return tokenize(
            idx + 1,
            &input[idx..input.len()],
            output,
            (int_str.to_string() + curr_token.to_string().as_str()).as_str(),
            &input.chars().nth(idx + 1).unwrap(),
        );
    }
}*/

/*fn tokenize(idx: usize, input: &str, mut output: Vec<String>) -> Vec<String> {
    if input.is_empty() {
        return output;
    }

    let curr_token = input.chars().nth(idx).unwrap();
    if curr_token == '(' {
        output.push("(".to_string());
        return tokenize(idx + 1, &input[idx..input.len()], output);
    } else if curr_token == ')' {
        output.push(")".to_string());
        return tokenize(idx + 1, &input[idx..input.len()], output);
    } else if curr_token == ' ' {
        let mut combined = "".to_string();
        while *output.last().unwrap() != "(" && *output.last().unwrap() != ")" {
            combined = combined.to_string().to_owned() + output.pop().unwrap().as_str();
        }
        println!("NUM: {}", combined);
        output.push(combined);
        return tokenize(idx + 1, &input[idx..input.len()], output);
    } else if curr_token == '+' {
        output.push("+".to_string());
        return tokenize(idx + 1, &input[idx..input.len()], output);
    } else if curr_token == '*' {
        output.push("*".to_string());
        return tokenize(idx + 1, &input[idx..input.len()], output);
    } else {
        output.push(curr_token.to_string());
        return tokenize(idx + 1, &input[idx..input.len()], output);
    }
}*/

fn tokenize(input: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut combined = String::new();

    for ch in input.chars() {
        match ch {
            '(' | ')' | '+' => {
                if !combined.is_empty() {
                    output.push(combined.clone());
                    combined.clear();
                }
                output.push(ch.to_string());
            }
            '0'..='9' => combined.push(ch),
            _ => {}
        }
    }

    if !combined.is_empty() {
        output.push(combined);
    }

    return output;
}

//// parse using shunting yard algorithm
fn parse(tokens: Vec<String>) -> Vec<String> {
    let mut operators: Vec<String> = Vec::new();
    let mut output: Vec<String> = Vec::new();

    for token in tokens {
        if token == String::from("+") {
            while *operators.last().unwrap() == String::from("+") {
                output.push(operators.pop().unwrap());
            }
            operators.push(token);
        } else if token == String::from("*") {
            while *operators.last().unwrap() == String::from("+")
                || *operators.last().unwrap() == String::from("*")
            {
                output.push(operators.pop().unwrap());
            }
            operators.push(token);
        } else if token == String::from("(") {
            operators.push(token);
        } else if token == String::from(")") {
            while *operators.last().unwrap() != String::from("(") {
                output.push(operators.pop().unwrap());
            }
            operators.pop();
        } else {
            // int
            output.push(token);
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    return output;
}

//// post order traversal into BST
/*fn createAST(post: Vec<&str>, n: usize) -> Node {
    let mut root = Node::Int(post[n - 1].parse().unwrap());

    let mut stack = Vec::new();
    stack.push(root);
    let mut idx = n - 2;
    while idx >= 0 {
        let curr_token = Node::Int(post[idx].parse().unwrap());

        let mut tmp = None;
        while stack.len() > 0 {
            if let Node::Int(val) = stack.last().unwrap() {
                let stack_val: u32 = post[idx].parse().unwrap();
                if stack_val >= *val {
                    break;
                }
            }

            tmp = stack.last();
        }
    }

    return root;
}*/

fn createAST(post: Vec<String>) -> Node {
    let mut nodes: Vec<Node> = Vec::new();

    for token in post.iter() {
        match token.to_string().as_str() {
            "+" => {
                let expr = Node::Expr {
                    op: (Operation::Add),
                    rhs: (Box::new(nodes.pop().unwrap())),
                    lhs: (Box::new(nodes.pop().unwrap())),
                };
                nodes.push(expr);
            }
            "*" => {
                let expr = Node::Expr {
                    op: (Operation::Mult),
                    rhs: (Box::new(nodes.pop().unwrap())),
                    lhs: (Box::new(nodes.pop().unwrap())),
                };
                nodes.push(expr);
            }
            _ => {
                let val = Node::Int(token.parse().unwrap());
                nodes.push(val);
            }
        }
    }

    return nodes.pop().unwrap();
}

fn main() {
    let expr: &str = "(13 + 2)";

    let tokens = tokenize(&expr);
    let parsed = parse(tokens);
    let ast = createAST(parsed);
}
