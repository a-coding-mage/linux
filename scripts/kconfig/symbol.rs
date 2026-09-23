// SPDX-License-Identifier: GPL-2.0
//! Kconfig symbol and choice evaluation.
// Original Kconfig implementation: Copyright (C) 2002 Roman Zippel.

use crate::expr::{Compare, Expr, Node, Tristate};
use crate::model::{Kconfig, MenuId, PropertyId, PropertyKind, SymbolId, SymbolType, Value};
use std::cmp::Ordering;

#[derive(Clone, Copy)]
enum Number {
    Signed(i64),
    Unsigned(u64),
}

fn parse_number(text: &str, kind: SymbolType) -> Option<Number> {
    if matches!(kind, SymbolType::Boolean | SymbolType::Tristate) {
        return Some(Number::Signed(match text {
            "n" => 0,
            "m" => 1,
            "y" => 2,
            _ => -1,
        }));
    }
    let text = text.trim_start_matches(|c: char| c.is_ascii_whitespace());
    let negative = text.starts_with('-');
    let unsigned = text.strip_prefix(['-', '+']).unwrap_or(text);
    let (radix, digits) = match kind {
        SymbolType::Int => (10, unsigned),
        SymbolType::Hex => (
            16,
            unsigned
                .strip_prefix("0x")
                .or_else(|| unsigned.strip_prefix("0X"))
                .unwrap_or(unsigned),
        ),
        _ if unsigned.starts_with("0x") || unsigned.starts_with("0X") => (16, &unsigned[2..]),
        _ if unsigned.starts_with('0') => (8, unsigned),
        _ => (10, unsigned),
    };
    if digits.is_empty() || !digits.bytes().all(|byte| (byte as char).is_digit(radix)) {
        return None;
    }
    let magnitude = u64::from_str_radix(digits, radix).ok()?;
    if kind == SymbolType::Hex {
        Some(Number::Unsigned(if negative {
            magnitude.wrapping_neg()
        } else {
            magnitude
        }))
    } else if negative && magnitude <= (1u64 << 63) {
        Some(Number::Signed((magnitude as i64).wrapping_neg()))
    } else if !negative && magnitude <= i64::MAX as u64 {
        Some(Number::Signed(magnitude as i64))
    } else {
        None
    }
}

fn compare_values(
    left: &Value,
    left_kind: SymbolType,
    right: &Value,
    right_kind: SymbolType,
) -> Ordering {
    if left_kind != SymbolType::String || right_kind != SymbolType::String {
        if let (Some(left), Some(right)) = (
            parse_number(&left.text, left_kind),
            parse_number(&right.text, right_kind),
        ) {
            return match (left, right) {
                (Number::Signed(left), Number::Signed(right)) => left.cmp(&right),
                (Number::Unsigned(left), Number::Unsigned(right)) => left.cmp(&right),
                (Number::Signed(left), Number::Unsigned(right)) => (left as u64).cmp(&right),
                (Number::Unsigned(left), Number::Signed(right)) => left.cmp(&(right as u64)),
            };
        }
    }
    left.text.cmp(&right.text)
}

impl Kconfig {
    pub(crate) fn effective_type(&self, symbol: SymbolId) -> SymbolType {
        match self.symbols[symbol].kind {
            SymbolType::Tristate if self.modules_value == Tristate::No => SymbolType::Boolean,
            kind => kind,
        }
    }

    pub(crate) fn evaluate(&mut self, expression: &Expr) -> Tristate {
        match expression.0.as_ref() {
            Node::Symbol(symbol) => {
                self.calculate(*symbol);
                self.symbols[*symbol].current.tri
            }
            Node::And(left, right) => self.evaluate(left).min(self.evaluate(right)),
            Node::Or(left, right) => self.evaluate(left).max(self.evaluate(right)),
            Node::Not(expression) => self.evaluate(expression).not(),
            Node::Compare(comparison, left, right) => {
                self.calculate(*left);
                self.calculate(*right);
                let left = &self.symbols[*left];
                let right = &self.symbols[*right];
                let order = compare_values(&left.current, left.kind, &right.current, right.kind);
                let yes = match comparison {
                    Compare::Equal => order == Ordering::Equal,
                    Compare::Unequal => order != Ordering::Equal,
                    Compare::Less => order == Ordering::Less,
                    Compare::LessEqual => order != Ordering::Greater,
                    Compare::Greater => order == Ordering::Greater,
                    Compare::GreaterEqual => order != Ordering::Less,
                };
                if yes {
                    Tristate::Yes
                } else {
                    Tristate::No
                }
            }
        }
    }

    fn calculate_visibility(&mut self, symbol: SymbolId) {
        if self.symbols[symbol].transitional {
            self.symbols[symbol].visible = Tristate::Yes;
            return;
        }
        let mut visible = Tristate::No;
        for property in self.symbols[symbol].properties.clone() {
            if matches!(self.properties[property].kind, PropertyKind::Prompt(_)) {
                visible = visible.max(self.evaluate(&self.properties[property].condition.clone()));
            }
        }
        if self.effective_type(symbol) != SymbolType::Tristate {
            visible = visible.boolean();
        }
        self.symbols[symbol].visible = visible;
        if self.symbols[symbol].choice.is_some() {
            return;
        }
        let direct = self.evaluate(&self.symbols[symbol].dependency.clone());
        let selected = self.evaluate(&self.symbols[symbol].selected.clone());
        let implied = self.evaluate(&self.symbols[symbol].implied.clone());
        let boolean = self.effective_type(symbol) == SymbolType::Boolean;
        self.symbols[symbol].direct_value = if boolean { direct.boolean() } else { direct };
        self.symbols[symbol].selected_value = if boolean {
            selected.boolean()
        } else {
            selected
        };
        self.symbols[symbol].implied_value = if boolean { implied.boolean() } else { implied };
    }

    fn default_property(&mut self, symbol: SymbolId) -> Option<(PropertyId, Tristate)> {
        for property in self.symbols[symbol].properties.clone() {
            if matches!(self.properties[property].kind, PropertyKind::Default(_)) {
                let visible = self.evaluate(&self.properties[property].condition.clone());
                if visible != Tristate::No {
                    return Some((property, visible));
                }
            }
        }
        None
    }

    pub(crate) fn active_range(&mut self, symbol: SymbolId) -> Option<(SymbolId, SymbolId)> {
        for property in self.symbols[symbol].properties.clone() {
            if let PropertyKind::Range(lower, upper) = self.properties[property].kind {
                if self.evaluate(&self.properties[property].condition.clone()) != Tristate::No {
                    return Some((lower, upper));
                }
            }
        }
        None
    }

    fn range_number(&mut self, symbol: SymbolId, fallback: SymbolType) -> i64 {
        self.calculate(symbol);
        let data = &self.symbols[symbol];
        let kind = if matches!(data.kind, SymbolType::Int | SymbolType::Hex) {
            data.kind
        } else {
            fallback
        };
        saturated_number(&data.current.text, kind)
    }

    pub(crate) fn calculate(&mut self, id: SymbolId) {
        if self.symbols[id].valid {
            return;
        }
        self.symbols[id].valid = true;
        let kind = self.symbols[id].kind;
        let mut value = Value {
            text: match kind {
                SymbolType::Int => "0",
                SymbolType::Hex => "0x0",
                SymbolType::String => "",
                SymbolType::Boolean | SymbolType::Tristate => "n",
                SymbolType::Unknown => {
                    self.symbols[id].current = Value {
                        text: self.symbols[id].name.clone().unwrap_or_default(),
                        tri: Tristate::No,
                    };
                    return;
                }
            }
            .into(),
            tri: Tristate::No,
        };
        self.symbols[id].write = false;
        self.calculate_visibility(id);
        self.symbols[id].write = self.symbols[id].visible != Tristate::No;
        self.symbols[id].current = value.clone();
        let mut default = None;
        match self.effective_type(id) {
            SymbolType::Boolean | SymbolType::Tristate => {
                if let Some(choice) = self.symbols[id].choice {
                    self.calculate_choice(choice);
                    value.tri = self.symbols[id].current.tri;
                } else {
                    if let Some(user) = self.symbols[id]
                        .user
                        .clone()
                        .filter(|_| self.symbols[id].visible != Tristate::No)
                    {
                        value.tri = user.tri.min(self.symbols[id].visible);
                    } else {
                        if self.symbols[id].selected_value != Tristate::No {
                            self.symbols[id].write = true;
                        }
                        if self.symbols[id].choice_menu.is_none() {
                            if let Some((property, visible)) = self.default_property(id) {
                                if let PropertyKind::Default(expression) =
                                    self.properties[property].kind.clone()
                                {
                                    value.tri = self.evaluate(&expression).min(visible);
                                    self.symbols[id].write |= value.tri != Tristate::No;
                                    default = expression.as_symbol();
                                }
                            }
                            if self.symbols[id].implied_value != Tristate::No {
                                self.symbols[id].write = true;
                                value.tri = value
                                    .tri
                                    .max(self.symbols[id].implied_value)
                                    .min(self.symbols[id].direct_value);
                            }
                        }
                    }
                    if self.symbols[id].direct_value < self.symbols[id].selected_value {
                        self.warn_unmet_dependency(id);
                    }
                    value.tri = value.tri.max(self.symbols[id].selected_value);
                }
                if self.effective_type(id) == SymbolType::Boolean {
                    value.tri = value.tri.boolean();
                }
                value.text = value.tri.to_string();
            }
            SymbolType::String | SymbolType::Hex | SymbolType::Int => {
                if let Some(user) = self.symbols[id]
                    .user
                    .clone()
                    .filter(|_| self.symbols[id].visible != Tristate::No)
                {
                    value.text = user.text;
                } else if let Some((property, _)) = self.default_property(id) {
                    if let PropertyKind::Default(expression) = &self.properties[property].kind {
                        if let Some(source) = expression.as_symbol() {
                            self.symbols[id].write = true;
                            self.calculate(source);
                            value.text = self.symbols[source].current.text.clone();
                            default = Some(source);
                        }
                    }
                }
            }
            SymbolType::Unknown => {}
        }
        if let Some(source) = default {
            if self.symbols[id].user.is_none()
                && self.symbols[source].transitional
                && self.symbols[source].user.is_some()
            {
                self.symbols[id].user = Some(value.clone());
            }
        }
        self.symbols[id].current = value;
        if matches!(kind, SymbolType::Int | SymbolType::Hex) {
            if let Some((lower, upper)) = self.active_range(id) {
                let number = saturated_number(&self.symbols[id].current.text, kind);
                let bound = if number < self.range_number(lower, kind) {
                    Some(lower)
                } else if number > self.range_number(upper, kind) {
                    Some(upper)
                } else {
                    None
                };
                if let Some(bound) = bound {
                    self.symbols[id].current.text = self.symbols[bound].current.text.clone();
                }
            }
        }
        if id == self.modules {
            self.modules_value = self.symbols[id].current.tri;
        }
        if self.symbols[id].choice_menu.is_some() || self.symbols[id].transitional {
            self.symbols[id].write = false;
        }
    }

    pub(crate) fn invalidate(&mut self) {
        for symbol in &mut self.symbols {
            if !symbol.constant {
                symbol.valid = false;
            }
        }
        self.changed = true;
        self.calculate(self.modules);
    }

    pub(crate) fn choice_default(&mut self, choice: MenuId) -> Option<SymbolId> {
        let symbol = self.menus[choice].symbol.expect("choice symbol");
        for property in self.symbols[symbol].properties.clone() {
            if let PropertyKind::Default(expression) = self.properties[property].kind.clone() {
                if self.evaluate(&self.properties[property].condition.clone()) != Tristate::No {
                    if let Some(member) = expression.as_symbol() {
                        if self.symbols[member].visible != Tristate::No {
                            return Some(member);
                        }
                    }
                }
            }
        }
        self.menu_depth_first(choice)
            .into_iter()
            .skip(1)
            .filter_map(|menu| self.menus[menu].symbol)
            .find(|&symbol| self.symbols[symbol].visible != Tristate::No)
    }

    pub(crate) fn calculate_choice(&mut self, choice: MenuId) -> Option<SymbolId> {
        let members = self.menus[choice].members.clone();
        for &member in &members {
            self.calculate_visibility(member);
        }
        let mut selected = members.iter().copied().find(|&member| {
            self.symbols[member].visible != Tristate::No
                && self.symbols[member]
                    .user
                    .as_ref()
                    .is_some_and(|user| user.tri == Tristate::Yes)
        });
        if selected.is_none() {
            selected = self.choice_default(choice).filter(|&member| {
                !self.symbols[member]
                    .user
                    .as_ref()
                    .is_some_and(|user| user.tri == Tristate::No)
            });
        }
        if selected.is_none() {
            selected = self
                .menu_depth_first(choice)
                .into_iter()
                .skip(1)
                .filter_map(|menu| self.menus[menu].symbol)
                .find(|&member| {
                    self.symbols[member].visible != Tristate::No
                        && self.symbols[member].user.is_none()
                });
        }
        if selected.is_none() {
            selected = members
                .iter()
                .rev()
                .copied()
                .find(|&member| self.symbols[member].visible != Tristate::No);
        }
        for member in members {
            if self.symbols[member].visible != Tristate::No {
                let symbol = &mut self.symbols[member];
                symbol.current = Value::tristate(if Some(member) == selected {
                    Tristate::Yes
                } else {
                    Tristate::No
                });
                symbol.valid = true;
                symbol.write = true;
            }
        }
        selected
    }

    pub(crate) fn set_user(&mut self, symbol: SymbolId, value: Value) {
        self.symbols[symbol].user = Some(value);
        if let Some(choice) = self.symbols[symbol].choice {
            self.menus[choice]
                .members
                .retain(|&member| member != symbol);
            self.menus[choice].members.insert(0, symbol);
        }
    }

    pub(crate) fn within_range(&self, symbol: SymbolId, value: Tristate) -> bool {
        let data = &self.symbols[symbol];
        let kind = self.effective_type(symbol);
        data.visible != Tristate::No
            && matches!(kind, SymbolType::Boolean | SymbolType::Tristate)
            && !(kind == SymbolType::Boolean && value == Tristate::Mod)
            && data.visible > data.selected_value
            && value >= data.selected_value
            && value <= data.visible
    }

    pub(crate) fn set_tristate(&mut self, symbol: SymbolId, value: Tristate) -> bool {
        self.calculate(symbol);
        if !self.within_range(symbol, value) {
            return false;
        }
        let changed = self.symbols[symbol].current.tri != value;
        self.symbols[symbol].user = Some(Value::tristate(value));
        if changed {
            self.invalidate();
        }
        true
    }

    pub(crate) fn set_choice(&mut self, choice: MenuId, selected: SymbolId) {
        let mut changed = false;
        for menu in self.menu_depth_first(choice).into_iter().skip(1) {
            if let Some(member) = self.menus[menu].symbol {
                if self.symbols[member].visible == Tristate::No {
                    continue;
                }
                let value = if member == selected {
                    Tristate::Yes
                } else {
                    Tristate::No
                };
                changed |= self.symbols[member].current.tri != value;
                self.set_user(member, Value::tristate(value));
            }
        }
        if changed {
            self.invalidate();
        }
    }

    pub(crate) fn string_valid(&self, symbol: SymbolId, text: &str) -> bool {
        match self.symbols[symbol].kind {
            SymbolType::String => true,
            SymbolType::Int => {
                let digits = text.strip_prefix('-').unwrap_or(text);
                !digits.is_empty()
                    && digits.bytes().all(|byte| byte.is_ascii_digit())
                    && !(digits.len() > 1 && digits.starts_with('0'))
            }
            SymbolType::Hex => {
                let digits = text
                    .strip_prefix("0x")
                    .or_else(|| text.strip_prefix("0X"))
                    .unwrap_or(text);
                !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_hexdigit())
            }
            SymbolType::Boolean | SymbolType::Tristate => matches!(
                text.as_bytes().first(),
                Some(b'y' | b'Y' | b'm' | b'M' | b'n' | b'N')
            ),
            SymbolType::Unknown => false,
        }
    }

    pub(crate) fn string_within_range(&mut self, symbol: SymbolId, text: &str) -> bool {
        if !self.string_valid(symbol, text) {
            return false;
        }
        let kind = self.symbols[symbol].kind;
        if matches!(kind, SymbolType::Boolean | SymbolType::Tristate) {
            let value = match text.as_bytes()[0] {
                b'y' | b'Y' => Tristate::Yes,
                b'm' | b'M' => Tristate::Mod,
                _ => Tristate::No,
            };
            return self.within_range(symbol, value);
        }
        if let Some((lower, upper)) = self.active_range(symbol) {
            let number = saturated_number(text, kind);
            return number >= self.range_number(lower, kind)
                && number <= self.range_number(upper, kind);
        }
        true
    }

    pub(crate) fn set_string(&mut self, symbol: SymbolId, text: &str) -> bool {
        if matches!(
            self.symbols[symbol].kind,
            SymbolType::Boolean | SymbolType::Tristate
        ) {
            return match text.as_bytes().first() {
                Some(b'y' | b'Y') => self.set_tristate(symbol, Tristate::Yes),
                Some(b'm' | b'M') => self.set_tristate(symbol, Tristate::Mod),
                Some(b'n' | b'N') => self.set_tristate(symbol, Tristate::No),
                _ => false,
            };
        }
        if !self.string_within_range(symbol, text) {
            return false;
        }
        let text = if self.symbols[symbol].kind == SymbolType::Hex
            && !text.starts_with("0x")
            && !text.starts_with("0X")
        {
            format!("0x{text}")
        } else {
            text.to_string()
        };
        if self.symbols[symbol]
            .user
            .as_ref()
            .is_some_and(|user| user.text == text)
        {
            return true;
        }
        self.symbols[symbol].user = Some(Value {
            text,
            tri: Tristate::No,
        });
        self.invalidate();
        true
    }

    pub(crate) fn default_value(&mut self, symbol: SymbolId) -> Value {
        self.calculate_visibility(symbol);
        self.calculate(self.modules);
        let kind = self.symbols[symbol].kind;
        let mut text = String::new();
        let mut tri = Tristate::No;
        if let Some((property, visible)) = self.default_property(symbol) {
            if let PropertyKind::Default(expression) = self.properties[property].kind.clone() {
                if matches!(kind, SymbolType::Boolean | SymbolType::Tristate) {
                    tri = self.evaluate(&expression).min(visible);
                } else if let Some(source) = expression.as_symbol() {
                    self.calculate(source);
                    text = self.symbols[source].current.text.clone();
                }
            }
        }
        tri = tri.max(self.symbols[symbol].selected_value);
        if kind == SymbolType::Boolean
            || (self.symbols[symbol].choice.is_none() && self.modules_value == Tristate::No)
        {
            tri = tri.boolean();
        }
        tri = tri.max(self.symbols[symbol].implied_value);
        if matches!(kind, SymbolType::Boolean | SymbolType::Tristate) {
            return Value::tristate(tri);
        }
        if text.is_empty() {
            text = match kind {
                SymbolType::Int => "0",
                SymbolType::Hex => "0x0",
                _ => "",
            }
            .into();
        }
        Value {
            text,
            tri: Tristate::No,
        }
    }

    fn warn_unmet_dependency(&mut self, symbol: SymbolId) {
        let data = &self.symbols[symbol];
        eprintln!(
            "\nWARNING: unmet direct dependencies detected for {}\n  Depends on [{}]: {}",
            data.display_name(),
            data.direct_value,
            self.format_expression(&data.dependency, true)
        );
        let selected = data.selected.clone();
        for (value, title) in [
            (Tristate::Yes, "  Selected by [y]:"),
            (Tristate::Mod, "  Selected by [m]:"),
        ] {
            let mut entries = Vec::new();
            self.match_reverse_dependencies(&selected, value, &mut entries);
            if !entries.is_empty() {
                eprintln!("{title}");
                for entry in entries {
                    eprintln!("  - {}", self.format_expression(&entry, true));
                }
            }
        }
        self.dependency_warnings += 1;
    }

    pub(crate) fn match_reverse_dependencies(
        &mut self,
        expression: &Expr,
        value: Tristate,
        output: &mut Vec<Expr>,
    ) {
        if let Node::Or(left, right) = expression.0.as_ref() {
            self.match_reverse_dependencies(left, value, output);
            self.match_reverse_dependencies(right, value, output);
        } else if self.evaluate(expression) == value {
            output.push(expression.clone());
        }
    }
}

fn saturated_number(text: &str, kind: SymbolType) -> i64 {
    let text = text.trim_start();
    let negative = text.starts_with('-');
    let text = text.strip_prefix(['-', '+']).unwrap_or(text);
    let radix = if kind == SymbolType::Hex { 16 } else { 10 };
    let text = if radix == 16 {
        text.strip_prefix("0x")
            .or_else(|| text.strip_prefix("0X"))
            .unwrap_or(text)
    } else {
        text
    };
    let mut magnitude = 0u64;
    for character in text.chars() {
        let Some(digit) = character.to_digit(radix) else {
            break;
        };
        magnitude = magnitude
            .saturating_mul(radix as u64)
            .saturating_add(digit as u64);
    }
    if negative {
        if magnitude >= 1u64 << 63 {
            i64::MIN
        } else {
            -(magnitude as i64)
        }
    } else {
        magnitude.min(i64::MAX as u64) as i64
    }
}
