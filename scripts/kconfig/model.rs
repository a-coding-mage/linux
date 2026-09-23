// SPDX-License-Identifier: GPL-2.0
//! Owned, shared data model for Kconfig parsing and evaluation.
// Original Kconfig implementation: Copyright (C) 2002 Roman Zippel.

use crate::expr::{Expr, Tristate};
use std::collections::HashMap;

pub(crate) type SymbolId = usize;
pub(crate) type MenuId = usize;
pub(crate) type PropertyId = usize;
pub(crate) const NO: SymbolId = 0;
pub(crate) const MOD: SymbolId = 1;
pub(crate) const YES: SymbolId = 2;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Location {
    pub(crate) filename: String,
    pub(crate) line: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}:{}", self.filename, self.line)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum SymbolType {
    #[default]
    Unknown,
    Boolean,
    Tristate,
    Int,
    Hex,
    String,
}

impl SymbolType {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Boolean => "bool",
            Self::Tristate => "tristate",
            Self::Int => "integer",
            Self::Hex => "hex",
            Self::String => "string",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct Value {
    pub(crate) text: String,
    pub(crate) tri: Tristate,
}

impl Value {
    pub(crate) fn tristate(value: Tristate) -> Self {
        Self {
            text: value.to_string(),
            tri: value,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Symbol {
    pub(crate) name: Option<String>,
    pub(crate) kind: SymbolType,
    pub(crate) constant: bool,
    pub(crate) transitional: bool,
    pub(crate) menus: Vec<MenuId>,
    pub(crate) properties: Vec<PropertyId>,
    pub(crate) choice: Option<MenuId>,
    pub(crate) choice_menu: Option<MenuId>,
    pub(crate) dependency: Expr,
    pub(crate) selected: Expr,
    pub(crate) implied: Expr,
    pub(crate) user: Option<Value>,
    pub(crate) automatic: Option<Value>,
    pub(crate) current: Value,
    pub(crate) visible: Tristate,
    pub(crate) direct_value: Tristate,
    pub(crate) selected_value: Tristate,
    pub(crate) implied_value: Tristate,
    pub(crate) valid: bool,
    pub(crate) write: bool,
}

impl Symbol {
    fn new(name: Option<String>, constant: bool) -> Self {
        Self {
            name,
            constant,
            kind: SymbolType::Unknown,
            transitional: false,
            menus: Vec::new(),
            properties: Vec::new(),
            choice: None,
            choice_menu: None,
            dependency: Expr::no(),
            selected: Expr::no(),
            implied: Expr::no(),
            user: None,
            automatic: None,
            current: Value::default(),
            visible: Tristate::No,
            direct_value: Tristate::Yes,
            selected_value: Tristate::No,
            implied_value: Tristate::No,
            valid: false,
            write: false,
        }
    }

    pub(crate) fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("<choice>")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MenuType {
    Choice,
    Comment,
    If,
    Menu,
    Normal,
}

#[derive(Clone, Debug)]
pub(crate) enum PropertyKind {
    Prompt(String),
    Default(Expr),
    Select(SymbolId),
    Imply(SymbolId),
    Range(SymbolId, SymbolId),
}

#[derive(Clone, Debug)]
pub(crate) struct Property {
    pub(crate) kind: PropertyKind,
    pub(crate) condition: Expr,
    pub(crate) menu: MenuId,
    pub(crate) location: Location,
}

#[derive(Clone, Debug)]
pub(crate) struct Menu {
    pub(crate) kind: MenuType,
    pub(crate) symbol: Option<SymbolId>,
    pub(crate) parent: Option<MenuId>,
    pub(crate) children: Vec<MenuId>,
    pub(crate) prompt: Option<PropertyId>,
    pub(crate) properties: Vec<PropertyId>,
    pub(crate) dependency: Expr,
    pub(crate) visibility: Expr,
    pub(crate) help: Option<String>,
    pub(crate) members: Vec<SymbolId>,
    pub(crate) location: Location,
}

pub(crate) struct Kconfig {
    pub(crate) symbols: Vec<Symbol>,
    pub(crate) names: HashMap<String, SymbolId>,
    constants: HashMap<String, SymbolId>,
    pub(crate) menus: Vec<Menu>,
    pub(crate) properties: Vec<Property>,
    pub(crate) modules: SymbolId,
    pub(crate) modules_value: Tristate,
    pub(crate) files: Vec<String>,
    pub(crate) environment: Vec<(String, String)>,
    pub(crate) errors: usize,
    pub(crate) warnings: usize,
    pub(crate) config_warnings: usize,
    pub(crate) dependency_warnings: usize,
    pub(crate) changed: bool,
}

impl Default for Kconfig {
    fn default() -> Self {
        let mut symbols = Vec::new();
        for (name, tri) in [
            ("n", Tristate::No),
            ("m", Tristate::Mod),
            ("y", Tristate::Yes),
        ] {
            let mut symbol = Symbol::new(Some(name.into()), true);
            symbol.kind = SymbolType::Tristate;
            symbol.current = Value::tristate(tri);
            symbol.valid = true;
            symbols.push(symbol);
        }
        Self {
            symbols,
            names: HashMap::new(),
            constants: HashMap::new(),
            menus: vec![Menu {
                kind: MenuType::Menu,
                symbol: None,
                parent: None,
                children: Vec::new(),
                prompt: None,
                properties: Vec::new(),
                dependency: Expr::yes(),
                visibility: Expr::yes(),
                help: None,
                members: Vec::new(),
                location: Location {
                    filename: String::new(),
                    line: 0,
                },
            }],
            properties: Vec::new(),
            modules: NO,
            modules_value: Tristate::No,
            files: Vec::new(),
            environment: Vec::new(),
            errors: 0,
            warnings: 0,
            config_warnings: 0,
            dependency_warnings: 0,
            changed: true,
        }
    }
}

impl Kconfig {
    pub(crate) fn lookup(&mut self, name: &str, quoted: bool) -> SymbolId {
        match name {
            "n" => return NO,
            "m" => return MOD,
            "y" => return YES,
            _ => {}
        }
        let map = if quoted {
            &mut self.constants
        } else {
            &mut self.names
        };
        if let Some(&id) = map.get(name) {
            return id;
        }
        let id = self.symbols.len();
        self.symbols.push(Symbol::new(Some(name.into()), quoted));
        map.insert(name.into(), id);
        id
    }

    pub(crate) fn anonymous_symbol(&mut self) -> SymbolId {
        let id = self.symbols.len();
        self.symbols.push(Symbol::new(None, false));
        id
    }

    pub(crate) fn add_menu(
        &mut self,
        parent: MenuId,
        kind: MenuType,
        symbol: Option<SymbolId>,
        location: Location,
    ) -> MenuId {
        let id = self.menus.len();
        self.menus.push(Menu {
            kind,
            symbol,
            parent: Some(parent),
            children: Vec::new(),
            prompt: None,
            properties: Vec::new(),
            dependency: Expr::yes(),
            visibility: Expr::yes(),
            help: None,
            members: Vec::new(),
            location,
        });
        self.menus[parent].children.push(id);
        if let Some(symbol) = symbol {
            self.symbols[symbol].menus.push(id);
        }
        id
    }

    pub(crate) fn add_property(
        &mut self,
        menu: MenuId,
        kind: PropertyKind,
        mut condition: Expr,
        location: Location,
    ) -> PropertyId {
        let id = self.properties.len();
        if let PropertyKind::Prompt(_) = &kind {
            let mut parent = if self.menus[menu].symbol.is_some() {
                self.menus[menu].parent
            } else {
                None
            };
            while let Some(ancestor) = parent {
                condition = condition.and(self.menus[ancestor].visibility.clone());
                parent = self.menus[ancestor].parent;
            }
            self.menus[menu].prompt = Some(id);
        }
        self.properties.push(Property {
            kind,
            condition,
            menu,
            location,
        });
        self.menus[menu].properties.push(id);
        if let Some(symbol) = self.menus[menu].symbol {
            self.symbols[symbol].properties.push(id);
        }
        id
    }

    pub(crate) fn error(&mut self, location: &Location, message: impl std::fmt::Display) {
        eprintln!("{location}: {message}");
        self.errors += 1;
    }

    pub(crate) fn warning(&mut self, location: &Location, message: impl std::fmt::Display) {
        eprintln!("{location}:warning: {message}");
        self.warnings += 1;
    }

    pub(crate) fn set_type(&mut self, menu: MenuId, kind: SymbolType) {
        let id = self.menus[menu].symbol.expect("type applies to symbols");
        let symbol = &mut self.symbols[id];
        if symbol.kind == SymbolType::Unknown {
            symbol.kind = kind;
        } else if symbol.kind != kind {
            let message = format!(
                "ignoring type redefinition of '{}' from '{}' to '{}'",
                symbol.display_name(),
                symbol.kind.name(),
                kind.name()
            );
            self.warning(&self.menus[menu].location.clone(), message);
        }
    }

    pub(crate) fn prompt(&self, menu: MenuId) -> Option<&str> {
        self.menus[menu]
            .prompt
            .and_then(|id| match &self.properties[id].kind {
                PropertyKind::Prompt(text) => Some(text.as_str()),
                _ => None,
            })
    }
}
