// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2024 Google LLC
//! Export records with explicit ownership and stable C-compatible iteration.

use crate::elf::ElfFile;
use crate::gendwarfksyms_header::{
    bytes, error, hash_bytes, io_error, Diagnostics, Result, SYMBOL_PTR_PREFIX,
};
use std::collections::HashMap;
use std::io::{BufRead, Write};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum SymbolState {
    Unprocessed,
    Mapped,
    Processed,
}

pub(crate) struct Symbol {
    pub(crate) name: Vec<u8>,
    pub(crate) addr: (u32, u64),
    pub(crate) state: SymbolState,
    pub(crate) die_addr: Option<usize>,
    pub(crate) ptr_die_addr: Option<usize>,
    pub(crate) crc: u32,
}

#[derive(Default)]
pub(crate) struct Symbols {
    pub(crate) entries: Vec<Symbol>,
    names: HashMap<Vec<u8>, Vec<usize>>,
    addresses: HashMap<(u32, u64), Vec<usize>>,
}

pub(crate) fn is_symbol_ptr(name: &[u8]) -> bool {
    name.starts_with(SYMBOL_PTR_PREFIX)
}

impl Symbols {
    fn matching(&self, name: &[u8]) -> Vec<usize> {
        if name.is_empty() {
            return Vec::new();
        }
        let name = name.strip_prefix(SYMBOL_PTR_PREFIX).unwrap_or(name);
        let Some(&first) = self.names.get(name).and_then(|ids| ids.last()) else {
            return Vec::new();
        };
        let mut result = vec![first];
        let addr = self.entries[first].addr;
        if addr.0 != 0 {
            if let Some(aliases) = self.addresses.get(&addr) {
                result.extend(aliases.iter().rev().copied().filter(|&id| id != first));
            }
        }
        result
    }

    pub(crate) fn read_exports(mut input: impl BufRead, diag: &mut Diagnostics) -> Result<Self> {
        let mut symbols = Self::default();
        let mut line = Vec::new();
        loop {
            line.clear();
            // Like getline(), terminate on EOF or an input-stream failure.
            match input.read_until(b'\n', &mut line) {
                Ok(0) | Err(_) => break,
                Ok(_) => (),
            }
            // getline feeds sscanf("%ms"), which reads only the first C word.
            let visible = line.split(|&byte| byte == 0).next().unwrap_or_default();
            let space = |byte: &u8| matches!(*byte, 9..=13 | 32);
            let name = visible
                .split(space)
                .find(|word| !word.is_empty())
                .ok_or_else(|| {
                    error("symbol_read_exports", &[b"malformed input line: ", visible])
                })?;
            if !symbols.matching(name).is_empty() {
                continue;
            }
            let id = symbols.entries.len();
            symbols.entries.push(Symbol {
                name: name.to_vec(),
                addr: (0, 0),
                state: SymbolState::Unprocessed,
                die_addr: None,
                ptr_die_addr: None,
                crc: 0,
            });
            symbols.names.entry(name.to_vec()).or_default().push(id);
            diag.debug("symbol_read_exports", &[name]);
        }
        diag.debug(
            "symbol_read_exports",
            &[
                symbols.entries.len().to_string().as_bytes(),
                b" exported symbols",
            ],
        );
        Ok(symbols)
    }

    pub(crate) fn get(&self, name: &[u8]) -> Option<usize> {
        self.matching(name)
            .into_iter()
            .filter(|&id| self.entries[id].state == SymbolState::Unprocessed)
            .last()
    }

    pub(crate) fn ordered_indices(&self) -> Vec<usize> {
        let mut ids: Vec<_> = (0..self.entries.len()).collect();
        ids.sort_by_key(|&id| {
            (
                hash_bytes(&self.entries[id].name) & 4095,
                std::cmp::Reverse(id),
            )
        });
        ids
    }

    fn targets(&self, id: usize, function: &str) -> Result<Vec<usize>> {
        let name = &self.entries[id].name;
        let targets = self.matching(name);
        if targets.is_empty() {
            return Err(error(function, &[b"no matching symbols: '", name, b"'"]));
        }
        Ok(targets)
    }

    pub(crate) fn set_die(&mut self, id: usize, addr: usize) -> Result<()> {
        for target in self.targets(id, "symbol_set_die")? {
            self.entries[target].die_addr = Some(addr);
            self.entries[target].state = SymbolState::Mapped;
        }
        Ok(())
    }

    pub(crate) fn set_ptr(&mut self, id: usize, addr: usize) -> Result<()> {
        for target in self.targets(id, "symbol_set_ptr")? {
            self.entries[target].ptr_die_addr = Some(addr);
        }
        Ok(())
    }

    pub(crate) fn set_crc(&mut self, id: usize, crc: u32, diag: &mut Diagnostics) -> Result<()> {
        for target in self.targets(id, "symbol_set_crc")? {
            let symbol = &mut self.entries[target];
            if symbol.state == SymbolState::Processed && symbol.crc != crc {
                diag.warn(
                    "set_crc",
                    &[
                        b"overriding version for symbol ",
                        &symbol.name,
                        format!(" (crc {:x} vs. {crc:x})", symbol.crc).as_bytes(),
                    ],
                );
            }
            symbol.state = SymbolState::Processed;
            symbol.crc = crc;
        }
        Ok(())
    }

    pub(crate) fn read_symtab(&mut self, data: &[u8], diag: &mut Diagnostics) -> Result<()> {
        let checked = |result: std::result::Result<_, String>| {
            result.map_err(|message| error("elf_for_each_global", &[message.as_bytes()]))
        };
        let image = checked(ElfFile::parse_any(data))?;
        let sections = image
            .sections()
            .map_err(|message| error("elf_for_each_global", &[message.as_bytes()]))?;
        let extended = sections
            .iter()
            .find(|section| section.kind == 18)
            .map(|section| image.section_data(section))
            .transpose()
            .map_err(|message| error("elf_for_each_global", &[message.as_bytes()]))?;
        let width = if image.word_size() == 8 { 24 } else { 16 };
        for section in sections.iter().filter(|section| section.kind == 2) {
            if section.entry_size != width {
                return Err(error(
                    "elf_for_each_global",
                    &[
                        format!("expected sh_entsize ({}) to be {width}", section.entry_size)
                            .as_bytes(),
                    ],
                ));
            }
            let mut table = *section;
            table.size -= table.size % width;
            let symbols = image
                .symbols(&table)
                .map_err(|message| error("elf_for_each_global", &[message.as_bytes()]))?;
            let strings = image
                .section(section.link as usize)
                .map_err(|message| error("elf_for_each_global", &[message.as_bytes()]))?;
            for (index, symbol) in symbols.iter().enumerate().skip(1) {
                if symbol.binding == 0 {
                    continue;
                }
                let name = image
                    .string(&strings, symbol.name)
                    .map_err(|message| error("elf_for_each_global", &[message.as_bytes()]))?;
                if name.is_empty() {
                    continue;
                }
                let section_id = if symbol.section == 0xffff {
                    let offset = index.checked_mul(4).ok_or_else(|| {
                        error("elf_for_each_global", &[b"extended symbol index overflow"])
                    })?;
                    let raw: [u8; 4] = extended
                        .and_then(|data| data.get(offset..offset + 4))
                        .ok_or_else(|| {
                            error("elf_for_each_global", &[b"invalid extended symbol index"])
                        })?
                        .try_into()
                        .unwrap();
                    if image.little_endian() {
                        u32::from_le_bytes(raw)
                    } else {
                        u32::from_be_bytes(raw)
                    }
                } else {
                    u32::from(symbol.section)
                };
                if section_id == 0 {
                    continue;
                }
                let addr = (section_id, symbol.value);
                for id in self.matching(name) {
                    let entry = &mut self.entries[id];
                    if entry.addr.0 == 0 {
                        entry.addr = addr;
                        self.addresses.entry(addr).or_default().push(id);
                        diag.debug(
                            "set_symbol_addr",
                            &[
                                &entry.name,
                                format!(" -> {{ {}, {:x} }}", addr.0, addr.1).as_bytes(),
                            ],
                        );
                    } else if entry.addr != addr {
                        diag.warn(
                            "set_symbol_addr",
                            &[b"multiple addresses for symbol ", &entry.name, b"?"],
                        );
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn print_versions(
        &self,
        out: &mut impl Write,
        diag: &mut Diagnostics,
    ) -> Result<()> {
        for id in self.ordered_indices() {
            let symbol = &self.entries[id];
            if symbol.state != SymbolState::Processed {
                diag.warn(
                    "symbol_print_versions",
                    &[b"no information for symbol ", &symbol.name],
                );
            }
            let line = bytes(&[
                b"#SYMVER ",
                &symbol.name,
                format!(" 0x{:08x}\n", symbol.crc).as_bytes(),
            ]);
            out.write_all(&line)
                .map_err(|err| io_error("symbol_print_versions", err))?;
        }
        Ok(())
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
