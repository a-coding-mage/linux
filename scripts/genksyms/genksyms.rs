// SPDX-License-Identifier: GPL-2.0-or-later
//! Generate kernel symbol version hashes from preprocessed C declarations.
// Copyright 1996, 1997 Linux International.
// Original implementation: Richard Henderson <rth@tamu.edu>, based on work
// by Bjorn Ekwall <bj0rn@blox.se>. Safe Rust implementation retains the
// genksyms token grammar and version-2 checksum format.
#![forbid(unsafe_code)]

mod genksyms_header;
mod keywords;
mod lexer;
mod parser;
mod parser_tables;

use genksyms_header::{Kind, Status, Symbol, Word};

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::ffi::{OsStrExt, OsStringExt};

fn print_words(words: &[Word], output: &mut Vec<u8>) {
    if words.is_empty() {
        output.extend_from_slice(b"(nil)");
    }
    for word in words {
        word.print(output);
        output.push(b' ');
    }
}

#[derive(Default)]
struct Symbols {
    index: HashMap<(bool, Vec<u8>), usize>,
    symbols: Vec<Symbol>,
    visited: Vec<usize>,
    enum_counter: i32,
    enum_expression: Vec<Word>,
    debug: usize,
    dump: bool,
    preserve: bool,
    warnings: bool,
    errors: usize,
    filename: Vec<u8>,
    line: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl Symbols {
    fn new() -> Self {
        Self {
            line: 1,
            ..Self::default()
        }
    }
    fn location(&mut self) {
        self.stderr.extend_from_slice(if self.filename.is_empty() {
            b"<stdin>"
        } else {
            &self.filename
        });
        self.stderr
            .extend_from_slice(format!(":{}: ", self.line).as_bytes());
    }
    fn error(&mut self, message: impl AsRef<[u8]>) {
        if self.warnings {
            self.location();
            self.stderr.extend_from_slice(message.as_ref());
            self.stderr.push(b'\n');
            self.errors += 1;
        }
    }
    fn type_name(&mut self, kind: Kind, name: &[u8]) {
        if kind != Kind::Normal {
            self.stderr.extend_from_slice(kind.name().as_bytes());
            self.stderr.push(b' ');
        }
        self.stderr.extend_from_slice(name);
    }
    fn find(&self, name: &[u8], kind: Kind, exact: bool) -> Option<usize> {
        self.index
            .get(&(kind.namespace(), name.to_vec()))
            .copied()
            .filter(|&id| {
                let symbol = &self.symbols[id];
                symbol.declared && (!exact || symbol.kind == kind)
            })
    }
    fn add(
        &mut self,
        name: Option<Vec<u8>>,
        kind: Kind,
        mut definition: Vec<Word>,
        external: bool,
        reference: bool,
    ) -> Option<usize> {
        if !reference && matches!(kind, Kind::Enum | Kind::EnumConst) {
            if kind == Kind::EnumConst {
                if !definition.is_empty() {
                    self.enum_expression = definition.clone();
                    self.enum_counter = 1;
                } else {
                    if !self.enum_expression.is_empty() {
                        definition.push(Word::plain("("));
                        definition.extend(self.enum_expression.clone());
                        definition.extend([Word::plain(")"), Word::plain("+")]);
                    }
                    definition.push(Word::plain(self.enum_counter.to_string()));
                    self.enum_counter = self.enum_counter.wrapping_add(1);
                }
            } else {
                self.enum_expression.clear();
                self.enum_counter = 0;
            }
            if definition.is_empty() {
                return None;
            }
        }
        let name = name?;
        let key = (kind.namespace(), name.clone());
        let mut status = Status::Unchanged;
        if let Some(&id) = self.index.get(&key) {
            let old = &self.symbols[id];
            if !reference {
                if old.kind == kind && old.definition == definition {
                    if !old.declared && old.override_version {
                        self.location();
                        self.type_name(kind, &name);
                        self.stderr.extend_from_slice(b" modversion is unchanged\n");
                    }
                    self.symbols[id].declared = true;
                    return Some(id);
                } else if old.declared {
                    self.error([b"redefinition of ".as_slice(), &name].concat());
                    return Some(id);
                } else if old.override_version && self.preserve {
                    self.location();
                    self.stderr.extend_from_slice(b"ignoring ");
                    self.type_name(kind, &name);
                    self.stderr.extend_from_slice(b" modversion change\n");
                    self.symbols[id].declared = true;
                    return Some(id);
                } else {
                    let unknown = old.kind.namespace()
                        && old.definition.ends_with(&[
                            Word::plain("{"),
                            Word::plain("UNKNOWN"),
                            Word::plain("}"),
                        ]);
                    status = if unknown {
                        Status::Defined
                    } else {
                        Status::Modified
                    };
                }
            }
        }
        if self.debug != 0 {
            self.stderr.extend_from_slice(b"Defn for ");
            if kind == Kind::Normal {
                self.stderr.extend_from_slice(b"type0 ");
            } else {
                self.stderr.extend_from_slice(kind.name().as_bytes());
                self.stderr.push(b' ');
            }
            self.stderr.extend_from_slice(&name);
            self.stderr.extend_from_slice(b" == <");
            if external {
                self.stderr.extend_from_slice(b"extern ");
            }
            print_words(&definition, &mut self.stderr);
            self.stderr.extend_from_slice(b">\n");
        }
        let id = self.symbols.len();
        self.symbols.push(Symbol {
            name,
            kind,
            definition,
            external,
            declared: !reference,
            status,
            override_version: false,
            visited: false,
        });
        self.index.insert(key, id);
        Some(id)
    }
    fn export(&mut self, name: &[u8]) {
        let Some(root) = self.find(name, Kind::Normal, false) else {
            self.error([b"export undefined symbol ".as_slice(), name].concat());
            return;
        };
        if self.dump {
            self.stderr.extend_from_slice(b"Export ");
            self.stderr.extend_from_slice(name);
            self.stderr.extend_from_slice(b" == <");
        }
        let mut crc = u32::MAX;
        let mut trail = vec![root];
        let mut expanded = std::collections::HashSet::from([root]);
        let mut stack = vec![(root, 0)];
        while let Some((id, offset)) = stack.last_mut() {
            let id = *id;
            if *offset == self.symbols[id].definition.len() {
                if *offset != 0 && !self.symbols[id].visited {
                    self.symbols[id].visited = true;
                    self.visited.push(id);
                }
                stack.pop();
                continue;
            }
            let word = self.symbols[id].definition[*offset].clone();
            *offset += 1;
            if word.kind == Kind::Normal {
                self.crc_word(&word.text, &mut crc);
                continue;
            }
            let target = self.find(&word.text, word.kind, false).or_else(|| {
                if !word.kind.namespace() {
                    return None;
                }
                self.error(
                    [
                        b"expand undefined ".as_slice(),
                        word.kind.name().as_bytes(),
                        b" ",
                        &word.text,
                    ]
                    .concat(),
                );
                self.add(
                    Some(word.text.clone()),
                    word.kind,
                    vec![
                        Word::plain(word.kind.name()),
                        Word::plain(&word.text),
                        Word::plain("{"),
                        Word::plain("UNKNOWN"),
                        Word::plain("}"),
                    ],
                    false,
                    false,
                )
            });
            let Some(target) = target else {
                // Invalid reference files used to dereference NULL here.
                self.error(
                    [
                        b"expand undefined ".as_slice(),
                        word.kind.name().as_bytes(),
                        b" ",
                        &word.text,
                    ]
                    .concat(),
                );
                self.crc_word(&word.text, &mut crc);
                continue;
            };
            if expanded.insert(target) {
                trail.push(target);
                stack.push((target, 0));
            } else {
                if word.kind.namespace() {
                    self.crc_word(word.kind.name().as_bytes(), &mut crc);
                }
                self.crc_word(&word.text, &mut crc);
            }
        }
        let mut changed = false;
        for id in trail.into_iter().rev() {
            let symbol = &self.symbols[id];
            if symbol.status == Status::Unchanged {
                continue;
            }
            let (kind, symbol_name, status) = (symbol.kind, symbol.name.clone(), symbol.status);
            if !changed {
                self.location();
                self.stderr.extend_from_slice(if self.preserve {
                    b"error: "
                } else {
                    b"warning: "
                });
                self.stderr.extend_from_slice(name);
                self.stderr
                    .extend_from_slice(b": modversion changed because of changes in ");
            } else {
                self.stderr.extend_from_slice(b", ");
            }
            self.type_name(kind, &symbol_name);
            if status == Status::Defined {
                self.stderr.extend_from_slice(b" (became defined)");
            }
            changed = true;
            if self.preserve {
                self.errors += 1;
            }
        }
        if changed {
            self.stderr.push(b'\n');
        }
        if self.dump {
            self.stderr.extend_from_slice(b">\n");
        }
        self.stdout.extend_from_slice(b"#SYMVER ");
        self.stdout.extend_from_slice(name);
        self.stdout
            .extend_from_slice(format!(" 0x{:08x}\n", crc ^ u32::MAX).as_bytes());
    }
    fn crc_word(&mut self, text: &[u8], crc: &mut u32) {
        if self.dump {
            self.stderr.extend_from_slice(text);
            self.stderr.push(b' ');
        }
        for &byte in text.iter().chain(std::iter::once(&b' ')) {
            *crc ^= u32::from(byte);
            for _ in 0..8 {
                *crc = (*crc >> 1) ^ (0xedb8_8320 & 0u32.wrapping_sub(*crc & 1));
            }
        }
    }
    fn dump_types(&self) -> Vec<u8> {
        let mut output = Vec::new();
        for &id in &self.visited {
            let symbol = &self.symbols[id];
            if symbol.override_version {
                output.extend_from_slice(b"override ");
            }
            Word {
                text: symbol.name.clone(),
                kind: symbol.kind,
            }
            .print(&mut output);
            output.push(b' ');
            if symbol.external {
                output.extend_from_slice(b"extern ");
            }
            print_words(&symbol.definition, &mut output);
            output.push(b'\n');
        }
        output
    }
    fn read_reference(&mut self, input: &[u8]) -> Result<(), Vec<u8>> {
        let mut position = 0;
        while position < input.len() {
            let Some(mut name) = reference_word(input, &mut position)? else {
                continue;
            };
            let override_version = name == Word::plain("override");
            if override_version {
                let Some(next) = reference_word(input, &mut position)? else {
                    continue;
                };
                name = next;
            }
            let mut first = reference_word(input, &mut position)?;
            let external = first.as_ref() == Some(&Word::plain("extern"));
            if external {
                first = reference_word(input, &mut position)?;
            }
            let mut definition = Vec::new();
            while let Some(word) = first {
                definition.push(word);
                first = reference_word(input, &mut position)?;
            }
            if let Some(id) = self.add(Some(name.text), name.kind, definition, external, true) {
                self.symbols[id].override_version = override_version;
            }
        }
        Ok(())
    }
}

fn reference_word(input: &[u8], position: &mut usize) -> Result<Option<Word>, Vec<u8>> {
    let mut token = Vec::new();
    let mut quoted = false;
    while let Some(&byte) = input.get(*position) {
        *position += 1;
        if byte == b' ' && !quoted {
            if token.is_empty() {
                continue;
            }
            break;
        } else if byte == b'"' {
            quoted = !quoted;
        } else if byte == b'\n' {
            if token.is_empty() {
                return Ok(None);
            }
            *position -= 1;
            break;
        }
        if token.len() == 255 {
            return Err(b"Token too long\n".to_vec());
        }
        token.push(byte);
    }
    if token.is_empty() {
        return Ok(None);
    }
    token.truncate(
        token
            .iter()
            .position(|&byte| byte == 0)
            .unwrap_or(token.len()),
    );
    let mut kind = Kind::Normal;
    if token.get(1) == Some(&b'#') {
        kind = match token[0] {
            b't' => Kind::Typedef,
            b'e' => Kind::Enum,
            b's' => Kind::Struct,
            b'u' => Kind::Union,
            b'E' => Kind::EnumConst,
            byte => {
                return Err(vec![
                    b'U', b'n', b'k', b'n', b'o', b'w', b'n', b' ', b't', b'y', b'p', b'e', b' ',
                    byte, b'\n',
                ])
            }
        };
        token.drain(..2);
    }
    Ok(Some(Word { text: token, kind }))
}

const USAGE: &[u8] = b"Usage:\ngenksyms [-adDTwqhVR] > /path/to/.tmp_obj.ver\n\n  -d, --debug           Increment the debug level (repeatable)\n  -D, --dump            Dump expanded symbol defs (for debugging only)\n  -r, --reference file  Read reference symbols from a file\n  -T, --dump-types file Dump expanded types into file\n  -p, --preserve        Preserve reference modversions or fail\n  -w, --warnings        Enable warnings\n  -q, --quiet           Disable warnings (default)\n  -h, --help            Print this message\n  -V, --version         Print the release version\n";

fn perror(path: &[u8], error: &io::Error, output: &mut Vec<u8>) {
    output.extend_from_slice(path);
    output.extend_from_slice(b": ");
    let message = error.to_string();
    output.extend_from_slice(
        message
            .split(" (os error ")
            .next()
            .unwrap_or(&message)
            .as_bytes(),
    );
    output.push(b'\n');
}

fn run(symbols: &mut Symbols) -> i32 {
    let args: Vec<Vec<u8>> = std::env::args_os().map(|arg| arg.into_vec()).collect();
    let mut reference = None;
    let mut dumpfile = None;
    let options: [(&[u8], u8, bool); 9] = [
        (b"debug", b'd', false),
        (b"warnings", b'w', false),
        (b"quiet", b'q', false),
        (b"dump", b'D', false),
        (b"reference", b'r', true),
        (b"dump-types", b'T', true),
        (b"preserve", b'p', false),
        (b"version", b'V', false),
        (b"help", b'h', false),
    ];
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        i += 1;
        if arg == b"--" {
            break;
        }
        if arg.len() < 2 || arg[0] != b'-' {
            if std::env::var_os("POSIXLY_CORRECT").is_some() {
                break;
            }
            continue;
        }
        let mut selected = Vec::new();
        if arg.starts_with(b"--") {
            let split = arg.iter().position(|&byte| byte == b'=');
            let name = &arg[2..split.unwrap_or(arg.len())];
            let mut matches: Vec<_> = options
                .iter()
                .filter(|option| option.0.starts_with(name))
                .collect();
            if let Some(exact) = options.iter().find(|option| option.0 == name) {
                matches = vec![exact];
            }
            if matches.len() != 1 {
                symbols.stderr.extend_from_slice(&args[0]);
                if matches.is_empty() {
                    symbols.stderr.extend_from_slice(b": unrecognized option '");
                    symbols.stderr.extend_from_slice(arg);
                    symbols.stderr.extend_from_slice(b"'\n");
                } else {
                    symbols.stderr.extend_from_slice(b": option '");
                    symbols.stderr.extend_from_slice(arg);
                    symbols
                        .stderr
                        .extend_from_slice(b"' is ambiguous; possibilities:");
                    for option in matches {
                        symbols.stderr.extend_from_slice(b" '--");
                        symbols.stderr.extend_from_slice(option.0);
                        symbols.stderr.push(b'\'');
                    }
                    symbols.stderr.push(b'\n');
                }
                symbols.stderr.extend_from_slice(USAGE);
                return 1;
            }
            let option = matches[0];
            if split.is_some() && !option.2 {
                symbols.stderr.extend_from_slice(&args[0]);
                symbols.stderr.extend_from_slice(b": option '--");
                symbols.stderr.extend_from_slice(option.0);
                symbols
                    .stderr
                    .extend_from_slice(b"' doesn't allow an argument\n");
                symbols.stderr.extend_from_slice(USAGE);
                return 1;
            }
            selected.push((option.1, split.map(|at| arg[at + 1..].to_vec()), true));
        } else {
            let mut offset = 1;
            while offset < arg.len() {
                let option = arg[offset];
                offset += 1;
                let value = if matches!(option, b'r' | b'T') && offset < arg.len() {
                    Some(arg[offset..].to_vec())
                } else {
                    None
                };
                selected.push((option, value, false));
                if matches!(option, b'r' | b'T') {
                    break;
                }
            }
        }
        for (option, mut value, long) in selected {
            if matches!(option, b'r' | b'T') && value.is_none() {
                if i == args.len() {
                    symbols.stderr.extend_from_slice(&args[0]);
                    if long {
                        symbols.stderr.extend_from_slice(b": option '--");
                        symbols.stderr.extend_from_slice(if option == b'r' {
                            b"reference"
                        } else {
                            b"dump-types"
                        });
                        symbols
                            .stderr
                            .extend_from_slice(b"' requires an argument\n");
                    } else {
                        symbols
                            .stderr
                            .extend_from_slice(b": option requires an argument -- '");
                        symbols.stderr.push(option);
                        symbols.stderr.extend_from_slice(b"'\n");
                    }
                    symbols.stderr.extend_from_slice(USAGE);
                    return 1;
                }
                value = Some(args[i].clone());
                i += 1;
            }
            match option {
                b'd' => symbols.debug += 1,
                b'D' => symbols.dump = true,
                b'w' => symbols.warnings = true,
                b'q' => symbols.warnings = false,
                b'p' => symbols.preserve = true,
                b'V' => symbols
                    .stderr
                    .extend_from_slice(b"genksyms version 2.5.60\n"),
                b'h' => {
                    symbols.stderr.extend_from_slice(USAGE);
                    return 0;
                }
                b'r' | b'T' => {
                    let path = value.unwrap_or_default();
                    let opened = if option == b'r' {
                        File::open(OsStr::from_bytes(&path))
                    } else {
                        File::create(OsStr::from_bytes(&path))
                    };
                    match opened {
                        Ok(file) => {
                            if option == b'r' {
                                reference = Some((file, path));
                            } else {
                                dumpfile = Some(file);
                            }
                        }
                        Err(error) => {
                            perror(&path, &error, &mut symbols.stderr);
                            return 1;
                        }
                    }
                }
                _ => {
                    symbols.stderr.extend_from_slice(&args[0]);
                    symbols.stderr.extend_from_slice(b": invalid option -- '");
                    symbols.stderr.push(option);
                    symbols.stderr.extend_from_slice(b"'\n");
                    symbols.stderr.extend_from_slice(USAGE);
                    return 1;
                }
            }
        }
    }
    if let Some((mut file, path)) = reference {
        let mut input = Vec::new();
        if let Err(error) = file.read_to_end(&mut input) {
            // The old feof loop never terminated on a reference read error.
            perror(&path, &error, &mut symbols.stderr);
            return 1;
        }
        if let Err(error) = symbols.read_reference(&input) {
            symbols.stderr.extend(error);
            return 1;
        }
    }
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        symbols
            .stderr
            .extend_from_slice(b"input in flex scanner failed\n");
        return 2;
    }
    parser::parse(&input, symbols);
    if let Some(mut file) = dumpfile {
        let _ = file.write_all(&symbols.dump_types());
    }
    if symbols.debug != 0 {
        let count = symbols.index.len();
        let ratio = count as f64 / 4096.0;
        let precision = if ratio == 0.0 {
            0
        } else {
            (5 - ratio.log10().floor() as i32).max(0) as usize
        };
        let mut ratio = format!("{ratio:.precision$}");
        if ratio.contains('.') {
            ratio = ratio.trim_end_matches('0').trim_end_matches('.').to_owned();
        }
        symbols
            .stderr
            .extend_from_slice(format!("Hash table occupancy {count}/4096 = {ratio}\n").as_bytes());
    }
    i32::from(symbols.errors != 0)
}

fn main() {
    let mut symbols = Symbols::new();
    let status = run(&mut symbols);
    let _ = io::stdout().lock().write_all(&symbols.stdout);
    let _ = io::stderr().lock().write_all(&symbols.stderr);
    std::process::exit(status);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
