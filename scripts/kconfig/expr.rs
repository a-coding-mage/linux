// SPDX-License-Identifier: GPL-2.0
//! Immutable Kconfig expressions and tristate operations.
// Original Kconfig implementation: Copyright (C) 2002 Roman Zippel.

use crate::model::{Kconfig, SymbolId, SymbolType, MOD, NO, YES};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Tristate {
    #[default]
    No,
    Mod,
    Yes,
}

impl Tristate {
    pub(crate) fn not(self) -> Self {
        match self {
            Self::No => Self::Yes,
            Self::Mod => Self::Mod,
            Self::Yes => Self::No,
        }
    }
    pub(crate) fn boolean(self) -> Self {
        if self == Self::No {
            Self::No
        } else {
            Self::Yes
        }
    }
}

impl std::fmt::Display for Tristate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::No => "n",
            Self::Mod => "m",
            Self::Yes => "y",
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Compare {
    Equal,
    Unequal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Node {
    Symbol(SymbolId),
    And(Expr, Expr),
    Or(Expr, Expr),
    Not(Expr),
    Compare(Compare, SymbolId, SymbolId),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Expr(pub(crate) Rc<Node>);

impl Expr {
    pub(crate) fn symbol(symbol: SymbolId) -> Self {
        Self(Rc::new(Node::Symbol(symbol)))
    }
    pub(crate) fn yes() -> Self {
        Self::symbol(YES)
    }
    pub(crate) fn no() -> Self {
        Self::symbol(NO)
    }
    pub(crate) fn is_yes(&self) -> bool {
        matches!(*self.0, Node::Symbol(YES))
    }
    pub(crate) fn is_no(&self) -> bool {
        matches!(*self.0, Node::Symbol(NO))
    }
    pub(crate) fn as_symbol(&self) -> Option<SymbolId> {
        if let Node::Symbol(id) = *self.0 {
            Some(id)
        } else {
            None
        }
    }
    pub(crate) fn compare(kind: Compare, left: SymbolId, right: SymbolId) -> Self {
        Self(Rc::new(Node::Compare(kind, left, right)))
    }
    pub(crate) fn and(self, right: Self) -> Self {
        if self.is_yes() || self == right {
            right
        } else if right.is_yes() {
            self
        } else if self.is_no() || right.is_no() {
            Self::no()
        } else {
            Self(Rc::new(Node::And(self, right)))
        }
    }
    pub(crate) fn or(self, right: Self) -> Self {
        if self.is_no() || self == right {
            right
        } else if right.is_no() {
            self
        } else if self.is_yes() || right.is_yes() {
            Self::yes()
        } else {
            Self(Rc::new(Node::Or(self, right)))
        }
    }
    pub(crate) fn not(self) -> Self {
        match self.0.as_ref() {
            Node::Symbol(NO) => Self::yes(),
            Node::Symbol(YES) => Self::no(),
            Node::Symbol(MOD) => self,
            Node::Not(expression) => expression.clone(),
            _ => Self(Rc::new(Node::Not(self))),
        }
    }
    pub(crate) fn equals_no(&self) -> Self {
        match self.0.as_ref() {
            Node::And(left, right) => left.equals_no().or(right.equals_no()),
            Node::Or(left, right) => left.equals_no().and(right.equals_no()),
            Node::Not(expression) => expression.equals_no().not(),
            Node::Symbol(id) => Self::compare(Compare::Equal, *id, NO),
            Node::Compare(..) => self.clone().not(),
        }
    }
    pub(crate) fn rewrite_modules(&self, modules: SymbolId) -> Self {
        match self.0.as_ref() {
            Node::Symbol(MOD) => self.clone().and(Self::symbol(modules)),
            Node::And(left, right) => left
                .rewrite_modules(modules)
                .and(right.rewrite_modules(modules)),
            Node::Or(left, right) => left
                .rewrite_modules(modules)
                .or(right.rewrite_modules(modules)),
            Node::Not(expression) => expression.rewrite_modules(modules).not(),
            _ => self.clone(),
        }
    }
    pub(crate) fn symbols(&self, output: &mut Vec<SymbolId>) {
        match self.0.as_ref() {
            Node::Symbol(id) => {
                if *id > YES && !output.contains(id) {
                    output.push(*id);
                }
            }
            Node::Compare(_, left, right) => {
                for id in [left, right] {
                    if *id > YES && !output.contains(id) {
                        output.push(*id);
                    }
                }
            }
            Node::And(left, right) | Node::Or(left, right) => {
                left.symbols(output);
                right.symbols(output);
            }
            Node::Not(expression) => expression.symbols(output),
        }
    }

    pub(crate) fn contains(&self, symbol: SymbolId) -> bool {
        match self.0.as_ref() {
            Node::Symbol(id) => *id == symbol,
            Node::Compare(_, left, right) => *left == symbol || *right == symbol,
            Node::And(left, right) | Node::Or(left, right) => {
                left.contains(symbol) || right.contains(symbol)
            }
            Node::Not(expression) => expression.contains(symbol),
        }
    }

    pub(crate) fn equivalent(&self, other: &Expr) -> bool {
        if self == other {
            return true;
        }
        let conjunction = match (self.0.as_ref(), other.0.as_ref()) {
            (Node::And(..), Node::And(..)) => true,
            (Node::Or(..), Node::Or(..)) => false,
            (Node::Not(left), Node::Not(right)) => return left.equivalent(right),
            _ => return false,
        };
        let mut left = Vec::new();
        let mut right = Vec::new();
        self.terms(conjunction, &mut left);
        other.terms(conjunction, &mut right);
        if left.len() != right.len() {
            return false;
        }
        for term in left {
            let Some(index) = right.iter().position(|other| term.equivalent(other)) else {
                return false;
            };
            right.remove(index);
        }
        true
    }

    pub(crate) fn depends_on(&self, symbol: SymbolId) -> bool {
        match self.0.as_ref() {
            Node::Symbol(id) => *id == symbol,
            Node::And(left, right) => left.depends_on(symbol) || right.depends_on(symbol),
            Node::Compare(Compare::Equal, id, MOD | YES)
            | Node::Compare(Compare::Unequal, id, NO) => *id == symbol,
            _ => false,
        }
    }

    pub(crate) fn contains_negated(&self, symbol: SymbolId) -> bool {
        match self.0.as_ref() {
            Node::And(left, right) | Node::Or(left, right) => {
                left.contains_negated(symbol) || right.contains_negated(symbol)
            }
            Node::Not(expression) => expression.as_symbol() == Some(symbol),
            Node::Compare(Compare::Equal, id, NO)
            | Node::Compare(Compare::Unequal, id, MOD | YES) => *id == symbol,
            _ => false,
        }
    }

    pub(crate) fn terms(&self, conjunction: bool, output: &mut Vec<Expr>) {
        match self.0.as_ref() {
            Node::And(left, right) if conjunction => {
                left.terms(conjunction, output);
                right.terms(conjunction, output);
            }
            Node::Or(left, right) if !conjunction => {
                left.terms(conjunction, output);
                right.terms(conjunction, output);
            }
            _ => output.push(self.clone()),
        }
    }
}

impl Kconfig {
    pub(crate) fn simplify(&self, expression: &Expr) -> Expr {
        match expression.0.as_ref() {
            Node::Symbol(_) => expression.clone(),
            Node::Compare(comparison, symbol, value)
                if self.symbols[*symbol].kind == SymbolType::Boolean =>
            {
                match (comparison, value) {
                    (Compare::Equal, &NO) | (Compare::Unequal, &YES) => Expr::symbol(*symbol).not(),
                    (Compare::Equal, &YES) | (Compare::Unequal, &NO) => Expr::symbol(*symbol),
                    (Compare::Equal, &MOD) => {
                        println!(
                            "boolean symbol {} tested for 'm'? test forced to 'n'",
                            self.symbols[*symbol].display_name()
                        );
                        Expr::no()
                    }
                    (Compare::Unequal, &MOD) => {
                        println!(
                            "boolean symbol {} tested for 'm'? test forced to 'y'",
                            self.symbols[*symbol].display_name()
                        );
                        Expr::yes()
                    }
                    _ => expression.clone(),
                }
            }
            Node::Compare(..) => expression.clone(),
            Node::Not(expression) => {
                let inner = self.simplify(expression);
                match inner.0.as_ref() {
                    Node::And(left, right) => {
                        self.simplify(&left.clone().not().or(right.clone().not()))
                    }
                    Node::Or(left, right) => {
                        self.simplify(&left.clone().not().and(right.clone().not()))
                    }
                    Node::Compare(kind, left, right) => Expr::compare(
                        match kind {
                            Compare::Equal => Compare::Unequal,
                            Compare::Unequal => Compare::Equal,
                            Compare::Less => Compare::GreaterEqual,
                            Compare::LessEqual => Compare::Greater,
                            Compare::Greater => Compare::LessEqual,
                            Compare::GreaterEqual => Compare::Less,
                        },
                        *left,
                        *right,
                    ),
                    _ => inner.not(),
                }
            }
            Node::And(left, right) | Node::Or(left, right) => {
                let conjunction = matches!(expression.0.as_ref(), Node::And(..));
                let mut terms = Vec::new();
                self.simplify(left).terms(conjunction, &mut terms);
                self.simplify(right).terms(conjunction, &mut terms);
                let mut changed = true;
                while changed {
                    changed = false;
                    'outer: for i in 0..terms.len() {
                        for j in i + 1..terms.len() {
                            if let Some(joined) = self.join_terms(&terms[i], &terms[j], conjunction)
                            {
                                terms[j] = joined;
                                terms.remove(i);
                                changed = true;
                                break 'outer;
                            }
                        }
                    }
                }
                terms.into_iter().fold(
                    if conjunction { Expr::yes() } else { Expr::no() },
                    |result, term| {
                        if conjunction {
                            result.and(term)
                        } else {
                            result.or(term)
                        }
                    },
                )
            }
        }
    }

    fn join_terms(&self, left: &Expr, right: &Expr, conjunction: bool) -> Option<Expr> {
        if left.equivalent(right) {
            return Some(left.clone());
        }
        let (ls, lk, lv) = atom(left)?;
        let (rs, rk, rv) = atom(right)?;
        if ls != rs
            || !matches!(
                self.symbols[ls].kind,
                SymbolType::Boolean | SymbolType::Tristate
            )
        {
            return None;
        }
        let boolean = self.symbols[ls].kind == SymbolType::Boolean;
        if !conjunction {
            if boolean && ((lk == 0 && rk == 1) || (lk == 1 && rk == 0)) {
                return Some(Expr::yes());
            }
            if !boolean && lk == 2 && rk == 2 && lv <= YES && rv <= YES && lv != rv {
                return Some(Expr::compare(
                    Compare::Unequal,
                    ls,
                    NO + MOD + YES - lv - rv,
                ));
            }
            return None;
        }
        for (kind, other_kind, other_value) in [(lk, rk, rv), (rk, lk, lv)] {
            if kind == 0 {
                if (other_kind == 2 && other_value == YES)
                    || (other_kind == 3 && other_value == MOD)
                {
                    return Some(Expr::compare(Compare::Equal, ls, YES));
                }
                if other_kind == 3 && other_value == NO {
                    return Some(Expr::symbol(ls));
                }
            }
        }
        if !boolean {
            if (lk == 2 && rk == 3) || (lk == 3 && rk == 2) {
                if self.symbols[lv].constant && self.symbols[rv].constant {
                    return Some(if lv == rv {
                        Expr::no()
                    } else if lk == 2 {
                        left.clone()
                    } else {
                        right.clone()
                    });
                }
            }
            if lk == 3 && rk == 3 && lv <= YES && rv <= YES && lv != rv {
                return Some(Expr::compare(Compare::Equal, ls, NO + MOD + YES - lv - rv));
            }
        }
        None
    }
}

// symbol, operation (bare/not/equal/unequal), comparison value.
fn atom(expression: &Expr) -> Option<(SymbolId, u8, SymbolId)> {
    match expression.0.as_ref() {
        Node::Symbol(symbol) => Some((*symbol, 0, NO)),
        Node::Not(inner) => inner.as_symbol().map(|symbol| (symbol, 1, NO)),
        Node::Compare(Compare::Equal, symbol, value) => Some((*symbol, 2, *value)),
        Node::Compare(Compare::Unequal, symbol, value) => Some((*symbol, 3, *value)),
        _ => None,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
