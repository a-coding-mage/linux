// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2002 Roman Zippel <zippel@linux-m68k.org>
//! Parse Kconfig statements and expressions into the shared owned model.

#[path = "lexer.rs"]
mod lexer;

use crate::expr::{Compare, Expr};
use crate::model::{Kconfig, Location, MenuId, MenuType, PropertyKind, SymbolId, SymbolType, NO};
use crate::preprocess::Flavor;
use lexer::{Kind, Lexer, Statement, Token};

#[derive(Clone, Copy)]
enum EntryKind {
    Config,
    MenuConfig,
    Choice,
    Menu,
    Comment,
}

struct Parser {
    model: Kconfig,
    lexer: Lexer,
    blocks: Vec<MenuId>,
    entry: Option<(MenuId, EntryKind)>,
    last: Location,
    seen_statement: bool,
}

struct Cursor<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl Cursor<'_> {
    fn peek(&self, value: &str) -> bool {
        self.tokens.get(self.position).is_some_and(|token| {
            token.text == value && matches!(token.kind, Kind::Keyword | Kind::Operator)
        })
    }
    fn take(&mut self, kind: Kind) -> Result<String, ()> {
        let token = self.tokens.get(self.position).ok_or(())?;
        if token.kind != kind {
            return Err(());
        }
        self.position += 1;
        Ok(token.text.clone())
    }
    fn done(&self) -> Result<(), ()> {
        if self.position == self.tokens.len() {
            Ok(())
        } else {
            Err(())
        }
    }
    fn symbol(&mut self, model: &mut Kconfig, constant: bool) -> Result<SymbolId, ()> {
        let token = self.tokens.get(self.position).ok_or(())?;
        if token.kind != Kind::Word && !(constant && token.kind == Kind::Quoted) {
            return Err(());
        }
        self.position += 1;
        Ok(model.lookup(&token.text, token.kind == Kind::Quoted))
    }
    fn expression(&mut self, model: &mut Kconfig, minimum: usize) -> Result<Expr, ()> {
        let mut raw_symbol = false;
        let mut left = if self.peek("!") {
            self.position += 1;
            // Comparisons require bare symbol operands in the grammar, so a
            // comparison after '!' belongs inside the negated expression.
            self.expression(model, 3)?.not()
        } else if self.peek("(") {
            self.position += 1;
            let expression = self.expression(model, 0)?;
            if !self.peek(")") {
                return Err(());
            }
            self.position += 1;
            expression
        } else {
            raw_symbol = true;
            Expr::symbol(self.symbol(model, true)?)
        };
        loop {
            let Some(token) = self.tokens.get(self.position) else {
                break;
            };
            if token.kind != Kind::Operator {
                break;
            }
            let (precedence, comparison) = match token.text.as_str() {
                "||" => (1, None),
                "&&" => (2, None),
                "=" => (3, Some(Compare::Equal)),
                "!=" => (3, Some(Compare::Unequal)),
                "<" => (4, Some(Compare::Less)),
                "<=" => (4, Some(Compare::LessEqual)),
                ">" => (4, Some(Compare::Greater)),
                ">=" => (4, Some(Compare::GreaterEqual)),
                _ => break,
            };
            if precedence < minimum {
                break;
            }
            self.position += 1;
            left = if let Some(comparison) = comparison {
                if !raw_symbol {
                    return Err(());
                }
                let right = self.symbol(model, true)?;
                Expr::compare(comparison, left.as_symbol().ok_or(())?, right)
            } else {
                let right = self.expression(model, precedence + 1)?;
                if precedence == 1 {
                    left.or(right)
                } else {
                    left.and(right)
                }
            };
            raw_symbol = false;
        }
        Ok(left)
    }
    fn condition(&mut self, model: &mut Kconfig) -> Result<Expr, ()> {
        if self.peek("if") {
            self.position += 1;
            self.expression(model, 0)
        } else {
            Ok(Expr::yes())
        }
    }
}

impl Parser {
    fn choice(&self) -> Option<MenuId> {
        self.blocks
            .iter()
            .rev()
            .copied()
            .find(|&id| self.model.menus[id].kind == MenuType::Choice)
    }
    fn prompt(&mut self, menu: MenuId, text: String, condition: Expr, location: &Location) {
        let trimmed = text.trim_start_matches(|c: char| c.is_ascii_whitespace() || c == '\u{b}');
        if trimmed.len() != text.len() {
            self.model.warning(location, "leading whitespace ignored");
        }
        if self.model.menus[menu].prompt.is_some() {
            self.model.warning(location, "prompt redefined");
        }
        self.model.add_property(
            menu,
            PropertyKind::Prompt(trimmed.into()),
            condition,
            location.clone(),
        );
    }
    fn finish_entry(&mut self, next: &Location) {
        let Some((menu, kind)) = self.entry.take() else {
            return;
        };
        let location = self.model.menus[menu].location.clone();
        match kind {
            EntryKind::Config => {
                if let Some(choice) = self.choice() {
                    let symbol = self.model.menus[menu].symbol.unwrap();
                    if self.model.menus[menu].prompt.is_none() {
                        self.model
                            .error(&location, "error: choice member must have a prompt");
                    }
                    if self.model.symbols[symbol].kind != SymbolType::Boolean {
                        self.model
                            .error(&location, "error: choice member must be bool");
                    }
                    self.model.menus[choice].members.push(symbol);
                    self.model.symbols[symbol].choice = Some(choice);
                }
            }
            EntryKind::MenuConfig => {
                if self.model.menus[menu].prompt.is_none() {
                    self.model
                        .error(next, "menuconfig statement without prompt");
                }
            }
            EntryKind::Choice => {
                if self.model.menus[menu].prompt.is_none() {
                    self.model
                        .error(&location, "error: choice must have a prompt");
                }
            }
            _ => {}
        }
    }
    fn syntax(&mut self, statement: &Statement, unknown: bool) {
        self.model.error(&statement.location, "syntax error");
        let diagnostic = if unknown {
            format!("unknown statement \"{}\"", statement.tokens[0].text)
        } else {
            "invalid statement".into()
        };
        self.model.error(&statement.location, diagnostic);
    }
    fn option(&mut self, statement: &Statement, menu: MenuId, kind: EntryKind) -> Result<(), ()> {
        let name = statement.tokens[0].text.as_str();
        let location = &statement.location;
        let mut cursor = Cursor {
            tokens: &statement.tokens,
            position: 1,
        };
        let config = matches!(kind, EntryKind::Config | EntryKind::MenuConfig);
        let choice = matches!(kind, EntryKind::Choice);
        match name {
            "bool" | "tristate" | "int" | "hex" | "string" if config => {
                let symbol_type = match name {
                    "bool" => SymbolType::Boolean,
                    "tristate" => SymbolType::Tristate,
                    "int" => SymbolType::Int,
                    "hex" => SymbolType::Hex,
                    _ => SymbolType::String,
                };
                let prompt = if cursor.position < cursor.tokens.len() {
                    let text = cursor.take(Kind::Quoted)?;
                    let condition = cursor.condition(&mut self.model)?;
                    Some((text, condition))
                } else {
                    None
                };
                cursor.done()?;
                if let Some((text, condition)) = prompt {
                    self.prompt(menu, text, condition, location);
                }
                self.model.set_type(menu, symbol_type);
            }
            "prompt" if config || choice => {
                let text = cursor.take(Kind::Quoted)?;
                let condition = cursor.condition(&mut self.model)?;
                cursor.done()?;
                self.prompt(menu, text, condition, location);
            }
            "transitional" if config => {
                cursor.done()?;
                let symbol = self.model.menus[menu].symbol.unwrap();
                self.model.symbols[symbol].transitional = true;
            }
            "default" | "def_bool" | "def_tristate" if config || (choice && name == "default") => {
                let expression = if choice {
                    Expr::symbol(cursor.symbol(&mut self.model, false)?)
                } else {
                    cursor.expression(&mut self.model, 0)?
                };
                let condition = cursor.condition(&mut self.model)?;
                cursor.done()?;
                self.model.add_property(
                    menu,
                    PropertyKind::Default(expression),
                    condition,
                    location.clone(),
                );
                if name == "def_bool" {
                    self.model.set_type(menu, SymbolType::Boolean);
                }
                if name == "def_tristate" {
                    self.model.set_type(menu, SymbolType::Tristate);
                }
            }
            "select" | "imply" if config => {
                let symbol = cursor.symbol(&mut self.model, false)?;
                let condition = cursor.condition(&mut self.model)?;
                cursor.done()?;
                let property = if name == "select" {
                    PropertyKind::Select(symbol)
                } else {
                    PropertyKind::Imply(symbol)
                };
                self.model
                    .add_property(menu, property, condition, location.clone());
            }
            "range" if config => {
                let low = cursor.symbol(&mut self.model, true)?;
                let high = cursor.symbol(&mut self.model, true)?;
                let condition = cursor.condition(&mut self.model)?;
                cursor.done()?;
                self.model.add_property(
                    menu,
                    PropertyKind::Range(low, high),
                    condition,
                    location.clone(),
                );
            }
            "modules" if config => {
                cursor.done()?;
                let symbol = self.model.menus[menu].symbol.unwrap();
                if self.model.modules != NO {
                    self.model.error(
                        location,
                        format!(
                            "symbol '{}' redefines option 'modules' already defined by symbol '{}'",
                            self.model.symbols[symbol].display_name(),
                            self.model.symbols[self.model.modules].display_name()
                        ),
                    );
                }
                self.model.modules = symbol;
            }
            "depends" => {
                if !cursor.peek("on") {
                    return Err(());
                }
                cursor.position += 1;
                let mut dependency = cursor.expression(&mut self.model, 0)?;
                if cursor.peek("if") {
                    cursor.position += 1;
                    dependency = dependency.or(cursor.expression(&mut self.model, 0)?.equals_no());
                }
                cursor.done()?;
                self.model.menus[menu].dependency =
                    self.model.menus[menu].dependency.clone().and(dependency);
            }
            "visible" if matches!(kind, EntryKind::Menu) => {
                let condition = cursor.condition(&mut self.model)?;
                cursor.done()?;
                self.model.menus[menu].visibility =
                    self.model.menus[menu].visibility.clone().and(condition);
            }
            "help" if config || choice => {
                cursor.done()?;
                let text = self.lexer.help();
                let location = &self.lexer.location();
                let symbol = self.model.menus[menu].symbol.unwrap();
                let symbol_name = self.model.symbols[symbol].display_name().to_owned();
                if self.model.menus[menu].help.is_some() {
                    self.model.error(
                        location,
                        format!("'{symbol_name}' defined with more than one help text"),
                    );
                }
                if text
                    .chars()
                    .all(|c| c.is_ascii_whitespace() || c == '\u{b}')
                {
                    self.model.error(
                        location,
                        format!("'{symbol_name}' defined with blank help text"),
                    );
                }
                self.model.menus[menu].help = Some(text);
            }
            _ => return Err(()),
        }
        Ok(())
    }
    fn statement(&mut self, statement: &Statement) -> Result<(), String> {
        let name = statement.tokens[0].text.as_str();
        let keyword = statement.tokens[0].kind == Kind::Keyword;
        let option = keyword
            && matches!(
                name,
                "bool"
                    | "tristate"
                    | "int"
                    | "hex"
                    | "string"
                    | "prompt"
                    | "transitional"
                    | "default"
                    | "def_bool"
                    | "def_tristate"
                    | "select"
                    | "imply"
                    | "range"
                    | "modules"
                    | "depends"
                    | "visible"
                    | "help"
            );
        if option {
            if let Some((menu, kind)) = self.entry {
                if self.option(statement, menu, kind).is_ok() {
                    return Ok(());
                }
            }
            self.finish_entry(&statement.location);
            self.syntax(statement, false);
            return Ok(());
        }
        self.finish_entry(&statement.location);
        let location = &statement.location;
        let in_choice = self.choice().is_some();
        let mut cursor = Cursor {
            tokens: &statement.tokens,
            position: 1,
        };
        if statement.tokens[0].kind == Kind::Word {
            if !in_choice
                && statement.tokens.len() == 3
                && statement.tokens[2].kind == Kind::Assignment
            {
                let flavor = match statement.tokens[1].text.as_str() {
                    ":=" => Flavor::Simple,
                    "+=" => Flavor::Append,
                    _ => Flavor::Recursive,
                };
                self.lexer.preprocessor.assign(
                    name,
                    &statement.tokens[2].text,
                    flavor,
                    &location.filename,
                    statement.end_line,
                )?;
            } else {
                self.syntax(statement, !in_choice);
            }
            self.seen_statement = true;
            return Ok(());
        }
        if !keyword
            || (in_choice
                && !matches!(
                    name,
                    "config" | "comment" | "if" | "endmenu" | "endchoice" | "endif"
                ))
        {
            self.syntax(statement, false);
            self.seen_statement = true;
            return Ok(());
        }
        let parent = *self.blocks.last().unwrap_or(&0);
        let result: Result<(), ()> = (|| {
            match name {
                "mainmenu" if !self.seen_statement && self.blocks.is_empty() => {
                    let text = cursor.take(Kind::Quoted)?;
                    cursor.done()?;
                    self.prompt(0, text, Expr::yes(), location);
                }
                "config" | "menuconfig" => {
                    let symbol = cursor.symbol(&mut self.model, false)?;
                    cursor.done()?;
                    let (menu_kind, entry_kind) = if name == "config" {
                        (MenuType::Normal, EntryKind::Config)
                    } else {
                        (MenuType::Menu, EntryKind::MenuConfig)
                    };
                    let menu =
                        self.model
                            .add_menu(parent, menu_kind, Some(symbol), location.clone());
                    self.entry = Some((menu, entry_kind));
                }
                "menu" | "comment" => {
                    let text = cursor.take(Kind::Quoted)?;
                    cursor.done()?;
                    let (menu_kind, entry_kind) = if name == "menu" {
                        (MenuType::Menu, EntryKind::Menu)
                    } else {
                        (MenuType::Comment, EntryKind::Comment)
                    };
                    let menu = self
                        .model
                        .add_menu(parent, menu_kind, None, location.clone());
                    self.prompt(menu, text, Expr::yes(), location);
                    self.entry = Some((menu, entry_kind));
                    if name == "menu" {
                        self.blocks.push(menu);
                    }
                }
                "choice" => {
                    cursor.done()?;
                    let symbol = self.model.anonymous_symbol();
                    let menu = self.model.add_menu(
                        parent,
                        MenuType::Choice,
                        Some(symbol),
                        location.clone(),
                    );
                    self.model.set_type(menu, SymbolType::Boolean);
                    self.model.symbols[symbol].choice_menu = Some(menu);
                    self.entry = Some((menu, EntryKind::Choice));
                    self.blocks.push(menu);
                }
                "if" => {
                    let dependency = cursor.expression(&mut self.model, 0)?;
                    cursor.done()?;
                    let menu = self
                        .model
                        .add_menu(parent, MenuType::If, None, location.clone());
                    self.model.menus[menu].dependency = dependency;
                    self.blocks.push(menu);
                }
                "endmenu" | "endchoice" | "endif" => {
                    cursor.done()?;
                    let Some(menu) = self.blocks.last().copied() else {
                        return Err(());
                    };
                    let expected = match self.model.menus[menu].kind {
                        MenuType::Choice => "choice",
                        MenuType::If => "if",
                        _ => "menu",
                    };
                    let actual = &name[3..];
                    if expected != actual {
                        self.model.error(
                            location,
                            format!("unexpected '{actual}' within {expected} block"),
                        );
                    } else if self.model.menus[menu].location.filename != location.filename {
                        self.model.error(
                            location,
                            format!("'{actual}' in different file than '{expected}'"),
                        );
                        eprintln!(
                            "{}: location of the '{expected}'",
                            self.model.menus[menu].location
                        );
                    }
                    self.blocks.pop();
                }
                "source" => {
                    cursor.take(Kind::Quoted)?;
                    cursor.done()?;
                }
                _ => return Err(()),
            }
            Ok(())
        })();
        if result.is_err() {
            self.syntax(statement, false);
        } else if name == "source" {
            self.lexer.include(&statement.tokens[1].text, location)?;
        }
        self.seen_statement = true;
        Ok(())
    }
    fn run(mut self) -> Result<Kconfig, String> {
        while let Some(statement) = self.lexer.next()? {
            self.last = statement.location.clone();
            self.statement(&statement)?;
        }
        self.last = self.lexer.location();
        self.finish_entry(&self.last.clone());
        if !self.blocks.is_empty() {
            self.model.error(&self.last.clone(), "syntax error");
            for &menu in self.blocks.iter().rev() {
                self.model.error(
                    &self.model.menus[menu].location.clone(),
                    "missing end statement for this entry",
                );
            }
        }
        self.model.files = self.lexer.files;
        self.model.environment = self.lexer.preprocessor.environment;
        self.model.warnings += self.lexer.warnings;
        if self.model.errors != 0 {
            return Err(String::new());
        }
        if self.model.menus[0].prompt.is_none() {
            self.model.add_property(
                0,
                PropertyKind::Prompt("Main menu".into()),
                Expr::yes(),
                self.last,
            );
        }
        Ok(self.model)
    }
}

pub(crate) fn parse(filename: &str) -> Result<Kconfig, String> {
    Parser {
        model: Kconfig::default(),
        lexer: Lexer::new(filename)?,
        blocks: Vec::new(),
        entry: None,
        last: Location {
            filename: filename.into(),
            line: 1,
        },
        seen_statement: false,
    }
    .run()
}
