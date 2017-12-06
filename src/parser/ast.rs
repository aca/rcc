use super::token::Token;
use super::token::UnOp;
use std::vec::IntoIter;
use std::iter::Peekable;


#[derive(Debug)]
pub struct Program {
    pub func: Vec<Function>
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub statement: Statement
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression)
}

#[derive(Debug)]
pub enum Expression {
    Int(i32),
    UnOp(UnOp, Box<Expression>)
}

pub fn parse_program(tokens: &mut Peekable<IntoIter<Token>>) -> Program {
    let mut functions = Vec::new();
    while let Some(_) = tokens.peek() {
        functions.push(parse_function(tokens));
    }

    if tokens.next().is_some() { panic!("Should be at the end") };
    Program { func: functions }
}

fn next_token(tokens: &mut Peekable<IntoIter<Token>>) -> Token {
    match tokens.next() {
        Some(token) => Ok(token),
        _ => Err("Token not found")
    }.expect("failed to parse")
}

fn parse_function(tokens: &mut Peekable<IntoIter<Token>>) -> Function {
    match next_token(tokens) {
        Token::Keyword(ref word) if word == "int" => Ok(()),
        other => Err(format!("Expected SemiColon, found {:?}", other))
    }.and_then(|_| {
        match next_token(tokens) {
            Token::Identifier(n) => Ok(n),
            other => Err(format!("Expected name, found {:?}", other))
        }
    }).and_then(|name| {
        match next_token(tokens) {
            Token::OpenParen => Ok(name),
            other => Err(format!("Expected OpenParen, found {:?}", other))
        }
    }).and_then(|name| {
        match next_token(tokens) {
            Token::CloseParen => Ok(name),
            other => Err(format!("Expected CloseParen, found {:?}", other))
        }
    }).and_then(|name| {
        match next_token(tokens) {
            Token::OpenBrace => Ok(name),
            other => Err(format!("Expected OpenBrace, found {:?}", other))
        }
    }).and_then(|name| {
        let statement = parse_statement(tokens);
        match next_token(tokens) {
            Token::CloseBrace => Ok(Function { name, statement }),
            other => Err(format!("Expected CloseBrace, found {:?}", other))
        }
    }).expect("failed to parse")
}

fn parse_statement(tokens: &mut Peekable<IntoIter<Token>>) -> Statement {
    match tokens.next() {
        Some(Token::Keyword(ref word)) if word == "return" => Ok(true),
        other => Err(format!("Expected return, found {:?}", other))
    }.expect("failed to parse");

    let exp = parse_expression(tokens);
    let state = Statement::Return(exp);

    let res = match tokens.next() {
        Some(Token::SemiColon) => Ok(state),
        other => Err(format!("Expected SemiColon, found {:?}", other))
    }.expect("failed to parse");

    res
}

fn parse_expression(tokens: &mut Peekable<IntoIter<Token>>) -> Expression {
    let lit = match tokens.next() {
        Some(Token::Literal(x)) => Ok(Expression::Int(x)),
        Some(Token::Operation(op)) => Ok(Expression::UnOp(op, Box::new(parse_expression(tokens)))),
        other => Err(format!("Expected int literal, found {:?}", other))
    };
    lit.expect("Failed to parse")
}