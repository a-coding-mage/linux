// SPDX-License-Identifier: GPL-2.0-or-later
// C global declaration parser for genksyms.
// Copyright 1996, 1997 Linux International.
// Original implementation: Richard Henderson and Bjorn Ekwall.

use super::lexer::{Lexer, Link, Node};
use super::parser_tables::*;
use super::{Kind, Symbols};

#[derive(Default)]
struct Declaration {
    typedef: bool,
    external: bool,
    name: Option<Vec<u8>>,
    specifiers: Link,
}

impl Declaration {
    fn record(&self, head: Link, lexer: &Lexer<'_>, symbols: &mut Symbols, kind: Kind) {
        symbols.add(
            self.name.clone(),
            kind,
            lexer.arena.range(head, None),
            self.external,
            false,
        );
    }
    fn compound(
        &self,
        keyword: Link,
        ident: Link,
        body: Link,
        kind: Kind,
        lexer: &mut Lexer<'_>,
        symbols: &mut Symbols,
    ) {
        let arena = &mut lexer.arena;
        let Some(identifier) = arena.get(ident) else {
            return;
        };
        let start = arena.get(body);
        if arena.nodes[identifier].source {
            arena.remove(keyword);
            arena.tag(ident, kind);
            arena.set(body, arena.get(ident));
        } else {
            let mut word = arena.nodes[identifier].word.clone();
            let name = word.text.clone();
            word.kind = kind;
            let keyword_node = arena.get(keyword);
            let index = arena.nodes.len();
            arena.nodes.push(Node {
                word,
                source: false,
                next: arena.get(keyword_node),
            });
            arena.set(body, Some(index));
            arena.set(keyword_node, None);
            symbols.add(
                Some(name),
                kind,
                arena.range(start, None),
                self.external,
                false,
            );
        }
    }
    fn reduce(
        &mut self,
        rule: usize,
        values: &[Link],
        lexer: &mut Lexer<'_>,
        symbols: &mut Symbols,
    ) -> Result<Link, ()> {
        let at = |number: usize| values.get(number - 1).copied().flatten();
        let mut result = at(1);
        match rule {
            4 => *self = Self::default(),
            5 => lexer.arena.set(at(2), None),
            6 | 8 => self.typedef = true,
            7 => result = at(4),
            9 => result = at(3),
            15 | 16 => result = at(2),
            17 => {
                if self.name.is_some() {
                    let semicolon = lexer.arena.get(at(3));
                    let head = lexer.arena.get(semicolon);
                    lexer.arena.set(semicolon, None);
                    self.record(
                        head,
                        lexer,
                        symbols,
                        if self.typedef {
                            Kind::Typedef
                        } else {
                            Kind::Normal
                        },
                    );
                    self.name = None;
                }
                result = at(3);
                lexer.dont_want_type = false;
            }
            18 | 62 | 91 | 96 | 112 | 117 | 123 | 131 | 140 => result = None,
            20 | 21 => {
                let last = if rule == 20 { at(1) } else { at(4) };
                let head = lexer.arena.get(last);
                lexer.arena.set(last, None);
                if rule == 21 {
                    lexer.arena.set(at(2), self.specifiers);
                }
                self.specifiers = lexer.arena.copy(self.specifiers);
                self.record(
                    head,
                    lexer,
                    symbols,
                    if self.typedef {
                        Kind::Typedef
                    } else {
                        Kind::Normal
                    },
                );
                self.name = None;
                result = last;
                lexer.dont_want_type = true;
            }
            22 => result = at(4).or(at(3)).or(at(2)).or(at(1)),
            23 => self.specifiers = None,
            25..=27 => self.specifiers = lexer.arena.get(at(2)),
            28 | 71 => lexer.arena.remove(at(1)),
            29 => lexer.dont_want_type = true,
            34 => self.external = true,
            35 => self.external = false,
            39 | 40 => {
                lexer.arena.remove(at(1));
                lexer.arena.tag(
                    at(3),
                    if rule == 39 {
                        Kind::Struct
                    } else {
                        Kind::Union
                    },
                );
                result = at(3);
            }
            41 => {
                lexer.arena.remove(at(1));
                lexer.arena.tag(at(2), Kind::Enum);
                result = at(2);
            }
            42 | 43 => {
                self.compound(
                    at(1),
                    at(3),
                    at(4),
                    if rule == 42 {
                        Kind::Struct
                    } else {
                        Kind::Union
                    },
                    lexer,
                    symbols,
                );
                result = at(4);
            }
            44 => {
                self.compound(at(1), at(2), at(3), Kind::Enum, lexer, symbols);
                result = at(3);
            }
            45 => {
                symbols.add(None, Kind::Enum, Vec::new(), false, false);
                result = at(2);
            }
            46 | 47 => result = at(3),
            60 => lexer.arena.tag(at(1), Kind::Typedef),
            61 | 95 | 99 | 127 => result = at(2).or(at(1)),
            66 | 67 | 72 | 77 | 79 | 85 | 88 | 106 | 120 | 128 | 130 | 132 | 139 | 143 => {
                result = at(2)
            }
            74 => {
                if self.name.is_some() {
                    symbols.error(b"unexpected second declaration name");
                    return Err(());
                }
                self.name = lexer.arena.name(at(1));
                lexer.dont_want_type = false;
            }
            75 | 76 | 78 | 82 | 84 | 86 | 102 | 105 | 107 | 134 => result = at(4),
            83 | 93 | 110 => lexer.dont_want_type = false,
            87 | 90 | 103 | 108 | 115 | 116 | 133 => result = at(3),
            94 | 121 => {
                result = at(3);
                lexer.dont_want_type = false;
            }
            100 | 122 => {
                result = at(2);
                lexer.dont_want_type = false;
            }
            104 => lexer.arena.remove(at(1)),
            111 => {
                let head = lexer.arena.get(at(2));
                lexer.arena.set(at(2), None);
                self.record(head, lexer, symbols, Kind::Normal);
                result = at(3);
            }
            114 => {
                let equal = lexer.arena.get(at(1));
                lexer.arena.set(at(2), lexer.arena.get(equal));
                result = at(2);
            }
            125 => lexer.dont_want_type = true,
            126 => {
                result = at(3);
                lexer.dont_want_type = true;
            }
            137 => {
                symbols.add(
                    lexer.arena.name(at(1)),
                    Kind::EnumConst,
                    Vec::new(),
                    false,
                    false,
                );
            }
            138 => {
                let expression = lexer
                    .arena
                    .range(lexer.arena.get(at(3)), lexer.arena.get(at(2)));
                symbols.add(
                    lexer.arena.name(at(1)),
                    Kind::EnumConst,
                    expression,
                    false,
                    false,
                );
            }
            142 => {
                if let Some(name) = lexer.arena.name(at(3)) {
                    symbols.export(&name);
                }
                result = at(5);
            }
            _ => {}
        }
        Ok(result)
    }
}

fn symbol_trace(symbols: &mut Symbols, prefix: &str, symbol: i16) {
    if symbols.debug > 1 {
        let category = if symbol < YYNTOKENS { "token" } else { "nterm" };
        symbols.stderr.extend_from_slice(
            format!("{prefix} {category} {} ()\n", NAMES[symbol as usize]).as_bytes(),
        );
    }
}

fn stack_trace(symbols: &mut Symbols, states: &[i16]) {
    if symbols.debug > 1 {
        symbols.stderr.extend_from_slice(b"Stack now");
        for state in states {
            symbols
                .stderr
                .extend_from_slice(format!(" {state}").as_bytes());
        }
        symbols.stderr.push(b'\n');
    }
}

fn enter(symbols: &mut Symbols, states: &[i16]) {
    if symbols.debug > 1 {
        symbols.stderr.extend_from_slice(
            format!("Entering state {}\n", states.last().unwrap_or(&0)).as_bytes(),
        );
    }
    stack_trace(symbols, states);
}

fn table_action(state: i16, token: i16) -> Option<i16> {
    let base = YYPACT[state as usize];
    if base == YYPACT_NINF {
        return None;
    }
    let index = base + token;
    (0..=YYLAST)
        .contains(&index)
        .then_some(index as usize)
        .filter(|&index| YYCHECK[index] == token)
        .map(|index| YYTABLE[index])
}

/// Execute the original LALR grammar using stable token links and owned state.
pub(super) fn parse(input: &[u8], symbols: &mut Symbols) {
    let mut lexer = Lexer::new(input);
    let mut declaration = Declaration::default();
    let mut states = vec![0i16];
    let mut values = vec![None];
    let mut lookahead: Option<(i16, Link)> = None;
    let mut last_value = None;
    let mut error_status = 0u8;
    let mut capacity = 200;
    if symbols.debug > 1 {
        symbols.stderr.extend_from_slice(b"Starting parse\n");
    }
    enter(symbols, &states);
    'parse: loop {
        let state = *states.last().unwrap_or(&0);
        if states.len() >= capacity {
            if capacity == 10000 {
                symbols.error(b"memory exhausted");
                break;
            }
            capacity = (capacity * 2).min(10000);
            if symbols.debug > 1 {
                symbols
                    .stderr
                    .extend_from_slice(format!("Stack size increased to {capacity}\n").as_bytes());
            }
        }
        if state == YYFINAL {
            break;
        }
        let mut action = None;
        if YYPACT[state as usize] != YYPACT_NINF {
            if lookahead.is_none() {
                if symbols.debug > 1 {
                    symbols.stderr.extend_from_slice(b"Reading a token\n");
                }
                let (raw, value) = lexer.next(symbols);
                let token = if raw <= 0 {
                    0
                } else {
                    YYTRANSLATE.get(raw as usize).copied().unwrap_or(2)
                };
                lookahead = Some((token, value));
                last_value = value;
            }
            let token = lookahead.unwrap().0;
            if token == 0 {
                if symbols.debug > 1 {
                    symbols.stderr.extend_from_slice(b"Now at end of input.\n");
                }
            } else {
                symbol_trace(symbols, "Next token is", token);
            }
            action = table_action(state, token);
        }
        let action = action.unwrap_or(-YYDEFACT[state as usize]);
        if action > 0 {
            error_status = error_status.saturating_sub(1);
            let (token, value) = lookahead.take().unwrap();
            symbol_trace(symbols, "Shifting", token);
            states.push(action);
            values.push(value);
            enter(symbols, &states);
            continue;
        }
        let mut explicit_error = false;
        if action < 0 {
            let rule = (-action) as usize;
            let length = YYR2[rule] as usize;
            if symbols.debug > 1 {
                symbols.stderr.extend_from_slice(
                    format!(
                        "Reducing stack by rule {} (line {}):\n",
                        rule - 1,
                        YYRLINE[rule]
                    )
                    .as_bytes(),
                );
                for (offset, &state) in states[states.len() - length..].iter().enumerate() {
                    symbol_trace(
                        symbols,
                        &format!("   ${} =", offset + 1),
                        YYSTOS[state as usize],
                    );
                }
            }
            let value =
                declaration.reduce(rule, &values[values.len() - length..], &mut lexer, symbols);
            states.truncate(states.len() - length);
            values.truncate(values.len() - length);
            match value {
                Ok(value) => {
                    symbol_trace(symbols, "-> $$ =", YYR1[rule]);
                    let previous = *states.last().unwrap_or(&0);
                    let lhs = (YYR1[rule] - YYNTOKENS) as usize;
                    let index = YYPGOTO[lhs] + previous;
                    let next =
                        if (0..=YYLAST).contains(&index) && YYCHECK[index as usize] == previous {
                            YYTABLE[index as usize]
                        } else {
                            YYDEFGOTO[lhs]
                        };
                    states.push(next);
                    values.push(value);
                    enter(symbols, &states);
                    continue;
                }
                Err(()) => {
                    explicit_error = true;
                    stack_trace(symbols, &states);
                }
            }
        }
        if !explicit_error {
            if error_status == 0 {
                symbols.error(b"syntax error");
            }
            if error_status == 3 {
                if lookahead.is_some_and(|(token, _)| token == 0) {
                    break;
                }
                if let Some((token, _)) = lookahead.take() {
                    symbol_trace(symbols, "Error: discarding", token);
                }
            }
        }
        error_status = 3;
        loop {
            let state = *states.last().unwrap_or(&0);
            if let Some(next) = table_action(state, 1).filter(|&next| next > 0) {
                symbol_trace(symbols, "Shifting", YYSTOS[next as usize]);
                states.push(next);
                values.push(last_value);
                enter(symbols, &states);
                continue 'parse;
            }
            if states.len() == 1 {
                break 'parse;
            }
            symbol_trace(symbols, "Error: popping", YYSTOS[state as usize]);
            states.pop();
            values.pop();
            stack_trace(symbols, &states);
        }
    }
    if let Some((token, _)) = lookahead {
        symbol_trace(symbols, "Cleanup: discarding lookahead", token);
    }
    stack_trace(symbols, &states);
    while states.len() > 1 {
        let state = states.pop().unwrap();
        symbol_trace(symbols, "Cleanup: popping", YYSTOS[state as usize]);
    }
}
