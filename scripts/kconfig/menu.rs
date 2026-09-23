// SPDX-License-Identifier: GPL-2.0
//! Menu dependency propagation, validation, automatic submenus, and help.
// Original Kconfig implementation: Copyright (C) 2002 Roman Zippel.

use crate::expr::{Compare, Expr, Node, Tristate};
use crate::model::{Kconfig, MenuId, MenuType, PropertyKind, SymbolId, SymbolType};
use std::fmt::Write;

impl Kconfig {
    pub(crate) fn menu_depth_first(&self, root: MenuId) -> Vec<MenuId> {
        let mut result = Vec::new();
        let mut pending = vec![root];
        while let Some(menu) = pending.pop() {
            result.push(menu);
            pending.extend(self.menus[menu].children.iter().rev());
        }
        result
    }

    pub(crate) fn finalize(&mut self) -> Result<(), String> {
        self.propagate_dependencies(0);
        let mut roots = std::mem::take(&mut self.menus[0].children);
        self.group_sequence(&mut roots, false);
        self.menus[0].children = roots;
        self.flatten_children(0);
        let mut warned = vec![false; self.symbols.len()];
        for menu in self.menu_depth_first(0) {
            if let Some(symbol) = self.menus[menu].symbol {
                if !warned[symbol] {
                    warned[symbol] = true;
                    self.check_properties(symbol, menu);
                }
            }
        }
        let mut checker = DependencyChecker::new(self.symbols.len());
        for menu in self.menu_depth_first(0) {
            if let Some(symbol) = self.menus[menu].symbol {
                if checker.check(self, symbol).is_some() {
                    self.errors += 1;
                }
                self.check_transitional(menu, symbol);
                if self.symbols[symbol].choice_menu.is_some() {
                    for child in self.menu_depth_first(menu).into_iter().skip(1) {
                        if let Some(member) = self.menus[child].symbol {
                            self.check_choice_member(child, member);
                        }
                    }
                }
            }
        }
        if self.errors == 0 {
            Ok(())
        } else {
            Err(String::new())
        }
    }

    fn propagate_dependencies(&mut self, parent: MenuId) {
        let inherited = self.menus[parent].dependency.clone();
        for menu in self.menus[parent].children.clone() {
            let dependency = inherited
                .clone()
                .and(self.menus[menu].dependency.rewrite_modules(self.modules));
            self.menus[menu].dependency = self.simplify(&dependency);
            for property in self.menus[menu].properties.clone() {
                let condition = self.menus[menu].dependency.clone().and(
                    self.properties[property]
                        .condition
                        .rewrite_modules(self.modules),
                );
                let condition = self.simplify(&condition);
                self.properties[property].condition = condition.clone();
                if let Some(symbol) = self.menus[menu].symbol {
                    let selected = Expr::symbol(symbol).and(condition);
                    match self.properties[property].kind {
                        PropertyKind::Select(target) => {
                            self.symbols[target].selected =
                                self.symbols[target].selected.clone().or(selected)
                        }
                        PropertyKind::Imply(target) => {
                            self.symbols[target].implied =
                                self.symbols[target].implied.clone().or(selected)
                        }
                        _ => {}
                    }
                }
            }
            if let Some(symbol) = self.menus[menu].symbol {
                self.symbols[symbol].dependency = self.symbols[symbol]
                    .dependency
                    .clone()
                    .or(self.menus[menu].dependency.clone());
            }
            self.propagate_dependencies(menu);
        }
    }

    fn group_sequence(&mut self, sequence: &mut Vec<MenuId>, inside_choice: bool) {
        let mut index = 0;
        while index < sequence.len() {
            self.group_entry(sequence, index, inside_choice);
            index += 1;
        }
    }

    fn group_entry(&mut self, sequence: &mut Vec<MenuId>, index: usize, inside_choice: bool) {
        let menu = sequence[index];
        if !self.menus[menu].children.is_empty() {
            let is_choice = self.menus[menu].kind == MenuType::Choice;
            let mut children = std::mem::take(&mut self.menus[menu].children);
            self.group_sequence(&mut children, is_choice);
            self.menus[menu].children = children;
        } else if let Some(symbol) = self.menus[menu].symbol.filter(|_| !inside_choice) {
            let base = self.menus[menu]
                .prompt
                .map(|property| self.properties[property].condition.clone())
                .unwrap_or_else(Expr::yes);
            let base = self.simplify(&base.equals_no().not());
            while let Some(&next) = sequence.get(index + 1) {
                let dependency = self.menus[next]
                    .prompt
                    .map(|property| self.properties[property].condition.clone())
                    .unwrap_or_else(|| self.menus[next].dependency.clone());
                if !dependency.contains(symbol) {
                    break;
                }
                if !dependency.depends_on(symbol) {
                    if dependency.contains_negated(symbol) {
                        break;
                    }
                    let dependency = self.simplify(&dependency.equals_no().not());
                    let mut terms = Vec::new();
                    let mut base_terms = Vec::new();
                    dependency.terms(true, &mut terms);
                    base.terms(true, &mut base_terms);
                    if base_terms.iter().any(|term| {
                        !term.is_yes() && !terms.iter().any(|other| term.equivalent(other))
                    }) {
                        break;
                    }
                }
                self.group_entry(sequence, index + 1, false);
                let child = sequence.remove(index + 1);
                self.menus[child].parent = Some(menu);
                self.menus[menu].children.push(child);
            }
        }
        self.flatten_children(menu);
    }

    fn flatten_children(&mut self, parent: MenuId) {
        let children = std::mem::take(&mut self.menus[parent].children);
        for child in children {
            self.menus[parent].children.push(child);
            if self.prompt(child).is_none_or(str::is_empty) {
                let flattened = std::mem::take(&mut self.menus[child].children);
                for grandchild in flattened {
                    self.menus[grandchild].parent = Some(parent);
                    self.menus[parent].children.push(grandchild);
                }
            }
        }
    }

    fn check_properties(&mut self, symbol: SymbolId, menu: MenuId) {
        if self.symbols[symbol].kind == SymbolType::Unknown {
            self.warning(
                &self.menus[menu].location.clone(),
                "config symbol defined without type",
            );
        }
        for property in self.symbols[symbol].properties.clone() {
            let property = self.properties[property].clone();
            let name = self.symbols[symbol].display_name().to_string();
            let kind = self.symbols[symbol].kind;
            match property.kind {
                PropertyKind::Default(expression) => {
                    let default = expression.as_symbol();
                    if matches!(kind, SymbolType::String | SymbolType::Int | SymbolType::Hex)
                        && default.is_none()
                    {
                        self.warning(
                            &property.location,
                            format!("default for config symbol '{name}' must be a single symbol"),
                        );
                    }
                    if let Some(default) = default {
                        if matches!(kind, SymbolType::Hex | SymbolType::Int)
                            && !self.valid_number_symbol(symbol, default)
                        {
                            self.warning(
                                &property.location,
                                format!("'{name}': number is invalid"),
                            );
                        }
                        if let Some(choice) = self.symbols[symbol].choice_menu {
                            if self.symbols[default].choice != Some(choice) {
                                self.warning(
                                    &property.location,
                                    format!(
                                        "choice default symbol '{}' is not contained in the choice",
                                        self.symbols[default].display_name()
                                    ),
                                );
                            }
                        }
                    }
                }
                PropertyKind::Select(target) | PropertyKind::Imply(target) => {
                    let operation = if matches!(property.kind, PropertyKind::Select(_)) {
                        "select"
                    } else {
                        "imply"
                    };
                    if !matches!(kind, SymbolType::Boolean | SymbolType::Tristate) {
                        self.warning(&property.location, format!("config symbol '{name}' uses {operation}, but is not bool or tristate"));
                    } else if !matches!(
                        self.symbols[target].kind,
                        SymbolType::Unknown | SymbolType::Boolean | SymbolType::Tristate
                    ) {
                        self.warning(&property.location, format!("'{}' has wrong type. '{operation}' only accept arguments of bool and tristate type", self.symbols[target].display_name()));
                    }
                }
                PropertyKind::Range(lower, upper) => {
                    if !matches!(kind, SymbolType::Int | SymbolType::Hex) {
                        self.warning(
                            &property.location,
                            "range is only allowed for int or hex symbols",
                        );
                    }
                    if !self.valid_number_symbol(symbol, lower)
                        || !self.valid_number_symbol(symbol, upper)
                    {
                        self.warning(&property.location, "range is invalid");
                    }
                }
                _ => {}
            }
        }
    }

    fn valid_number_symbol(&self, target: SymbolId, bound: SymbolId) -> bool {
        matches!(self.symbols[bound].kind, SymbolType::Int | SymbolType::Hex)
            || (self.symbols[bound].kind == SymbolType::Unknown
                && self.string_valid(target, self.symbols[bound].display_name()))
    }

    fn check_transitional(&mut self, menu: MenuId, symbol: SymbolId) {
        if !self.symbols[symbol].transitional {
            return;
        }
        let location =
            if !self.menus[menu].dependency.is_yes() || !self.menus[menu].visibility.is_yes() {
                Some(self.menus[menu].location.clone())
            } else {
                self.symbols[symbol]
                    .properties
                    .first()
                    .map(|&property| self.properties[property].location.clone())
            };
        if let Some(location) = location {
            self.error(
                &location,
                "error: transitional symbols can only have help sections",
            );
        }
    }

    fn check_choice_member(&mut self, menu: MenuId, symbol: SymbolId) {
        for property in self.symbols[symbol].properties.clone() {
            let property = self.properties[property].clone();
            if matches!(property.kind, PropertyKind::Default(_)) {
                self.error(
                    &property.location,
                    "error: defaults for choice values not supported",
                );
            }
            if property.menu != menu && matches!(property.kind, PropertyKind::Prompt(_)) {
                self.error(
                    &property.location,
                    "error: choice value must not have a prompt in another entry",
                );
            }
        }
    }

    pub(crate) fn menu_visible(&mut self, menu: MenuId) -> bool {
        let Some(prompt) = self.menus[menu].prompt else {
            return false;
        };
        if self.evaluate(&self.menus[menu].visibility.clone()) == Tristate::No {
            return false;
        }
        if let Some(symbol) = self.menus[menu].symbol {
            self.calculate(symbol);
        }
        if self.evaluate(&self.properties[prompt].condition.clone()) != Tristate::No {
            return true;
        }
        if self.menus[menu]
            .symbol
            .is_none_or(|symbol| self.symbols[symbol].current.tri == Tristate::No)
        {
            return false;
        }
        self.menus[menu]
            .children
            .clone()
            .into_iter()
            .any(|child| self.menu_visible(child))
    }

    pub(crate) fn format_expression(&self, expression: &Expr, values: bool) -> String {
        self.format_expression_inner(expression, values, 0)
    }

    fn format_expression_inner(&self, expression: &Expr, values: bool, parent: u8) -> String {
        let (text, precedence) = match expression.0.as_ref() {
            Node::Symbol(symbol) => (self.format_symbol(*symbol, values), 6),
            Node::Not(inner) => (
                format!("!{}", self.format_expression_inner(inner, values, 3)),
                3,
            ),
            Node::And(left, right) => (
                format!(
                    "{} && {}",
                    self.format_expression_inner(left, values, 2),
                    self.format_expression_inner(right, values, 2)
                ),
                2,
            ),
            Node::Or(left, right) => (
                format!(
                    "{} || {}",
                    self.format_expression_inner(left, values, 1),
                    self.format_expression_inner(right, values, 1)
                ),
                1,
            ),
            Node::Compare(kind, left, right) => {
                let token = match kind {
                    Compare::Equal => "=",
                    Compare::Unequal => "!=",
                    Compare::Less => "<",
                    Compare::LessEqual => "<=",
                    Compare::Greater => ">",
                    Compare::GreaterEqual => ">=",
                };
                (
                    format!(
                        "{}{token}{}",
                        self.format_symbol(*left, values),
                        self.format_symbol(*right, values)
                    ),
                    if matches!(kind, Compare::Equal | Compare::Unequal) {
                        4
                    } else {
                        5
                    },
                )
            }
        };
        if precedence < parent {
            format!("({text})")
        } else {
            text
        }
    }

    fn format_symbol(&self, symbol: SymbolId, values: bool) -> String {
        let symbol = &self.symbols[symbol];
        if values && symbol.kind != SymbolType::Unknown && symbol.name.is_some() {
            format!("{} [={}]", symbol.display_name(), symbol.current.text)
        } else {
            symbol.display_name().to_string()
        }
    }

    pub(crate) fn extended_help(&mut self, menu: MenuId) -> String {
        let mut output = String::new();
        let symbol = self.menus[menu].symbol;
        if let Some(help) = &self.menus[menu].help {
            if let Some(name) = symbol.and_then(|id| self.symbols[id].name.as_ref()) {
                let _ = write!(output, "CONFIG_{name}:\n\n");
            }
            let _ = writeln!(output, "{help}");
        } else {
            output.push_str("There is no help available for this option.\n");
        }
        let Some(symbol) = symbol else {
            return output;
        };
        self.calculate(symbol);
        if let Some(name) = &self.symbols[symbol].name {
            let _ = writeln!(
                output,
                "Symbol: {name} [={}]\nType  : {}",
                self.symbols[symbol].current.text,
                self.symbols[symbol].kind.name()
            );
            if matches!(self.symbols[symbol].kind, SymbolType::Int | SymbolType::Hex) {
                if let Some((lower, upper)) = self.active_range(symbol) {
                    let _ = writeln!(
                        output,
                        "Range : [{} {}]",
                        self.format_symbol(lower, true),
                        self.format_symbol(upper, true)
                    );
                }
            }
        }
        for prompted in [true, false] {
            for definition in self.symbols[symbol].menus.clone() {
                let prompt = self.menus[definition].prompt;
                if prompt.is_some() != prompted {
                    continue;
                }
                let _ = writeln!(output, "Defined at {}", self.menus[definition].location);
                if let Some(prompt) = prompt {
                    let _ = writeln!(
                        output,
                        "  Prompt: {}",
                        self.prompt(definition).unwrap_or_default()
                    );
                    self.help_dependency(
                        &mut output,
                        &self.menus[definition].dependency,
                        "  Depends on: ",
                    );
                    if !self.properties[prompt]
                        .condition
                        .equivalent(&self.menus[definition].dependency)
                    {
                        self.help_dependency(
                            &mut output,
                            &self.properties[prompt].condition,
                            "  Visible if: ",
                        );
                    }
                    let mut ancestors = vec![definition];
                    let mut parent = self.menus[definition].parent;
                    while let Some(menu) = parent.filter(|&menu| menu != 0 && ancestors.len() < 8) {
                        ancestors.push(menu);
                        parent = self.menus[menu].parent;
                    }
                    output.push_str("  Location:\n");
                    for (index, ancestor) in ancestors.into_iter().rev().enumerate() {
                        let _ = write!(
                            output,
                            "{}-> {}",
                            " ".repeat(4 + 2 * index),
                            self.prompt(ancestor).unwrap_or_default()
                        );
                        if let Some(symbol) = self.menus[ancestor].symbol {
                            let _ = write!(
                                output,
                                " ({} [={}])",
                                self.symbols[symbol].display_name(),
                                self.symbols[symbol].current.text
                            );
                        }
                        output.push('\n');
                    }
                } else {
                    self.help_dependency(
                        &mut output,
                        &self.menus[definition].dependency,
                        "  Depends on: ",
                    );
                }
            }
        }
        for implied in [false, true] {
            let targets: Vec<_> = self.symbols[symbol]
                .properties
                .iter()
                .filter_map(|&property| match self.properties[property].kind {
                    PropertyKind::Select(target) if !implied => {
                        Some(self.format_symbol(target, true))
                    }
                    PropertyKind::Imply(target) if implied => {
                        Some(self.format_symbol(target, true))
                    }
                    _ => None,
                })
                .collect();
            if !targets.is_empty() {
                let _ = writeln!(
                    output,
                    "{}: {}",
                    if implied { "Implies" } else { "Selects" },
                    targets.join(" && ")
                );
            }
            let expression = if implied {
                self.symbols[symbol].implied.clone()
            } else {
                self.symbols[symbol].selected.clone()
            };
            if !expression.is_no() {
                for value in [Tristate::Yes, Tristate::Mod, Tristate::No] {
                    let mut entries = Vec::new();
                    self.match_reverse_dependencies(&expression, value, &mut entries);
                    if !entries.is_empty() {
                        let _ = writeln!(
                            output,
                            "{} by [{value}]:",
                            if implied { "Implied" } else { "Selected" }
                        );
                        for entry in entries {
                            let _ =
                                writeln!(output, "  - {}", self.format_expression(&entry, true));
                        }
                    }
                }
            }
        }
        output.push_str("\n\n");
        output
    }

    fn help_dependency(&self, output: &mut String, expression: &Expr, prefix: &str) {
        if !expression.is_yes() {
            let _ = writeln!(
                output,
                "{prefix}{}",
                self.format_expression(expression, true)
            );
        }
    }
}

#[derive(Clone)]
enum Relation {
    Depends,
    Selected,
    Implied,
    Condition(&'static str),
    Value(&'static str),
    Choice,
}

#[derive(Clone)]
struct DependencyFrame {
    symbol: SymbolId,
    relation: Relation,
}

struct DependencyChecker {
    checked: Vec<bool>,
    active: Vec<bool>,
    stack: Vec<DependencyFrame>,
}

impl DependencyChecker {
    fn new(count: usize) -> Self {
        Self {
            checked: vec![false; count],
            active: vec![false; count],
            stack: Vec::new(),
        }
    }

    fn check(&mut self, config: &Kconfig, symbol: SymbolId) -> Option<SymbolId> {
        if self.active[symbol] {
            self.report(config, symbol);
            return Some(symbol);
        }
        if self.checked[symbol] {
            return None;
        }
        if let Some(choice) = config.symbols[symbol].choice {
            self.stack.push(DependencyFrame {
                symbol,
                relation: Relation::Choice,
            });
            let result = self.check(config, config.menus[choice].symbol.expect("choice symbol"));
            self.stack.pop();
            return result;
        }
        if let Some(choice) = config.symbols[symbol].choice_menu {
            self.stack.push(DependencyFrame {
                symbol,
                relation: Relation::Choice,
            });
            let members: Vec<_> = config
                .menu_depth_first(choice)
                .into_iter()
                .skip(1)
                .filter_map(|menu| config.menus[menu].symbol)
                .collect();
            for &member in &members {
                self.checked[member] = true;
                self.active[member] = true;
            }
            self.checked[symbol] = true;
            self.active[symbol] = true;
            let mut result = self.check_dependencies(config, symbol);
            self.active[symbol] = false;
            if result.is_none() {
                for &member in &members {
                    result = self.check_dependencies(config, member);
                    if result.is_some() {
                        break;
                    }
                }
            }
            for member in members {
                self.active[member] = false;
            }
            if result.is_some_and(|member| config.symbols[member].choice == Some(choice)) {
                result = Some(symbol);
            }
            self.stack.pop();
            return result;
        }
        self.checked[symbol] = true;
        self.active[symbol] = true;
        let result = self.check_dependencies(config, symbol);
        self.active[symbol] = false;
        result
    }

    fn check_dependencies(&mut self, config: &Kconfig, symbol: SymbolId) -> Option<SymbolId> {
        let data = &config.symbols[symbol];
        let mut dependencies = vec![
            (data.dependency.clone(), Relation::Depends),
            (data.selected.clone(), Relation::Selected),
            (data.implied.clone(), Relation::Implied),
        ];
        for &property in &data.properties {
            let property = &config.properties[property];
            let kind = match &property.kind {
                PropertyKind::Select(_) | PropertyKind::Imply(_) => continue,
                PropertyKind::Prompt(_) => {
                    if config.menus[property.menu].kind == MenuType::Menu {
                        "menu"
                    } else {
                        "prompt"
                    }
                }
                PropertyKind::Default(_) => "default",
                PropertyKind::Range(..) => "range",
            };
            dependencies.push((property.condition.clone(), Relation::Condition(kind)));
            if let PropertyKind::Default(expression) = &property.kind {
                if data.choice_menu.is_none() {
                    dependencies.push((expression.clone(), Relation::Value(kind)));
                }
            }
        }
        for (expression, relation) in dependencies {
            self.stack.push(DependencyFrame { symbol, relation });
            let mut referenced = Vec::new();
            expression.symbols(&mut referenced);
            for other in referenced {
                if let Some(result) = self.check(config, other) {
                    self.stack.pop();
                    return Some(result);
                }
            }
            self.stack.pop();
        }
        None
    }

    fn report(&self, config: &Kconfig, mut last: SymbolId) {
        let mut frames = self.stack.clone();
        if let Some(choice) = config.symbols[last].choice {
            frames.push(DependencyFrame {
                symbol: last,
                relation: Relation::Choice,
            });
            last = config.menus[choice].symbol.expect("choice symbol");
        }
        let Some(start) = frames.iter().rposition(|frame| frame.symbol == last) else {
            eprintln!("unexpected recursive dependency error");
            return;
        };
        for index in start..frames.len() {
            let frame = &frames[index];
            let symbol = config.symbols[frame.symbol].display_name();
            let next_id = frames.get(index + 1).map_or(last, |next| next.symbol);
            let next = config.symbols[next_id].display_name();
            if frame.symbol == last {
                eprintln!("error: recursive dependency detected!");
            }
            if let Some(choice) = config.symbols[next_id].choice_menu {
                eprintln!(
                    "\tsymbol {symbol} is part of choice block at {}",
                    config.menus[choice].location
                );
                continue;
            }
            match frame.relation {
                Relation::Depends => eprintln!("\tsymbol {symbol} depends on {next}"),
                Relation::Selected => eprintln!("\tsymbol {symbol} is selected by {next}"),
                Relation::Implied => eprintln!("\tsymbol {symbol} is implied by {next}"),
                Relation::Condition(property) => {
                    eprintln!("\tsymbol {symbol} {property} is visible depending on {next}")
                }
                Relation::Value(property) => {
                    eprintln!("\tsymbol {symbol} {property} value contains {next}")
                }
                Relation::Choice => {
                    eprintln!("\tsymbol {symbol} unknown is visible depending on {next}")
                }
            }
        }
        eprintln!("For a resolution refer to Documentation/kbuild/kconfig-language.rst\nsubsection \"Kconfig recursive dependency limitations\"\n");
    }
}
