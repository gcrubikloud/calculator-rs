use std::io;
use std::prelude::v1::Vec;

use indextree::Arena;
use indextree::NodeId;
use regex::Regex;

fn split_tokens(expr: &str) -> Vec<String> {
    println!("split_tokens()");
    println!("expr = {}", expr);
    let re = Regex::new(r"([\d\\.]+|[\\+-/*()])").unwrap();
    let mut tokens = Vec::new();
    for capture in re.captures_iter(expr) {
        tokens.push(capture[1].to_string());
        println!("token = '{}'", &capture[1])
    }
    return tokens;
}

#[allow(dead_code)]
fn get_expr_from_cli() -> Box<str> {
    println!("get_expr_from_cli()");
    let mut expr = String::new();
    io::stdin().read_line(&mut expr)
        .expect("Failed to read line");
    return expr.into_boxed_str();
}

fn get_mock_expr() -> Box<str> {
    println!("get_mock_expr()");
    let expr = "2+3-4*10/(1.2*32)".to_string();
    return expr.into_boxed_str();
}

fn get_expr() -> Box<str> {
    println!("get_expr()");
    return get_mock_expr();
}

fn is_operator(token: &String) -> bool {
    match token.as_str() {
        "+" => true,
        "-" => true,
        "*" => true,
        "/" => true,
        _ => false
    }
}

fn is_bracket(token: &String) -> bool {
    match token.as_str() {
        "(" => true,
        ")" => true,
        _ => false
    }
}

//https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/
//https://github.com/saschagrunert/indextree
//https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html
#[allow(dead_code)]
fn create_expr_tree() {
    println!("create_expr_tree()");
// Create a new arena
    let arena = &mut Arena::new();

// Add some new nodes to the arena
    let lhs = arena.new_node("1");
    let operator = arena.new_node("+");
    let rhs = arena.new_node("2");

    // "+" has previous_child "1" and
    assert!(operator.append(lhs, arena).is_ok());
    assert!(operator.append(rhs, arena).is_ok());

    assert_eq!(lhs.ancestors(arena).into_iter().count(), 2);
    println!("ancestors lhs:");
    for idx in lhs.ancestors(arena).into_iter() {
        println!("{}", idx);
        println!("id: {}\ndata={:?}", idx, arena.get(idx));
    }
    println!("ancestors rhs:");
    for idx in rhs.ancestors(arena).into_iter() {
        println!("id: {}\ndata={:?}", idx, arena.get(idx));
    }

    println!("from operator:");
    let operands: Vec<NodeId> = operator.children(arena).into_iter().collect();
    println!("{}", operands.len());
    for operand in operands {
        println!("operand={:?}", arena.get(operand));
    }
}


#[allow(unused_variables)]
fn run_calculator() {
    println!("run_calculator()");
    let raw_expr = "1+2".to_string().into_boxed_str();
    let tokens: Vec<String> = split_tokens(&raw_expr[..]);
    for token in tokens.iter() {
        println!("'{}' is operator {}", token, is_operator(token));
    }
}


fn main() {
    println!("main()");
    run_calculator();
    create_expr_tree();
}
