// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2024 Google LLC
//! Checked kABI rule records with last-definition-wins lookup.

use crate::elf::ElfFile;
use crate::gendwarfksyms_header::{bytes, error, Diagnostics, Result};
use std::collections::HashMap;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Kind {
    DeclOnly,
    EnumeratorIgnore,
    EnumeratorValue,
    ByteSize,
    TypeString,
}

#[derive(Default)]
pub(crate) struct Rules {
    entries: HashMap<(Kind, Vec<u8>), Vec<u8>>,
}

impl Rules {
    pub(crate) fn read(data: &[u8], diag: &mut Diagnostics) -> Result<Self> {
        let mut rules = Self::default();
        if !diag.options.stable {
            return Ok(rules);
        }
        let image = ElfFile::parse_any(data)
            .map_err(|message| error("kabi_read_rules", &[message.as_bytes()]))?;
        let section = image
            .find_section(b".discard.gendwarfksyms.kabi_rules")
            .map_err(|message| error("kabi_read_rules", &[message.as_bytes()]))?;
        let Some(section) = section else {
            diag.debug("kabi_read_rules", &[b"kABI rules not found"]);
            return Ok(rules);
        };
        let mut remaining = image
            .section_data(&section)
            .map_err(|message| error("kabi_read_rules", &[message.as_bytes()]))?;
        if remaining.len() < 6 {
            return Err(error(
                "kabi_read_rules",
                &[format!("kABI rule section too small: {} bytes", remaining.len()).as_bytes()],
            ));
        }
        if remaining.last() != Some(&0) {
            return Err(error(
                "kabi_read_rules",
                &[b"kABI rules are not null-terminated"],
            ));
        }
        while remaining.len() > 6 {
            let version = field(&mut remaining)?;
            if version != b"1" {
                return Err(error(
                    "kabi_read_rules",
                    &[b"unsupported kABI rule version: '", version, b"'"],
                ));
            }
            let tag = field(&mut remaining)?;
            let kind = match tag {
                b"declonly" => Kind::DeclOnly,
                b"enumerator_ignore" => Kind::EnumeratorIgnore,
                b"enumerator_value" => Kind::EnumeratorValue,
                b"byte_size" => Kind::ByteSize,
                b"type_string" => Kind::TypeString,
                _ => {
                    return Err(error(
                        "kabi_read_rules",
                        &[b"unsupported kABI rule type: '", tag, b"'"],
                    ))
                }
            };
            let target = field(&mut remaining)?;
            let value = field(&mut remaining)?;
            rules
                .entries
                .insert((kind, target.to_vec()), value.to_vec());
            diag.debug(
                "kabi_read_rules",
                &[
                    b"kABI rule: type: '",
                    tag,
                    b"', target: '",
                    target,
                    b"', value: '",
                    value,
                    b"'",
                ],
            );
        }
        if !remaining.is_empty() {
            diag.warn(
                "kabi_read_rules",
                &[b"unexpected data at the end of the kABI rules section"],
            );
        }
        Ok(rules)
    }

    fn get(&self, kind: Kind, target: &[u8]) -> Option<&[u8]> {
        if target.is_empty() {
            return None;
        }
        self.entries
            .get(&(kind, target.to_vec()))
            .map(Vec::as_slice)
    }

    fn enumerator(&self, kind: Kind, fqn: &[u8], field: &[u8]) -> Option<&[u8]> {
        if fqn.is_empty() || field.is_empty() {
            return None;
        }
        self.get(kind, &bytes(&[fqn, b" ", field]))
    }

    pub(crate) fn is_declonly(&self, fqn: &[u8]) -> bool {
        self.get(Kind::DeclOnly, fqn).is_some()
    }

    pub(crate) fn is_enumerator_ignored(&self, fqn: &[u8], field: &[u8]) -> bool {
        self.enumerator(Kind::EnumeratorIgnore, fqn, field)
            .is_some()
    }

    pub(crate) fn enumerator_value(&self, fqn: &[u8], field: &[u8]) -> Result<Option<u64>> {
        self.enumerator(Kind::EnumeratorValue, fqn, field)
            .map(unsigned)
            .transpose()
    }

    pub(crate) fn byte_size(&self, fqn: &[u8]) -> Result<Option<u64>> {
        self.get(Kind::ByteSize, fqn).map(unsigned).transpose()
    }

    pub(crate) fn type_string(&self, name: &[u8]) -> Option<&[u8]> {
        self.get(Kind::TypeString, name)
    }
}

fn field<'a>(remaining: &mut &'a [u8]) -> Result<&'a [u8]> {
    let end = remaining
        .iter()
        .position(|&byte| byte == 0)
        .ok_or_else(|| error("get_rule_field", &[b"unexpected end of kABI rules"]))?;
    let (value, tail) = remaining.split_at(end);
    *remaining = &tail[1..];
    Ok(value)
}

/// strtoul(base 10) semantics, without C locale or unchecked arithmetic.
fn unsigned(value: &[u8]) -> Result<u64> {
    let invalid = || {
        error(
            "get_ulong_value",
            &[b"invalid unsigned value '", value, b"'"],
        )
    };
    if value.is_empty() {
        return Ok(0);
    }
    let mut input = value;
    while input
        .first()
        .is_some_and(|&byte| matches!(byte, 9..=13 | 32))
    {
        input = &input[1..];
    }
    let negative = input.first() == Some(&b'-');
    if matches!(input.first(), Some(b'+' | b'-')) {
        input = &input[1..];
    }
    if input.is_empty() {
        return Err(invalid());
    }
    let mut result = 0u64;
    for &byte in input {
        if !byte.is_ascii_digit() {
            return Err(invalid());
        }
        result = result
            .checked_mul(10)
            .and_then(|n| n.checked_add(u64::from(byte - b'0')))
            .filter(|&n| n <= std::ffi::c_ulong::MAX as u64)
            .ok_or_else(invalid)?;
    }
    Ok(if negative {
        (0u64.wrapping_sub(result)) & std::ffi::c_ulong::MAX as u64
    } else {
        result
    })
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
