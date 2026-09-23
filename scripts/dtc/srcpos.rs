// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright 2007 Jon Loeliger, Freescale Semiconductor, Inc. */

use crate::dtc_header::SourcePos;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Read};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

impl SourcePos {
    pub(crate) fn through(&self, last: &Self) -> Self {
        Self {
            last_line: last.last_line,
            last_column: last.last_column,
            ..self.clone()
        }
    }

    pub(crate) fn render(&self) -> Vec<u8> {
        let mut result = if self.file.is_empty() {
            b"<no-file>".to_vec()
        } else {
            self.file.clone()
        };
        let suffix = if self.first_line != self.last_line {
            format!(
                ":{}.{}-{}.{}",
                self.first_line, self.first_column, self.last_line, self.last_column
            )
        } else if self.first_column != self.last_column {
            format!(
                ":{}.{}-{}",
                self.first_line, self.first_column, self.last_column
            )
        } else {
            format!(":{}.{}", self.first_line, self.first_column)
        };
        result.extend_from_slice(suffix.as_bytes());
        result
    }
}

pub(crate) fn comment(
    positions: &[SourcePos],
    initial_path: &[u8],
    first: bool,
    level: u32,
) -> Option<Vec<u8>> {
    if positions.is_empty() {
        return (level > 1).then(|| b"<no-file>:<no-line>".to_vec());
    }
    let mut result = Vec::new();
    for pos in positions {
        if !result.is_empty() {
            result.extend_from_slice(b", ");
        }
        let common = pos
            .file
            .iter()
            .zip(initial_path)
            .take_while(|(a, b)| a == b)
            .count();
        let slash = pos.file[..common].iter().rposition(|&x| x == b'/');
        if level <= 1 && slash.is_some() {
            let slash = slash.unwrap();
            let parents = initial_path[slash + 1..]
                .iter()
                .filter(|&&x| x == b'/')
                .count();
            for _ in 0..parents {
                result.extend_from_slice(b"../");
            }
            result.extend_from_slice(&pos.file[slash + 1..]);
        } else {
            result.extend_from_slice(&pos.file);
        }
        let suffix = if level > 1 {
            format!(
                ":{}:{}-{}:{}",
                pos.first_line, pos.first_column, pos.last_line, pos.last_column
            )
        } else {
            format!(":{}", if first { pos.first_line } else { pos.last_line })
        };
        result.extend_from_slice(suffix.as_bytes());
    }
    Some(result)
}

pub(crate) fn join_path(directory: &[u8], name: &[u8]) -> Vec<u8> {
    let mut result = directory.to_vec();
    if result.last() != Some(&b'/') {
        result.push(b'/');
    }
    result.extend_from_slice(name);
    result
}

pub(crate) fn errno_text(error: &io::Error) -> String {
    let message = error.to_string();
    message
        .split(" (os error ")
        .next()
        .unwrap_or(&message)
        .to_owned()
}

pub(crate) struct Input {
    pub(crate) id: usize,
    pub(crate) bytes: Vec<u8>,
    pub(crate) offset: usize,
    pub(crate) name: Vec<u8>,
    pub(crate) directory: Option<Vec<u8>>,
    pub(crate) line: i32,
    pub(crate) column: i32,
}

pub(crate) struct Sources {
    pub(crate) stack: Vec<Input>,
    pub(crate) paths: Vec<Vec<u8>>,
    pub(crate) dependencies: Vec<Vec<u8>>,
    pub(crate) initial_path: Vec<u8>,
    pub(crate) initial_cpp: bool,
    names: Vec<Vec<u8>>,
    pushes: usize,
}

impl Sources {
    pub(crate) fn new(paths: Vec<Vec<u8>>) -> Self {
        Self {
            stack: Vec::new(),
            paths,
            dependencies: Vec::new(),
            initial_path: Vec::new(),
            initial_cpp: true,
            names: Vec::new(),
            pushes: 0,
        }
    }

    pub(crate) fn open(&mut self, name: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Vec<u8>> {
        let (name, mut stream) = self.open_stream(name)?;
        let mut bytes = Vec::new();
        stream.read_to_end(&mut bytes).map_err(|e| {
            format!("Error reading file into data: {}", errno_text(&e)).into_bytes()
        })?;
        Ok((name, bytes))
    }

    pub(crate) fn open_stream(&mut self, name: &[u8]) -> Result<(Vec<u8>, Box<dyn Read>), Vec<u8>> {
        if name == b"-" {
            self.dependencies.push(b"<stdin>".to_vec());
            return Ok((b"<stdin>".to_vec(), Box::new(io::stdin())));
        }
        let current = self.stack.last().and_then(|input| input.directory.clone());
        let candidates = std::iter::once(current).chain(self.paths.iter().cloned().map(Some));
        let mut error = io::Error::from_raw_os_error(2);
        for directory in candidates {
            let full = match directory {
                Some(directory) if !name.starts_with(b"/") => join_path(&directory, name),
                _ => name.to_vec(),
            };
            let file = match File::open(Path::new(OsStr::from_bytes(&full))) {
                Ok(file) => file,
                Err(e) => {
                    error = e;
                    continue;
                }
            };
            self.dependencies.push(full.clone());
            return Ok((full, Box::new(file)));
        }
        let mut message = b"Couldn't open \"".to_vec();
        message.extend_from_slice(name);
        message.extend_from_slice(format!("\": {}\n", errno_text(&error)).as_bytes());
        Err(message)
    }

    pub(crate) fn push(&mut self, name: &[u8]) -> Result<(), Vec<u8>> {
        // The C implementation counts all pushes, not only active nesting.
        self.pushes += 1;
        if self.pushes > 200 {
            return Err(b"Includes nested too deeply".to_vec());
        }
        let (name, bytes) = self.open(name)?;
        if self.pushes == 1 {
            self.initial_path = name.clone();
        }
        let directory = name
            .iter()
            .rposition(|&x| x == b'/')
            .map(|i| name[..i].to_vec());
        let id = self.names.len();
        self.names.push(name.clone());
        self.stack.push(Input {
            id,
            bytes,
            offset: 0,
            name,
            directory,
            line: 1,
            column: 1,
        });
        Ok(())
    }

    pub(crate) fn set_line(&mut self, name: Vec<u8>, line: i32) {
        if self.initial_cpp {
            self.initial_path = name.clone();
            self.initial_cpp = false;
        }
        if let Some(input) = self.stack.last_mut() {
            self.names[input.id] = name.clone();
            input.name = name;
            input.line = line;
        }
    }

    pub(crate) fn consume(&mut self, len: usize) -> (Vec<u8>, SourcePos) {
        let input = self.stack.last_mut().expect("active input");
        let mut pos = SourcePos {
            file: input.name.clone(),
            file_id: Some(input.id),
            first_line: input.line,
            first_column: input.column,
            ..SourcePos::default()
        };
        let bytes = input.bytes[input.offset..input.offset + len].to_vec();
        input.offset += len;
        for &byte in &bytes {
            if byte == b'\n' {
                input.line += 1;
                input.column = 1;
            } else {
                input.column += 1;
            }
        }
        pos.last_line = input.line;
        pos.last_column = input.column;
        (bytes, pos)
    }

    pub(crate) fn snapshot(&self, pos: &SourcePos) -> SourcePos {
        let mut snapshot = pos.clone();
        if let Some(id) = snapshot.file_id.take() {
            snapshot.file = self.names[id].clone();
        }
        snapshot
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
