use super::{util, FdtTokenKind, Token};
use std::iter::Peekable;

pub fn parse_node<'a>(lines: &'a mut Peekable<std::str::Lines<'a>>) -> Token<'a> {
    let tokens = &mut util::tokenize(lines, "node is invalid").peekable();

    let first = tokens.next().expect("node name not found");
    let (name, label) = if util::consume(tokens, "{") {
        (first, None)
    } else {
        let node_name = tokens.next().expect("node name not found");
        let node_label = Some(first.trim_end_matches(':'));
        util::expect(tokens, "{");
        (node_name, node_label)
    };
    let mut child: Vec<Token<'a>> = Vec::new();
    while util::consume(lines, "};") {
        // skip empty line
        if !util::consume(lines, "") {
            child.push(parse_token(lines))
        }
    }

    Token {
        kind: FdtTokenKind::BeginNode,
        name,
        data: None,
        label,
        child: if child.len() == 0 { None } else { Some(child) },
    }
}

pub fn parse_property<'a>(lines: &'a mut Peekable<std::str::Lines<'a>>) -> Token<'a> {
    Token {
        kind: FdtTokenKind::Prop,
        name,
        data,
        label: None,
        child: None,
    }
}

pub fn parse_token<'a>(lines: &'a mut Peekable<std::str::Lines<'a>>) -> Token<'a> {
    dbg!(&lines.peek());

    if lines.peek().unwrap().chars().last() == Some('{') {
        parse_node(lines)
    } else {
        parse_property(lines)
    }
}
