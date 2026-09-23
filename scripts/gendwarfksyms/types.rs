// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2024 Google LLC
//! Expand owned DIE fragments into symtypes records and symbol-version CRCs.

use crate::cache::Cache;
use crate::die::{Die, DieMap, DieState, Fragment};
use crate::gendwarfksyms_header::{bytes, error, io_error, Diagnostics, Result};
use crate::kabi::Rules;
use crate::symbols::{SymbolState, Symbols};
use std::collections::{BTreeMap, HashSet};
use std::io::Write;
use std::rc::Rc;

#[derive(Default)]
struct Expansion {
    // Sharing immutable fragments keeps nested expansion iterative without
    // copying complete type definitions at every reference.
    fragments: Rc<[Vec<u8>]>,
    length: usize,
}

impl Expansion {
    fn new(fragments: Vec<Vec<u8>>) -> Self {
        let length = fragments.iter().map(Vec::len).sum();
        Self {
            fragments: fragments.into(),
            length,
        }
    }

    fn print(&self, diag: &mut Diagnostics) {
        for fragment in self.fragments.iter() {
            diag.print(&[fragment]);
        }
    }
}

#[derive(Default)]
struct TypeMap {
    names: BTreeMap<Vec<u8>, usize>,
    entries: Vec<(Vec<u8>, Expansion)>,
}

impl TypeMap {
    fn add(&mut self, name: &[u8], expansion: Expansion, diag: &mut Diagnostics) -> usize {
        let id = if let Some(&id) = self.names.get(name) {
            if expansion.length <= self.entries[id].1.length {
                return id;
            }
            if diag.options.dump_types {
                diag.debug("type_map_add", &[b"replacing ", name]);
            }
            self.entries[id].1 = expansion;
            id
        } else {
            let id = self.entries.len();
            self.entries.push((name.to_vec(), expansion));
            self.names.insert(name.to_vec(), id);
            if diag.options.dump_types {
                diag.debug("type_map_add", &[b"adding ", name]);
            }
            id
        };
        if diag.options.dump_types {
            diag.print(&[name, b" "]);
            self.entries[id].1.print(diag);
            diag.print(&[b"\n"]);
        }
        id
    }

    fn get(&mut self, name: &[u8], rules: &Rules, diag: &mut Diagnostics) -> Result<Option<usize>> {
        if let Some(&id) = self.names.get(name) {
            return Ok(Some(id));
        }
        if diag.options.stable {
            if let Some(override_) = rules.type_string(name) {
                return Ok(Some(self.add(name, parse(name, override_)?, diag)));
            }
        }
        Ok(None)
    }

    fn write(&self, output: Option<&mut dyn Write>) -> Result<()> {
        let Some(output) = output else {
            return Ok(());
        };
        for (name, &id) in &self.names {
            output
                .write_all(name)
                .and_then(|()| output.write_all(b" "))
                .map_err(|err| io_error("type_map_write", err))?;
            for fragment in self.entries[id].1.fragments.iter() {
                output
                    .write_all(fragment)
                    .map_err(|err| io_error("type_list_write", err))?;
            }
            output
                .write_all(b"\n")
                .map_err(|err| io_error("type_map_write", err))?;
        }
        Ok(())
    }
}

fn is_type_prefix(value: &[u8]) -> bool {
    matches!(value.first(), Some(b's' | b'u' | b'e' | b't')) && value.get(1) == Some(&b'#')
}

fn type_name(die: &Die, diag: &mut Diagnostics) -> Option<Vec<u8>> {
    if die.state == DieState::Incomplete {
        diag.warn(
            "get_type_name",
            &[format!("found incomplete cache entry: {die:p}").as_bytes()],
        );
        return None;
    }
    if matches!(die.state, DieState::Symbol | DieState::Fqn) {
        return None;
    }
    let name = die.fqn.as_deref().filter(|name| !name.is_empty())?;
    // DW_TAG_class_type, structure_type, union_type, enumeration_type, typedef.
    let prefix = match die.tag {
        0x02 | 0x13 => b's',
        0x17 => b'u',
        0x04 => b'e',
        0x16 => b't',
        _ => return None,
    };
    let quote = if name.contains(&b' ') {
        &b"'"[..]
    } else {
        &b""[..]
    };
    Some(bytes(&[&[prefix, b'#'], quote, name, quote]))
}

fn parse(name: &[u8], input: &[u8]) -> Result<Expansion> {
    if input.is_empty() {
        return Err(error(
            "type_parse",
            &[b"empty type string override for '", name, b"'"],
        ));
    }
    let (mut start, mut position) = (0, 0);
    let mut fragments = Vec::new();
    while position < input.len() {
        if !is_type_prefix(&input[position..]) {
            position += 1;
            continue;
        }
        let mut end = position + 2;
        let quoted = input.get(end) == Some(&b'\'');
        if quoted {
            end += 1;
        }
        let marker = if quoted { b'\'' } else { b' ' };
        while end < input.len() && input[end] != marker {
            end += 1;
        }
        if quoted && end == input.len() {
            return Err(error(
                "type_parse",
                &[
                    b"incomplete ",
                    &input[position..position + 1],
                    b"# type reference for '",
                    name,
                    b"' (string : '",
                    input,
                    b"')",
                ],
            ));
        }
        let empty = end == position + if quoted { 3 } else { 2 };
        if empty {
            return Err(error(
                "type_parse",
                &[
                    b"empty ",
                    &input[position..position + 1],
                    b"# type name for '",
                    name,
                    b"' (string: '",
                    input,
                    b"')",
                ],
            ));
        }
        if quoted {
            end += 1;
        }
        if position > start {
            fragments.push(input[start..position].to_vec());
        }
        fragments.push(input[position..end].to_vec());
        start = end;
        position = end;
    }
    if start < input.len() {
        fragments.push(input[start..].to_vec());
    }
    Ok(Expansion::new(fragments))
}

fn expand(
    name: &[u8],
    index: usize,
    dies: &DieMap,
    rules: &Rules,
    diag: &mut Diagnostics,
) -> Result<Expansion> {
    if diag.options.stable {
        if let Some(override_) = rules.type_string(name) {
            return parse(name, override_);
        }
    }
    let mut output = Vec::new();
    let mut stack = vec![(index, 0)];
    let mut active = HashSet::from([index]);
    while let Some((index, offset)) = stack.last_mut() {
        let fragments = &dies.entries[*index].fragments;
        let Some(fragment) = fragments.get(*offset) else {
            active.remove(index);
            stack.pop();
            continue;
        };
        *offset += 1;
        match fragment {
            Fragment::String(value) => output.push(value.clone()),
            Fragment::Linebreak(_) => {
                if !matches!(fragments.get(*offset), Some(Fragment::Linebreak(_))) {
                    output.push(b" ".to_vec());
                }
            }
            Fragment::Die(addr) => {
                let child = dies
                    .find(*addr, DieState::Complete)
                    .or_else(|| dies.find(*addr, DieState::Unexpanded))
                    .ok_or_else(|| {
                        error(
                            "__type_expand",
                            &[format!("unknown child: {addr:x}").as_bytes()],
                        )
                    })?;
                if let Some(name) = type_name(&dies.entries[child], diag) {
                    output.push(name);
                } else {
                    // Named recursion becomes a reference above. An anonymous
                    // cycle is malformed input; reject it instead of overflowing
                    // the C implementation's recursive call stack.
                    if !active.insert(child) {
                        return Err(error(
                            "__type_expand",
                            &[format!("recursive anonymous type: {addr:x}").as_bytes()],
                        ));
                    }
                    stack.push((child, 0));
                }
            }
        }
    }
    Ok(Expansion::new(output))
}

const fn crc_table() -> [u32; 256] {
    let mut table = [0; 256];
    let mut index = 0;
    while index < 256 {
        let mut value = index as u32;
        let mut bit = 0;
        while bit < 8 {
            value = (value >> 1) ^ if value & 1 != 0 { 0xedb8_8320 } else { 0 };
            bit += 1;
        }
        table[index] = value;
        index += 1;
    }
    table
}

fn version(
    expansion: &Expansion,
    types: &mut TypeMap,
    rules: &Rules,
    diag: &mut Diagnostics,
) -> Result<(u32, Vec<u8>)> {
    const CRC_TABLE: [u32; 256] = crc_table();
    let mut crc = !0u32;
    let mut expanded = Vec::new();
    let mut seen = Cache::default();
    // The root expansion is not in type_map and has a NULL C name.
    let mut stack = vec![(None::<usize>, expansion.fragments.clone(), 0)];
    while let Some((name_id, fragments, offset)) = stack.last_mut() {
        let Some(fragment) = fragments.get(*offset) else {
            stack.pop();
            continue;
        };
        *offset += 1;
        if is_type_prefix(fragment) {
            let child = types.get(fragment, rules, diag)?.ok_or_else(|| {
                let name = name_id.map_or(&b"(null)"[..], |id| types.entries[id].0.as_slice());
                error(
                    "__calculate_version",
                    &[
                        b"unknown type reference to '",
                        fragment,
                        b"' when expanding '",
                        name,
                        b"'",
                    ],
                )
            })?;
            if !seen.was_expanded(child) {
                seen.mark_expanded(child);
                stack.push((Some(child), types.entries[child].1.fragments.clone(), 0));
                continue;
            }
        }
        for &byte in fragment {
            crc = CRC_TABLE[((crc ^ byte as u32) & 0xff) as usize] ^ (crc >> 8);
        }
        if diag.options.dump_versions {
            expanded.extend_from_slice(fragment);
        }
    }
    Ok((!crc, expanded))
}

pub(crate) fn generate(
    dies: &mut DieMap,
    symbols: &mut Symbols,
    rules: &Rules,
    diag: &mut Diagnostics,
    output: Option<&mut dyn Write>,
) -> Result<()> {
    let mut types = TypeMap::default();
    for mut index in dies.ordered_indices() {
        if dies.entries[index].mapped {
            continue;
        }
        dies.entries[index].mapped = true;
        if dies.entries[index].state == DieState::Unexpanded {
            if let Some(complete) = dies.find(dies.entries[index].addr, DieState::Complete) {
                index = complete;
                if dies.entries[index].mapped {
                    continue;
                }
                dies.entries[index].mapped = true;
            }
        }
        if let Some(name) = type_name(&dies.entries[index], diag) {
            diag.debug("expand_type", &[&name]);
            let expansion = expand(&name, index, dies, rules, diag)?;
            types.add(&name, expansion, diag);
        }
    }
    for id in symbols.ordered_indices() {
        if !diag.options.symtypes && symbols.entries[id].state == SymbolState::Processed {
            continue;
        }
        let Some(addr) = symbols.entries[id].die_addr else {
            continue;
        };
        let Some(index) = dies.find(addr, DieState::Symbol) else {
            continue;
        };
        let name = symbols.entries[id].name.clone();
        let expansion = expand(&name, index, dies, rules, diag)?;
        if symbols.entries[id].state != SymbolState::Processed {
            let (crc, expanded) = version(&expansion, &mut types, rules, diag)?;
            symbols.set_crc(id, crc, diag)?;
            diag.debug("expand_symbol", &[&name, format!(" = {crc:x}").as_bytes()]);
            if diag.options.dump_versions {
                diag.print(&[&name, b" ", &expanded, b"\n"]);
            }
        }
        if diag.options.symtypes {
            types.add(&name, expansion, diag);
        }
    }
    types.write(output)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
