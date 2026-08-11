//! wit — the language minds are written in. Written for this world, owned
//! by this repo, zero dependencies. Small on purpose: smallness is what
//! buys the guarantees the ecology stands on.
//!
//! The guarantees (mechanical, not reviewed):
//! - **Termination.** Every statement, expression node, and repeat
//!   iteration burns 1 fuel from ONE tank; capability calls burn their
//!   declared cost from the same tank. A dry tank stops the program.
//! - **A complete effect bound.** The host's capability table is the whole
//!   world a program can touch; nothing else resolves.
//! - **Checked arithmetic.** Overflow, /0, %0 are diagnostics, never
//!   wraparound — a wrong-but-clean result is worse than an error.
//! - **Bounded output.** print clips at the byte cap (never mid-char).
//! - **Bounded nesting.** The parser's depth guard makes deep nesting a
//!   diagnostic, never a stack abort; the genome byte cap bounds the rest.
//!
//! Grammar: `let x = e;`  `x = e;`  `print e;`  `e;` (bare calls welcome)
//! `if e { } else if e { } else { }`  `repeat e { }` (count evaluated once)
//! Values: i64 + bool. Operators: `+ - * / % < <= > >= == != && || ! -`
//! Comments: `//` and nested `/* */`. No while, no functions, no recursion.

pub struct Limits {
    pub fuel: u64,
    pub output_bytes: usize,
}

#[derive(Debug)]
pub struct Outcome {
    pub output: String,
    pub fuel_used: u64,
}

#[derive(Debug)]
pub struct Diag {
    pub code: &'static str,
    pub msg: String,
    pub span: (usize, usize),
}

impl Diag {
    fn new(code: &'static str, msg: String, span: (usize, usize)) -> Self {
        Diag { code, msg, span }
    }

    /// Caret-rendered, for organisms' last words and inject errors.
    pub fn render(&self, src: &str) -> String {
        let (start, end) = self.span;
        let start = start.min(src.len());
        let mut line_start = 0;
        let mut line_no = 1;
        for (i, c) in src.char_indices() {
            if i >= start {
                break;
            }
            if c == '\n' {
                line_start = i + 1;
                line_no += 1;
            }
        }
        let line_end = src[line_start..].find('\n').map(|p| line_start + p).unwrap_or(src.len());
        let line = &src[line_start..line_end];
        let col = start - line_start;
        let width = end.min(line_end).saturating_sub(start).max(1);
        format!(
            "{}: {}\nline {line_no}, col {}\n  {line}\n  {}{}",
            self.code,
            self.msg,
            col + 1,
            " ".repeat(col),
            "^".repeat(width)
        )
    }
}

/// A capability: the only kind of hole in the world's wall.
pub struct Cap {
    pub name: &'static str,
    pub arity: usize,
    pub cost: u64,
    pub doc: &'static str,
}

/// The host seam. `call` is TOTAL by design: the physics never faults, it
/// returns sentinels. Arguments arrive arity-checked, integers only.
pub trait Host {
    fn caps(&self) -> &'static [Cap];
    fn call(&mut self, idx: usize, args: &[i64]) -> i64;
}

// ---- lexer ----

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tok {
    Int(i64),
    True,
    False,
    Ident, // text via span
    Let,
    Print,
    If,
    Else,
    Repeat,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semi,
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Lt,
    Le,
    Gt,
    Ge,
    EqEq,
    Ne,
    AndAnd,
    OrOr,
    Bang,
    Eof,
}

#[derive(Clone, Copy)]
struct Token {
    kind: Tok,
    span: (usize, usize),
}

fn lex(src: &str) -> Result<Vec<Token>, Diag> {
    let b = src.as_bytes();
    let mut toks = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        match c {
            b' ' | b'\t' | b'\r' | b'\n' => i += 1,
            b'/' if b.get(i + 1) == Some(&b'/') => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                let open = i;
                let mut depth = 1;
                i += 2;
                while i < b.len() && depth > 0 {
                    if b[i] == b'/' && b.get(i + 1) == Some(&b'*') {
                        depth += 1;
                        i += 2;
                    } else if b[i] == b'*' && b.get(i + 1) == Some(&b'/') {
                        depth -= 1;
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                if depth > 0 {
                    return Err(Diag::new("E-COMMENT", "unterminated /* comment".into(), (open, open + 2)));
                }
            }
            b'0'..=b'9' => {
                let start = i;
                let mut val: i64 = 0;
                while i < b.len() && (b[i].is_ascii_digit() || b[i] == b'_') {
                    if b[i] != b'_' {
                        val = val
                            .checked_mul(10)
                            .and_then(|v| v.checked_add((b[i] - b'0') as i64))
                            .ok_or_else(|| {
                                Diag::new("E-INT", "integer literal out of range".into(), (start, i + 1))
                            })?;
                    }
                    i += 1;
                }
                toks.push(Token { kind: Tok::Int(val), span: (start, i) });
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let start = i;
                while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                    i += 1;
                }
                let kind = match &src[start..i] {
                    "let" => Tok::Let,
                    "print" => Tok::Print,
                    "if" => Tok::If,
                    "else" => Tok::Else,
                    "repeat" => Tok::Repeat,
                    "true" => Tok::True,
                    "false" => Tok::False,
                    _ => Tok::Ident,
                };
                toks.push(Token { kind, span: (start, i) });
            }
            _ => {
                let two = |a: u8, b2: u8| c == a && b.get(i + 1) == Some(&b2);
                let (kind, len) = if two(b'<', b'=') {
                    (Tok::Le, 2)
                } else if two(b'>', b'=') {
                    (Tok::Ge, 2)
                } else if two(b'=', b'=') {
                    (Tok::EqEq, 2)
                } else if two(b'!', b'=') {
                    (Tok::Ne, 2)
                } else if two(b'&', b'&') {
                    (Tok::AndAnd, 2)
                } else if two(b'|', b'|') {
                    (Tok::OrOr, 2)
                } else {
                    let k = match c {
                        b'(' => Tok::LParen,
                        b')' => Tok::RParen,
                        b'{' => Tok::LBrace,
                        b'}' => Tok::RBrace,
                        b',' => Tok::Comma,
                        b';' => Tok::Semi,
                        b'=' => Tok::Assign,
                        b'+' => Tok::Plus,
                        b'-' => Tok::Minus,
                        b'*' => Tok::Star,
                        b'/' => Tok::Slash,
                        b'%' => Tok::Percent,
                        b'<' => Tok::Lt,
                        b'>' => Tok::Gt,
                        b'!' => Tok::Bang,
                        _ => {
                            return Err(Diag::new(
                                "E-CHAR",
                                format!("character {:?} starts no wit token", src[i..].chars().next().unwrap()),
                                (i, i + 1),
                            ))
                        }
                    };
                    (k, 1)
                };
                toks.push(Token { kind, span: (i, i + len) });
                i += len;
                continue;
            }
        }
    }
    toks.push(Token { kind: Tok::Eof, span: (src.len(), src.len()) });
    Ok(toks)
}

// ---- AST ----

enum Expr {
    Int(i64),
    Bool(bool),
    Var(String),
    Call(String, Vec<Expr>, (usize, usize)),
    Unary(Tok, Box<Expr>, (usize, usize)),
    Binary(Tok, Box<Expr>, Box<Expr>, (usize, usize)),
}

enum Stmt {
    Let(String, Expr),
    Assign(String, Expr, (usize, usize)),
    Print(Expr),
    Expr(Expr),
    If(Vec<(Expr, Vec<Stmt>)>, Option<Vec<Stmt>>),
    Repeat(Expr, Vec<Stmt>, (usize, usize)),
}

pub struct Program {
    stmts: Vec<Stmt>,
}

// ---- parser ----

const MAX_DEPTH: u32 = 96;

struct Parser<'a> {
    src: &'a str,
    toks: Vec<Token>,
    pos: usize,
    depth: u32,
}

pub fn parse(src: &str) -> Result<Program, Diag> {
    let toks = lex(src)?;
    let mut p = Parser { src, toks, pos: 0, depth: 0 };
    let mut stmts = Vec::new();
    while p.peek().kind != Tok::Eof {
        stmts.push(p.stmt()?);
    }
    Ok(Program { stmts })
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Token {
        self.toks[self.pos]
    }
    fn next(&mut self) -> Token {
        let t = self.toks[self.pos];
        if self.pos + 1 < self.toks.len() {
            self.pos += 1;
        }
        t
    }
    fn text(&self, t: Token) -> String {
        self.src[t.span.0..t.span.1].to_string()
    }
    fn expect(&mut self, kind: Tok, what: &str) -> Result<Token, Diag> {
        let t = self.next();
        if t.kind == kind {
            Ok(t)
        } else {
            Err(Diag::new("E-PARSE", format!("expected {what}"), t.span))
        }
    }
    fn enter(&mut self, span: (usize, usize)) -> Result<(), Diag> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            return Err(Diag::new("E-DEPTH", "nests deeper than the guard allows".into(), span));
        }
        Ok(())
    }
    fn leave(&mut self) {
        self.depth -= 1;
    }

    fn stmt(&mut self) -> Result<Stmt, Diag> {
        let t = self.peek();
        match t.kind {
            Tok::Let => {
                self.next();
                let name = self.expect(Tok::Ident, "a variable name after `let`")?;
                self.expect(Tok::Assign, "`=`")?;
                let e = self.expr()?;
                self.expect(Tok::Semi, "`;`")?;
                Ok(Stmt::Let(self.text(name), e))
            }
            Tok::Print => {
                self.next();
                let e = self.expr()?;
                self.expect(Tok::Semi, "`;`")?;
                Ok(Stmt::Print(e))
            }
            Tok::If => {
                self.next();
                let mut arms = vec![(self.expr()?, self.block()?)];
                let mut els = None;
                while self.peek().kind == Tok::Else {
                    self.next();
                    if self.peek().kind == Tok::If {
                        self.next();
                        arms.push((self.expr()?, self.block()?));
                    } else {
                        els = Some(self.block()?);
                        break;
                    }
                }
                Ok(Stmt::If(arms, els))
            }
            Tok::Repeat => {
                self.next();
                let e = self.expr()?;
                let body = self.block()?;
                Ok(Stmt::Repeat(e, body, t.span))
            }
            Tok::Ident if self.toks[self.pos + 1].kind == Tok::Assign => {
                let name = self.next();
                self.next(); // =
                let e = self.expr()?;
                self.expect(Tok::Semi, "`;`")?;
                Ok(Stmt::Assign(self.text(name), e, name.span))
            }
            _ => {
                // Bare expression statement: `harvest();` is a sentence.
                let e = self.expr()?;
                self.expect(Tok::Semi, "`;`")?;
                Ok(Stmt::Expr(e))
            }
        }
    }

    fn block(&mut self) -> Result<Vec<Stmt>, Diag> {
        let open = self.expect(Tok::LBrace, "`{`")?;
        self.enter(open.span)?;
        let mut stmts = Vec::new();
        while self.peek().kind != Tok::RBrace {
            if self.peek().kind == Tok::Eof {
                return Err(Diag::new("E-PARSE", "expected `}`".into(), self.peek().span));
            }
            stmts.push(self.stmt()?);
        }
        self.next();
        self.leave();
        Ok(stmts)
    }

    fn expr(&mut self) -> Result<Expr, Diag> {
        self.binary(0)
    }

    /// Precedence climbing. Each fold charges the depth guard: the guard
    /// bounds the AST spine the evaluator later walks, not just nesting —
    /// a lesson inherited from the lineage (long `1+1+…` chains).
    fn binary(&mut self, min_prec: u8) -> Result<Expr, Diag> {
        let mut lhs = self.unary()?;
        let mut charged = 0u32;
        loop {
            let t = self.peek();
            let (prec, kind) = match t.kind {
                Tok::OrOr => (1, t.kind),
                Tok::AndAnd => (2, t.kind),
                Tok::EqEq | Tok::Ne => (3, t.kind),
                Tok::Lt | Tok::Le | Tok::Gt | Tok::Ge => (4, t.kind),
                Tok::Plus | Tok::Minus => (5, t.kind),
                Tok::Star | Tok::Slash | Tok::Percent => (6, t.kind),
                _ => break,
            };
            if prec < min_prec {
                break;
            }
            self.next();
            self.enter(t.span)?;
            charged += 1;
            let rhs = self.binary(prec + 1)?;
            lhs = Expr::Binary(kind, Box::new(lhs), Box::new(rhs), t.span);
        }
        for _ in 0..charged {
            self.leave();
        }
        Ok(lhs)
    }

    fn unary(&mut self) -> Result<Expr, Diag> {
        let t = self.peek();
        match t.kind {
            Tok::Minus | Tok::Bang => {
                self.next();
                self.enter(t.span)?;
                let e = self.unary()?;
                self.leave();
                Ok(Expr::Unary(t.kind, Box::new(e), t.span))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr, Diag> {
        let t = self.next();
        match t.kind {
            Tok::Int(v) => Ok(Expr::Int(v)),
            Tok::True => Ok(Expr::Bool(true)),
            Tok::False => Ok(Expr::Bool(false)),
            Tok::LParen => {
                self.enter(t.span)?;
                let e = self.expr()?;
                self.expect(Tok::RParen, "`)`")?;
                self.leave();
                Ok(e)
            }
            Tok::Ident => {
                if self.peek().kind == Tok::LParen {
                    self.next();
                    self.enter(t.span)?;
                    let mut args = Vec::new();
                    if self.peek().kind != Tok::RParen {
                        loop {
                            args.push(self.expr()?);
                            if self.peek().kind == Tok::Comma {
                                self.next();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(Tok::RParen, "`)`")?;
                    self.leave();
                    Ok(Expr::Call(self.text(t), args, t.span))
                } else {
                    Ok(Expr::Var(self.text(t)))
                }
            }
            _ => Err(Diag::new("E-PARSE", "expected an expression".into(), t.span)),
        }
    }
}

// ---- evaluator ----

#[derive(Clone, Copy, PartialEq)]
enum Value {
    Int(i64),
    Bool(bool),
}

struct Eval<'a> {
    host: &'a mut dyn Host,
    fuel: u64,
    fuel_start: u64,
    vars: Vec<(String, Value)>,
    scopes: Vec<usize>,
    out: String,
    out_cap: usize,
}

type EResult = Result<Value, Diag>;

const NO_SPAN: (usize, usize) = (0, 0);

pub fn run(src: &str, limits: Limits, host: &mut dyn Host) -> Result<Outcome, Diag> {
    let program = parse(src)?;
    let mut ev = Eval {
        host,
        fuel: limits.fuel,
        fuel_start: limits.fuel,
        vars: Vec::new(),
        scopes: Vec::new(),
        out: String::new(),
        out_cap: limits.output_bytes,
    };
    for s in &program.stmts {
        ev.stmt(s)?;
    }
    Ok(Outcome { output: ev.out, fuel_used: ev.fuel_start - ev.fuel })
}

impl<'a> Eval<'a> {
    fn burn(&mut self, cost: u64, span: (usize, usize)) -> Result<(), Diag> {
        if self.fuel < cost {
            self.fuel = 0;
            return Err(Diag::new("E-FUEL", "fuel exhausted — the tank is dry".into(), span));
        }
        self.fuel -= cost;
        Ok(())
    }

    fn stmt(&mut self, s: &Stmt) -> Result<(), Diag> {
        self.burn(1, NO_SPAN)?;
        match s {
            Stmt::Let(name, e) => {
                let v = self.expr(e)?;
                self.vars.push((name.clone(), v));
            }
            Stmt::Assign(name, e, span) => {
                let v = self.expr(e)?;
                match self.vars.iter_mut().rev().find(|(n, _)| n == name) {
                    Some(slot) => slot.1 = v,
                    None => {
                        return Err(Diag::new("E-UNDEF", format!("`{name}` has no visible `let`"), *span))
                    }
                }
            }
            Stmt::Print(e) => {
                let v = self.expr(e)?;
                let line = match v {
                    Value::Int(i) => format!("{i}\n"),
                    Value::Bool(b) => format!("{b}\n"),
                };
                if self.out.len() + line.len() <= self.out_cap {
                    self.out.push_str(&line);
                }
            }
            Stmt::Expr(e) => {
                self.expr(e)?;
            }
            Stmt::If(arms, els) => {
                for (cond, body) in arms {
                    if self.truthy(cond)? {
                        return self.block(body);
                    }
                }
                if let Some(body) = els {
                    return self.block(body);
                }
            }
            Stmt::Repeat(count, body, span) => {
                let n = match self.expr(count)? {
                    Value::Int(n) if n >= 0 => n,
                    Value::Int(_) => {
                        return Err(Diag::new("E-NEGREP", "repeat with a negative count".into(), *span))
                    }
                    Value::Bool(_) => {
                        return Err(Diag::new("E-TYPE", "repeat count must be an integer".into(), *span))
                    }
                };
                for _ in 0..n {
                    self.burn(1, *span)?;
                    self.block(body)?;
                }
            }
        }
        Ok(())
    }

    fn block(&mut self, body: &[Stmt]) -> Result<(), Diag> {
        self.scopes.push(self.vars.len());
        let r = body.iter().try_for_each(|s| self.stmt(s));
        let mark = self.scopes.pop().unwrap();
        self.vars.truncate(mark);
        r
    }

    fn truthy(&mut self, e: &Expr) -> Result<bool, Diag> {
        match self.expr(e)? {
            Value::Bool(b) => Ok(b),
            Value::Int(_) => Err(Diag::new("E-TYPE", "condition must be a bool".into(), NO_SPAN)),
        }
    }

    fn expr(&mut self, e: &Expr) -> EResult {
        self.burn(1, NO_SPAN)?;
        match e {
            Expr::Int(v) => Ok(Value::Int(*v)),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            // An unbound name reads as 0. This is a deliberate genetics
            // decision, not laxity: gene splices routinely tear a line
            // from the `let` it referenced, and when that read was fatal
            // (E-UNDEF, whole tank forfeited) most horizontal transfer of
            // multi-line cognition was poison — round 13 found the amber's
            // mind-reserve full of it. A torn gene is now a dead gene, not
            // a fatal one: degraded, cheap, and polishable by selection.
            // Assignment still requires a visible `let`.
            Expr::Var(name) => Ok(self
                .vars
                .iter()
                .rev()
                .find(|(n, _)| n == name)
                .map(|(_, v)| *v)
                .unwrap_or(Value::Int(0))),
            Expr::Call(name, args, span) => {
                let caps = self.host.caps();
                let idx = caps
                    .iter()
                    .position(|c| c.name == name.as_str())
                    .ok_or_else(|| {
                        Diag::new("E-CAP", format!("no capability named `{name}`"), *span)
                    })?;
                let cap = &caps[idx];
                if args.len() != cap.arity {
                    return Err(Diag::new(
                        "E-ARGS",
                        format!("`{name}` takes {} argument(s), got {}", cap.arity, args.len()),
                        *span,
                    ));
                }
                let mut vals = Vec::with_capacity(args.len());
                for a in args {
                    match self.expr(a)? {
                        Value::Int(i) => vals.push(i),
                        Value::Bool(_) => {
                            return Err(Diag::new(
                                "E-TYPE",
                                format!("`{name}` takes integers, got a bool"),
                                *span,
                            ))
                        }
                    }
                }
                self.burn(cap.cost, *span)?;
                Ok(Value::Int(self.host.call(idx, &vals)))
            }
            Expr::Unary(op, e, span) => {
                let v = self.expr(e)?;
                match (op, v) {
                    (Tok::Minus, Value::Int(i)) => i
                        .checked_neg()
                        .map(Value::Int)
                        .ok_or_else(|| Diag::new("E-OVERFLOW", "negation overflowed".into(), *span)),
                    (Tok::Bang, Value::Bool(b)) => Ok(Value::Bool(!b)),
                    _ => Err(Diag::new("E-TYPE", "operator got the wrong type".into(), *span)),
                }
            }
            Expr::Binary(op, l, r, span) => self.binary(*op, l, r, *span),
        }
    }

    fn binary(&mut self, op: Tok, l: &Expr, r: &Expr, span: (usize, usize)) -> EResult {
        // Short-circuit first: the unevaluated side must not run.
        if op == Tok::AndAnd || op == Tok::OrOr {
            let lv = match self.expr(l)? {
                Value::Bool(b) => b,
                _ => return Err(Diag::new("E-TYPE", "&&/|| take bools".into(), span)),
            };
            if (op == Tok::AndAnd && !lv) || (op == Tok::OrOr && lv) {
                return Ok(Value::Bool(lv));
            }
            return match self.expr(r)? {
                Value::Bool(b) => Ok(Value::Bool(b)),
                _ => Err(Diag::new("E-TYPE", "&&/|| take bools".into(), span)),
            };
        }
        let lv = self.expr(l)?;
        let rv = self.expr(r)?;
        let type_err = || Diag::new("E-TYPE", "operator got the wrong types".into(), span);
        match op {
            Tok::EqEq | Tok::Ne => {
                let eq = match (lv, rv) {
                    (Value::Int(a), Value::Int(b)) => a == b,
                    (Value::Bool(a), Value::Bool(b)) => a == b,
                    _ => return Err(type_err()),
                };
                Ok(Value::Bool(if op == Tok::EqEq { eq } else { !eq }))
            }
            _ => {
                let (Value::Int(a), Value::Int(b)) = (lv, rv) else { return Err(type_err()) };
                let int = |r: Option<i64>, what: &str| {
                    r.map(Value::Int)
                        .ok_or_else(|| Diag::new("E-OVERFLOW", format!("{what} overflowed"), span))
                };
                match op {
                    Tok::Plus => int(a.checked_add(b), "addition"),
                    Tok::Minus => int(a.checked_sub(b), "subtraction"),
                    Tok::Star => int(a.checked_mul(b), "multiplication"),
                    Tok::Slash if b == 0 => Err(Diag::new("E-DIV0", "division by zero".into(), span)),
                    Tok::Slash => int(a.checked_div(b), "division"),
                    Tok::Percent if b == 0 => Err(Diag::new("E-DIV0", "remainder by zero".into(), span)),
                    Tok::Percent => int(a.checked_rem(b), "remainder"),
                    Tok::Lt => Ok(Value::Bool(a < b)),
                    Tok::Le => Ok(Value::Bool(a <= b)),
                    Tok::Gt => Ok(Value::Bool(a > b)),
                    Tok::Ge => Ok(Value::Bool(a >= b)),
                    _ => Err(type_err()),
                }
            }
        }
    }
}
