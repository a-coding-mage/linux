// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Masahiro Yamada <yamada.masahiro@socionext.com>
//! Kconfig variables, user functions, builtins, and environment dependencies.

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Flavor {
    Simple,
    Recursive,
    Append,
}

struct Variable {
    value: String,
    flavor: Flavor,
    expansions: usize,
}

#[derive(Default)]
pub(crate) struct Preprocessor {
    variables: HashMap<String, Variable>,
    /// Referenced environment variables, in first-use order, for auto.conf.cmd.
    pub(crate) environment: Vec<(String, String)>,
}

#[derive(Clone, Copy)]
struct Location<'a> {
    filename: &'a str,
    line: usize,
}

impl Location<'_> {
    fn error(self, message: impl std::fmt::Display) -> String {
        format!("{}:{}: {message}", self.filename, self.line)
    }
}

impl Preprocessor {
    pub(crate) fn assign(
        &mut self,
        name: &str,
        value: &str,
        mut flavor: Flavor,
        filename: &str,
        line: usize,
    ) -> Result<(), String> {
        let mut append = false;
        if flavor == Flavor::Append {
            flavor = if let Some(variable) = self.variables.get(name) {
                append = true;
                variable.flavor
            } else {
                Flavor::Recursive
            };
        }
        let value = if flavor == Flavor::Simple {
            self.expand(value, filename, line)?
        } else {
            c_string(value).to_owned()
        };
        if append {
            let variable = self.variables.get_mut(name).unwrap();
            variable.value.push(' ');
            variable.value.push_str(&value);
        } else {
            self.variables.insert(
                name.to_owned(),
                Variable {
                    value,
                    flavor,
                    expansions: 0,
                },
            );
        }
        Ok(())
    }

    pub(crate) fn expand(
        &mut self,
        input: &str,
        filename: &str,
        line: usize,
    ) -> Result<String, String> {
        self.expand_with_args(input, &[], Location { filename, line })
    }

    /// Consume an unquoted Kconfig token; the count is a byte offset in input.
    pub(crate) fn expand_token(
        &mut self,
        input: &str,
        filename: &str,
        line: usize,
    ) -> Result<(String, usize), String> {
        self.expand_until(input, true, &[], Location { filename, line })
    }

    /// Input starts immediately after '$'; only '$(...)' is special in Kconfig.
    pub(crate) fn expand_dollar(
        &mut self,
        input: &str,
        filename: &str,
        line: usize,
    ) -> Result<(String, usize), String> {
        self.dollar_with_args(input, &[], Location { filename, line })
    }

    fn expand_with_args(
        &mut self,
        input: &str,
        args: &[String],
        location: Location<'_>,
    ) -> Result<String, String> {
        self.expand_until(input, false, args, location)
            .map(|(text, _)| text)
    }

    fn expand_until(
        &mut self,
        input: &str,
        token: bool,
        args: &[String],
        location: Location<'_>,
    ) -> Result<(String, usize), String> {
        let input = c_string(input);
        let bytes = input.as_bytes();
        let mut result = String::new();
        let mut start = 0;
        let mut position = 0;
        while position < bytes.len() {
            if bytes[position] == b'$' {
                result.push_str(&input[start..position]);
                let (expansion, consumed) =
                    self.dollar_with_args(&input[position + 1..], args, location)?;
                result.push_str(&expansion);
                position += 1 + consumed;
                start = position;
            } else if token
                && !(bytes[position].is_ascii_alphanumeric()
                    || matches!(bytes[position], b'_' | b'-'))
            {
                break;
            } else {
                position += 1;
            }
        }
        result.push_str(&input[start..position]);
        Ok((result, position))
    }

    fn dollar_with_args(
        &mut self,
        input: &str,
        args: &[String],
        location: Location<'_>,
    ) -> Result<(String, usize), String> {
        let input = c_string(input);
        if !input.starts_with('(') {
            return Ok(("$".to_owned(), 0));
        }
        let mut depth = 0;
        for (position, &byte) in input.as_bytes().iter().enumerate().skip(1) {
            match byte {
                b'(' => depth += 1,
                b')' if depth == 0 => {
                    return self
                        .eval_clause(&input[1..position], args, location)
                        .map(|result| (result, position + 1));
                }
                b')' => depth -= 1,
                _ => {}
            }
        }
        Err(location.error(format!(
            "unterminated reference to '{}': missing ')'",
            &input[1..]
        )))
    }

    fn eval_clause(
        &mut self,
        input: &str,
        args: &[String],
        location: Location<'_>,
    ) -> Result<String, String> {
        if let Some(number) = argument_number(input) {
            if number > 0 && number <= args.len() {
                return Ok(args[number - 1].clone());
            }
        }
        let mut parts = Vec::new();
        let mut depth = 0;
        let mut start = 0;
        for (position, &byte) in input.as_bytes().iter().enumerate() {
            match byte {
                b',' if depth == 0 => {
                    if parts.len() >= 16 {
                        return Err(location.error("too many function arguments"));
                    }
                    parts.push(&input[start..position]);
                    start = position + 1;
                }
                b'(' => depth += 1,
                b')' => depth -= 1,
                _ => {}
            }
        }
        if parts.len() >= 16 {
            return Err(location.error("too many function arguments"));
        }
        parts.push(&input[start..]);

        let name = self.expand_with_args(parts[0], args, location)?;
        let mut arguments = Vec::with_capacity(parts.len() - 1);
        for part in &parts[1..] {
            arguments.push(self.expand_with_args(part, args, location)?);
        }
        if let Some(variable) = self.variables.get_mut(&name) {
            if arguments.is_empty() && variable.expansions != 0 {
                return Err(location.error(format!(
                    "Recursive variable '{name}' references itself (eventually)"
                )));
            }
            if variable.expansions > 1000 {
                return Err(location.error("Too deep recursive expansion"));
            }
            if variable.flavor == Flavor::Simple {
                return Ok(variable.value.clone());
            }
            variable.expansions += 1;
            let value = variable.value.clone();
            let result = self.expand_with_args(&value, &arguments, location);
            self.variables.get_mut(&name).unwrap().expansions -= 1;
            return result;
        }
        if let Some(result) = self.builtin(&name, &arguments, location) {
            return result;
        }
        if arguments.is_empty() && !name.is_empty() {
            if let Some((_, value)) = self.environment.iter().find(|(key, _)| key == &name) {
                return Ok(value.clone());
            }
            if let Some(value) = std::env::var_os(&name) {
                let value = value.to_string_lossy().into_owned();
                self.environment.push((name, value.clone()));
                return Ok(value);
            }
        }
        Ok(String::new())
    }

    fn builtin(
        &self,
        name: &str,
        args: &[String],
        location: Location<'_>,
    ) -> Option<Result<String, String>> {
        let count = match name {
            "error-if" | "warning-if" => 2,
            "filename" | "lineno" => 0,
            "info" | "shell" => 1,
            _ => return None,
        };
        if args.len() != count {
            let quantity = if args.len() < count { "few" } else { "many" };
            return Some(Err(location.error(format!(
                "too {quantity} function arguments passed to '{name}'"
            ))));
        }
        Some(match name {
            "error-if" if args[0] == "y" => Err(location.error(&args[1])),
            "warning-if" if args[0] == "y" => {
                let _ = writeln!(io::stderr().lock(), "{}", location.error(&args[1]));
                Ok(String::new())
            }
            "info" => {
                let _ = writeln!(io::stdout().lock(), "{}", args[0]);
                Ok(String::new())
            }
            "filename" => Ok(location.filename.to_owned()),
            "lineno" => Ok(location.line.to_string()),
            "shell" => shell(&args[0], location),
            _ => Ok(String::new()),
        })
    }
}

fn c_string(input: &str) -> &str {
    input.split('\0').next().unwrap_or("")
}

/// strtoul accepts leading whitespace and signs, but no trailing characters.
fn argument_number(input: &str) -> Option<usize> {
    let mut text = input.trim_start_matches(|c: char| c.is_ascii_whitespace() || c == '\u{b}');
    let negative = text.starts_with('-');
    if text.starts_with(['+', '-']) {
        text = &text[1..];
    }
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    match text.parse::<usize>() {
        Ok(number) => Some(if negative {
            number.wrapping_neg()
        } else {
            number
        }),
        Err(_) => Some(usize::MAX),
    }
}

fn shell(command: &str, location: Location<'_>) -> Result<String, String> {
    let mut child = Command::new("/bin/sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|error| location.error(format!("{command}: {error}")))?;
    let mut bytes = Vec::new();
    let result = child
        .stdout
        .take()
        .unwrap()
        .take(4096)
        .read_to_end(&mut bytes);
    let waited = child.wait();
    result.map_err(|error| location.error(format!("{command}: {error}")))?;
    waited.map_err(|error| location.error(format!("{command}: {error}")))?;
    bytes.truncate(4095);
    while bytes.last() == Some(&b'\n') {
        bytes.pop();
    }
    for byte in &mut bytes {
        if *byte == b'\n' {
            *byte = b' ';
        }
    }
    if let Some(end) = bytes.iter().position(|&b| b == 0) {
        bytes.truncate(end);
    }
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}
