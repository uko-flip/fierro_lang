use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Mutex;
use std::io;

use lazy_static::lazy_static;

#[derive(Debug)]
pub enum Expression {
    Number(i32),
    BinaryOperation(Box<Expression>, Operand, Box<Expression>),
    UnaryOperation(Box<Expression>, Operand),
    GetVariable(String),
    InsertVariable(String, Box<Expression>),

    In,
    For(Box<Expression>, Box<Expression>, Box<Expression>, Box<Expression>)
}

#[derive(Debug)]
pub enum Operand {
    Plus,
    Minus,
    Mul,
    Div,

    Equal,
    Bigger,
    Lower,

    IfCondition,
    PrintCondition,
    WhileCondition,

    Block
}

lazy_static! {
    static ref SCOPE: Mutex<HashMap<String, i32>> = {
        let mut m = HashMap::new();
        m.insert("her".to_string(), 228);
        Mutex::new(m)
    };
}

impl Expression {
    pub fn evaluate(&self) -> i32 {
        match self {
            Expression::Number(n) => *n,
            Expression::BinaryOperation(l, op, r) => match op {
                Operand::Plus => l.evaluate() + r.evaluate(),
                Operand::Minus => l.evaluate() - r.evaluate(),
                Operand::Mul => l.evaluate() * r.evaluate(),
                Operand::Div => l.evaluate() / r.evaluate(),

                Operand::Equal => match l.evaluate() == r.evaluate() {
                    true => 1,
                    false => 0
                },
                Operand::Bigger => match l.evaluate() > r.evaluate() {
                    true => 1,
                    false => 0
                },
                Operand::Lower => match l.evaluate() < r.evaluate() {
                    true => 1,
                    false => 0
                },
                Operand::IfCondition => match l.evaluate() != 0 {
                    true => r.evaluate(),
                    false => -1
                },
                Operand::WhileCondition => {
                    while l.evaluate() != 0 {
                        r.evaluate();
                    }; 0
                },
                Operand::Block => { l.evaluate(); r.evaluate() },
                _ => 48
            },
            Expression::UnaryOperation(l, op) => match op {
                Operand::PrintCondition => { println!("{}", l.evaluate()); 0 },
                _ => -56

            },
            Expression::GetVariable(s) => { *SCOPE.lock().unwrap().get(s).unwrap() },
            Expression::InsertVariable(s, e) => {
                let value = e.evaluate();
                SCOPE.lock().unwrap().insert(s.to_string(), value);
                0 },
            Expression::In => {
                let mut input_text = String::new();
                io::stdin().read_line(&mut input_text).expect("failed to read line");
                input_text.trim().parse::<i32>().unwrap()
            },
            Expression::For(a, b, c, d) => {
                a.evaluate();
                while b.evaluate() != 0 {
                    d.evaluate();
                    c.evaluate();
                };
                0
            }
        }
    }
}