// SPDX-License-Identifier: GPL-2.0
//! Generate a newc/crc initramfs archive from the gen_init_cpio file-list format.
//! Keep the archive and command-line interface compatible with gen_init_cpio.c.
//
// Original work by Jeff Garzik. External file lists, symlinks, pipes, and
// FIFOs by Thayne Harbaugh. Hard-link support by Luciano Rocha.

use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Seek, Write};
use std::os::fd::AsFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::time::{SystemTime, UNIX_EPOCH};

const PATH_MAX: usize = 4096;
const HEADER_LEN: usize = 110;
const LINE_SIZE: usize = 2 * PATH_MAX + 50;
const REGULAR: u32 = 0o100000;
const DIRECTORY: u32 = 0o040000;
const SYMLINK: u32 = 0o120000;
const FIFO: u32 = 0o010000;
const SOCKET: u32 = 0o140000;
const BLOCK: u32 = 0o060000;
const CHARACTER: u32 = 0o020000;

// This is also the historical best-effort alignment rule for alignments that
// are multiples of four but not powers of two.
fn padding(offset: u32, alignment: u32) -> usize {
    ((alignment - (offset & (alignment - 1))) % alignment) as usize
}

fn display(bytes: &[u8]) -> std::borrow::Cow<'_, str> {
    String::from_utf8_lossy(bytes)
}

fn is_space(byte: &u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

fn trim_space(bytes: &[u8]) -> &[u8] {
    &bytes[bytes
        .iter()
        .position(|byte| !is_space(byte))
        .unwrap_or(bytes.len())..]
}

#[derive(Clone, Copy)]
struct Attributes {
    mode: u32,
    uid: u32,
    gid: u32,
}

struct Archive {
    output: File,
    offset: u32,
    inode: u32,
    timestamp: u32,
    override_timestamp: bool,
    checksum: bool,
    alignment: u32,
}

impl Archive {
    fn write(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.output.write_all(bytes)?;
        self.offset = self.offset.wrapping_add(bytes.len() as u32);
        Ok(())
    }

    fn pad(&mut self, count: usize) -> io::Result<()> {
        self.write(&[0; PATH_MAX][..count])
    }

    fn align(&mut self, alignment: u32) -> io::Result<()> {
        self.pad(padding(self.offset, alignment))
    }

    fn header(&mut self, fields: [u32; 13]) -> io::Result<()> {
        let mut header = String::with_capacity(HEADER_LEN);
        header.push_str(if self.checksum { "070702" } else { "070701" });
        for field in fields {
            use std::fmt::Write;
            write!(&mut header, "{field:08X}").expect("writing to a String cannot fail");
        }
        self.write(header.as_bytes())
    }

    fn name(&mut self, name: &[u8], extra_padding: usize) -> io::Result<()> {
        self.write(name)?;
        self.write(&[0])?;
        if extra_padding == 0 {
            self.align(4)
        } else {
            self.pad(extra_padding)
        }
    }

    fn special(
        &mut self,
        name: &[u8],
        attributes: Attributes,
        links: u32,
        device: (u32, u32),
        target: Option<&[u8]>,
    ) -> io::Result<()> {
        let name = name.strip_prefix(b"/").unwrap_or(name);
        self.header([
            self.inode,
            attributes.mode,
            attributes.uid,
            attributes.gid,
            links,
            self.timestamp,
            target.map_or(0, |s| s.len() as u32 + 1),
            3,
            1,
            device.0,
            device.1,
            name.len() as u32 + 1,
            0,
        ])?;
        self.inode = self.inode.wrapping_add(1);
        self.name(name, 0)?;
        if let Some(target) = target {
            self.write(target)?;
            self.write(&[0])?;
            self.align(4)?;
        }
        Ok(())
    }

    fn regular(
        &mut self,
        names: &[&[u8]],
        location: &[u8],
        attributes: Attributes,
    ) -> Result<(), ()> {
        let mut input = File::open(OsStr::from_bytes(location)).map_err(|_| {
            eprintln!("File {} could not be opened for reading", display(location));
        })?;
        let metadata = input.metadata().map_err(|_| {
            eprintln!("File {} could not be stat()'ed", display(location));
        })?;
        let timestamp = if self.override_timestamp {
            self.timestamp
        } else if metadata.mtime() < 0 {
            eprintln!("{}: Timestamp negative, clipping.", display(location));
            0
        } else if metadata.mtime() > u32::MAX as i64 {
            eprintln!(
                "{}: Timestamp exceeds maximum cpio timestamp, clipping.",
                display(location)
            );
            u32::MAX
        } else {
            metadata.mtime() as u32
        };
        let size = u32::try_from(metadata.len()).map_err(|_| {
            eprintln!("{}: Size exceeds maximum cpio file size", display(location));
        })?;
        let checksum = if self.checksum {
            file_checksum(&mut input, size).map_err(|_| {
                eprintln!("Failed to checksum file {}", display(location));
            })?
        } else {
            0
        };

        for (index, name) in names.iter().enumerate() {
            let name = name.strip_prefix(b"/").unwrap_or(name);
            let size = if index + 1 == names.len() { size } else { 0 };
            let namesize = name.len() as u32 + 1;
            let mut name_padding = 0;
            if self.alignment != 0 && size > self.alignment {
                name_padding = padding(
                    self.offset
                        .wrapping_add(HEADER_LEN as u32)
                        .wrapping_add(namesize),
                    self.alignment,
                );
                if namesize as usize + name_padding > PATH_MAX {
                    eprintln!(
                        "{}: best-effort alignment {} missed",
                        display(name),
                        self.alignment
                    );
                    name_padding = 0;
                }
            }
            self.header([
                self.inode,
                attributes.mode | REGULAR,
                attributes.uid,
                attributes.gid,
                names.len() as u32,
                timestamp,
                size,
                3,
                1,
                0,
                0,
                namesize + name_padding as u32,
                if size == 0 { 0 } else { checksum },
            ])
            .map_err(|_| ())?;
            self.name(name, name_padding).map_err(|_| ())?;
            let mut remaining = size as usize;
            let mut buffer = [0; 65536];
            while remaining != 0 {
                let limit = remaining.min(buffer.len());
                let read = match input.read(&mut buffer[..limit]) {
                    Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                    Ok(read) if read != 0 => read,
                    _ => {
                        eprintln!("Can not read {} file", display(location));
                        return Err(());
                    }
                };
                self.write(&buffer[..read])
                    .map_err(|_| eprintln!("writing filebuf failed"))?;
                remaining -= read;
            }
            self.align(4).map_err(|_| ())?;
        }
        self.inode = self.inode.wrapping_add(1);
        Ok(())
    }

    fn trailer(&mut self) -> io::Result<()> {
        self.header([0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 11, 0])?;
        self.name(b"TRAILER!!!", 0)?;
        self.align(512)?;
        match self.output.sync_all() {
            Err(error) if error.kind() == io::ErrorKind::InvalidInput => Ok(()),
            result => result,
        }
    }
}

fn file_checksum(input: &mut File, size: u32) -> io::Result<u32> {
    let mut remaining = size as usize;
    let mut checksum = 0u32;
    let mut buffer = [0; 65536];
    while remaining != 0 {
        let count = remaining.min(buffer.len());
        input.read_exact(&mut buffer[..count])?;
        checksum = buffer[..count]
            .iter()
            .fold(checksum, |sum, &byte| sum.wrapping_add(byte as u32));
        remaining -= count;
    }
    input.rewind()?;
    Ok(checksum)
}

// Scan bytes, rather than UTF-8 strings: Unix filenames, link targets, and
// environment values need not be Unicode. Numeric conversions retain sscanf's
// prefix parsing, signed uid/gid conversion, and octal permission syntax.
struct Scanner<'a> {
    remaining: &'a [u8],
}

impl<'a> Scanner<'a> {
    fn whitespace(&mut self) {
        self.remaining = trim_space(self.remaining);
    }

    fn word(&mut self) -> Option<&'a [u8]> {
        self.whitespace();
        let length = self
            .remaining
            .iter()
            .position(is_space)
            .unwrap_or(self.remaining.len())
            .min(PATH_MAX);
        if length == 0 {
            return None;
        }
        let (word, rest) = self.remaining.split_at(length);
        self.remaining = rest;
        Some(word)
    }

    fn number(&mut self, radix: u32) -> Option<u32> {
        self.whitespace();
        let (number, length) = parse_number(self.remaining, radix)?;
        self.remaining = &self.remaining[length..];
        Some(number as u32)
    }

    fn character(&mut self) -> Option<u8> {
        self.whitespace();
        let (&byte, rest) = self.remaining.split_first()?;
        self.remaining = rest;
        Some(byte)
    }

    fn attributes(&mut self) -> Option<Attributes> {
        Some(Attributes {
            mode: self.number(8)?,
            uid: self.number(10)?,
            gid: self.number(10)?,
        })
    }
}

fn parse_number(bytes: &[u8], radix: u32) -> Option<(u64, usize)> {
    let mut index = 0;
    let negative = bytes.first() == Some(&b'-');
    if negative || bytes.first() == Some(&b'+') {
        index += 1;
    }
    let start = index;
    let mut number = 0u64;
    let mut overflow = false;
    while let Some(&byte) = bytes.get(index) {
        if !byte.is_ascii_digit() || (byte - b'0') as u32 >= radix {
            break;
        }
        match number
            .checked_mul(radix as u64)
            .and_then(|n| n.checked_add((byte - b'0') as u64))
        {
            Some(n) => number = n,
            None => overflow = true,
        }
        index += 1;
    }
    if index == start {
        None
    } else {
        Some((
            if overflow {
                u64::MAX
            } else if negative {
                number.wrapping_neg()
            } else {
                number
            },
            index,
        ))
    }
}

fn expand_environment(location: &[u8]) -> Vec<u8> {
    let mut expanded = location.to_vec();
    // C expands inserted values recursively. Detect cycles so malformed user
    // input cannot hang the build forever.
    let mut seen = std::collections::HashSet::new();
    while seen.insert(expanded.clone()) {
        let Some(start) = expanded.windows(2).position(|window| window == b"${") else {
            break;
        };
        let Some(end) = expanded[start + 2..].iter().position(|&byte| byte == b'}') else {
            break;
        };
        let end = start + 2 + end;
        let value = env::var_os(OsStr::from_bytes(&expanded[start + 2..end]));
        let mut replacement = expanded[..start].to_vec();
        if let Some(value) = value {
            replacement.extend_from_slice(value.as_bytes());
        }
        replacement.extend_from_slice(&expanded[end + 1..]);
        replacement.truncate(PATH_MAX);
        expanded = replacement;
    }
    expanded
}

fn entry(archive: &mut Archive, kind: &[u8], line: &[u8]) -> Result<(), ()> {
    let mut scanner = Scanner { remaining: line };
    let parsed = (|| -> Option<Result<(), ()>> {
        let name = scanner.word()?;
        match kind {
            b"file" => {
                let location = scanner.word()?;
                let attributes = scanner.attributes()?;
                let mut names = vec![name];
                scanner.whitespace();
                while scanner.remaining.first().is_some_and(u8::is_ascii_graphic) {
                    names.push(scanner.word()?);
                    scanner.whitespace();
                }
                Some(archive.regular(&names, &expand_environment(location), attributes))
            }
            b"slink" => {
                let target = scanner.word()?;
                let mut attributes = scanner.attributes()?;
                attributes.mode |= SYMLINK;
                Some(
                    archive
                        .special(name, attributes, 1, (0, 0), Some(target))
                        .map_err(|_| ()),
                )
            }
            b"nod" => {
                let mut attributes = scanner.attributes()?;
                let device_type = scanner.character()?;
                let major = scanner.number(10)?;
                let minor = scanner.number(10)?;
                attributes.mode |= if device_type == b'b' {
                    BLOCK
                } else {
                    CHARACTER
                };
                Some(
                    archive
                        .special(name, attributes, 1, (major, minor), None)
                        .map_err(|_| ()),
                )
            }
            _ => {
                let mut attributes = scanner.attributes()?;
                attributes.mode |= match kind {
                    b"dir" => DIRECTORY,
                    b"pipe" => FIFO,
                    _ => SOCKET,
                };
                Some(
                    archive
                        .special(name, attributes, 2, (0, 0), None)
                        .map_err(|_| ()),
                )
            }
        }
    })();
    parsed.unwrap_or_else(|| {
        if matches!(kind, b"dir" | b"pipe" | b"sock") {
            // The generic C handler prints these arguments in this order.
            eprint!("Unrecognized {} format '{}'", display(line), display(kind));
        } else {
            let kind = if kind == b"slink" { &b"dir"[..] } else { kind };
            eprint!("Unrecognized {} format '{}'", display(kind), display(line));
        }
        Err(())
    })
}

fn usage(program: &OsStr) {
    eprint!(
        "Usage:\n\t{} [-t <timestamp>] [-c] [-o <output_file>] [-a <data_align>] <cpio_list>\n\n\
<cpio_list> is a file containing newline separated entries that\n\
describe the files to be included in the initramfs archive:\n\n\
# a comment\n\
file <name> <location> <mode> <uid> <gid> [<hard links>]\n\
dir <name> <mode> <uid> <gid>\n\
nod <name> <mode> <uid> <gid> <dev_type> <maj> <min>\n\
slink <name> <target> <mode> <uid> <gid>\n\
pipe <name> <mode> <uid> <gid>\n\
sock <name> <mode> <uid> <gid>\n\n\
<name>       name of the file/dir/nod/etc in the archive\n\
<location>   location of the file in the current filesystem\n\
\x20            expands shell variables quoted with ${{}}\n\
<target>     link target\n\
<mode>       mode/permissions of the file\n\
<uid>        user id (0=root)\n\
<gid>        group id (0=root)\n\
<dev_type>   device type (b=block, c=character)\n\
<maj>        major number of nod\n\
<min>        minor number of nod\n\
<hard links> space separated list of other links to file\n\n\
example:\n\
# A simple initramfs\n\
dir /dev 0755 0 0\n\
nod /dev/console 0600 0 0 c 5 1\n\
dir /root 0700 0 0\n\
dir /sbin 0755 0 0\n\
file /sbin/kinit /usr/src/klibc/kinit/kinit 0755 0 0\n\n\
<timestamp> is time in seconds since Epoch that will be used\n\
as mtime for symlinks, directories, regular and special files.\n\
The default is to use the current time for all files, but\n\
preserve modification time for regular files.\n\
-c: calculate and store 32-bit checksums for file data.\n\
<output_file>: write cpio to this file instead of stdout\n\
<data_align>: attempt to align file data by zero-padding the\n\
filename field up to data_align. Must be a multiple of 4.\n\
Alignment is best-effort; PATH_MAX limits filename padding.\n",
        program.to_string_lossy()
    );
}

struct Options {
    timestamp: u32,
    override_timestamp: bool,
    checksum: bool,
    alignment: u32,
    output: Option<File>,
    list: OsString,
}

fn options(program: &OsStr, args: &[OsString]) -> Result<Options, i32> {
    let mut timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(u64::MAX, |t| t.as_secs());
    let mut override_timestamp = false;
    let mut checksum = false;
    let mut alignment = 0;
    let mut output = None;
    let mut positional = Vec::new();
    let mut parse_options = true;
    let mut index = 0;
    while index < args.len() {
        let argument = args[index].as_bytes();
        index += 1;
        if parse_options && argument == b"--" {
            parse_options = false;
            continue;
        }
        if !parse_options || argument.len() < 2 || argument[0] != b'-' {
            positional.push(args[index - 1].clone());
            if env::var_os("POSIXLY_CORRECT").is_some() {
                parse_options = false;
            }
            continue;
        }
        let mut option_index = 1;
        while option_index < argument.len() {
            let option = argument[option_index];
            option_index += 1;
            match option {
                b'h' => {
                    usage(program);
                    return Err(0);
                }
                b'c' => {
                    checksum = true;
                    continue;
                }
                b't' | b'o' | b'a' => {}
                _ => {
                    eprintln!(
                        "{}: invalid option -- '{}'",
                        program.to_string_lossy(),
                        option as char
                    );
                    usage(program);
                    return Err(1);
                }
            }
            let value = if option_index < argument.len() {
                let value = &argument[option_index..];
                option_index = argument.len();
                value
            } else if let Some(value) = args.get(index) {
                index += 1;
                value.as_bytes()
            } else {
                eprintln!(
                    "{}: option requires an argument -- '{}'",
                    program.to_string_lossy(),
                    option as char
                );
                usage(program);
                return Err(1);
            };
            if option == b'o' {
                output = Some(
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .mode(0o600)
                        .open(OsStr::from_bytes(value))
                        .map_err(|_| {
                            eprintln!("failed to open {}", display(value));
                            usage(program);
                            1
                        })?,
                );
            } else {
                let number_text = trim_space(value);
                let number = parse_number(number_text, 10)
                    .filter(|(_, length)| *length == number_text.len());
                let Some((number, _)) =
                    number.filter(|(number, _)| option != b'a' || *number as u32 & 3 == 0)
                else {
                    eprintln!(
                        "Invalid {}: {}",
                        if option == b't' {
                            "timestamp"
                        } else {
                            "data_align"
                        },
                        display(value)
                    );
                    usage(program);
                    return Err(1);
                };
                if option == b't' {
                    timestamp = if number_text.starts_with(b"-")
                        && number_text[1..].iter().any(|&byte| byte != b'0')
                    {
                        u64::MAX
                    } else {
                        number
                    };
                    override_timestamp = true;
                } else {
                    alignment = number as u32;
                }
            }
        }
    }
    let timestamp = u32::try_from(timestamp).map_err(|_| {
        eprintln!("ERROR: Timestamp out of range for cpio format");
        1
    })?;
    if positional.len() != 1 {
        usage(program);
        return Err(1);
    }
    Ok(Options {
        timestamp,
        override_timestamp,
        checksum,
        alignment,
        output,
        list: positional.remove(0),
    })
}

// fgets splits overlong lines into LINE_SIZE-1 byte pieces. Keep that boundary
// so line numbers and the accepted file-list grammar agree with the C tool.
fn read_line(reader: &mut dyn BufRead, line: &mut Vec<u8>) -> io::Result<usize> {
    line.clear();
    while line.len() < LINE_SIZE - 1 {
        let available = reader.fill_buf()?;
        if available.is_empty() {
            break;
        }
        let available = &available[..available.len().min(LINE_SIZE - 1 - line.len())];
        let count = available
            .iter()
            .position(|&byte| byte == b'\n')
            .map_or(available.len(), |i| i + 1);
        let newline = available[count - 1] == b'\n';
        line.extend_from_slice(&available[..count]);
        reader.consume(count);
        if newline {
            break;
        }
    }
    Ok(line.len())
}

fn run() -> Result<(), i32> {
    let mut arguments = env::args_os();
    let program = arguments
        .next()
        .unwrap_or_else(|| OsString::from("gen_init_cpio"));
    let options = options(&program, &arguments.collect::<Vec<_>>())?;
    let mut list: Box<dyn BufRead> = if options.list == "-" {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(&options.list).map_err(
            |error| {
                let error = error.to_string();
                let error = error.split(" (os error ").next().unwrap_or(&error);
                eprintln!(
                    "ERROR: unable to open '{}': {}\n",
                    options.list.to_string_lossy(),
                    error
                );
                usage(&program);
                1
            },
        )?))
    };
    let output = match options.output {
        Some(output) => output,
        None => File::from(io::stdout().as_fd().try_clone_to_owned().map_err(|error| {
            eprintln!("Unable to open stdout: {error}");
            1
        })?),
    };
    let mut archive = Archive {
        output,
        offset: 0,
        inode: 721,
        timestamp: options.timestamp,
        override_timestamp: options.override_timestamp,
        checksum: options.checksum,
        alignment: options.alignment,
    };
    let mut line = Vec::with_capacity(LINE_SIZE);
    let mut line_number = 0;
    let mut failed = false;
    loop {
        match read_line(&mut *list, &mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(error) => {
                eprintln!("Unable to read file list: {error}");
                return Err(255);
            }
        }
        line_number += 1;
        let line = line.split(|&byte| byte == 0).next().unwrap_or(&[]);
        if line.first() == Some(&b'#') {
            continue;
        }
        let start = line.iter().position(|byte| !matches!(byte, b' ' | b'\t'));
        let Some(start) = start else {
            eprintln!(
                "ERROR: incorrect format, could not locate file type line {}: '{}'",
                line_number,
                display(line)
            );
            failed = true;
            break;
        };
        let end = line[start..]
            .iter()
            .position(|byte| matches!(byte, b' ' | b'\t'))
            .map_or(line.len(), |i| start + i);
        let kind = &line[start..end];
        if kind.first() == Some(&b'\n') || line.len() == kind.len() {
            continue;
        }
        // strtok changes only the first type delimiter. Leading whitespace was
        // never accepted as an entry prefix by the original parser.
        let printed_kind = if start == 0 { kind } else { &line[..end] };
        let args = line.get(end + 1..).unwrap_or(&[]);
        let args = args
            .split(|&byte| byte == b'\n')
            .find(|piece| !piece.is_empty());
        if args.is_none() {
            eprintln!(
                "ERROR: incorrect format, newline required line {}: '{}'",
                line_number,
                display(printed_kind)
            );
            failed = true;
        }
        if start != 0
            || !matches!(
                kind,
                b"file" | b"nod" | b"dir" | b"slink" | b"pipe" | b"sock"
            )
        {
            eprintln!(
                "unknown file type line {}: '{}'",
                line_number,
                display(printed_kind)
            );
        } else if entry(&mut archive, kind, args.unwrap_or(&[])).is_err() {
            failed = true;
            eprintln!(" line {line_number}");
        }
    }
    if failed {
        return Err(255);
    }
    archive.trailer().map_err(|_| 255)
}

fn main() {
    if let Err(status) = run() {
        std::process::exit(status);
    }
}
