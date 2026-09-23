// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2002 Roman Zippel <zippel@linux-m68k.org>
//! Stateful Kconfig lexer, including source files and indentation-based help.

use crate::model::Location;
use crate::preprocess::Preprocessor;
use std::collections::HashMap;
use std::io::Write;

const KEYWORDS: &[&str] = &[
    "bool",
    "choice",
    "comment",
    "config",
    "def_bool",
    "def_tristate",
    "default",
    "depends",
    "endchoice",
    "endif",
    "endmenu",
    "help",
    "hex",
    "if",
    "imply",
    "int",
    "mainmenu",
    "menu",
    "menuconfig",
    "modules",
    "on",
    "prompt",
    "range",
    "select",
    "source",
    "string",
    "transitional",
    "tristate",
    "visible",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind {
    Word,
    Quoted,
    Keyword,
    Operator,
    Assignment,
}
#[derive(Clone, Debug)]
pub(super) struct Token {
    pub(super) kind: Kind,
    pub(super) text: String,
}
pub(super) struct Statement {
    pub(super) tokens: Vec<Token>,
    pub(super) location: Location,
    pub(super) end_line: usize,
}
struct File {
    name: String,
    contents: String,
    position: usize,
    line: usize,
    included_at: Option<Location>,
}
pub(super) struct Lexer {
    stack: Vec<File>,
    seen: HashMap<String, Location>,
    pub(super) files: Vec<String>,
    pub(super) preprocessor: Preprocessor,
    pub(super) warnings: usize,
    last_location: Location,
}

fn read(name: &str) -> Option<String> {
    std::fs::read_to_string(name).ok().or_else(|| {
        if std::path::Path::new(name).is_absolute() {
            return None;
        }
        std::env::var_os("srctree")
            .and_then(|root| std::fs::read_to_string(std::path::Path::new(&root).join(name)).ok())
    })
}

impl Lexer {
    pub(super) fn new(name: &str) -> Result<Self, String> {
        let contents = read(name).ok_or_else(|| format!("can't find file {name}"))?;
        let mut seen = HashMap::new();
        seen.insert(
            name.to_owned(),
            Location {
                filename: "(null)".into(),
                line: 0,
            },
        );
        Ok(Self {
            stack: vec![File {
                name: name.into(),
                contents,
                position: 0,
                line: 1,
                included_at: None,
            }],
            seen,
            files: vec![name.into()],
            preprocessor: Preprocessor::default(),
            warnings: 0,
            last_location: Location {
                filename: name.into(),
                line: 1,
            },
        })
    }
    pub(super) fn include(&mut self, name: &str, location: &Location) -> Result<(), String> {
        let contents =
            read(name).ok_or_else(|| format!("{location}: can't open file \"{name}\""))?;
        if self.stack.iter().any(|file| file.name == name) {
            let mut message = format!("Recursive inclusion detected.\nInclusion path:\n  current file : {name}\n  included from: {location}");
            for file in self.stack.iter().rev() {
                if let Some(parent) = &file.included_at {
                    message.push_str(&format!("\n  included from: {parent}"));
                }
            }
            return Err(message);
        }
        if let Some(first) = self.seen.get(name) {
            return Err(format!("{location}: error: repeated inclusion of {name}\n{first}: note: location of first inclusion of {name}"));
        }
        self.seen.insert(name.into(), location.clone());
        self.files.push(name.into());
        self.stack.push(File {
            name: name.into(),
            contents,
            position: 0,
            line: 1,
            included_at: Some(location.clone()),
        });
        Ok(())
    }
    pub(super) fn next(&mut self) -> Result<Option<Statement>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut location = None;
        loop {
            let Some(file) = self.stack.last_mut() else {
                return Ok(None);
            };
            if file.position == file.contents.len() {
                if !tokens.is_empty() {
                    eprintln!(
                        "{}:{}:warning: no new line at end of file",
                        file.name, file.line
                    );
                    self.warnings += 1;
                    return Ok(Some(Statement {
                        tokens,
                        location: location.unwrap(),
                        end_line: file.line,
                    }));
                }
                self.last_location = Location {
                    filename: file.name.clone(),
                    line: file.line,
                };
                self.stack.pop();
                continue;
            }
            let byte = file.contents.as_bytes()[file.position];
            if matches!(byte, b' ' | b'\t') {
                file.position += 1;
                continue;
            }
            if byte == b'#' {
                let size = file.contents[file.position..]
                    .find('\n')
                    .unwrap_or(file.contents.len() - file.position);
                file.position += size;
                continue;
            }
            if byte == b'\\' && file.contents.as_bytes().get(file.position + 1) == Some(&b'\n') {
                file.position += 2;
                file.line += 1;
                continue;
            }
            if byte == b'\n' {
                file.position += 1;
                file.line += 1;
                if tokens.is_empty() {
                    continue;
                }
                return Ok(Some(Statement {
                    tokens,
                    location: location.unwrap(),
                    end_line: file.line,
                }));
            }
            let token_line = file.line;
            let start = file.position;
            let token = if matches!(byte, b'\'' | b'"') {
                let quote = byte;
                file.position += 1;
                let mut text = String::new();
                loop {
                    if file.position == file.contents.len() {
                        break;
                    }
                    let current = file.contents.as_bytes()[file.position];
                    if current == quote {
                        file.position += 1;
                        break;
                    }
                    if current == b'\n' {
                        eprintln!(
                            "{}:{}:warning: multi-line strings not supported",
                            file.name,
                            location.as_ref().map_or(token_line, |l: &Location| l.line)
                        );
                        self.warnings += 1;
                        break;
                    }
                    if current == b'$' {
                        let end = file.contents[file.position..]
                            .find('\n')
                            .map_or(file.contents.len(), |n| file.position + n);
                        let (expanded, count) = self.preprocessor.expand_dollar(
                            &file.contents[file.position + 1..end],
                            &file.name,
                            file.line,
                        )?;
                        text.push_str(&expanded);
                        file.position += count + 1;
                    } else if current == b'\\' {
                        file.position += 1;
                        if file.position < file.contents.len()
                            && file.contents.as_bytes()[file.position] != b'\n'
                        {
                            let next = file.contents[file.position..].chars().next().unwrap();
                            text.push(next);
                            file.position += next.len_utf8();
                        }
                    } else {
                        let current = file.contents[file.position..].chars().next().unwrap();
                        text.push(current);
                        file.position += current.len_utf8();
                    }
                }
                Token {
                    kind: Kind::Quoted,
                    text,
                }
            } else if byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'$') {
                let mut end = start;
                while let Some(&b) = file.contents.as_bytes().get(end) {
                    if !b.is_ascii_alphanumeric() && !matches!(b, b'_' | b'-' | b'$') {
                        break;
                    }
                    end += 1;
                }
                let raw = &file.contents[start..end];
                if raw.contains('$') {
                    let line_end = file.contents[start..]
                        .find('\n')
                        .map_or(file.contents.len(), |n| start + n);
                    let (text, count) = self.preprocessor.expand_token(
                        &file.contents[start..line_end],
                        &file.name,
                        file.line,
                    )?;
                    file.position += count;
                    if text.is_empty() {
                        continue;
                    }
                    Token {
                        kind: Kind::Word,
                        text,
                    }
                } else {
                    file.position = end;
                    Token {
                        kind: if KEYWORDS.contains(&raw) {
                            Kind::Keyword
                        } else {
                            Kind::Word
                        },
                        text: raw.into(),
                    }
                }
            } else {
                let rest = &file.contents[start..];
                let operator = [
                    "||", "&&", "!=", "<=", ">=", ":=", "+=", "=", "<", ">", "!", "(", ")",
                ]
                .into_iter()
                .find(|op| rest.starts_with(op));
                if let Some(operator) = operator {
                    file.position += operator.len();
                    Token {
                        kind: Kind::Operator,
                        text: operator.into(),
                    }
                } else {
                    let mut warning = format!(
                        "{}:{}:warning: ignoring unsupported character '",
                        file.name, file.line
                    )
                    .into_bytes();
                    let current = file.contents[start..].chars().next().unwrap();
                    warning.extend_from_slice(current.to_string().as_bytes());
                    warning.extend_from_slice(b"'\n");
                    let _ = std::io::stderr().lock().write_all(&warning);
                    self.warnings += 1;
                    file.position += current.len_utf8();
                    continue;
                }
            };
            if location.is_none() {
                location = Some(Location {
                    filename: file.name.clone(),
                    line: token_line,
                });
            }
            tokens.push(token);
            if tokens.len() == 2
                && tokens[0].kind == Kind::Word
                && matches!(tokens[1].text.as_str(), "=" | ":=" | "+=")
            {
                let end = file.contents[file.position..]
                    .find('\n')
                    .map_or(file.contents.len(), |n| file.position + n);
                let value = file.contents[file.position..end]
                    .trim_start_matches([' ', '\t'])
                    .to_owned();
                tokens.push(Token {
                    kind: Kind::Assignment,
                    text: value,
                });
                file.position = end;
            }
        }
    }
    pub(super) fn location(&self) -> Location {
        self.stack.last().map_or_else(
            || self.last_location.clone(),
            |file| Location {
                filename: file.name.clone(),
                line: file.line,
            },
        )
    }

    pub(super) fn help(&mut self) -> String {
        let Some(file) = self.stack.last_mut() else {
            return String::new();
        };
        let mut text = String::new();
        let mut first_indent = 0;
        let mut last_indent = 0;
        while file.position < file.contents.len() {
            let start = file.position;
            let mut end = start;
            let mut indent = 0;
            while let Some(&byte) = file.contents.as_bytes().get(end) {
                match byte {
                    b' ' => indent += 1,
                    b'\t' => indent = (indent & !7) + 8,
                    _ => break,
                }
                end += 1;
            }
            if file.contents.as_bytes().get(end) == Some(&b'\n') {
                file.position = end + 1;
                file.line += 1;
                if file
                    .contents
                    .as_bytes()
                    .get(file.position)
                    .is_some_and(|b| !matches!(b, b' ' | b'\t' | b'\n'))
                {
                    break;
                }
                text.push('\n');
                continue;
            }
            if end > start {
                file.position = end;
                last_indent = indent;
                if first_indent > 0 {
                    if indent < first_indent {
                        break;
                    }
                    text.extend(std::iter::repeat_n(' ', indent - first_indent));
                }
            }
            if file.position == file.contents.len() {
                break;
            }
            let end = file.contents[file.position..]
                .find('\n')
                .map_or(file.contents.len(), |n| file.position + n);
            text.push_str(file.contents[file.position..end].trim_end_matches([' ', '\t']));
            file.position = end;
            if first_indent == 0 {
                first_indent = last_indent;
            }
        }
        text
    }
}
