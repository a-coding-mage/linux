// SPDX-License-Identifier: GPL-2.0-or-later

//! Historical module-source digest, with Kbuild dependency-file selection.
// Derived from the C implementation by Andrew Tridgell (1997-1998),
// Steve French (2002), the Cryptoapi developers, David S. Miller (2002),
// and James Morris (2002), in turn based on Colin Plumb's public-domain MD4.

use std::ffi::OsStr;
use std::fs;
use std::os::unix::ffi::OsStrExt;

struct Digest {
    hash: [u32; 4],
    block: [u8; 64],
    length: u64,
}

impl Digest {
    fn new() -> Self {
        Self {
            hash: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476],
            block: [0; 64],
            length: 0,
        }
    }

    fn transform(&mut self, words: [u32; 16]) {
        let mut state = self.hash;
        for round in 0..3 {
            for step in 0..16 {
                let a = (4 - step % 4) % 4;
                let b = state[(a + 1) % 4];
                let c = state[(a + 2) % 4];
                let d = state[(a + 3) % 4];
                let (function, constant, word, shift) = match round {
                    0 => ((b & c) | (!b & d), 0, step, [3, 7, 11, 19][step % 4]),
                    1 => (
                        (b & c) | (b & d) | (c & d),
                        0x5a827999,
                        (step % 4) * 4 + step / 4,
                        [3, 5, 9, 13][step % 4],
                    ),
                    _ => (
                        b ^ c ^ d,
                        0x6ed9eba1,
                        [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15][step],
                        [3, 9, 11, 15][step % 4],
                    ),
                };
                state[a] = state[a]
                    .wrapping_add(function)
                    .wrapping_add(words[word])
                    .wrapping_add(constant)
                    .rotate_left(shift);
            }
        }
        for (hash, value) in self.hash.iter_mut().zip(state) {
            *hash = hash.wrapping_add(value);
        }
    }

    fn words(&self) -> [u32; 16] {
        // The original uses ntohl here, despite naming this conversion le32.
        std::array::from_fn(|i| {
            u32::from_be_bytes(self.block[i * 4..i * 4 + 4].try_into().unwrap())
        })
    }

    fn add(&mut self, byte: u8) {
        self.block[(self.length & 63) as usize] = byte;
        self.length = self.length.wrapping_add(1);
        if self.length & 63 == 0 {
            self.transform(self.words());
        }
    }

    fn finish(mut self) -> String {
        let offset = (self.length & 63) as usize;
        self.block[offset] = 0x80;
        self.block[offset + 1..].fill(0);
        if offset >= 56 {
            self.transform(self.words());
            self.block.fill(0);
        }
        let mut words = self.words();
        words[14] = (self.length << 3) as u32;
        words[15] = (self.length >> 29) as u32;
        self.transform(words);
        let mut result = self
            .hash
            .iter()
            .map(|word| format!("{:08X}", word.to_be()))
            .collect::<String>();
        // modpost passes sizeof(srcversion)-1 (24) to snprintf, including NUL.
        result.truncate(23);
        result
    }
}

fn read(path: &[u8]) -> Result<Vec<u8>, String> {
    let mut data = fs::read(OsStr::from_bytes(path)).map_err(|error| {
        format!(
            "{}: {}\n",
            String::from_utf8_lossy(path),
            crate::io_error(&error)
        )
    })?;
    data.truncate(
        data.iter()
            .position(|&byte| byte == 0)
            .unwrap_or(data.len()),
    );
    Ok(data)
}

fn whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

fn parse_file(path: &[u8], digest: &mut Digest) -> Result<(), String> {
    let file = read(path)?;
    let mut pos = 0;
    while let Some(&byte) = file.get(pos) {
        if byte == b'\\' && file.get(pos + 1) == Some(&b'\n') {
            pos += 2;
            continue;
        }
        if whitespace(byte) {
            pos += 1;
            continue;
        }
        if byte == b'"' {
            digest.add(byte);
            pos += 1;
            while let Some(&next) = file.get(pos) {
                digest.add(next);
                pos += 1;
                if next == b'"' && file[pos - 2] != b'\\' {
                    break;
                }
            }
            continue;
        }
        if byte == b'/' && file.get(pos + 1) == Some(&b'*') {
            pos += 2;
            while pos < file.len() {
                pos += 1;
                if file[pos - 2..pos] == *b"*/" {
                    break;
                }
            }
            continue;
        }
        digest.add(byte);
        pos += 1;
    }
    Ok(())
}

pub(crate) struct SourceVersion {
    pub(crate) checksum: Option<String>,
    pub(crate) warnings: Vec<String>,
}

/// Hash source files and same-directory dependencies in the module object list.
pub(crate) fn get_src_version(module: &str) -> Result<SourceVersion, String> {
    let objects = read(format!("{module}.mod").as_bytes())?;
    let mut digest = Digest::new();
    let mut result = SourceVersion {
        checksum: None,
        warnings: Vec::new(),
    };
    for object in objects
        .split(|&byte| byte == b'\n')
        .filter(|name| !name.is_empty())
    {
        if object.ends_with(b".a") {
            continue;
        }
        let dirlen = object
            .iter()
            .rposition(|&byte| byte == b'/')
            .map_or(0, |pos| pos + 1);
        let directory = &object[..dirlen];
        let mut command_name = directory.to_vec();
        command_name.push(b'.');
        command_name.extend_from_slice(&object[dirlen..]);
        command_name.extend_from_slice(b".cmd");
        let command = read(&command_name)?;
        let mut check_files = false;
        for line in command.split(|&byte| byte == b'\n') {
            let line = &line[line.iter().take_while(|&&byte| whitespace(byte)).count()..];
            if line.starts_with(b"source_") {
                let Some(space) = line.iter().rposition(|&byte| byte == b' ') else {
                    result.warnings.push(format!(
                        "malformed line: {}\n",
                        String::from_utf8_lossy(line)
                    ));
                    return Ok(result);
                };
                parse_file(&line[space + 1..], &mut digest)?;
                continue;
            }
            if line.starts_with(b"deps_") {
                check_files = true;
                continue;
            }
            if !check_files {
                continue;
            }
            if !line.ends_with(b"\\") {
                break;
            }
            let path = line
                .split(|&byte| whitespace(byte))
                .next()
                .unwrap_or_default();
            // Match the original exact-parent check, not all subdirectories.
            // For a root-level object the C predicate never selects a dependency.
            if directory.is_empty() {
                continue;
            }
            if let Some(start) = path
                .windows(directory.len())
                .position(|part| part == directory)
            {
                if start
                    .checked_add(directory.len())
                    .and_then(|end| end.checked_sub(1))
                    == path.iter().rposition(|&byte| byte == b'/')
                {
                    parse_file(path, &mut digest)?;
                }
            }
        }
    }
    result.checksum = Some(digest.finish());
    Ok(result)
}
