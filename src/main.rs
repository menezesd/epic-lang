use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::process;
use std::rc::Rc;

const BASE: u32 = 1_000_000_000;

#[derive(Clone, Debug, Eq, PartialEq)]
struct BigInt {
    sign: i8,
    digits: Vec<u32>,
}

impl BigInt {
    fn zero() -> Self {
        Self {
            sign: 0,
            digits: Vec::new(),
        }
    }

    fn one() -> Self {
        Self {
            sign: 1,
            digits: vec![1],
        }
    }

    fn from_i64(n: i64) -> Self {
        if n == 0 {
            return Self::zero();
        }
        let sign = if n < 0 { -1 } else { 1 };
        let mut x = if n < 0 { -(n as i128) } else { n as i128 };
        let mut digits = Vec::new();
        while x > 0 {
            digits.push((x % BASE as i128) as u32);
            x /= BASE as i128;
        }
        Self { sign, digits }
    }

    fn parse(text: &str) -> Self {
        let mut result = Self::zero();
        for b in text.bytes() {
            result = result.mul_small(10);
            result = result.add_abs_small((b - b'0') as u32);
        }
        result
    }

    fn normalize(&mut self) {
        while self.digits.last() == Some(&0) {
            self.digits.pop();
        }
        if self.digits.is_empty() {
            self.sign = 0;
        }
    }

    fn is_zero(&self) -> bool {
        self.sign == 0
    }

    fn abs_cmp(&self, other: &Self) -> Ordering {
        if self.digits.len() != other.digits.len() {
            return self.digits.len().cmp(&other.digits.len());
        }
        for (a, b) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            if a != b {
                return a.cmp(b);
            }
        }
        Ordering::Equal
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if self.sign != other.sign {
            return self.sign.cmp(&other.sign);
        }
        match self.sign {
            -1 => other.abs_cmp(self),
            0 => Ordering::Equal,
            _ => self.abs_cmp(other),
        }
    }

    fn add_abs_small(mut self, n: u32) -> Self {
        if n == 0 {
            return self;
        }
        if self.sign == 0 {
            return Self {
                sign: 1,
                digits: vec![n],
            };
        }
        let mut carry = n as u64;
        for d in &mut self.digits {
            let sum = *d as u64 + carry;
            *d = (sum % BASE as u64) as u32;
            carry = sum / BASE as u64;
            if carry == 0 {
                break;
            }
        }
        if carry > 0 {
            self.digits.push(carry as u32);
        }
        self
    }

    fn mul_small(&self, n: u32) -> Self {
        if self.is_zero() || n == 0 {
            return Self::zero();
        }
        let mut digits = Vec::with_capacity(self.digits.len() + 1);
        let mut carry = 0u64;
        for d in &self.digits {
            let prod = *d as u64 * n as u64 + carry;
            digits.push((prod % BASE as u64) as u32);
            carry = prod / BASE as u64;
        }
        if carry > 0 {
            digits.push(carry as u32);
        }
        let mut out = Self {
            sign: self.sign,
            digits,
        };
        out.normalize();
        out
    }

    fn add_abs(a: &Self, b: &Self) -> Vec<u32> {
        let n = a.digits.len().max(b.digits.len());
        let mut digits = Vec::with_capacity(n + 1);
        let mut carry = 0u64;
        for i in 0..n {
            let av = *a.digits.get(i).unwrap_or(&0) as u64;
            let bv = *b.digits.get(i).unwrap_or(&0) as u64;
            let sum = av + bv + carry;
            digits.push((sum % BASE as u64) as u32);
            carry = sum / BASE as u64;
        }
        if carry > 0 {
            digits.push(carry as u32);
        }
        digits
    }

    fn sub_abs(a: &Self, b: &Self) -> Vec<u32> {
        let mut digits = Vec::with_capacity(a.digits.len());
        let mut borrow = 0i64;
        for i in 0..a.digits.len() {
            let av = a.digits[i] as i64 - borrow;
            let bv = *b.digits.get(i).unwrap_or(&0) as i64;
            if av < bv {
                digits.push((av + BASE as i64 - bv) as u32);
                borrow = 1;
            } else {
                digits.push((av - bv) as u32);
                borrow = 0;
            }
        }
        digits
    }

    fn add(&self, other: &Self) -> Self {
        if self.sign == 0 {
            return other.clone();
        }
        if other.sign == 0 {
            return self.clone();
        }
        let (sign, digits) = if self.sign == other.sign {
            (self.sign, Self::add_abs(self, other))
        } else {
            match self.abs_cmp(other) {
                Ordering::Greater => (self.sign, Self::sub_abs(self, other)),
                Ordering::Less => (other.sign, Self::sub_abs(other, self)),
                Ordering::Equal => return Self::zero(),
            }
        };
        let mut out = Self { sign, digits };
        out.normalize();
        out
    }

    fn neg(&self) -> Self {
        let mut out = self.clone();
        out.sign = -out.sign;
        out
    }

    fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        let mut out = vec![0u64; self.digits.len() + other.digits.len()];
        for (i, a) in self.digits.iter().enumerate() {
            let mut carry = 0u64;
            for (j, b) in other.digits.iter().enumerate() {
                let idx = i + j;
                let cur = out[idx] + *a as u64 * *b as u64 + carry;
                out[idx] = cur % BASE as u64;
                carry = cur / BASE as u64;
            }
            if carry > 0 {
                out[i + other.digits.len()] += carry;
            }
        }
        let mut digits: Vec<u32> = out.into_iter().map(|x| x as u32).collect();
        while digits.last() == Some(&0) {
            digits.pop();
        }
        Self {
            sign: self.sign * other.sign,
            digits,
        }
    }

    fn abs(&self) -> Self {
        let mut out = self.clone();
        if out.sign < 0 {
            out.sign = 1;
        }
        out
    }

    fn div_mod_abs(a: &Self, b: &Self) -> Option<(Self, Self)> {
        if b.is_zero() {
            return None;
        }
        if a.abs_cmp(b) == Ordering::Less {
            return Some((Self::zero(), a.clone()));
        }
        let mut q_be = Vec::with_capacity(a.digits.len());
        let mut rem = Self::zero();
        for digit in a.digits.iter().rev() {
            rem.digits.insert(0, *digit);
            rem.sign = if rem.digits.is_empty() { 0 } else { 1 };
            rem.normalize();
            let mut lo = 0u32;
            let mut hi = BASE - 1;
            let mut best = 0u32;
            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                let prod = b.mul_small(mid);
                if prod.abs_cmp(&rem) != Ordering::Greater {
                    best = mid;
                    lo = mid.saturating_add(1);
                } else if mid == 0 {
                    break;
                } else {
                    hi = mid - 1;
                }
            }
            q_be.push(best);
            rem = rem.sub(&b.mul_small(best));
            rem.sign = if rem.digits.is_empty() { 0 } else { 1 };
        }
        q_be.reverse();
        let mut q = Self {
            sign: 1,
            digits: q_be,
        };
        q.normalize();
        Some((q, rem))
    }

    fn div_floor(&self, other: &Self) -> Option<Self> {
        let (mut q, r) = Self::div_mod_abs(&self.abs(), &other.abs())?;
        if q.is_zero() {
            q.sign = 0;
        } else {
            q.sign = self.sign * other.sign;
        }
        if self.sign * other.sign < 0 && !r.is_zero() {
            q = q.sub(&Self::one());
        }
        Some(q)
    }

    fn modulo(&self, other: &Self) -> Option<Self> {
        let q = self.div_floor(other)?;
        Some(self.sub(&q.mul(other)))
    }

    fn to_i64(&self) -> Option<i64> {
        let mut out: i128 = 0;
        for d in self.digits.iter().rev() {
            out = out.checked_mul(BASE as i128)?;
            out = out.checked_add(*d as i128)?;
        }
        out *= self.sign as i128;
        i64::try_from(out).ok()
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.sign == 0 {
            return write!(f, "0");
        }
        if self.sign < 0 {
            write!(f, "-")?;
        }
        let mut it = self.digits.iter().rev();
        if let Some(first) = it.next() {
            write!(f, "{}", first)?;
        }
        for d in it {
            write!(f, "{:09}", d)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Ident(String),
    Number(String),
    Keyword(&'static str),
    Sym(&'static str),
    Eof,
}

fn syntax_error() -> ! {
    println!("syntax error");
    process::exit(1);
}

fn runtime_error() -> ! {
    println!("runtime error");
    process::exit(1);
}

struct Lexer<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            bytes: src.as_bytes(),
            pos: 0,
        }
    }

    fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if matches!(b, b' ' | b'\t' | b'\n' | b'\r') {
                self.pos += 1;
            } else if b == b'/' && self.peek(1) == Some(b'/') {
                self.pos += 2;
                while self.pos < self.bytes.len() && !matches!(self.bytes[self.pos], b'\n' | b'\r')
                {
                    self.pos += 1;
                }
            } else if b.is_ascii_digit() {
                let start = self.pos;
                while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
                    self.pos += 1;
                }
                tokens.push(Token::Number(
                    String::from_utf8(self.bytes[start..self.pos].to_vec()).unwrap(),
                ));
            } else if b.is_ascii_alphabetic() || b == b'_' {
                let start = self.pos;
                while self.pos < self.bytes.len()
                    && (self.bytes[self.pos].is_ascii_alphanumeric()
                        || self.bytes[self.pos] == b'_')
                {
                    self.pos += 1;
                }
                let text = String::from_utf8(self.bytes[start..self.pos].to_vec()).unwrap();
                match text.as_str() {
                    "break" | "continue" | "do" | "else" | "false" | "for" | "func" | "if"
                    | "in" | "len" | "none" | "print" | "return" | "then" | "true" | "while" => {
                        tokens.push(Token::Keyword(Box::leak(text.into_boxed_str())))
                    }
                    _ => tokens.push(Token::Ident(text)),
                }
            } else {
                let two = if self.pos + 1 < self.bytes.len() {
                    Some(&self.bytes[self.pos..self.pos + 2])
                } else {
                    None
                };
                if let Some(sym) = match two {
                    Some(b"<=") => Some("<="),
                    Some(b">=") => Some(">="),
                    Some(b"==") => Some("=="),
                    Some(b"!=") => Some("!="),
                    Some(b"..") => Some(".."),
                    _ => None,
                } {
                    self.pos += 2;
                    tokens.push(Token::Sym(sym));
                } else if let Some(sym) = match b {
                    b'(' => Some("("),
                    b')' => Some(")"),
                    b'{' => Some("{"),
                    b'}' => Some("}"),
                    b'[' => Some("["),
                    b']' => Some("]"),
                    b',' => Some(","),
                    b';' => Some(";"),
                    b'+' => Some("+"),
                    b'-' => Some("-"),
                    b'*' => Some("*"),
                    b'/' => Some("/"),
                    b'%' => Some("%"),
                    b'!' => Some("!"),
                    b'&' => Some("&"),
                    b'|' => Some("|"),
                    b'<' => Some("<"),
                    b'>' => Some(">"),
                    b'=' => Some("="),
                    _ => None,
                } {
                    self.pos += 1;
                    tokens.push(Token::Sym(sym));
                } else {
                    syntax_error();
                }
            }
        }
        tokens.push(Token::Eof);
        tokens
    }

    fn peek(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }
}

#[derive(Clone, Debug)]
struct Program {
    functions: Vec<Function>,
}

#[derive(Clone, Debug)]
struct Function {
    name: String,
    params: Vec<String>,
    body: Vec<Stmt>,
}

#[derive(Clone, Debug)]
enum Stmt {
    Block(Vec<Stmt>),
    Print(Expr),
    Continue,
    Break,
    Return(Option<Expr>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    For(String, Expr, Expr, Box<Stmt>),
    While(Expr, Box<Stmt>),
    Assign(String, Expr),
    AssignIdx(Expr, Expr, Expr),
    Expr(Expr),
    Null,
}

#[derive(Clone, Debug)]
enum Expr {
    Int(BigInt),
    Bool(bool),
    None,
    Var(String),
    List(Vec<Expr>),
    Call(String, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Unary(String, Box<Expr>),
    Binary(String, Box<Expr>, Box<Expr>),
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn parse_program(&mut self) -> Program {
        let mut functions = Vec::new();
        while !self.at_eof() {
            functions.push(self.parse_function());
        }
        Program { functions }
    }

    fn parse_function(&mut self) -> Function {
        self.expect_keyword("func");
        let name = self.expect_ident();
        self.expect_sym("(");
        let mut params = Vec::new();
        if !self.check_sym(")") {
            loop {
                params.push(self.expect_ident());
                if self.match_sym(",") {
                    continue;
                }
                break;
            }
        }
        self.expect_sym(")");
        let body = self.parse_block_body();
        Function { name, params, body }
    }

    fn parse_block_body(&mut self) -> Vec<Stmt> {
        self.expect_sym("{");
        let mut stmts = Vec::new();
        while !self.check_sym("}") {
            if self.at_eof() {
                syntax_error();
            }
            stmts.push(self.parse_stmt());
        }
        self.expect_sym("}");
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        if self.check_sym("{") {
            return Stmt::Block(self.parse_block_body());
        }
        if self.match_keyword("print") {
            let expr = self.parse_expr();
            self.expect_sym(";");
            return Stmt::Print(expr);
        }
        if self.match_keyword("continue") {
            self.expect_sym(";");
            return Stmt::Continue;
        }
        if self.match_keyword("break") {
            self.expect_sym(";");
            return Stmt::Break;
        }
        if self.match_keyword("return") {
            let expr = if self.check_sym(";") {
                None
            } else {
                Some(self.parse_expr())
            };
            self.expect_sym(";");
            return Stmt::Return(expr);
        }
        if self.match_keyword("if") {
            let cond = self.parse_expr();
            self.expect_keyword("then");
            let then_stmt = self.parse_stmt();
            let else_stmt = if self.match_keyword("else") {
                Some(Box::new(self.parse_stmt()))
            } else {
                None
            };
            return Stmt::If(cond, Box::new(then_stmt), else_stmt);
        }
        if self.match_keyword("for") {
            let var = self.expect_ident();
            self.expect_keyword("in");
            let lo = self.parse_expr();
            self.expect_sym("..");
            let hi = self.parse_expr();
            self.expect_keyword("do");
            let body = self.parse_stmt();
            return Stmt::For(var, lo, hi, Box::new(body));
        }
        if self.match_keyword("while") {
            let cond = self.parse_expr();
            self.expect_keyword("do");
            let body = self.parse_stmt();
            return Stmt::While(cond, Box::new(body));
        }
        if self.match_sym(";") {
            return Stmt::Null;
        }
        if let Token::Ident(name) = self.current().clone() {
            if self.next_is_sym("=") {
                self.pos += 2;
                let rhs = self.parse_expr();
                self.expect_sym(";");
                return Stmt::Assign(name, rhs);
            }
        }
        let expr = self.parse_expr();
        if self.match_sym("=") {
            let Expr::Index(base, idx) = expr else {
                syntax_error();
            };
            let rhs = self.parse_expr();
            self.expect_sym(";");
            Stmt::AssignIdx(*base, *idx, rhs)
        } else {
            self.expect_sym(";");
            Stmt::Expr(expr)
        }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_binary(1)
    }

    fn parse_binary(&mut self, min_prec: u8) -> Expr {
        let mut left = self.parse_unary();
        while let Some((op, prec)) = self.binary_op() {
            if prec < min_prec {
                break;
            }
            self.pos += 1;
            let right = self.parse_binary(prec + 1);
            left = Expr::Binary(op.to_string(), Box::new(left), Box::new(right));
        }
        left
    }

    fn parse_unary(&mut self) -> Expr {
        if let Some(op) = self.unary_op() {
            self.pos += 1;
            let expr = self.parse_unary();
            Expr::Unary(op.to_string(), Box::new(expr))
        } else {
            self.parse_postfix()
        }
    }

    fn parse_postfix(&mut self) -> Expr {
        let mut expr = self.parse_primary();
        while self.match_sym("[") {
            let idx = self.parse_expr();
            self.expect_sym("]");
            expr = Expr::Index(Box::new(expr), Box::new(idx));
        }
        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current().clone() {
            Token::Number(n) => {
                self.pos += 1;
                Expr::Int(BigInt::parse(&n))
            }
            Token::Keyword("true") => {
                self.pos += 1;
                Expr::Bool(true)
            }
            Token::Keyword("false") => {
                self.pos += 1;
                Expr::Bool(false)
            }
            Token::Keyword("none") => {
                self.pos += 1;
                Expr::None
            }
            Token::Ident(name) => {
                self.pos += 1;
                if self.match_sym("(") {
                    let mut args = Vec::new();
                    if !self.check_sym(")") {
                        loop {
                            args.push(self.parse_expr());
                            if self.match_sym(",") {
                                continue;
                            }
                            break;
                        }
                    }
                    self.expect_sym(")");
                    Expr::Call(name, args)
                } else {
                    Expr::Var(name)
                }
            }
            Token::Sym("(") => {
                self.pos += 1;
                let expr = self.parse_expr();
                self.expect_sym(")");
                expr
            }
            Token::Sym("[") => {
                self.pos += 1;
                let mut items = Vec::new();
                if !self.check_sym("]") {
                    loop {
                        items.push(self.parse_expr());
                        if self.match_sym(",") {
                            continue;
                        }
                        break;
                    }
                }
                self.expect_sym("]");
                Expr::List(items)
            }
            _ => syntax_error(),
        }
    }

    fn unary_op(&self) -> Option<&'static str> {
        match self.current() {
            Token::Sym("+") => Some("+"),
            Token::Sym("-") => Some("-"),
            Token::Sym("!") => Some("!"),
            Token::Keyword("len") => Some("len"),
            _ => None,
        }
    }

    fn binary_op(&self) -> Option<(&'static str, u8)> {
        match self.current() {
            Token::Sym("*") => Some(("*", 5)),
            Token::Sym("/") => Some(("/", 5)),
            Token::Sym("%") => Some(("%", 5)),
            Token::Sym("+") => Some(("+", 4)),
            Token::Sym("-") => Some(("-", 4)),
            Token::Sym("<") => Some(("<", 3)),
            Token::Sym("<=") => Some(("<=", 3)),
            Token::Sym(">") => Some((">", 3)),
            Token::Sym(">=") => Some((">=", 3)),
            Token::Sym("==") => Some(("==", 3)),
            Token::Sym("!=") => Some(("!=", 3)),
            Token::Sym("&") => Some(("&", 2)),
            Token::Sym("|") => Some(("|", 2)),
            _ => None,
        }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn at_eof(&self) -> bool {
        matches!(self.current(), Token::Eof)
    }

    fn check_sym(&self, sym: &str) -> bool {
        matches!(self.current(), Token::Sym(s) if *s == sym)
    }

    fn next_is_sym(&self, sym: &str) -> bool {
        matches!(self.tokens.get(self.pos + 1), Some(Token::Sym(s)) if *s == sym)
    }

    fn match_sym(&mut self, sym: &str) -> bool {
        if self.check_sym(sym) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expect_sym(&mut self, sym: &str) {
        if !self.match_sym(sym) {
            syntax_error();
        }
    }

    fn match_keyword(&mut self, kw: &str) -> bool {
        if matches!(self.current(), Token::Keyword(k) if *k == kw) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expect_keyword(&mut self, kw: &str) {
        if !self.match_keyword(kw) {
            syntax_error();
        }
    }

    fn expect_ident(&mut self) -> String {
        match self.current().clone() {
            Token::Ident(name) => {
                self.pos += 1;
                name
            }
            _ => syntax_error(),
        }
    }
}

#[derive(Clone, Debug)]
enum Value {
    Int(BigInt),
    Bool(bool),
    None,
    List(Rc<RefCell<Vec<Value>>>),
}

impl Value {
    fn type_id(&self) -> u8 {
        match self {
            Value::Int(_) => 0,
            Value::Bool(_) => 1,
            Value::None => 2,
            Value::List(_) => 3,
        }
    }

    fn format(&self) -> String {
        match self {
            Value::Int(n) => n.to_string(),
            Value::Bool(true) => "True".to_string(),
            Value::Bool(false) => "False".to_string(),
            Value::None => "None".to_string(),
            Value::List(items) => {
                let parts: Vec<String> = items.borrow().iter().map(|v| v.format()).collect();
                format!("[{}]", parts.join(", "))
            }
        }
    }

    fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::None, Value::None) => true,
            (Value::List(a), Value::List(b)) => {
                let av = a.borrow();
                let bv = b.borrow();
                av.len() == bv.len() && av.iter().zip(bv.iter()).all(|(x, y)| x.equals(y))
            }
            _ => false,
        }
    }
}

enum Flow {
    None,
    Break,
    Continue,
    Return(Value),
}

struct Interpreter {
    funcs: HashMap<String, Function>,
}

impl Interpreter {
    fn new(program: Program) -> Self {
        let mut funcs = HashMap::new();
        for f in program.functions {
            if funcs.insert(f.name.clone(), f).is_some() {
                runtime_error();
            }
        }
        Self { funcs }
    }

    fn run(&self) {
        let main = self.funcs.get("main").unwrap_or_else(|| runtime_error());
        if !main.params.is_empty() {
            runtime_error();
        }
        let mut env = HashMap::new();
        match self.exec_block(&main.body, &mut env) {
            Flow::None | Flow::Return(_) => {}
            Flow::Break | Flow::Continue => runtime_error(),
        }
    }

    fn exec_block(&self, stmts: &[Stmt], env: &mut HashMap<String, Value>) -> Flow {
        for stmt in stmts {
            match self.exec_stmt(stmt, env) {
                Flow::None => {}
                flow => return flow,
            }
        }
        Flow::None
    }

    fn exec_stmt(&self, stmt: &Stmt, env: &mut HashMap<String, Value>) -> Flow {
        match stmt {
            Stmt::Block(stmts) => self.exec_block(stmts, env),
            Stmt::Print(expr) => {
                println!("{}", self.eval(expr, env).format());
                Flow::None
            }
            Stmt::Continue => Flow::Continue,
            Stmt::Break => Flow::Break,
            Stmt::Return(expr) => Flow::Return(
                expr.as_ref()
                    .map(|e| self.eval(e, env))
                    .unwrap_or(Value::None),
            ),
            Stmt::If(cond, then_stmt, else_stmt) => match self.eval(cond, env) {
                Value::Bool(true) => self.exec_stmt(then_stmt, env),
                Value::Bool(false) => {
                    if let Some(stmt) = else_stmt {
                        self.exec_stmt(stmt, env)
                    } else {
                        Flow::None
                    }
                }
                _ => runtime_error(),
            },
            Stmt::For(var, lo, hi, body) => {
                let lo = match self.eval(lo, env) {
                    Value::Int(n) => n.to_i64().unwrap_or_else(|| runtime_error()),
                    _ => runtime_error(),
                };
                let hi = match self.eval(hi, env) {
                    Value::Int(n) => n.to_i64().unwrap_or_else(|| runtime_error()),
                    _ => runtime_error(),
                };
                let old = env.get(var).cloned();
                for i in lo..hi {
                    env.insert(var.clone(), Value::Int(BigInt::from_i64(i)));
                    match self.exec_stmt(body, env) {
                        Flow::None | Flow::Continue => {}
                        Flow::Break => break,
                        flow @ Flow::Return(_) => {
                            Self::restore_for_var(env, var, old);
                            return flow;
                        }
                    }
                }
                Self::restore_for_var(env, var, old);
                Flow::None
            }
            Stmt::While(cond, body) => {
                loop {
                    match self.eval(cond, env) {
                        Value::Bool(true) => {}
                        Value::Bool(false) => break,
                        _ => runtime_error(),
                    }
                    match self.exec_stmt(body, env) {
                        Flow::None | Flow::Continue => {}
                        Flow::Break => break,
                        flow @ Flow::Return(_) => return flow,
                    }
                }
                Flow::None
            }
            Stmt::Assign(name, expr) => {
                let val = self.eval(expr, env);
                env.insert(name.clone(), val);
                Flow::None
            }
            Stmt::AssignIdx(base, idx, rhs) => {
                let base = self.eval(base, env);
                let idx = self.index_value(&self.eval(idx, env));
                let rhs = self.eval(rhs, env);
                match base {
                    Value::List(items) => {
                        let mut items = items.borrow_mut();
                        if idx >= items.len() {
                            runtime_error();
                        }
                        items[idx] = rhs;
                    }
                    _ => runtime_error(),
                }
                Flow::None
            }
            Stmt::Expr(expr) => {
                self.eval(expr, env);
                Flow::None
            }
            Stmt::Null => Flow::None,
        }
    }

    fn restore_for_var(env: &mut HashMap<String, Value>, var: &str, old: Option<Value>) {
        if let Some(value) = old {
            env.insert(var.to_string(), value);
        }
    }

    fn eval(&self, expr: &Expr, env: &mut HashMap<String, Value>) -> Value {
        match expr {
            Expr::Int(n) => Value::Int(n.clone()),
            Expr::Bool(b) => Value::Bool(*b),
            Expr::None => Value::None,
            Expr::Var(name) => env.get(name).cloned().unwrap_or_else(|| runtime_error()),
            Expr::List(items) => {
                let values = items.iter().map(|e| self.eval(e, env)).collect();
                Value::List(Rc::new(RefCell::new(values)))
            }
            Expr::Call(name, args) => self.call(name, args, env),
            Expr::Index(base, idx) => {
                let base = self.eval(base, env);
                let idx = self.index_value(&self.eval(idx, env));
                match base {
                    Value::List(items) => items
                        .borrow()
                        .get(idx)
                        .cloned()
                        .unwrap_or_else(|| runtime_error()),
                    _ => runtime_error(),
                }
            }
            Expr::Unary(op, expr) => {
                let val = self.eval(expr, env);
                self.eval_unary(op, val)
            }
            Expr::Binary(op, left, right) => {
                let a = self.eval(left, env);
                let b = self.eval(right, env);
                self.eval_binary(op, a, b)
            }
        }
    }

    fn call(&self, name: &str, args: &[Expr], env: &mut HashMap<String, Value>) -> Value {
        let function = self.funcs.get(name).unwrap_or_else(|| runtime_error());
        if function.params.len() != args.len() {
            runtime_error();
        }
        let mut call_env = HashMap::new();
        for (param, arg) in function.params.iter().zip(args.iter()) {
            call_env.insert(param.clone(), self.eval(arg, env));
        }
        match self.exec_block(&function.body, &mut call_env) {
            Flow::None => Value::None,
            Flow::Return(value) => value,
            Flow::Break | Flow::Continue => runtime_error(),
        }
    }

    fn eval_unary(&self, op: &str, val: Value) -> Value {
        Self::eval_unary_raw(op, val)
    }

    fn eval_unary_raw(op: &str, val: Value) -> Value {
        match (op, val) {
            ("+", Value::Int(n)) => Value::Int(n),
            ("-", Value::Int(n)) => Value::Int(n.neg()),
            ("!", Value::Bool(b)) => Value::Bool(!b),
            ("len", Value::List(items)) => {
                Value::Int(BigInt::from_i64(items.borrow().len() as i64))
            }
            _ => runtime_error(),
        }
    }

    fn eval_binary(&self, op: &str, a: Value, b: Value) -> Value {
        Self::eval_binary_raw(op, a, b)
    }

    fn eval_binary_raw(op: &str, a: Value, b: Value) -> Value {
        if a.type_id() != b.type_id() {
            runtime_error();
        }
        match (op, a, b) {
            ("+", Value::Int(a), Value::Int(b)) => Value::Int(a.add(&b)),
            ("-", Value::Int(a), Value::Int(b)) => Value::Int(a.sub(&b)),
            ("*", Value::Int(a), Value::Int(b)) => Value::Int(a.mul(&b)),
            ("/", Value::Int(a), Value::Int(b)) => {
                Value::Int(a.div_floor(&b).unwrap_or_else(|| runtime_error()))
            }
            ("%", Value::Int(a), Value::Int(b)) => {
                Value::Int(a.modulo(&b).unwrap_or_else(|| runtime_error()))
            }
            ("<", Value::Int(a), Value::Int(b)) => Value::Bool(a.cmp(&b) == Ordering::Less),
            ("<=", Value::Int(a), Value::Int(b)) => Value::Bool(a.cmp(&b) != Ordering::Greater),
            (">", Value::Int(a), Value::Int(b)) => Value::Bool(a.cmp(&b) == Ordering::Greater),
            (">=", Value::Int(a), Value::Int(b)) => Value::Bool(a.cmp(&b) != Ordering::Less),
            ("==", a, b) => Value::Bool(a.equals(&b)),
            ("!=", a, b) => Value::Bool(!a.equals(&b)),
            ("&", Value::Bool(a), Value::Bool(b)) => Value::Bool(a & b),
            ("|", Value::Bool(a), Value::Bool(b)) => Value::Bool(a | b),
            ("+", Value::List(a), Value::List(b)) => {
                let mut out = a.borrow().clone();
                out.extend(b.borrow().iter().cloned());
                Value::List(Rc::new(RefCell::new(out)))
            }
            ("<", Value::Bool(a), Value::Bool(b)) => Value::Bool(!a & b),
            ("<=", Value::Bool(a), Value::Bool(b)) => Value::Bool(!a | b),
            (">", Value::Bool(a), Value::Bool(b)) => Value::Bool(a & !b),
            (">=", Value::Bool(a), Value::Bool(b)) => Value::Bool(a | !b),
            _ => runtime_error(),
        }
    }

    fn index_value(&self, value: &Value) -> usize {
        Self::index_value_raw(value)
    }

    fn index_value_raw(value: &Value) -> usize {
        match value {
            Value::Int(n) => {
                let i = n.to_i64().unwrap_or_else(|| runtime_error());
                if i < 0 {
                    runtime_error();
                }
                i as usize
            }
            _ => runtime_error(),
        }
    }
}

type Env = HashMap<String, Value>;
type ExprProc = Rc<dyn Fn(&AnalyzedInterpreter, &mut Env) -> Value>;
type StmtProc = Rc<dyn Fn(&AnalyzedInterpreter, &mut Env) -> Flow>;

#[derive(Clone)]
struct AnalyzedFunction {
    params: Vec<String>,
    body: StmtProc,
}

struct AnalyzedInterpreter {
    funcs: HashMap<String, AnalyzedFunction>,
}

impl AnalyzedInterpreter {
    fn new(program: Program) -> Self {
        let mut analyzer = Analyzer::new();
        for function in program.functions {
            analyzer.analyze_function(function);
        }
        Self {
            funcs: analyzer.funcs,
        }
    }

    fn run(&self) {
        let main = self.funcs.get("main").unwrap_or_else(|| runtime_error());
        if !main.params.is_empty() {
            runtime_error();
        }
        let mut env = HashMap::new();
        match (main.body)(self, &mut env) {
            Flow::None | Flow::Return(_) => {}
            Flow::Break | Flow::Continue => runtime_error(),
        }
    }

    fn call(&self, name: &str, args: &[ExprProc], env: &mut Env) -> Value {
        let function = self.funcs.get(name).unwrap_or_else(|| runtime_error());
        let params = function.params.clone();
        let body = Rc::clone(&function.body);
        if params.len() != args.len() {
            runtime_error();
        }

        let mut call_env = HashMap::new();
        for (param, arg) in params.iter().zip(args.iter()) {
            call_env.insert(param.clone(), arg(self, env));
        }

        match body(self, &mut call_env) {
            Flow::None => Value::None,
            Flow::Return(value) => value,
            Flow::Break | Flow::Continue => runtime_error(),
        }
    }
}

struct Analyzer {
    funcs: HashMap<String, AnalyzedFunction>,
}

impl Analyzer {
    fn new() -> Self {
        Self {
            funcs: HashMap::new(),
        }
    }

    fn analyze_function(&mut self, function: Function) {
        let analyzed = AnalyzedFunction {
            params: function.params,
            body: self.analyze_block(function.body),
        };
        if self.funcs.insert(function.name, analyzed).is_some() {
            runtime_error();
        }
    }

    fn analyze_block(&self, stmts: Vec<Stmt>) -> StmtProc {
        let stmt_procs: Vec<StmtProc> = stmts
            .into_iter()
            .map(|stmt| self.analyze_stmt(stmt))
            .collect();
        Rc::new(move |runtime, env| {
            for proc in &stmt_procs {
                match proc(runtime, env) {
                    Flow::None => {}
                    flow => return flow,
                }
            }
            Flow::None
        })
    }

    fn analyze_stmt(&self, stmt: Stmt) -> StmtProc {
        match stmt {
            Stmt::Block(stmts) => self.analyze_block(stmts),
            Stmt::Print(expr) => {
                let expr_proc = self.analyze_expr(expr);
                Rc::new(move |runtime, env| {
                    println!("{}", expr_proc(runtime, env).format());
                    Flow::None
                })
            }
            Stmt::Continue => Rc::new(|_, _| Flow::Continue),
            Stmt::Break => Rc::new(|_, _| Flow::Break),
            Stmt::Return(expr) => {
                let expr_proc = expr.map(|expr| self.analyze_expr(expr));
                Rc::new(move |runtime, env| {
                    Flow::Return(
                        expr_proc
                            .as_ref()
                            .map(|proc| proc(runtime, env))
                            .unwrap_or(Value::None),
                    )
                })
            }
            Stmt::If(cond, then_stmt, else_stmt) => {
                let cond_proc = self.analyze_expr(cond);
                let then_proc = self.analyze_stmt(*then_stmt);
                let else_proc = else_stmt.map(|stmt| self.analyze_stmt(*stmt));
                Rc::new(move |runtime, env| match cond_proc(runtime, env) {
                    Value::Bool(true) => then_proc(runtime, env),
                    Value::Bool(false) => {
                        if let Some(proc) = &else_proc {
                            proc(runtime, env)
                        } else {
                            Flow::None
                        }
                    }
                    _ => runtime_error(),
                })
            }
            Stmt::For(var, lo, hi, body) => {
                let lo_proc = self.analyze_expr(lo);
                let hi_proc = self.analyze_expr(hi);
                let body_proc = self.analyze_stmt(*body);
                Rc::new(move |runtime, env| {
                    let lo = match lo_proc(runtime, env) {
                        Value::Int(n) => n.to_i64().unwrap_or_else(|| runtime_error()),
                        _ => runtime_error(),
                    };
                    let hi = match hi_proc(runtime, env) {
                        Value::Int(n) => n.to_i64().unwrap_or_else(|| runtime_error()),
                        _ => runtime_error(),
                    };
                    let old = env.get(&var).cloned();
                    for i in lo..hi {
                        env.insert(var.clone(), Value::Int(BigInt::from_i64(i)));
                        match body_proc(runtime, env) {
                            Flow::None | Flow::Continue => {}
                            Flow::Break => break,
                            flow @ Flow::Return(_) => {
                                Interpreter::restore_for_var(env, &var, old);
                                return flow;
                            }
                        }
                    }
                    Interpreter::restore_for_var(env, &var, old);
                    Flow::None
                })
            }
            Stmt::While(cond, body) => {
                let cond_proc = self.analyze_expr(cond);
                let body_proc = self.analyze_stmt(*body);
                Rc::new(move |runtime, env| {
                    loop {
                        match cond_proc(runtime, env) {
                            Value::Bool(true) => {}
                            Value::Bool(false) => break,
                            _ => runtime_error(),
                        }
                        match body_proc(runtime, env) {
                            Flow::None | Flow::Continue => {}
                            Flow::Break => break,
                            flow @ Flow::Return(_) => return flow,
                        }
                    }
                    Flow::None
                })
            }
            Stmt::Assign(name, expr) => {
                let expr_proc = self.analyze_expr(expr);
                Rc::new(move |runtime, env| {
                    let value = expr_proc(runtime, env);
                    env.insert(name.clone(), value);
                    Flow::None
                })
            }
            Stmt::AssignIdx(base, idx, rhs) => {
                let base_proc = self.analyze_expr(base);
                let idx_proc = self.analyze_expr(idx);
                let rhs_proc = self.analyze_expr(rhs);
                Rc::new(move |runtime, env| {
                    let base = base_proc(runtime, env);
                    let idx = Interpreter::index_value_raw(&idx_proc(runtime, env));
                    let rhs = rhs_proc(runtime, env);
                    match base {
                        Value::List(items) => {
                            let mut items = items.borrow_mut();
                            if idx >= items.len() {
                                runtime_error();
                            }
                            items[idx] = rhs;
                        }
                        _ => runtime_error(),
                    }
                    Flow::None
                })
            }
            Stmt::Expr(expr) => {
                let expr_proc = self.analyze_expr(expr);
                Rc::new(move |runtime, env| {
                    expr_proc(runtime, env);
                    Flow::None
                })
            }
            Stmt::Null => Rc::new(|_, _| Flow::None),
        }
    }

    fn analyze_expr(&self, expr: Expr) -> ExprProc {
        match expr {
            Expr::Int(n) => Rc::new(move |_, _| Value::Int(n.clone())),
            Expr::Bool(b) => Rc::new(move |_, _| Value::Bool(b)),
            Expr::None => Rc::new(|_, _| Value::None),
            Expr::Var(name) => {
                Rc::new(move |_, env| env.get(&name).cloned().unwrap_or_else(|| runtime_error()))
            }
            Expr::List(items) => {
                let item_procs: Vec<ExprProc> = items
                    .into_iter()
                    .map(|expr| self.analyze_expr(expr))
                    .collect();
                Rc::new(move |runtime, env| {
                    let values = item_procs.iter().map(|proc| proc(runtime, env)).collect();
                    Value::List(Rc::new(RefCell::new(values)))
                })
            }
            Expr::Call(name, args) => {
                let arg_procs: Vec<ExprProc> = args
                    .into_iter()
                    .map(|expr| self.analyze_expr(expr))
                    .collect();
                Rc::new(move |runtime, env| runtime.call(&name, &arg_procs, env))
            }
            Expr::Index(base, idx) => {
                let base_proc = self.analyze_expr(*base);
                let idx_proc = self.analyze_expr(*idx);
                Rc::new(move |runtime, env| {
                    let base = base_proc(runtime, env);
                    let idx = Interpreter::index_value_raw(&idx_proc(runtime, env));
                    match base {
                        Value::List(items) => items
                            .borrow()
                            .get(idx)
                            .cloned()
                            .unwrap_or_else(|| runtime_error()),
                        _ => runtime_error(),
                    }
                })
            }
            Expr::Unary(op, expr) => {
                let expr_proc = self.analyze_expr(*expr);
                Rc::new(move |runtime, env| {
                    let value = expr_proc(runtime, env);
                    Interpreter::eval_unary_raw(&op, value)
                })
            }
            Expr::Binary(op, left, right) => {
                let left_proc = self.analyze_expr(*left);
                let right_proc = self.analyze_expr(*right);
                Rc::new(move |runtime, env| {
                    let left = left_proc(runtime, env);
                    let right = right_proc(runtime, env);
                    Interpreter::eval_binary_raw(&op, left, right)
                })
            }
        }
    }
}

fn main() {
    let mut use_analyze = false;
    let mut source_file = None;
    for arg in env::args().skip(1) {
        if arg == "--analyze" {
            use_analyze = true;
        } else if arg.starts_with("--") {
            eprintln!("Usage: epic-lang-rs [--analyze] <source_file>");
            process::exit(1);
        } else {
            source_file = Some(arg);
        }
    }

    let Some(path) = source_file else {
        eprintln!("Usage: epic-lang-rs [--analyze] <source_file>");
        process::exit(1);
    };
    let src = fs::read_to_string(path).unwrap_or_else(|_| runtime_error());
    let tokens = Lexer::new(&src).tokenize();
    let program = Parser::new(tokens).parse_program();
    if use_analyze {
        AnalyzedInterpreter::new(program).run();
    } else {
        Interpreter::new(program).run();
    }
}
