// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2024 Google LLC
//! Explicit-stack DWARF expansion into owned, reusable type fragments.

#![allow(non_upper_case_globals)] // Match the public DWARF wire-constant names.

use crate::cache::Cache;
use crate::die::{DieMap, DieState, Fragment};
use crate::gendwarfksyms_header::{bytes, error, Diagnostics, Result};
use crate::kabi::Rules;
use crate::reader::{constants::*, Die as Node, Module};
use crate::symbols::{is_symbol_ptr, SymbolState, Symbols};
use std::collections::HashSet;

#[derive(Clone)]
struct Expansion {
    expand: bool,
    current_fqn: Vec<u8>,
}

struct State {
    first: bool,
    expansion: Expansion,
    expanded: HashSet<usize>,
    original_name: Option<Vec<u8>>,
}

impl State {
    fn new() -> Self {
        Self {
            first: false,
            expansion: Expansion {
                expand: true,
                current_fqn: Vec::new(),
            },
            expanded: HashSet::new(),
            original_name: None,
        }
    }
}

#[derive(Clone, Copy)]
enum Filter {
    All,
    Tag(i32),
    Structure,
}

type ActiveKey = (usize, u32, bool, usize);
enum Action<'a> {
    Type(Node<'a>, Option<usize>),
    Finish {
        node: Node<'a>,
        parent: Option<usize>,
        cache: usize,
        want: DieState,
        saved: Expansion,
        created: bool,
        active: ActiveKey,
    },
    Text(Option<usize>, Vec<u8>),
    Line(Option<usize>, i32),
    Fqn(Node<'a>, usize),
    Attribute(Node<'a>, usize, u32),
    TypeAttribute(Node<'a>, usize),
    Children(Node<'a>, usize, Filter),
    Child(Node<'a>, usize, Filter),
    First(bool),
    MemberTail(Node<'a>, usize, Option<Vec<u8>>),
    StructureContents(Node<'a>, usize, Filter),
    StructureEnd(Node<'a>, usize, bool),
    Cached(Node<'a>, usize, usize),
}

struct Engine<'a> {
    symbols: &'a mut Symbols,
    dies: &'a mut DieMap,
    rules: &'a Rules,
    diag: &'a mut Diagnostics,
    source_files: Cache,
    linebreak: bool,
    indentation: i64,
}

fn symbol_name(node: Node<'_>) -> Option<&[u8]> {
    node.string(DW_AT_linkage_name)
        .or_else(|| node.string(DW_AT_name))
}

fn expanded_type(tag: i32) -> bool {
    matches!(
        tag,
        DW_TAG_class_type | DW_TAG_structure_type | DW_TAG_union_type | DW_TAG_enumeration_type
    )
}

fn push_actions<'a>(stack: &mut Vec<Action<'a>>, actions: impl IntoIterator<Item = Action<'a>>) {
    let actions: Vec<_> = actions.into_iter().collect();
    stack.extend(actions.into_iter().rev());
}

fn text<'a>(cache: usize, value: &[u8]) -> Action<'a> {
    Action::Text(Some(cache), value.to_vec())
}

impl Engine<'_> {
    fn trace(&mut self, color: u8, parts: &[&[u8]]) {
        if self.diag.options.dump_die_map && self.diag.options.dump_dies {
            self.diag.print(&[format!("\x1b[{color}m<").as_bytes()]);
            self.diag.print(parts);
            self.diag.print(&[b">\x1b[39m"]);
        }
    }

    fn emit(&mut self, cache: Option<usize>, value: &[u8]) {
        if self.diag.options.dump_dies {
            if self.linebreak {
                self.diag.print(&[b"\n"]);
                for _ in 0..self.indentation.max(0) {
                    self.diag.print(&[b"  "]);
                }
                self.linebreak = false;
            }
            self.diag.print(&[value]);
        }
        if let Some(id) = cache {
            // Diagnostic cache handles are stable IDs, not borrowed addresses.
            self.trace(
                91,
                &[
                    format!("cache 0x{:x} string '", id + 1).as_bytes(),
                    value,
                    b"'",
                ],
            );
            self.dies.entries[id]
                .fragments
                .push(Fragment::String(value.to_vec()));
        }
    }

    fn line(&mut self, cache: Option<usize>, change: i32) {
        self.indentation += i64::from(change);
        self.linebreak = true;
        if let Some(id) = cache {
            self.dies.entries[id]
                .fragments
                .push(Fragment::Linebreak(change));
        }
    }

    fn comma(&mut self, cache: usize, state: &mut State) {
        if state.first {
            state.first = false;
        } else {
            self.emit(Some(cache), b" ,");
            self.line(Some(cache), 0);
        }
    }

    fn fqn(&mut self, cache: usize, node: Node<'_>) -> Vec<u8> {
        if self.dies.entries[cache].fqn.is_none() {
            let value = self
                .dies
                .find(node.addr(), DieState::Fqn)
                .and_then(|id| self.dies.entries[id].fqn.clone())
                .unwrap_or_default();
            self.dies.entries[cache].fqn = Some(value);
        }
        self.dies.entries[cache].fqn.clone().unwrap_or_default()
    }

    fn resolve_fqns(&mut self, cu: Node<'_>) -> Result<()> {
        let mut stack: Vec<_> = cu
            .children()?
            .into_iter()
            .rev()
            .map(|node| (node, None::<Vec<u8>>))
            .collect();
        let mut visited = HashSet::new();
        while let Some((node, parent)) = stack.pop() {
            if self.dies.find(node.addr(), DieState::Fqn).is_some() {
                continue;
            }
            if !visited.insert(node.addr()) {
                return Err(error("resolve_fqns", &[b"cyclic DWARF containment"]));
            }
            let scoped = matches!(
                node.tag(),
                DW_TAG_namespace | DW_TAG_class_type | DW_TAG_structure_type
            );
            let name = node.string(DW_AT_name);
            let (fqn, prefix) =
                if let Some(parent) = parent.as_deref().filter(|_| scoped || name.is_some()) {
                    let joined = bytes(&[parent, b"::", name.unwrap_or(b"<anonymous>")]);
                    (name.map(|_| joined.clone()), scoped.then_some(joined))
                } else if let Some(name) = name {
                    (Some(name.to_vec()), scoped.then(|| name.to_vec()))
                } else {
                    (None, None)
                };
            if let Some(fqn) = fqn.filter(|name| !name.is_empty()) {
                let id = self.dies.get_or_insert(node.addr(), DieState::Fqn);
                self.dies.entries[id].fqn = Some(fqn);
                self.dies.entries[id].state = DieState::Fqn;
            }
            for child in node.children()?.into_iter().rev() {
                stack.push((child, prefix.clone()));
            }
        }
        Ok(())
    }

    fn private(&mut self, node: Node<'_>) -> Result<bool> {
        let Some(number) = node.udata(DW_AT_decl_file) else {
            return Ok(false);
        };
        let key = usize::try_from(number)
            .map_err(|_| error("is_definition_private", &[b"file index overflow"]))?;
        let cached = self.source_files.get(key);
        if cached >= 0 {
            return Ok(cached != 0);
        }
        let private = node.source_file(number)?.ends_with(b".c");
        self.source_files.set(key, i32::from(private));
        Ok(private)
    }

    fn definition(&mut self, node: Node<'_>, cache: usize) -> Result<bool> {
        if node.flag(DW_AT_declaration) == Some(true) {
            return Ok(false);
        }
        let fqn = self.dies.entries[cache].fqn.as_deref().unwrap_or_default();
        if self.rules.is_declonly(fqn) {
            return Ok(false);
        }
        Ok(!self.private(node)?)
    }

    fn attribute(&mut self, node: Node<'_>, cache: usize, attr: u32) -> Result<()> {
        let Some(mut value) = node.udata(attr) else {
            return Ok(());
        };
        let name = match attr {
            DW_AT_accessibility => "accessibility",
            DW_AT_alignment => "alignment",
            DW_AT_bit_size => "bit_size",
            DW_AT_encoding => "encoding",
            DW_AT_data_bit_offset => "data_bit_offset",
            DW_AT_data_member_location => "data_member_location",
            DW_AT_discr_value => "discr_value",
            DW_AT_byte_size => {
                if self.diag.options.stable {
                    let fqn = self.dies.entries[cache].fqn.as_deref().unwrap_or_default();
                    if let Some(overridden) = self.rules.byte_size(fqn)? {
                        value = overridden;
                    }
                }
                "byte_size"
            }
            _ => return Err(error("process_type", &[b"unknown numeric attribute"])),
        };
        self.emit(Some(cache), format!(" {name}({value})").as_bytes());
        Ok(())
    }

    fn modifier<'a>(&self, node: Node<'a>, cache: usize, name: &[u8]) -> Vec<Action<'a>> {
        vec![
            text(cache, name),
            Action::Fqn(node, cache),
            text(cache, b" {"),
            Action::Line(Some(cache), 1),
            Action::TypeAttribute(node, cache),
            Action::Line(Some(cache), -1),
            text(cache, b"}"),
            Action::Attribute(node, cache, DW_AT_byte_size),
            Action::Attribute(node, cache, DW_AT_alignment),
        ]
    }

    fn subroutine<'a>(&self, node: Node<'a>, cache: usize, name: &[u8]) -> Vec<Action<'a>> {
        vec![
            text(cache, name),
            text(cache, b" ("),
            Action::Line(Some(cache), 1),
            Action::Children(node, cache, Filter::Tag(DW_TAG_formal_parameter)),
            Action::Line(Some(cache), -1),
            text(cache, b")"),
            Action::Line(Some(cache), 0),
            text(cache, b"-> "),
            Action::TypeAttribute(node, cache),
        ]
    }

    fn structure<'a>(
        &self,
        node: Node<'a>,
        cache: usize,
        name: &[u8],
        filter: Filter,
    ) -> Vec<Action<'a>> {
        vec![
            text(cache, name),
            Action::Fqn(node, cache),
            text(cache, b" {"),
            Action::Line(Some(cache), 1),
            Action::StructureContents(node, cache, filter),
        ]
    }

    fn body<'a>(
        &mut self,
        node: Node<'a>,
        cache: usize,
        state: &mut State,
    ) -> Result<Vec<Action<'a>>> {
        let modifier = match node.tag() {
            DW_TAG_atomic_type => Some(b"atomic_type".as_slice()),
            DW_TAG_const_type => Some(b"const_type".as_slice()),
            DW_TAG_immutable_type => Some(b"immutable_type".as_slice()),
            DW_TAG_packed_type => Some(b"packed_type".as_slice()),
            DW_TAG_pointer_type => Some(b"pointer_type".as_slice()),
            DW_TAG_reference_type => Some(b"reference_type".as_slice()),
            DW_TAG_restrict_type => Some(b"restrict_type".as_slice()),
            DW_TAG_rvalue_reference_type => Some(b"rvalue_reference_type".as_slice()),
            DW_TAG_shared_type => Some(b"shared_type".as_slice()),
            DW_TAG_template_type_parameter => Some(b"template_type_parameter_type".as_slice()),
            DW_TAG_volatile_type => Some(b"volatile_type".as_slice()),
            DW_TAG_typedef => Some(b"typedef_type".as_slice()),
            _ => None,
        };
        if let Some(name) = modifier {
            return Ok(self.modifier(node, cache, name));
        }
        Ok(match node.tag() {
            DW_TAG_formal_parameter | DW_TAG_member => {
                let mut name = node.string(DW_AT_name).map(<[u8]>::to_vec);
                if self.diag.options.stable {
                    if name
                        .as_ref()
                        .is_some_and(|name| name.starts_with(b"__kabi_"))
                    {
                        name = None;
                    }
                    state.original_name = None;
                }
                self.comma(cache, state);
                vec![
                    text(
                        cache,
                        if node.tag() == DW_TAG_member {
                            b"member "
                        } else {
                            b"formal_parameter "
                        },
                    ),
                    Action::TypeAttribute(node, cache),
                    Action::MemberTail(node, cache, name),
                ]
            }
            DW_TAG_subrange_type => {
                let count = node
                    .udata(DW_AT_count)
                    .or_else(|| node.udata(DW_AT_upper_bound).map(|n| n.wrapping_add(1)));
                vec![text(
                    cache,
                    count
                        .map(|n| format!("[{n}]"))
                        .unwrap_or_else(|| "[]".into())
                        .as_bytes(),
                )]
            }
            DW_TAG_array_type => vec![
                text(cache, b"array_type"),
                Action::Children(node, cache, Filter::Tag(DW_TAG_subrange_type)),
                text(cache, b" {"),
                Action::Line(Some(cache), 1),
                Action::TypeAttribute(node, cache),
                Action::Line(Some(cache), -1),
                text(cache, b"}"),
            ],
            DW_TAG_subroutine_type => self.subroutine(node, cache, b"subroutine_type"),
            DW_TAG_variant | DW_TAG_variant_part => {
                self.comma(cache, state);
                let variant = node.tag() == DW_TAG_variant;
                let mut actions = vec![
                    text(
                        cache,
                        if variant {
                            b"variant {"
                        } else {
                            b"variant_part {"
                        },
                    ),
                    Action::Line(Some(cache), 1),
                    Action::Children(
                        node,
                        cache,
                        if variant {
                            Filter::Tag(DW_TAG_member)
                        } else {
                            Filter::All
                        },
                    ),
                    Action::Line(Some(cache), -1),
                    text(cache, b"}"),
                ];
                if variant {
                    actions.push(Action::Attribute(node, cache, DW_AT_discr_value));
                }
                actions
            }
            DW_TAG_class_type => self.structure(node, cache, b"class_type", Filter::Structure),
            DW_TAG_structure_type => {
                self.structure(node, cache, b"structure_type", Filter::Structure)
            }
            DW_TAG_union_type => match self.union_status(node)? {
                KabiStatus::Normal => self.structure(node, cache, b"union_type", Filter::Structure),
                KabiStatus::Ignored => Vec::new(),
                KabiStatus::Reserved(placeholder, name) => {
                    state.original_name = name;
                    vec![Action::Type(placeholder, Some(cache))]
                }
            },
            DW_TAG_enumeration_type => self.structure(
                node,
                cache,
                b"enumeration_type",
                Filter::Tag(DW_TAG_enumerator),
            ),
            DW_TAG_enumerator => {
                let mut override_value = None;
                if self.diag.options.stable {
                    let fqn = self.fqn(cache, node);
                    if self
                        .rules
                        .is_enumerator_ignored(&state.expansion.current_fqn, &fqn)
                    {
                        return Ok(Vec::new());
                    }
                    override_value = self
                        .rules
                        .enumerator_value(&state.expansion.current_fqn, &fqn)?;
                }
                self.comma(cache, state);
                let mut actions = vec![text(cache, b"enumerator"), Action::Fqn(node, cache)];
                if let Some(value) = override_value.or_else(|| node.udata(DW_AT_const_value)) {
                    actions.push(text(cache, b" = "));
                    actions.push(text(cache, value.to_string().as_bytes()));
                }
                actions
            }
            DW_TAG_base_type => vec![
                text(cache, b"base_type"),
                Action::Fqn(node, cache),
                Action::Attribute(node, cache, DW_AT_byte_size),
                Action::Attribute(node, cache, DW_AT_encoding),
                Action::Attribute(node, cache, DW_AT_alignment),
            ],
            DW_TAG_unspecified_type => vec![text(cache, b"unspecified_type")],
            tag => {
                return Err(error(
                    "process_type",
                    &[format!("unexpected type: {tag:x}").as_bytes()],
                ))
            }
        })
    }

    fn union_status<'a>(&self, node: Node<'a>) -> Result<KabiStatus<'a>> {
        if !self.diag.options.stable {
            return Ok(KabiStatus::Normal);
        }
        for member in node
            .children()?
            .into_iter()
            .filter(|node| node.tag() == DW_TAG_member)
            .take(2)
        {
            let ty = member.reference(DW_AT_type).ok_or_else(|| {
                error(
                    "check_union_member_kabi_status",
                    &[b"union member missing a type?"],
                )
            })?;
            if let Some(status) = named_kabi_status(member, ty) {
                return Ok(status);
            }
            if ty.tag() == DW_TAG_structure_type {
                if let Some(first) = ty
                    .children()?
                    .into_iter()
                    .find(|node| node.tag() == DW_TAG_member)
                {
                    let name = first.string(DW_AT_name).unwrap_or_default();
                    if name.starts_with(b"__kabi_reserved") || name.starts_with(b"__kabi_renamed") {
                        let placeholder = first.reference(DW_AT_type).ok_or_else(|| {
                            error(
                                "check_struct_member_kabi_status",
                                &[b"structure member missing a type?"],
                            )
                        })?;
                        if let Some(status) = named_kabi_status(first, placeholder) {
                            return Ok(status);
                        }
                    } else if name.starts_with(b"__kabi_ignored") {
                        return Ok(KabiStatus::Ignored);
                    }
                }
            }
        }
        Ok(KabiStatus::Normal)
    }

    fn ignored(&self, node: Node<'_>) -> Result<bool> {
        if !self.diag.options.stable {
            return Ok(false);
        }
        let ty = node
            .reference(DW_AT_type)
            .ok_or_else(|| error("is_kabi_ignored", &[b"member missing a type?"]))?;
        Ok(ty.tag() == DW_TAG_union_type && matches!(self.union_status(ty)?, KabiStatus::Ignored))
    }

    fn run<'a>(&mut self, actions: Vec<Action<'a>>, state: &mut State) -> Result<()> {
        let mut stack = Vec::new();
        let mut active = HashSet::new();
        push_actions(&mut stack, actions);
        while let Some(action) = stack.pop() {
            match action {
                Action::Type(node, parent) => {
                    let saved = state.expansion.clone();
                    let tag = node.tag();
                    let mut want = DieState::Complete;
                    if expanded_type(tag) {
                        if state.expanded.contains(&node.addr()) {
                            state.expansion.expand = false;
                        }
                        if state.expansion.expand {
                            state.expanded.insert(node.addr());
                        } else {
                            want = DieState::Unexpanded;
                        }
                    }
                    // A recursive typedef can legitimately be revisited after
                    // discovering an aggregate. Reject only cycles that make
                    // no progress in either aggregate expansion state.
                    let key = (
                        node.addr(),
                        want as u32,
                        state.expansion.expand,
                        state.expanded.len(),
                    );
                    if !active.insert(key) {
                        return Err(error(
                            "process_type",
                            &[b"cyclic DWARF type without an aggregate boundary"],
                        ));
                    }
                    let cache = self.dies.get_or_insert(node.addr(), want);
                    let created = self.dies.entries[cache].state != want;
                    if created {
                        self.trace(
                            92,
                            &[format!(
                                "addr 0x{:x} tag {tag:x} -- {} -> {}",
                                node.addr(),
                                self.dies.entries[cache].state.name(),
                                want.name()
                            )
                            .as_bytes()],
                        );
                    } else {
                        self.trace(
                            92,
                            &[format!(
                                "cached addr 0x{:x} tag {tag:x} -- {}",
                                node.addr(),
                                want.name()
                            )
                            .as_bytes()],
                        );
                    }
                    stack.push(Action::Finish {
                        node,
                        parent,
                        cache,
                        want,
                        saved,
                        created,
                        active: key,
                    });
                    if created {
                        let actions = self.body(node, cache, state)?;
                        push_actions(&mut stack, actions);
                    } else {
                        stack.push(Action::Cached(node, cache, 0));
                    }
                }
                Action::Finish {
                    node,
                    parent,
                    cache,
                    want,
                    saved,
                    created,
                    active: key,
                } => {
                    if created {
                        let parent_label = parent
                            .map(|id| format!("0x{:x}", id + 1))
                            .unwrap_or_else(|| "(nil)".into());
                        self.trace(
                            91,
                            &[format!(
                                "parent {parent_label} cache 0x{:x} die addr 0x{:x} tag {:x}",
                                cache + 1,
                                node.addr(),
                                node.tag()
                            )
                            .as_bytes()],
                        );
                        self.dies.entries[cache].tag = node.tag();
                        self.dies.entries[cache].state = want;
                    }
                    if let Some(parent) = parent {
                        self.dies.entries[parent]
                            .fragments
                            .push(Fragment::Die(node.addr()));
                    }
                    state.expansion = saved;
                    active.remove(&key);
                }
                Action::Text(cache, value) => self.emit(cache, &value),
                Action::Line(cache, change) => self.line(cache, change),
                Action::Fqn(node, cache) => {
                    let value = self.fqn(cache, node);
                    if !value.is_empty() {
                        self.emit(Some(cache), b" ");
                    }
                    self.emit(Some(cache), &value);
                }
                Action::Attribute(node, cache, attr) => self.attribute(node, cache, attr)?,
                Action::TypeAttribute(node, cache) => {
                    if let Some(ty) = node.reference(DW_AT_type) {
                        stack.push(Action::Type(ty, Some(cache)));
                    } else {
                        self.emit(Some(cache), b"base_type void");
                    }
                }
                Action::Children(node, cache, filter) => {
                    state.first = true;
                    stack.push(Action::First(false));
                    for child in node.children()?.into_iter().rev() {
                        stack.push(Action::Child(child, cache, filter));
                    }
                }
                Action::Child(node, cache, filter) => {
                    let include = match filter {
                        Filter::All => true,
                        Filter::Tag(tag) => node.tag() == tag,
                        Filter::Structure => match node.tag() {
                            DW_TAG_member => !self.ignored(node)?,
                            DW_TAG_variant_part => true,
                            DW_TAG_class_type
                            | DW_TAG_enumeration_type
                            | DW_TAG_structure_type
                            | DW_TAG_template_type_parameter
                            | DW_TAG_union_type
                            | DW_TAG_subprogram => false,
                            tag => {
                                return Err(error(
                                    "___process_structure_type",
                                    &[format!("unexpected structure_type child: {tag:x}")
                                        .as_bytes()],
                                ))
                            }
                        },
                    };
                    if include {
                        stack.push(Action::Type(node, Some(cache)));
                    }
                }
                Action::First(first) => state.first = first,
                Action::MemberTail(node, cache, mut name) => {
                    if self.diag.options.stable && state.original_name.is_some() {
                        name.clone_from(&state.original_name);
                    }
                    if let Some(name) = name {
                        self.emit(Some(cache), b" ");
                        self.emit(Some(cache), &name);
                    }
                    for attr in [
                        DW_AT_accessibility,
                        DW_AT_bit_size,
                        DW_AT_data_bit_offset,
                        DW_AT_data_member_location,
                    ] {
                        self.attribute(node, cache, attr)?;
                    }
                }
                Action::StructureContents(node, cache, filter) => {
                    let expand = state.expansion.expand && self.definition(node, cache)?;
                    stack.push(Action::StructureEnd(node, cache, expand));
                    if expand {
                        state.expansion.current_fqn =
                            self.dies.entries[cache].fqn.clone().unwrap_or_default();
                        stack.push(Action::Children(node, cache, filter));
                    }
                }
                Action::StructureEnd(node, cache, expand) => {
                    self.line(Some(cache), -1);
                    self.emit(Some(cache), b"}");
                    if expand {
                        self.attribute(node, cache, DW_AT_byte_size)?;
                        self.attribute(node, cache, DW_AT_alignment)?;
                    }
                }
                Action::Cached(node, cache, index) => {
                    let Some(fragment) = self.dies.entries[cache].fragments.get(index).cloned()
                    else {
                        continue;
                    };
                    stack.push(Action::Cached(node, cache, index + 1));
                    match fragment {
                        Fragment::String(value) => {
                            self.trace(
                                94,
                                &[
                                    format!("cache 0x{:x} STRING '", cache + 1).as_bytes(),
                                    &value,
                                    b"'",
                                ],
                            );
                            self.emit(None, &value);
                        }
                        Fragment::Linebreak(change) => self.line(None, change),
                        Fragment::Die(address) => {
                            let child = node.from_address(address)?;
                            self.trace(
                                94,
                                &[format!(
                                    "cache 0x{:x} DIE addr {address:x} tag {:x}",
                                    cache + 1,
                                    child.tag()
                                )
                                .as_bytes()],
                            );
                            stack.push(Action::Type(child, None));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn symbol(&mut self, id: usize, node: Node<'_>, subprogram: bool) -> Result<()> {
        self.symbols.set_die(id, node.addr())?;
        let cache = self.dies.get_or_insert(node.addr(), DieState::Symbol);
        if self.dies.entries[cache].state != DieState::Incomplete {
            return Ok(());
        }
        self.dies.entries[cache].tag = node.tag();
        self.diag
            .debug("process_symbol", &[&self.symbols.entries[id].name]);
        let actions = if subprogram {
            self.subroutine(node, cache, b"subprogram")
        } else {
            vec![
                text(cache, b"variable "),
                Action::TypeAttribute(node, cache),
            ]
        };
        self.run(actions, &mut State::new())?;
        self.dies.entries[cache].state = DieState::Symbol;
        if self.diag.options.dump_dies {
            self.diag.print(&[b"\n"]);
        }
        Ok(())
    }

    fn exported(&mut self, cu: Node<'_>) -> Result<()> {
        let mut stack: Vec<_> = cu.children()?.into_iter().rev().collect();
        let mut visited = HashSet::new();
        while let Some(node) = stack.pop() {
            if !visited.insert(node.addr()) {
                return Err(error(
                    "process_exported_symbols",
                    &[b"cyclic DWARF containment"],
                ));
            }
            match node.tag() {
                DW_TAG_namespace | DW_TAG_class_type | DW_TAG_structure_type => {
                    stack.extend(node.children()?.into_iter().rev());
                }
                DW_TAG_subprogram | DW_TAG_variable => {
                    let source = node.reference(DW_AT_abstract_origin).unwrap_or(node);
                    let symbol = symbol_name(node)
                        .and_then(|name| self.symbols.get(name))
                        .or_else(|| symbol_name(source).and_then(|name| self.symbols.get(name)));
                    let Some(id) = symbol else {
                        continue;
                    };
                    let name = symbol_name(source).unwrap_or_default();
                    if is_symbol_ptr(name) {
                        let pointer = source
                            .reference(DW_AT_type)
                            .filter(|node| node.tag() == DW_TAG_pointer_type)
                            .ok_or_else(|| {
                                error("save_symbol_ptr", &[name, b" must be a pointer type!"])
                            })?;
                        let ty = pointer.reference(DW_AT_type).ok_or_else(|| {
                            error(
                                "save_symbol_ptr",
                                &[name, b" pointer missing a type attribute?"],
                            )
                        })?;
                        self.symbols.set_ptr(
                            id,
                            if ty.tag() == DW_TAG_subroutine_type {
                                ty.addr()
                            } else {
                                pointer.addr()
                            },
                        )?;
                    } else {
                        self.symbol(id, source, node.tag() == DW_TAG_subprogram)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn pointers(&mut self, cu: Node<'_>) -> Result<()> {
        for id in self.symbols.ordered_indices() {
            let symbol = &self.symbols.entries[id];
            if symbol.state != SymbolState::Unprocessed {
                continue;
            }
            let Some(address) = symbol.ptr_die_addr else {
                continue;
            };
            self.diag.debug("process_symbol_ptr", &[&symbol.name]);
            let node = cu.from_address(address).map_err(|_| {
                error(
                    "process_symbol_ptr",
                    &[
                        b"dwarf_die_addr_die failed for symbol ptr: '",
                        &symbol.name,
                        b"'",
                    ],
                )
            })?;
            self.symbol(id, node, node.tag() == DW_TAG_subroutine_type)?;
        }
        Ok(())
    }
}

enum KabiStatus<'a> {
    Normal,
    Reserved(Node<'a>, Option<Vec<u8>>),
    Ignored,
}

fn named_kabi_status<'a>(member: Node<'a>, ty: Node<'a>) -> Option<KabiStatus<'a>> {
    let suffix = member.string(DW_AT_name)?.strip_prefix(b"__kabi_")?;
    if suffix.starts_with(b"reserved") {
        Some(KabiStatus::Reserved(ty, None))
    } else if suffix.starts_with(b"ignored") {
        Some(KabiStatus::Ignored)
    } else {
        suffix
            .strip_prefix(b"renamed")
            .map(|name| KabiStatus::Reserved(ty, Some(name.to_vec())))
    }
}

pub(crate) fn process_module(
    module: Module<'_>,
    symbols: &mut Symbols,
    dies: &mut DieMap,
    rules: &Rules,
    diag: &mut Diagnostics,
) -> Result<()> {
    diag.debug("process_module", &[module.name()]);
    let mut engine = Engine {
        symbols,
        dies,
        rules,
        diag,
        source_files: Cache::default(),
        linebreak: false,
        indentation: 0,
    };
    for cu in module.units()? {
        engine.resolve_fqns(cu)?;
        engine.exported(cu)?;
        engine.pointers(cu)?;
        engine.source_files.clear();
    }
    Ok(())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
