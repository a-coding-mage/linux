// SPDX-License-Identifier: GPL-2.0-only

//! Postprocess kernel module exports, references, versions, and device aliases.
// Copyright 2003 Kai Germaschewski
// Copyright 2002-2004 Rusty Russell, IBM Corporation
// Copyright 2006-2008 Sam Ravnborg
// Based in part on module-init-tools/depmod.c and file2alias.

#[path = "../elf-parse.rs"]
mod elf;
mod file2alias;
mod modpost_header;
mod sumversion;
mod symsearch;

use elf::{ElfFile, Section, Symbol};
use modpost_header::{Export, Module, Options, Unresolved};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::{env, fs, io};
use symsearch::SymbolSearch;

#[cfg(modpost_target_offsets)]
const EMBEDDED_OFFSETS: Option<&str> = Some(include_str!(env!("MODPOST_DEVICETABLE_OFFSETS")));
#[cfg(not(modpost_target_offsets))]
const EMBEDDED_OFFSETS: Option<&str> = None;
const MAX_UNRESOLVED: usize = 10;
const PATH_MAX: usize = 4096;

fn io_error(error: &io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error ").next().unwrap_or(&text).to_owned()
}

fn read_text(filename: &str) -> Result<String, String> {
    let bytes = fs::read(filename).map_err(|e| format!("{filename}: {}\n", io_error(&e)))?;
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8(bytes[..end].to_vec())
        .map_err(|_| format!("{filename}: invalid UTF-8 text\n"))
}

fn basename(name: &str) -> &str {
    name.rsplit('/').next().unwrap_or(name)
}

fn unique_push(list: &mut Vec<String>, value: &str) {
    if !value.is_empty() && !list.iter().any(|entry| entry == value) {
        list.push(value.to_owned());
    }
}

fn c_unsigned(text: &str, mut radix: u32) -> (u64, usize) {
    let bytes = text.as_bytes();
    let mut pos = bytes
        .iter()
        .take_while(|b| matches!(b, b' ' | b'\t'..=b'\r'))
        .count();
    let negative = bytes.get(pos) == Some(&b'-');
    if matches!(bytes.get(pos), Some(b'+' | b'-')) {
        pos += 1;
    }
    if (radix == 0 || radix == 16)
        && bytes.get(pos) == Some(&b'0')
        && matches!(bytes.get(pos + 1), Some(b'x' | b'X'))
        && bytes.get(pos + 2).is_some_and(|b| b.is_ascii_hexdigit())
    {
        radix = 16;
        pos += 2;
    } else if radix == 0 {
        radix = if bytes.get(pos) == Some(&b'0') { 8 } else { 10 };
    }
    let digits = pos;
    let mut value = 0u64;
    let mut overflow = false;
    while let Some(digit) = bytes.get(pos).and_then(|b| (*b as char).to_digit(radix)) {
        match value
            .checked_mul(u64::from(radix))
            .and_then(|n| n.checked_add(u64::from(digit)))
        {
            Some(next) => value = next,
            None => overflow = true,
        }
        pos += 1;
    }
    if pos == digits {
        return (0, 0);
    }
    if overflow {
        value = u64::MAX;
    } else if negative {
        value = value.wrapping_neg();
    }
    (value, pos)
}

fn parse_options() -> Result<Options, String> {
    let mut args = env::args_os()
        .map(|arg| {
            arg.into_string()
                .map_err(|_| "ERROR: modpost: non-UTF-8 command-line argument\n".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter();
    let program = args.next().unwrap_or_else(|| "modpost".into());
    let mut options = Options::default();
    let mut stopped = false;
    let posix = env::var_os("POSIXLY_CORRECT").is_some();
    while let Some(arg) = args.next() {
        if stopped || !arg.starts_with('-') || arg == "-" {
            options.objects.push(arg);
            stopped |= posix;
            continue;
        }
        if arg == "--" {
            stopped = true;
            continue;
        }
        let mut chars = arg[1..].char_indices().peekable();
        while let Some((pos, option)) = chars.next() {
            let value = if matches!(option, 'i' | 'T' | 'o' | 'u' | 'd') {
                if pos + 2 < arg.len() {
                    chars.by_ref().for_each(drop);
                    Some(arg[pos + 2..].to_owned())
                } else {
                    Some(args.next().ok_or_else(|| {
                        format!("{program}: option requires an argument -- '{option}'\n")
                    })?)
                }
            } else {
                None
            };
            match option {
                'e' => options.external = true,
                'i' => options.dumps.push(value.unwrap()),
                'M' => options.module_enabled = true,
                'm' => options.modversions = true,
                'n' => options.ignore_missing = true,
                'o' => options.dump_output = value,
                'a' => options.all_versions = true,
                'T' => options.files_source = value,
                't' => options.trim = true,
                'u' => options.whitelist = value,
                'W' => (),
                'w' => options.warn_unresolved = true,
                'E' => options.mismatch_error = true,
                'N' => options.allow_missing_namespace = true,
                'd' => options.namespace_output = value,
                'b' => options.basic_versions = true,
                'x' => options.extended_versions = true,
                _ => return Err(format!("{program}: invalid option -- '{option}'\n")),
            }
        }
    }
    Ok(options)
}

struct Object<'a> {
    elf: ElfFile<'a>,
    sections: Vec<Section>,
    section_names: Vec<String>,
    symbols: Vec<Symbol>,
    names: Vec<String>,
    indices: Vec<u32>,
    search: SymbolSearch,
    machine: u16,
    export_section: u32,
}

impl<'a> Object<'a> {
    fn parse(data: &'a [u8], filename: &str) -> Result<Self, String> {
        let elf = ElfFile::parse(data, 1 << 1)?;
        let sections = elf.sections()?;
        let mut names = Vec::new();
        for section in &sections {
            if section.kind != 8 && section.offset > data.len() as u64 {
                return Err(format!(
                    "{filename} is truncated. sechdrs[i].sh_offset={} > sizeof(*hrd)={}\n",
                    section.offset,
                    if elf.word_size() == 8 { 64 } else { 52 }
                ));
            }
            let name = std::str::from_utf8(elf.section_name(section)?)
                .map_err(|_| "invalid UTF-8 section name")?;
            if name == ".modinfo" && section.kind == 8 {
                return Err(format!("{filename} has NOBITS .modinfo\n"));
            }
            elf.section_data(section)?;
            names.push(name.to_owned());
        }
        let table = sections
            .iter()
            .rev()
            .find(|section| section.kind == 2)
            .ok_or_else(|| format!("{filename} has no symtab?\n"))?;
        if let Some(extended) = sections.iter().rev().find(|section| section.kind == 18) {
            if extended.link as usize != table.index {
                return Err(format!(
                    "{filename}: SYMTAB_SHNDX has bad sh_link: {}!={}\n",
                    extended.link, table.index
                ));
            }
        }
        let strings = elf.section(table.link as usize)?;
        let symbols = elf.symbols(table)?;
        let mut symbol_names = Vec::with_capacity(symbols.len());
        let mut indices = Vec::with_capacity(symbols.len());
        for (index, symbol) in symbols.iter().enumerate() {
            let name = std::str::from_utf8(elf.string(&strings, symbol.name)?)
                .map_err(|_| "invalid UTF-8 symbol name")?;
            symbol_names.push(remove_dot(name).to_owned());
            indices.push(elf.symbol_section_index(table, index, symbol)?);
        }
        let machine = elf.machine()?;
        let search = SymbolSearch::new(symbols.iter().enumerate().filter_map(|(index, symbol)| {
            // Build from the original names, before remove_dot() changes them.
            let name = elf.string(&strings, symbol.name).ok()?;
            valid_name(name).then_some((
                indices[index],
                if machine == 40 && symbol.kind == 2 {
                    symbol.value & !1
                } else {
                    symbol.value
                },
                index,
            ))
        }));
        let export_section = names
            .iter()
            .position(|name| name == ".export_symbol")
            .unwrap_or(0) as u32;
        Ok(Self {
            elf,
            sections,
            section_names: names,
            symbols,
            names: symbol_names,
            indices,
            search,
            machine,
            export_section,
        })
    }

    fn sec_name(&self, index: u32) -> &str {
        self.section_names
            .get(index as usize)
            .map(String::as_str)
            .unwrap_or("")
    }

    fn named_data(&self, name: &str) -> Result<&[u8], String> {
        self.section_names
            .iter()
            .rposition(|entry| entry == name)
            .map(|index| self.elf.section_data(&self.sections[index]))
            .unwrap_or(Ok(&[]))
    }

    fn data_at(&self, section: u32, offset: u64) -> Result<&[u8], String> {
        let section = self
            .sections
            .get(section as usize)
            .ok_or("invalid ELF symbol section index")?;
        let data = self.elf.section_data(section)?;
        data.get(usize::try_from(offset).map_err(|_| "ELF symbol offset overflow")?..)
            .ok_or_else(|| "ELF symbol offset outside section".into())
    }

    fn target_value(&self, value: u64) -> u64 {
        if self.elf.word_size() == 4 {
            value & 0xffff_ffff
        } else {
            value
        }
    }
}

fn valid_name(name: &[u8]) -> bool {
    !name.is_empty() && !name.starts_with(b".L") && !name.starts_with(b"L0") && name[0] != b'$'
}

fn remove_dot(name: &str) -> &str {
    if let Some(pos) = name.find('.').filter(|&pos| pos != 0) {
        let tail = &name[pos + 1..];
        let digits = tail.bytes().take_while(u8::is_ascii_digit).count();
        if digits > 0 && matches!(tail.as_bytes().get(digits), None | Some(b'.')) {
            return &name[..pos];
        }
    }
    name
}

fn strings(data: &[u8]) -> Result<Vec<String>, String> {
    if data.last().is_some_and(|&byte| byte != 0) {
        return Err("unterminated ELF metadata string\n".into());
    }
    data.split(|&byte| byte == 0)
        .filter(|part| !part.is_empty())
        .map(|part| {
            std::str::from_utf8(part)
                .map(str::to_owned)
                .map_err(|_| "invalid UTF-8 ELF metadata string\n".into())
        })
        .collect()
}

struct Modpost {
    options: Options,
    modules: Vec<Module>,
    exports: Vec<Export>,
    by_name: HashMap<String, Vec<usize>>,
    tables: file2alias::Tables,
    error: bool,
    mismatches: usize,
    unresolved: usize,
}

impl Modpost {
    fn log(&mut self, error: bool, module: Option<usize>, text: &str) {
        self.error |= error;
        eprint!("{}: modpost: ", if error { "ERROR" } else { "WARNING" });
        if let Some(index) = module {
            let module = &self.modules[index];
            eprint!(
                "{}{}: ",
                module.name,
                if module.vmlinux { "" } else { ".ko" }
            );
        }
        eprint!("{text}");
    }

    fn fatal<T>(&mut self, message: &str) -> Result<T, String> {
        self.log(true, None, message);
        Err(String::new())
    }

    fn find_export(&self, name: &str, module: Option<usize>) -> Option<usize> {
        self.by_name
            .get(name.strip_prefix('.').unwrap_or(name))?
            .iter()
            .rev()
            .copied()
            .find(|&index| module.is_none_or(|module| self.exports[index].module == module))
    }

    fn add_export(&mut self, module: usize, name: &str, gpl_only: bool, namespace: &str) -> usize {
        if let Some(previous) = self.find_export(name, None) {
            let owner = self.exports[previous].module;
            if !self.options.external || self.modules[owner].vmlinux || owner == module {
                let owner = &self.modules[owner];
                let text = format!(
                    "symbol '{name}' exported twice. Previous export was in {}{}\n",
                    owner.name,
                    if owner.vmlinux { "" } else { ".ko" }
                );
                self.log(true, Some(module), &text);
            }
        }
        let index = self.exports.len();
        self.exports.push(Export {
            name: name.into(),
            module,
            namespace: namespace.into(),
            crc: None,
            is_function: false,
            gpl_only,
            used: false,
        });
        self.modules[module].exports.push(index);
        self.by_name.entry(name.into()).or_default().push(index);
        index
    }

    fn read_dump(&mut self, filename: &str) -> Result<(), String> {
        let data = read_text(filename)?;
        for line in data.split_terminator('\n') {
            let parts: Vec<_> = line.splitn(5, '\t').collect();
            if parts.len() != 5 {
                return self.fatal("parse error in symbol dump file\n");
            }
            let (crc, end) = c_unsigned(parts[0], 16);
            let (name, module_name, license, namespace) = (parts[1], parts[2], parts[3], parts[4]);
            if end != parts[0].len() || name.is_empty() || module_name.is_empty() {
                return self.fatal("parse error in symbol dump file\n");
            }
            let gpl = match license {
                "EXPORT_SYMBOL_GPL" => true,
                "EXPORT_SYMBOL" => false,
                _ => {
                    self.log(
                        true,
                        None,
                        &format!("{name}: unknown license {license}. skip"),
                    );
                    continue;
                }
            };
            let module = self
                .modules
                .iter()
                .position(|module| {
                    module.dump.as_deref() == Some(filename) && module.name == module_name
                })
                .unwrap_or_else(|| {
                    let index = self.modules.len();
                    let mut module = Module::new(module_name.into());
                    module.dump = Some(filename.into());
                    self.modules.push(module);
                    index
                });
            let export = self.add_export(module, name, gpl, namespace);
            self.exports[export].crc = Some(crc as u32);
        }
        Ok(())
    }

    fn read_object(&mut self, filename: &str) -> Result<(), String> {
        if !filename.ends_with(".o") {
            self.log(
                true,
                None,
                &format!("{filename}: filename must be suffixed with .o\n"),
            );
            return Ok(());
        }
        let data = match fs::read(filename).and_then(|bytes| {
            if bytes.is_empty() {
                Err(io::Error::from_raw_os_error(22))
            } else {
                Ok(bytes)
            }
        }) {
            Ok(data) => data,
            Err(error) if self.options.ignore_missing => {
                eprintln!("{filename}: {} (ignored)", io_error(&error));
                return Ok(());
            }
            Err(error) => return Err(format!("{filename}: {}\n", io_error(&error))),
        };
        let header_size = if data.get(4) == Some(&1) { 52 } else { 64 };
        if data.len() < header_size || !data.starts_with(b"\x7fELF") {
            return Ok(());
        }
        if !matches!(data[5], 1 | 2) {
            return self.fatal("target endian is unknown\n");
        }
        let little = data[5] == 1;
        let integer = |start: usize, size: usize| {
            let bytes = &data[start..start + size];
            if little {
                bytes
                    .iter()
                    .rev()
                    .fold(0u64, |n, &b| (n << 8) | u64::from(b))
            } else {
                bytes.iter().fold(0u64, |n, &b| (n << 8) | u64::from(b))
            }
        };
        if integer(16, 2) != 1 {
            return self.fatal(&format!("{filename}: not relocatable object."));
        }
        let section_offset = if data[4] == 1 {
            integer(32, 4)
        } else {
            integer(40, 8)
        };
        if section_offset > data.len() as u64 {
            return self.fatal(&format!("section header offset={section_offset} in file '{filename}' is bigger than filesize={}\n", data.len()));
        }
        let object = match Object::parse(&data, filename) {
            Ok(object) => object,
            Err(mut message) => {
                if !message.ends_with('\n') {
                    message = format!("{filename}: {message}\n");
                }
                return self.fatal(&message);
            }
        };
        let module = self.modules.len();
        self.modules
            .push(Module::new(filename[..filename.len() - 2].into()));
        self.modules[module].word_size = object.elf.word_size();
        self.modules[module].no_trim = strings(object.named_data(".no_trim_symbol")?)?;
        let modinfo = strings(object.named_data(".modinfo")?)?;
        if !self.modules[module].vmlinux {
            let licenses: Vec<_> = modinfo
                .iter()
                .filter_map(|info| info.strip_prefix("license="))
                .collect();
            if licenses.is_empty() {
                self.log(true, Some(module), "missing MODULE_LICENSE()\n");
            }
            self.modules[module].gpl_compatible = licenses.iter().all(|license| {
                matches!(
                    *license,
                    "GPL"
                        | "GPL v2"
                        | "GPL and additional rights"
                        | "Dual BSD/GPL"
                        | "Dual MIT/GPL"
                        | "Dual MPL/GPL"
                )
            });
            for namespace in modinfo
                .iter()
                .filter_map(|info| info.strip_prefix("import_ns="))
            {
                if namespace.starts_with("module:") {
                    self.log(
                        true,
                        Some(module),
                        &format!("explicitly importing namespace '{namespace}' is not allowed.\n"),
                    );
                }
                unique_push(&mut self.modules[module].imported_namespaces, namespace);
            }
            if !modinfo.iter().any(|info| info.starts_with("description=")) {
                self.log(false, Some(module), "missing MODULE_DESCRIPTION()\n");
            }
        }
        for (index, symbol) in object.symbols.iter().enumerate() {
            let name = &object.names[index];
            match symbol.section {
                0xfff2 if !name.starts_with("__gnu_lto_") => {
                    self.log(false, Some(module), &format!("'{name}' is COMMON symbol\n"))
                }
                0 if matches!(symbol.binding, 1 | 2) && !ignore_undefined(object.machine, name) => {
                    if matches!(object.machine, 2 | 43) && symbol.kind == 13 {
                        continue;
                    }
                    let name = if matches!(object.machine, 2 | 43) && name.starts_with('.') {
                        let mut bytes = name.as_bytes().to_vec();
                        bytes[0] = b'_';
                        if let Some(byte) = bytes.get_mut(1) {
                            byte.make_ascii_uppercase();
                        }
                        String::from_utf8_lossy(&bytes).into_owned()
                    } else {
                        name.clone()
                    };
                    self.modules[module]
                        .unresolved
                        .push(Unresolved::new(name, symbol.binding == 2));
                }
                0 | 0xfff2 => (),
                _ => {
                    self.modules[module].has_init |= name == "init_module";
                    self.modules[module].has_cleanup |= name == "cleanup_module";
                }
            }
            if symbol.section == 0
                || object.indices[index] as usize >= object.sections.len()
                || symbol.kind != 1
                || !name.starts_with("__mod_device_table__")
            {
                continue;
            }
            let section = &object.sections[object.indices[index] as usize];
            if symbol
                .value
                .checked_add(symbol.size)
                .is_none_or(|end| end > section.size)
            {
                return self.fatal(&format!(
                    "{filename}: device table extends beyond its section\n"
                ));
            }
            let zero_data;
            let bytes = if section.kind == 8 {
                let size =
                    usize::try_from(symbol.size).map_err(|_| "device table size overflow")?;
                let mut zeros = Vec::new();
                zeros
                    .try_reserve_exact(size)
                    .map_err(|_| "device table allocation is too large\n")?;
                zeros.resize(size, 0);
                zero_data = zeros;
                &zero_data[..]
            } else {
                object
                    .data_at(object.indices[index], symbol.value)?
                    .get(..usize::try_from(symbol.size).map_err(|_| "device table size overflow")?)
                    .ok_or("device table extends beyond its section")?
            };
            let entry = &mut self.modules[module];
            let diagnostics = self.tables.handle(
                &entry.name,
                entry.vmlinux,
                name,
                bytes,
                object.elf.little_endian(),
                (object.elf.word_size() * 8) as u32,
                &mut entry.aliases,
            );
            for diagnostic in diagnostics {
                let fatal = matches!(diagnostic.severity, file2alias::Severity::Fatal);
                self.log(
                    !matches!(diagnostic.severity, file2alias::Severity::Warning),
                    None,
                    &diagnostic.message,
                );
                if fatal {
                    return Err(String::new());
                }
            }
        }
        self.check_sections(module, &object)?;
        if !self.modules[module].vmlinux
            && (self.options.all_versions
                || modinfo.iter().any(|info| info.starts_with("version=")))
        {
            let result = sumversion::get_src_version(&self.modules[module].name)?;
            self.modules[module].source_version = result.checksum;
            for message in result.warnings {
                self.log(false, None, &message);
            }
        }
        if self.options.modversions {
            self.modules[module]
                .unresolved
                .push(Unresolved::new("module_layout".into(), false));
            self.set_crcs(module)?;
        }
        Ok(())
    }

    fn set_crcs(&mut self, module: usize) -> Result<(), String> {
        let filename = if self.modules[module].vmlinux {
            ".vmlinux.objs".into()
        } else {
            format!("{}.mod", self.modules[module].name)
        };
        if filename.len() >= PATH_MAX {
            self.log(
                true,
                None,
                &format!(
                    "{}: too long path was truncated\n",
                    &filename[..PATH_MAX - 1]
                ),
            );
            return Ok(());
        }
        for object in read_text(&filename)?
            .split('\n')
            .take_while(|object| !object.is_empty())
        {
            let base = basename(object);
            let directory = &object[..object.len() - base.len()];
            let stem = base
                .strip_suffix(".thinlto-native.o")
                .unwrap_or_else(|| base.strip_suffix(".o").unwrap_or(base));
            let filename = format!("{directory}.{stem}.o.cmd");
            if filename.len() >= PATH_MAX {
                self.log(
                    true,
                    None,
                    &format!(
                        "{}: too long path was truncated\n",
                        &filename[..PATH_MAX - 1]
                    ),
                );
                continue;
            }
            let data = read_text(&filename)?;
            // Leading newline is intentional: a marker at byte zero is ignored.
            let mut rest = data.as_str();
            while let Some(marker) = rest.find("\n#SYMVER ") {
                rest = &rest[marker + 9..];
                let Some(space) = rest.find(' ') else {
                    break;
                };
                let name = &rest[..space];
                rest = &rest[space + 1..];
                if !rest.as_bytes().first().is_some_and(u8::is_ascii_digit) {
                    continue;
                }
                let (crc, end) = c_unsigned(rest, 0);
                rest = &rest[end..];
                if !rest.starts_with('\n') {
                    continue;
                }
                if let Some(index) = self.find_export(name, Some(module)) {
                    self.exports[index].crc = Some(crc as u32);
                }
            }
        }
        Ok(())
    }
}

fn ignore_undefined(machine: u16, name: &str) -> bool {
    if matches!(name, "__this_module" | "_GLOBAL_OFFSET_TABLE_")
        || name.starts_with("__start_")
        || name.starts_with("__stop_")
    {
        return true;
    }
    let prefixes: &[&str] = match machine {
        20 => &[
            "_restgpr_",
            "_savegpr_",
            "_rest32gpr_",
            "_save32gpr_",
            "_restvr_",
            "_savevr_",
        ],
        21 => &[
            "_restgpr0_",
            "_savegpr0_",
            "_restgpr1_",
            "_savegpr1_",
            "_restfpr_",
            "_savefpr_",
            "_restvr_",
            "_savevr_",
        ],
        _ => &[],
    };
    (machine == 21 && name == ".TOC.") || prefixes.iter().any(|prefix| name.starts_with(prefix))
}

const SECTION_WHITELIST: &[&str] = &[
    ".comment*",
    ".debug*",
    ".zdebug*",
    ".GCC.command.line",
    ".mdebug*",
    ".pdr",
    ".stab*",
    ".note*",
    ".got*",
    ".toc*",
    ".xt.prop",
    ".xt.lit",
    ".arcextmap*",
    ".gnu.linkonce.arcext*",
    ".cmem*",
    ".fmt_slot*",
    ".gnu.lto*",
    ".discard.*",
    ".llvm.call-graph-profile",
    "__llvm_covfun",
    "__llvm_covmap",
    ".klp.symid",
];
const TEXT_SECTIONS: &[&str] = &[
    ".text",
    ".text.*",
    ".sched.text",
    ".kprobes.text",
    ".cpuidle.text",
    ".noinstr.text",
    ".ltext",
    ".ltext.*",
];
const DATA_SECTIONS: &[&str] = &[".data", ".data.rel"];
const OTHER_TEXT_SECTIONS: &[&str] = &[
    ".ref.text",
    ".head.text",
    ".spinlock.text",
    ".fixup",
    ".entry.text",
    ".exception.text",
    ".coldtext",
    ".softirqentry.text",
    ".irqentry.text",
];
const PCI_INIT_SECTIONS: &[&str] = &[
    ".pci_fixup_early",
    ".pci_fixup_header",
    ".pci_fixup_final",
    ".pci_fixup_enable",
    ".pci_fixup_resume",
    ".pci_fixup_resume_early",
    ".pci_fixup_suspend",
];

// The fixed section patterns use only '*'; '/' and leading '.' are ordinary.
fn wildcard(pattern: &str, text: &str) -> bool {
    let (pattern, text) = (pattern.as_bytes(), text.as_bytes());
    let (mut p, mut t, mut star, mut retry) = (0, 0, None, 0);
    while t < text.len() {
        if p < pattern.len() && pattern[p] == b'*' {
            star = Some(p);
            p += 1;
            retry = t;
        } else if p < pattern.len() && pattern[p] == text[t] {
            p += 1;
            t += 1;
        } else if let Some(position) = star {
            retry += 1;
            t = retry;
            p = position + 1;
        } else {
            return false;
        }
    }
    while p < pattern.len() && pattern[p] == b'*' {
        p += 1;
    }
    p == pattern.len()
}

fn matches_patterns(name: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|pattern| wildcard(pattern, name))
}

fn is_text(name: &str) -> bool {
    matches!(name, ".init.text" | ".exit.text")
        || matches_patterns(name, TEXT_SECTIONS)
        || matches_patterns(name, OTHER_TEXT_SECTIONS)
}

fn mismatch(from: &str, to: &str) -> bool {
    if to.is_empty() {
        return false;
    }
    if (matches_patterns(from, TEXT_SECTIONS) || matches_patterns(from, DATA_SECTIONS))
        && (to.starts_with(".init.") || to.starts_with(".exit."))
    {
        return true;
    }
    if from.starts_with(".init.") && to.starts_with(".exit.") {
        return true;
    }
    if (from.starts_with(".exit.") || matches_patterns(from, PCI_INIT_SECTIONS))
        && to.starts_with(".init.")
    {
        return true;
    }
    from == "__ex_table" && (to == ".altinstr_replacement" || !is_text(to))
}

fn allowed_reference(from: &str, from_symbol: &str, to: &str, to_symbol: &str) -> bool {
    if matches_patterns(from, DATA_SECTIONS) {
        if matches!(to, ".init.setup" | ".init.rodata" | ".init.data")
            && from_symbol.starts_with("__param")
        {
            return true;
        }
        if to == ".init.text" && from_symbol.starts_with("__param_ops_") {
            return true;
        }
        if (to.starts_with(".init.") || to.starts_with(".exit."))
            && matches_patterns(from_symbol, &["*_ops", "*_ops.llvm.*", "*_console"])
        {
            return true;
        }
    }
    (from.starts_with(".head.text") && to.starts_with(".init."))
        || matches!(to_symbol, "__init_begin" | "_sinittext" | "_einittext")
        || (is_text(from) && to.starts_with(".init.") && wildcard("*.constprop.*", from_symbol))
}

fn target_integer(data: &[u8], offset: usize, width: usize, little: bool) -> Result<u32, String> {
    let value = data
        .get(offset..offset + width)
        .ok_or("relocation location outside section")?;
    Ok(if little {
        value.iter().rev().fold(0, |n, &b| (n << 8) | u32::from(b))
    } else {
        value.iter().fold(0, |n, &b| (n << 8) | u32::from(b))
    })
}

fn sign_extend(value: u32, index: u32) -> u64 {
    i64::from(((value << (31 - index)) as i32) >> (31 - index)) as u64
}

fn implicit_addend(
    object: &Object<'_>,
    section: u32,
    offset: u64,
    symbol: &Symbol,
    kind: u32,
) -> Result<u64, String> {
    let data = object.data_at(section, offset)?;
    let word = || target_integer(data, 0, 4, object.elf.little_endian());
    let halves = || -> Result<(u32, u32), String> {
        Ok((
            target_integer(data, 0, 2, object.elf.little_endian())?,
            target_integer(data, 2, 2, object.elf.little_endian())?,
        ))
    };
    let value = match object.machine {
        3 => match kind {
            1 => u64::from(word()?),
            2 => u64::from(word()?.wrapping_add(4)),
            _ => u64::MAX,
        },
        8 => {
            let instruction = word()?;
            match kind {
                6 => u64::from(instruction & 0xffff),
                4 => u64::from((instruction & 0x03ff_ffff) << 2),
                2 => u64::from(instruction),
                _ => u64::MAX,
            }
        }
        40 => match kind {
            2 | 3 => u64::from(word()?).wrapping_add(symbol.value),
            43 | 44 => {
                let instruction = word()?;
                sign_extend(((instruction & 0xf0000) >> 4) | (instruction & 0xfff), 15)
                    .wrapping_add(symbol.value)
            }
            1 | 28 | 29 => sign_extend((word()? & 0x00ff_ffff) << 2, 25)
                .wrapping_add(symbol.value)
                .wrapping_add(8),
            47 | 48 => {
                let (upper, lower) = halves()?;
                sign_extend(
                    ((upper & 0xf) << 12)
                        | ((upper & 0x400) << 1)
                        | ((lower & 0x7000) >> 4)
                        | (lower & 0xff),
                    15,
                )
                .wrapping_add(symbol.value)
            }
            51 => {
                let (upper, lower) = halves()?;
                let (sign, j1, j2) = ((upper >> 10) & 1, (lower >> 13) & 1, (lower >> 11) & 1);
                sign_extend(
                    (sign << 20)
                        | (j2 << 19)
                        | (j1 << 18)
                        | ((upper & 0x3f) << 12)
                        | ((lower & 0x7ff) << 1),
                    20,
                )
                .wrapping_add(symbol.value)
                .wrapping_add(4)
            }
            10 | 30 => {
                let (upper, lower) = halves()?;
                let (sign, j1, j2) = ((upper >> 10) & 1, (lower >> 13) & 1, (lower >> 11) & 1);
                sign_extend(
                    (sign << 24)
                        | ((!(j1 ^ sign) & 1) << 23)
                        | ((!(j2 ^ sign) & 1) << 22)
                        | ((upper & 0x3ff) << 12)
                        | ((lower & 0x7ff) << 1),
                    24,
                )
                .wrapping_add(symbol.value)
                .wrapping_add(4)
            }
            _ => u64::MAX,
        },
        _ => return Err("Please add code to calculate addend for this architecture\n".into()),
    };
    Ok(object.target_value(value))
}

impl Modpost {
    fn check_export(
        &mut self,
        module: usize,
        object: &Object<'_>,
        offset: u64,
        target: usize,
    ) -> Result<(), String> {
        let label = object
            .search
            .nearest(offset, object.export_section, false, u64::MAX);
        let label_name = label
            .map(|index| object.names[index].as_str())
            .unwrap_or("");
        let Some(exported_name) = label_name.strip_prefix("__export_symbol_") else {
            self.log(
                true,
                Some(module),
                &format!(".export_symbol section contains strange symbol '{label_name}'\n"),
            );
            return Ok(());
        };
        let symbol = &object.symbols[target];
        if !matches!(symbol.binding, 1 | 2) {
            self.log(
                true,
                Some(module),
                &format!("local symbol '{exported_name}' was exported\n"),
            );
            return Ok(());
        }
        let name = &object.names[target];
        if exported_name != name {
            self.log(true, Some(module), &format!(".export_symbol section references '{name}', but it does not seem to be an export symbol\n"));
            return Ok(());
        }
        let label = label.unwrap();
        let data = object.data_at(object.indices[label], object.symbols[label].value)?;
        let license_end = data
            .iter()
            .position(|&b| b == 0)
            .ok_or("unterminated export license")?;
        let license = std::str::from_utf8(&data[..license_end])
            .map_err(|_| "invalid UTF-8 export license\n")?;
        let gpl = match license {
            "GPL" => true,
            "" => false,
            _ => {
                self.log(
                    true,
                    Some(module),
                    &format!("unknown license '{license}' was specified for '{name}'\n"),
                );
                return Ok(());
            }
        };
        let data = &data[license_end + 1..];
        let namespace_end = data
            .iter()
            .position(|&b| b == 0)
            .ok_or("unterminated export namespace")?;
        let namespace = std::str::from_utf8(&data[..namespace_end])
            .map_err(|_| "invalid UTF-8 export namespace\n")?;
        let export = self.add_export(module, name, gpl, namespace);
        self.exports[export].is_function = symbol.kind == 2
            || (object.elf.word_size() == 8 && object.machine == 15 && symbol.kind == 13);
        let section = object.sec_name(object.indices[target]);
        if section.starts_with(".init.") {
            self.log(false, Some(module), &format!("EXPORT_SYMBOL used for init symbol '{name}'. Remove __init or EXPORT_SYMBOL.\n"));
        } else if section.starts_with(".exit.") {
            self.log(false, Some(module), &format!("EXPORT_SYMBOL used for exit symbol '{name}'. Remove __exit or EXPORT_SYMBOL.\n"));
        }
        Ok(())
    }

    fn check_reference(
        &mut self,
        module: usize,
        object: &Object<'_>,
        target: usize,
        section: u32,
        offset: u64,
        address: u64,
    ) -> Result<(), String> {
        if self.options.module_enabled && object.export_section == section {
            return self.check_export(module, object, offset, target);
        }
        let from = object.sec_name(section);
        let to = object.sec_name(object.indices[target]);
        if !mismatch(from, to) {
            return Ok(());
        }
        let from_symbol = object.search.nearest(offset, section, false, u64::MAX);
        let from_name = from_symbol
            .map(|index| object.names[index].as_str())
            .unwrap_or("");
        let target = if valid_name(object.names[target].as_bytes()) {
            target
        } else {
            object
                .search
                .nearest(address, object.indices[target], true, 20)
                .unwrap_or(target)
        };
        let to_name = &object.names[target];
        if allowed_reference(from, from_name, to, to_name) {
            return Ok(());
        }
        self.mismatches += 1;
        let source_offset = offset.wrapping_sub(
            from_symbol
                .filter(|_| !from_name.is_empty())
                .map(|index| object.symbols[index].value)
                .unwrap_or(0),
        );
        let destination = if to_name.is_empty() {
            format!("0x{:x}", address as u32)
        } else {
            to_name.clone()
        };
        self.log(false, Some(module), &format!("section mismatch in reference: {from_name}{}0x{:x} (section: {from}) -> {destination} (section: {to})\n", if from_name.is_empty() { "" } else { "+" }, source_offset as u32));
        if from == "__ex_table" {
            if to == ".altinstr_replacement" {
                return self.fatal(&format!("The relocation at {from}+0x{offset:x} references\nsection \"{to}\" which is black-listed.\nSomething is seriously wrong and should be fixed.\nYou might get more information about where this is\ncoming from by using scripts/check_extable.sh {}\n", self.modules[module].name));
            }
            if object
                .sections
                .get(object.indices[target] as usize)
                .is_some_and(|section| section.flags & 4 != 0)
            {
                self.log(false, None, &format!("The relocation at {from}+0x{offset:x} references\nsection \"{to}\" which is not in the list of\nauthorized sections.  If you're adding a new section\nand/or if this reference is valid, add \"{to}\" to the\nlist of authorized sections to jump to on fault.\nThis can be achieved by adding \"{to}\" to\nOTHER_TEXT_SECTIONS in scripts/mod/modpost.c.\n"));
            } else {
                self.log(
                    true,
                    None,
                    &format!("{from}+0x{offset:x} references non-executable section '{to}'\n"),
                );
            }
        }
        Ok(())
    }

    fn check_sections(&mut self, module: usize, object: &Object<'_>) -> Result<(), String> {
        for section in &object.sections {
            let name = object.sec_name(section.index as u32);
            if section.kind == 1
                && section.flags & 2 == 0
                && !matches_patterns(name, SECTION_WHITELIST)
            {
                self.log(false, Some(module), &format!("unexpected non-allocatable section '{name}'.\nDid you forget to use \"ax\"/\"aw\" in a .S file?\nNote that for example <linux/init.h> contains\nsection definitions for use in .S files.\n\n"));
            }
            if !matches!(section.kind, 4 | 9) {
                continue;
            }
            let from = object.sec_name(section.info);
            if matches_patterns(from, SECTION_WHITELIST) {
                continue;
            }
            let relocations = object.elf.relocations(section)?;
            for relocation in relocations {
                if relocation.addend.is_some()
                    && ((object.machine == 243 && from == "__ex_table" && relocation.kind == 39)
                        || (object.machine == 258
                            && (matches!(relocation.kind, 100 | 102)
                                || (from == "__ex_table" && relocation.kind == 55))))
                {
                    continue;
                }
                let target = relocation.symbol as usize;
                let symbol = object
                    .symbols
                    .get(target)
                    .ok_or("invalid ELF relocation symbol index")?;
                let address = if let Some(addend) = relocation.addend {
                    object.target_value(symbol.value.wrapping_add(addend as u64))
                } else {
                    match implicit_addend(
                        object,
                        section.info,
                        relocation.offset,
                        symbol,
                        relocation.kind,
                    ) {
                        Ok(address) => address,
                        Err(message) => return self.fatal(&message),
                    }
                };
                self.check_reference(
                    module,
                    object,
                    target,
                    section.info,
                    relocation.offset,
                    address,
                )?;
            }
        }
        Ok(())
    }

    fn check_exports(&mut self, module: usize) {
        if basename(&self.modules[module].name).len() >= 64 - self.modules[module].word_size {
            self.log(true, Some(module), "module name is too long\n");
        }
        for index in 0..self.modules[module].unresolved.len() {
            let unresolved = &self.modules[module].unresolved[index];
            let name = unresolved.name.clone();
            let Some(export) = self.find_export(&name, None) else {
                if !unresolved.weak {
                    if self.unresolved < MAX_UNRESOLVED {
                        self.log(
                            !self.options.warn_unresolved,
                            Some(module),
                            &format!("symbol '{name}' undefined!\n"),
                        );
                    }
                    self.unresolved += 1;
                }
                continue;
            };
            if self.exports[export].module == module {
                self.log(
                    true,
                    Some(module),
                    &format!("symbol '{name}' was exported without definition\n"),
                );
                continue;
            }
            let exported = &mut self.exports[export];
            exported.used = true;
            let unresolved = &mut self.modules[module].unresolved[index];
            unresolved.module = Some(exported.module);
            unresolved.crc = exported.crc;
            let namespace = exported.namespace.clone();
            let gpl_only = exported.gpl_only;
            let name = exported.name.clone();
            let module_entry = &self.modules[module];
            if !namespace.is_empty()
                && !module_entry.imported_namespaces.contains(&namespace)
                && !module_namespace(&namespace, basename(&module_entry.name))
            {
                self.log(!self.options.allow_missing_namespace, Some(module),
                    &format!("module uses symbol '{name}' from namespace '{namespace}', but does not import it.\n"));
                unique_push(&mut self.modules[module].missing_namespaces, &namespace);
            }
            if !self.modules[module].gpl_compatible && gpl_only {
                self.log(
                    true,
                    Some(module),
                    &format!("GPL-incompatible module uses GPL-only symbol '{name}'\n"),
                );
            }
        }
    }

    fn exported_output(&mut self, module: usize, output: &mut String) {
        output.push('\n');
        for index in self.modules[module].exports.clone() {
            let symbol = &self.exports[index];
            if self.options.trim && !symbol.used {
                continue;
            }
            writeln!(
                output,
                "KSYMTAB_{}({}, \"{}\");",
                if symbol.is_function { "FUNC" } else { "DATA" },
                symbol.name,
                symbol.namespace
            )
            .unwrap();
            writeln!(
                output,
                "SYMBOL_FLAGS({}, 0x{:02x});",
                symbol.name,
                u8::from(symbol.gpl_only)
            )
            .unwrap();
        }
        if !self.options.modversions {
            return;
        }
        output.push('\n');
        for index in self.modules[module].exports.clone() {
            let symbol = &self.exports[index];
            if self.options.trim && !symbol.used {
                continue;
            }
            let (name, crc) = (symbol.name.clone(), symbol.crc);
            if crc.is_none() {
                self.log(false, Some(module), &format!("EXPORT symbol '{name}' version generation failed, symbol will not be versioned.\nIs '{name}' prototyped in <asm/asm-prototypes.h>?\n"));
            }
            writeln!(output, "SYMBOL_CRC({name}, 0x{:08x});", crc.unwrap_or(0)).unwrap();
        }
    }

    fn version_output(&mut self, module: usize, output: &mut String) {
        if self.options.basic_versions {
            output.push_str("\nstatic const struct modversion_info ____versions[]\n__used __section(\"__versions\") = {\n");
            for index in 0..self.modules[module].unresolved.len() {
                let symbol = &self.modules[module].unresolved[index];
                if symbol.module.is_none() {
                    continue;
                }
                let (name, crc) = (symbol.name.clone(), symbol.crc);
                let Some(crc) = crc else {
                    self.log(
                        false,
                        Some(module),
                        &format!("symbol '{name}' has no CRC!\n"),
                    );
                    continue;
                };
                if name.len() >= 64 - self.modules[module].word_size {
                    if self.options.extended_versions {
                        continue;
                    }
                    self.log(true, Some(module), &format!("too long symbol '{name}'\n"));
                    break;
                }
                writeln!(output, "\t{{ 0x{crc:08x}, \"{name}\" }},").unwrap();
            }
            output.push_str("};\n");
        }
        if self.options.extended_versions {
            output.push_str("\nstatic const u32 ____version_ext_crcs[]\n__used __section(\"__version_ext_crcs\") = {\n");
            for index in 0..self.modules[module].unresolved.len() {
                let symbol = &self.modules[module].unresolved[index];
                if symbol.module.is_none() {
                    continue;
                }
                if let Some(crc) = symbol.crc {
                    writeln!(output, "\t0x{crc:08x},").unwrap();
                } else {
                    let text = format!("symbol '{}' has no CRC!\n", symbol.name);
                    self.log(false, Some(module), &text);
                }
            }
            output.push_str("};\nstatic const char ____version_ext_names[]\n__used __section(\"__version_ext_names\") =\n");
            for symbol in &self.modules[module].unresolved {
                if symbol.module.is_some() && symbol.crc.is_some() {
                    writeln!(output, "\t\"{}\\0\"", symbol.name).unwrap();
                }
            }
            output.push_str(";\n");
        }
    }

    fn write_file(&self, path: &str, contents: &str, only_if_changed: bool) -> Result<(), String> {
        if self.error {
            return Ok(());
        }
        if only_if_changed && fs::read(path).is_ok_and(|old| old == contents.as_bytes()) {
            return Ok(());
        }
        fs::write(path, contents).map_err(|error| format!("{path}: {}\n", io_error(&error)))
    }

    fn write_module(&mut self, module: usize) -> Result<(), String> {
        let mut output = String::new();
        if self.modules[module].vmlinux {
            output.push_str("#include <linux/export-internal.h>\n");
            self.exported_output(module, &mut output);
            output.push_str("#include <linux/module.h>\n#undef __MODULE_INFO_PREFIX\n#define __MODULE_INFO_PREFIX\n");
            for alias in &self.modules[module].aliases {
                writeln!(
                    output,
                    "MODULE_INFO({}.alias, \"{}\");",
                    alias.builtin_modname.as_deref().unwrap_or("(null)"),
                    alias.text
                )
                .unwrap();
            }
            return self.write_file(".vmlinux.export.c", &output, true);
        }
        output.push_str("#include <linux/module.h>\n#include <linux/export-internal.h>\n#include <linux/compiler.h>\n\nMODULE_INFO(name, KBUILD_MODNAME);\n\n__visible struct module __this_module\n__section(\".gnu.linkonce.this_module\") = {\n\t.name = KBUILD_MODNAME,\n");
        if self.modules[module].has_init {
            output.push_str("\t.init = init_module,\n");
        }
        if self.modules[module].has_cleanup {
            output.push_str("#ifdef CONFIG_MODULE_UNLOAD\n\t.exit = cleanup_module,\n#endif\n");
        }
        output.push_str("\t.arch = MODULE_ARCH_INIT,\n};\n");
        if !self.options.external {
            output.push_str("\nMODULE_INFO(intree, \"Y\");\n");
        }
        if self.modules[module].name.starts_with("drivers/staging") {
            output.push_str("\nMODULE_INFO(staging, \"Y\");\n");
        }
        if self.modules[module].name.starts_with("tools/testing") {
            output.push_str("\nMODULE_INFO(test, \"Y\");\n");
        }
        self.exported_output(module, &mut output);
        self.version_output(module, &mut output);
        output.push_str("\nMODULE_INFO(depends, \"");
        let mut dependencies = Vec::new();
        for symbol in &self.modules[module].unresolved {
            if let Some(owner) = symbol.module {
                if !self.modules[owner].vmlinux && !dependencies.contains(&owner) {
                    dependencies.push(owner);
                }
            }
        }
        for (index, &owner) in dependencies.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            output.push_str(basename(&self.modules[owner].name));
        }
        output.push_str("\");\n\n");
        for alias in &self.modules[module].aliases {
            writeln!(output, "MODULE_ALIAS(\"{}\");", alias.text).unwrap();
        }
        if let Some(version) = &self.modules[module].source_version {
            writeln!(output, "\nMODULE_INFO(srcversion, \"{version}\");").unwrap();
        }
        let filename = format!("{}.mod.c", self.modules[module].name);
        if filename.len() >= PATH_MAX {
            self.log(
                true,
                None,
                &format!(
                    "{}: too long path was truncated\n",
                    &filename[..PATH_MAX - 1]
                ),
            );
            return Ok(());
        }
        self.write_file(&filename, &output, true)
    }

    fn run(&mut self) -> Result<(), String> {
        for filename in self.options.dumps.clone() {
            self.read_dump(&filename)?;
        }
        for filename in self.options.objects.clone() {
            self.read_object(&filename)?;
        }
        if let Some(filename) = self.options.files_source.clone() {
            let bytes = fs::read(&filename).map_err(|error| {
                format!(
                    "ERROR: modpost: Can't open filenames file {filename}: {}",
                    io_error(&error)
                )
            })?;
            // fgets(PATH_MAX) splits an overlong line into individual records.
            for line in bytes.split_inclusive(|&byte| byte == b'\n') {
                for part in line.chunks(PATH_MAX - 1) {
                    let part = part.strip_suffix(b"\n").unwrap_or(part);
                    let end = part.iter().position(|&b| b == 0).unwrap_or(part.len());
                    let name = std::str::from_utf8(&part[..end])
                        .map_err(|_| "invalid UTF-8 object filename\n")?;
                    self.read_object(name)?;
                }
            }
        }
        for module in 0..self.modules.len() {
            for name in self.modules[module].no_trim.clone() {
                if let Some(export) = self.find_export(&name, None) {
                    self.exports[export].used = true;
                }
            }
            if self.modules[module].dump.is_none() && !self.modules[module].vmlinux {
                self.check_exports(module);
            }
        }
        if let Some(filename) = self.options.whitelist.clone() {
            for name in read_text(&filename)?.split('\n') {
                if let Some(export) = self.find_export(name, None) {
                    self.exports[export].used = true;
                }
            }
        }
        for module in 0..self.modules.len() {
            if self.modules[module].dump.is_none() {
                self.write_module(module)?;
            }
        }
        if let Some(filename) = &self.options.namespace_output {
            let mut output = String::new();
            for module in &self.modules {
                if module.dump.is_some() || module.missing_namespaces.is_empty() {
                    continue;
                }
                write!(output, "{}.ko:", module.name).unwrap();
                for namespace in &module.missing_namespaces {
                    write!(output, " {namespace}").unwrap();
                }
                output.push('\n');
            }
            self.write_file(filename, &output, true)?;
        }
        if let Some(filename) = &self.options.dump_output {
            let mut output = String::new();
            for module in &self.modules {
                if module.dump.is_some() {
                    continue;
                }
                for &index in &module.exports {
                    let symbol = &self.exports[index];
                    if self.options.trim && !symbol.used {
                        continue;
                    }
                    writeln!(
                        output,
                        "0x{:08x}\t{}\t{}\tEXPORT_SYMBOL{}\t{}",
                        symbol.crc.unwrap_or(0),
                        symbol.name,
                        module.name,
                        if symbol.gpl_only { "_GPL" } else { "" },
                        symbol.namespace
                    )
                    .unwrap();
                }
            }
            self.write_file(filename, &output, false)?;
        }
        if self.mismatches > 0 && self.options.mismatch_error {
            self.log(true, None, "Section mismatches detected.\nSet CONFIG_SECTION_MISMATCH_WARN_ONLY=y to allow them.\n");
        }
        if self.unresolved > MAX_UNRESOLVED {
            self.log(
                false,
                None,
                &format!(
                    "suppressed {} unresolved symbol warnings because there were too many)\n",
                    self.unresolved - MAX_UNRESOLVED
                ),
            );
        }
        Ok(())
    }
}

fn module_namespace(namespace: &str, module: &str) -> bool {
    namespace.strip_prefix("module:").is_some_and(|patterns| {
        patterns.split_terminator(',').any(|pattern| {
            pattern
                .strip_suffix('*')
                .map_or(pattern == module, |prefix| module.starts_with(prefix))
        })
    })
}

fn main() {
    let result = (|| {
        let options = parse_options()?;
        let runtime_offsets;
        let offsets = if let Some(offsets) = EMBEDDED_OFFSETS {
            Some(offsets)
        } else if let Some(path) = env::var_os("MODPOST_DEVICETABLE_OFFSETS") {
            runtime_offsets = fs::read_to_string(&path).map_err(|error| {
                format!(
                    "{}: {}\n",
                    std::path::Path::new(&path).display(),
                    io_error(&error)
                )
            })?;
            Some(runtime_offsets.as_str())
        } else {
            None
        };
        let tables = offsets
            .map(file2alias::Tables::parse)
            .transpose()?
            .unwrap_or_default();
        let mut modpost = Modpost {
            options,
            modules: Vec::new(),
            exports: Vec::new(),
            by_name: HashMap::new(),
            tables,
            error: false,
            mismatches: 0,
            unresolved: 0,
        };
        modpost.run()?;
        Ok::<_, String>(modpost.error)
    })();
    match result {
        Ok(false) => (),
        Ok(true) => std::process::exit(1),
        Err(message) => {
            eprint!("{message}");
            std::process::exit(1);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
