// SPDX-License-Identifier: GPL-2.0-only
// Generate assembler source containing compressed kernel symbol information.
// Copyright 2002 Kai Germaschewski

//! Generate assembler tables containing compressed kernel symbol information.

use std::cmp::Ordering;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::process::ExitCode;

const KSYM_NAME_LEN: usize = 512;
const USAGE: &str = "Usage: kallsyms [--all-symbols] in.map > out.S\n";

#[derive(Default)]
struct Options {
    all_symbols: bool,
    pc_relative: bool,
    input: OsString,
}

fn options() -> Result<Options, String> {
    let mut args = env::args_os();
    let program = args.next().unwrap_or_default();
    let mut result = Options::default();
    let mut input = None;
    let mut scan_options = true;
    let posix = env::var_os("POSIXLY_CORRECT").is_some();
    for arg in args {
        let bytes = arg.as_encoded_bytes();
        if scan_options && bytes == b"--" {
            scan_options = false;
            continue;
        }
        if scan_options && bytes.starts_with(b"--") {
            let option = &bytes[2..];
            let end = option
                .iter()
                .position(|&c| c == b'=')
                .unwrap_or(option.len());
            let name = &option[..end];
            if name.is_empty() {
                return Err(format!(
                    "{}: option '{}' is ambiguous; possibilities: '--all-symbols' '--pc-relative'\n{USAGE}",
                    program.to_string_lossy(), arg.to_string_lossy()
                ));
            }
            let full = [b"all-symbols".as_slice(), b"pc-relative".as_slice()]
                .into_iter()
                .find(|candidate| candidate.starts_with(name));
            match full {
                Some(full) if end == option.len() => {
                    if full == b"all-symbols" {
                        result.all_symbols = true;
                    } else {
                        result.pc_relative = true;
                    }
                }
                Some(full) => {
                    return Err(format!(
                        "{}: option '--{}' doesn't allow an argument\n{USAGE}",
                        program.to_string_lossy(),
                        String::from_utf8_lossy(full)
                    ));
                }
                None => {
                    return Err(format!(
                        "{}: unrecognized option '{}'\n{USAGE}",
                        program.to_string_lossy(),
                        arg.to_string_lossy()
                    ));
                }
            }
        } else if scan_options && bytes.starts_with(b"-") && bytes.len() > 1 {
            return Err(format!(
                "{}: invalid option -- '{}'\n{USAGE}",
                program.to_string_lossy(),
                char::from(bytes[1])
            ));
        } else {
            if input.is_none() {
                input = Some(arg);
            }
            if posix {
                scan_options = false;
            }
        }
    }
    result.input = input.ok_or_else(|| USAGE.to_owned())?;
    Ok(result)
}

struct Symbol {
    addr: u64,
    seq: usize,
    // Keep the original bytes for assembly comments and the name-sorted index.
    // Symbol names in an nm map need not be UTF-8.
    name: Vec<u8>,
    compressed: Vec<u8>,
}

impl Symbol {
    fn name(&self) -> &[u8] {
        &self.name[1..]
    }
    fn is_weak(&self) -> bool {
        matches!(self.name[0], b'w' | b'W')
    }

    fn may_be_linker_provide(&self) -> bool {
        let name = self.name();
        name.len() >= 8
            && name.starts_with(b"__")
            && (name.starts_with(b"__start_")
                || name.starts_with(b"__stop_")
                || name.starts_with(b"__end_")
                || name.ends_with(b"_start")
                || name.ends_with(b"_end"))
    }

    fn prefix_underscores(&self) -> usize {
        self.name().iter().take_while(|&&c| c == b'_').count()
    }
}

fn compare_symbols(a: &Symbol, b: &Symbol) -> Ordering {
    a.addr
        .cmp(&b.addr)
        .then_with(|| a.is_weak().cmp(&b.is_weak()))
        .then_with(|| a.may_be_linker_provide().cmp(&b.may_be_linker_provide()))
        .then_with(|| a.prefix_underscores().cmp(&b.prefix_underscores()))
        .then_with(|| a.seq.cmp(&b.seq))
}

struct AddressRange {
    start_name: &'static [u8],
    end_name: &'static [u8],
    start: u64,
    end: u64,
}

struct Map {
    text: u64,
    ranges: [AddressRange; 2],
    symbols: Vec<Symbol>,
}

fn ignored_symbol(name: &[u8], kind: u8) -> bool {
    matches!(kind, b'u' | b'n')
        || (kind.eq_ignore_ascii_case(&b'A')
            && !matches!(
                name,
                b"__kernel_syscall_via_break"
                    | b"__kernel_syscall_via_epc"
                    | b"__kernel_sigtramp"
                    | b"__gp"
            ))
}

fn hex_digit(byte: u8) -> Option<u64> {
    match byte {
        b'0'..=b'9' => Some(u64::from(byte - b'0')),
        b'a'..=b'f' => Some(u64::from(byte - b'a' + 10)),
        b'A'..=b'F' => Some(u64::from(byte - b'A' + 10)),
        _ => None,
    }
}

// Match strtoull(..., 16), including its accepted whitespace/sign/prefix and
// saturation on overflow. The delimiters after the address are exactly spaces.
fn parse_line(line: &[u8]) -> Result<(u64, u8, &[u8]), String> {
    let mut pos = 0;
    while line
        .get(pos)
        .is_some_and(|c| c.is_ascii_whitespace() || *c == 0x0b)
    {
        pos += 1;
    }
    let negative = line.get(pos) == Some(&b'-');
    if negative || line.get(pos) == Some(&b'+') {
        pos += 1;
    }
    if line.get(pos) == Some(&b'0')
        && matches!(line.get(pos + 1), Some(b'x' | b'X'))
        && line.get(pos + 2).and_then(|&c| hex_digit(c)).is_some()
    {
        pos += 2;
    }
    let start = pos;
    let mut value = 0u64;
    let mut overflow = false;
    while let Some(digit) = line.get(pos).and_then(|&c| hex_digit(c)) {
        if let Some(next) = value.checked_mul(16).and_then(|v| v.checked_add(digit)) {
            value = next;
        } else {
            overflow = true;
        }
        pos += 1;
    }
    if pos == start
        || line.get(pos) != Some(&b' ')
        || !line.get(pos + 1).is_some_and(u8::is_ascii)
        || line.get(pos + 2) != Some(&b' ')
    {
        return Err("line format error\n".to_owned());
    }
    value = if overflow {
        u64::MAX
    } else if negative {
        value.wrapping_neg()
    } else {
        value
    };
    let name = &line[pos + 3..];
    let end = name.iter().position(|&c| c == 0).unwrap_or(name.len());
    Ok((value, line[pos + 1], &name[..end]))
}

fn io_message(context: &str, error: io::Error) -> String {
    let message = error.to_string();
    let message = message.split(" (os error ").next().unwrap_or(&message);
    format!("{context}: {message}\n")
}

impl Map {
    fn read(path: &Path) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| io_message(&path.to_string_lossy(), e))?;
        let mut input = BufReader::new(file);
        let mut map = Self {
            text: 0,
            ranges: [
                AddressRange {
                    start_name: b"_stext",
                    end_name: b"_etext",
                    start: 0,
                    end: 0,
                },
                AddressRange {
                    start_name: b"_sinittext",
                    end_name: b"_einittext",
                    start: 0,
                    end: 0,
                },
            ],
            symbols: Vec::new(),
        };
        let mut line = Vec::new();
        loop {
            line.clear();
            if input
                .read_until(b'\n', &mut line)
                .map_err(|e| io_message("read_symbol", e))?
                == 0
            {
                break;
            }
            if line.last() == Some(&b'\n') {
                line.pop();
            }
            let (addr, kind, name) = parse_line(&line)?;
            if name.len() >= KSYM_NAME_LEN {
                let mut error = io::stderr().lock();
                error.write_all(b"Symbol ").and_then(|()| error.write_all(name))
                    .and_then(|()| writeln!(error, " too long for kallsyms ({} >= {KSYM_NAME_LEN}).\nPlease increase KSYM_NAME_LEN both in kernel and kallsyms.c", name.len()))
                    .map_err(|e| io_message("stderr", e))?;
                continue;
            }
            if name == b"_text" {
                map.text = addr;
            }
            if ignored_symbol(name, kind) {
                continue;
            }
            for range in &mut map.ranges {
                if name == range.start_name {
                    range.start = addr;
                }
                if name == range.end_name {
                    range.end = addr;
                }
            }
            let mut typed_name = Vec::with_capacity(name.len() + 1);
            typed_name.push(kind);
            typed_name.extend_from_slice(name);
            map.symbols.push(Symbol {
                addr,
                seq: map.symbols.len(),
                compressed: typed_name.clone(),
                name: typed_name,
            });
        }
        Ok(map)
    }

    fn filter_and_sort(&mut self, all_symbols: bool) {
        if !all_symbols {
            let ranges = &self.ranges;
            self.symbols.retain(|symbol| {
                let name = symbol.name();
                name.starts_with(b"__start_")
                    || name.starts_with(b"__stop_")
                    || (ranges
                        .iter()
                        .any(|r| (r.start..=r.end).contains(&symbol.addr))
                        && !ranges
                            .iter()
                            .any(|r| symbol.addr == r.end && name != r.end_name))
            });
        }
        self.symbols.sort_unstable_by(compare_symbols);
    }
}

#[derive(Clone, Copy)]
enum Token {
    Unused,
    Literal(u8),
    Pair([u8; 2]),
}

struct Compression {
    tokens: [Token; 256],
    profit: Vec<i64>,
}

impl Compression {
    fn count(&mut self, symbol: &[u8], delta: i64) {
        for pair in symbol.windows(2) {
            self.profit[usize::from(pair[0]) | (usize::from(pair[1]) << 8)] += delta;
        }
    }

    fn build(symbols: &mut [Symbol]) -> Self {
        let mut result = Self {
            tokens: [Token::Unused; 256],
            profit: vec![0; 0x10000],
        };
        for symbol in symbols.iter() {
            result.count(&symbol.compressed, 1);
            for &byte in &symbol.compressed {
                result.tokens[usize::from(byte)] = Token::Literal(byte);
            }
        }
        // Descending free codes and ascending profit ties are part of the
        // deterministic compression, including code zero used last.
        for code in (0..256).rev() {
            if !matches!(result.tokens[code], Token::Unused) {
                continue;
            }
            let mut best = 0;
            for index in 1..result.profit.len() {
                if result.profit[index] > result.profit[best] {
                    best = index;
                }
            }
            if result.profit[best] == 0 {
                break;
            }
            let pair = [best as u8, (best >> 8) as u8];
            result.tokens[code] = Token::Pair(pair);
            for symbol in symbols.iter_mut() {
                let bytes = &mut symbol.compressed;
                if !bytes.windows(2).any(|window| window == pair) {
                    continue;
                }
                result.count(bytes, -1);
                let mut read = 0;
                let mut write = 0;
                while read < bytes.len() {
                    if bytes[read..].starts_with(&pair) {
                        bytes[write] = code as u8;
                        read += 2;
                    } else {
                        bytes[write] = bytes[read];
                        read += 1;
                    }
                    write += 1;
                }
                bytes.truncate(write);
                result.count(bytes, 1);
            }
        }
        result
    }

    fn expand(&self, code: u8, bytes: &mut Vec<u8>) {
        match self.tokens[usize::from(code)] {
            Token::Unused => (),
            Token::Literal(byte) => bytes.push(byte),
            Token::Pair(pair) => {
                self.expand(pair[0], bytes);
                self.expand(pair[1], bytes);
            }
        }
    }
}

fn output_label(output: &mut impl Write, label: &str) -> io::Result<()> {
    writeln!(output, ".globl {label}\n\t.balign 4\n{label}:")
}

fn comment(output: &mut impl Write, name: &[u8]) -> io::Result<()> {
    output.write_all(b"\t/* ")?;
    output.write_all(name)?;
    output.write_all(b" */\n")
}

fn write_src(
    output: &mut impl Write,
    map: &Map,
    compression: &Compression,
    pc_relative: bool,
) -> io::Result<()> {
    writeln!(output, "\t.section .rodata, \"a\"")?;
    output_label(output, "kallsyms_num_syms")?;
    writeln!(output, "\t.long\t{}\n", map.symbols.len())?;
    output_label(output, "kallsyms_names")?;
    let mut markers = Vec::new();
    let mut offset = 0u32;
    for (index, symbol) in map.symbols.iter().enumerate() {
        if index & 0xff == 0 {
            markers.push(offset);
        }
        let len = symbol.compressed.len();
        if len == 0 || len > 0x3fff {
            let adjective = if len == 0 { "zero" } else { "huge" };
            return Err(io::Error::other(format!(
                "kallsyms failure: unexpected {adjective} symbol length"
            )));
        }
        if len <= 0x7f {
            write!(output, "\t.byte 0x{len:02x}")?;
            offset = offset.wrapping_add(len as u32 + 1);
        } else {
            write!(
                output,
                "\t.byte 0x{:02x}, 0x{:02x}",
                (len & 0x7f) | 0x80,
                (len >> 7) & 0x7f
            )?;
            offset = offset.wrapping_add(len as u32 + 2);
        }
        for byte in &symbol.compressed {
            write!(output, ", 0x{byte:02x}")?;
        }
        comment(output, &symbol.name)?;
    }
    writeln!(output, ".size kallsyms_names, . - kallsyms_names\n")?;
    output_label(output, "kallsyms_markers")?;
    for marker in markers {
        writeln!(output, "\t.long\t{marker}")?;
    }
    writeln!(output, ".size kallsyms_markers, . - kallsyms_markers\n")?;
    output_label(output, "kallsyms_token_table")?;
    let mut token_index = Vec::with_capacity(256);
    offset = 0;
    for code in 0..=255 {
        let mut expansion = Vec::new();
        compression.expand(code, &mut expansion);
        token_index.push(offset);
        output.write_all(b"\t.asciz\t\"")?;
        output.write_all(&expansion)?;
        output.write_all(b"\"\n")?;
        offset += expansion.len() as u32 + 1;
    }
    writeln!(
        output,
        ".size kallsyms_token_table, . - kallsyms_token_table\n"
    )?;
    output_label(output, "kallsyms_token_index")?;
    for index in token_index {
        writeln!(output, "\t.short\t{index}")?;
    }
    writeln!(output)?;
    output_label(output, "kallsyms_offsets")?;
    for symbol in &map.symbols {
        if pc_relative {
            let relative = symbol.addr.wrapping_sub(map.text) as i64;
            let relative = i32::try_from(relative).map_err(|_| {
                let addr = if symbol.addr == 0 {
                    "0".to_owned()
                } else {
                    format!("{:#x}", symbol.addr)
                };
                io::Error::other(format!(
                    "kallsyms failure: relative symbol value {addr} out of range"
                ))
            })?;
            write!(output, "\t.long\t_text - . + ({relative})")?;
        } else {
            let addr = symbol.addr as u32;
            if addr == 0 {
                write!(output, "\t.long\t0")?;
            } else {
                write!(output, "\t.long\t{addr:#x}")?;
            }
        }
        comment(output, &symbol.name)?;
    }
    writeln!(output, ".size kallsyms_offsets, . - kallsyms_offsets\n")?;
    let mut names: Vec<_> = map.symbols.iter().enumerate().collect();
    names.sort_unstable_by(|(ia, a), (ib, b)| {
        a.name()
            .cmp(b.name())
            .then_with(|| a.addr.cmp(&b.addr))
            .then_with(|| ia.cmp(ib))
    });
    output_label(output, "kallsyms_seqs_of_names")?;
    for (index, symbol) in names {
        write!(
            output,
            "\t.byte 0x{:02x}, 0x{:02x}, 0x{:02x}",
            (index >> 16) as u8,
            (index >> 8) as u8,
            index as u8
        )?;
        comment(output, &symbol.name)?;
    }
    writeln!(output)
}

fn run() -> Result<(), String> {
    let options = options()?;
    let mut map = Map::read(Path::new(&options.input))?;
    map.filter_and_sort(options.all_symbols);
    let compression = Compression::build(&mut map.symbols);
    let mut output = BufWriter::new(io::stdout().lock());
    let result = write_src(&mut output, &map, &compression, options.pc_relative);
    // Preserve preceding output on an offset-range failure, just as stdio does.
    output.flush().map_err(|e| io_message("stdout", e))?;
    result.map_err(|e| format!("{e}\n"))
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let _ = io::stderr().write_all(error.as_bytes());
            ExitCode::FAILURE
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
