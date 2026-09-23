// SPDX-License-Identifier: GPL-2.0
//! Configuration input, atomic output, and Kbuild dependency generation.
// Original Kconfig implementation: Copyright (C) 2002 Roman Zippel.

use std::collections::HashSet;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};

use crate::expr::Tristate;
use crate::model::{Kconfig, MenuId, MenuType, SymbolId, SymbolType, Value};

pub(crate) fn enabled(name: &str) -> bool {
    env::var_os(name).is_some_and(|value| !value.is_empty())
}

pub(crate) fn config_name() -> PathBuf {
    env_path("KCONFIG_CONFIG", ".config")
}

fn env_path(name: &str, default: &str) -> PathBuf {
    env::var_os(name)
        .map(PathBuf::from)
        .unwrap_or_else(|| default.into())
}

fn prefix() -> String {
    env::var("CONFIG_").unwrap_or_else(|_| "CONFIG_".into())
}

fn source_read(path: &Path) -> io::Result<String> {
    fs::read_to_string(path).or_else(|error| {
        if path.is_absolute() || error.kind() != io::ErrorKind::NotFound {
            return Err(error);
        }
        match env::var_os("srctree") {
            Some(root) => fs::read_to_string(Path::new(&root).join(path)),
            None => Err(error),
        }
    })
}

pub(crate) fn message(silent: bool, message: impl std::fmt::Display) {
    if !silent {
        println!("#\n# {message}\n#");
    }
}

fn warning(kconf: &mut Kconfig, file: &Path, line: usize, message: impl std::fmt::Display) {
    eprintln!("{}:{line}:warning: {message}", file.display());
    kconf.config_warnings += 1;
}

pub(crate) fn string_valid(kind: SymbolType, text: &str) -> bool {
    match kind {
        SymbolType::String => true,
        SymbolType::Int => {
            let digits = text.strip_prefix('-').unwrap_or(text);
            !digits.is_empty()
                && (digits == "0" || !digits.starts_with('0'))
                && digits.bytes().all(|ch| ch.is_ascii_digit())
        }
        SymbolType::Hex => {
            let digits = text
                .strip_prefix("0x")
                .or_else(|| text.strip_prefix("0X"))
                .unwrap_or(text);
            !digits.is_empty() && digits.bytes().all(|ch| ch.is_ascii_hexdigit())
        }
        _ => false,
    }
}

fn unquote(text: &str) -> Option<String> {
    let mut result = String::new();
    let mut chars = text.strip_prefix('"')?.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '"' => return Some(result),
            '\\' => result.push(chars.next()?),
            _ => result.push(ch),
        }
    }
    None
}

fn parse_value(
    kind: SymbolType,
    text: &str,
    automatic: bool,
) -> Result<Option<Value>, &'static str> {
    match kind {
        SymbolType::Boolean | SymbolType::Tristate => {
            let tri = match text.as_bytes().first() {
                Some(b'n') => Tristate::No,
                Some(b'y') => Tristate::Yes,
                Some(b'm') if kind == SymbolType::Tristate => Tristate::Mod,
                _ => return Err("invalid value"),
            };
            Ok(Some(Value::tristate(tri)))
        }
        SymbolType::String | SymbolType::Int | SymbolType::Hex => {
            let value = if kind == SymbolType::String && !automatic {
                // As in conf_set_sym_val(), an unquoted string is ignored.
                if !text.starts_with('"') {
                    return Ok(None);
                }
                unquote(text).ok_or("invalid string found")?
            } else {
                text.to_owned()
            };
            if !string_valid(kind, &value) {
                return Err("invalid value");
            }
            Ok(Some(Value {
                text: value,
                tri: Tristate::No,
            }))
        }
        SymbolType::Unknown => Ok(None),
    }
}

fn entries<'a>(text: &'a str, prefix: &str) -> Vec<(usize, Result<(&'a str, &'a str), &'a str>)> {
    text.lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let line = line.split('\0').next().unwrap_or_default();
            let entry = if line.is_empty() {
                return None;
            } else if line.starts_with('#') {
                let name = line
                    .strip_prefix("# ")?
                    .strip_prefix(prefix)?
                    .strip_suffix(" is not set")?;
                if name.contains(' ') {
                    return None;
                }
                Ok((name, "n"))
            } else {
                line.strip_prefix(prefix)
                    .and_then(|s| s.split_once('='))
                    .ok_or(line)
            };
            Some((index + 1, entry))
        })
        .collect()
}

pub(crate) fn read_simple(
    kconf: &mut Kconfig,
    name: Option<&Path>,
    silent: bool,
) -> io::Result<()> {
    let mut path = name.map(Path::to_path_buf).unwrap_or_else(config_name);
    let text = match source_read(&path) {
        Ok(text) => text,
        Err(original_error)
            if name.is_none() && original_error.kind() == io::ErrorKind::NotFound =>
        {
            kconf.changed = true;
            let mut found = None;
            for candidate in env::var("KCONFIG_DEFCONFIG_LIST")
                .unwrap_or_default()
                .split_whitespace()
            {
                if let Ok(text) = source_read(Path::new(candidate)) {
                    path = candidate.into();
                    message(silent, format_args!("using defaults found in {candidate}"));
                    found = Some(text);
                    break;
                }
            }
            found.ok_or(original_error)?
        }
        Err(error) => return Err(error),
    };
    for symbol in &mut kconf.symbols {
        symbol.user = None;
    }
    kconf.config_warnings = 0;
    for (line, entry) in entries(&text, &prefix()) {
        let (name, value) = match entry {
            Ok(entry) => entry,
            Err(data) => {
                warning(kconf, &path, line, format_args!("unexpected data: {data}"));
                continue;
            }
        };
        let Some(&id) = kconf.names.get(name) else {
            if env::var_os("KCONFIG_WARN_UNKNOWN_SYMBOLS").is_some() {
                warning(kconf, &path, line, format_args!("unknown symbol: {name}"));
            }
            kconf.changed = true;
            continue;
        };
        if kconf.symbols[id].user.is_some() {
            warning(
                kconf,
                &path,
                line,
                format_args!("override: reassigning to symbol {name}"),
            );
        }
        match parse_value(kconf.symbols[id].kind, value, false) {
            Ok(Some(value)) => kconf.set_user(id, value),
            Ok(None) => {}
            Err("invalid string found") => warning(kconf, &path, line, "invalid string found"),
            Err(_) => warning(
                kconf,
                &path,
                line,
                format_args!("symbol value '{value}' invalid for {name}"),
            ),
        }
    }
    let changed = kconf.changed;
    kconf.invalidate();
    kconf.changed = changed;
    Ok(())
}

pub(crate) fn read(kconf: &mut Kconfig, name: Option<&Path>, silent: bool) -> io::Result<()> {
    kconf.changed = false;
    let result = read_simple(kconf, name, silent);
    kconf.calculate(kconf.modules);
    result?;
    for id in symbol_order(kconf) {
        kconf.calculate(id);
        let sym = &kconf.symbols[id];
        if sym.choice_menu.is_some() {
            continue;
        }
        if let Some(user) = &sym.user {
            if sym.write && values_equal(sym.kind, user, &sym.current) {
                continue;
            }
        } else if !sym.write {
            continue;
        }
        kconf.changed = true;
    }
    if kconf.config_warnings > 0 {
        kconf.changed = true;
    }
    Ok(())
}

fn values_equal(kind: SymbolType, left: &Value, right: &Value) -> bool {
    if matches!(kind, SymbolType::Boolean | SymbolType::Tristate) {
        left.tri == right.tri
    } else {
        left.text == right.text
    }
}

// Preserve the C hash-table traversal order in generated headers and auto.conf.
pub(crate) fn symbol_order(kconf: &Kconfig) -> Vec<SymbolId> {
    let mut ids: Vec<_> = (3..kconf.symbols.len()).collect();
    ids.sort_by_key(|&id| {
        let hash = kconf.symbols[id].name.as_ref().map_or(0, |name| {
            name.bytes().fold(2_166_136_261u32, |hash, byte| {
                (hash ^ byte as u32).wrapping_mul(0x0100_0193)
            })
        });
        (hash % 16_384, std::cmp::Reverse(id))
    });
    ids
}

fn escape(text: &str) -> String {
    let mut escaped = String::from("\"");
    for ch in text.chars() {
        if ch == '"' || ch == '\\' {
            escaped.push('\\');
        }
        escaped.push(ch);
    }
    escaped.push('"');
    escaped
}

pub(crate) fn symbol_line(kconf: &Kconfig, id: SymbolId, list: bool, automatic: bool) -> String {
    let symbol = &kconf.symbols[id];
    let Some(name) = &symbol.name else {
        return String::new();
    };
    if symbol.kind == SymbolType::Unknown {
        return String::new();
    }
    let prefix = prefix();
    if matches!(symbol.kind, SymbolType::Boolean | SymbolType::Tristate)
        && symbol.current.tri == Tristate::No
        && !list
    {
        return if automatic {
            String::new()
        } else {
            format!("# {prefix}{name} is not set\n")
        };
    }
    let value = if symbol.kind == SymbolType::String && !automatic {
        escape(&symbol.current.text)
    } else {
        symbol.current.text.clone()
    };
    format!("{prefix}{name}={value}\n")
}

fn heading(kconf: &Kconfig, c_style: bool) -> String {
    let prompt = kconf.prompt(0).unwrap_or("Main menu");
    if c_style {
        format!("/*\n * Automatically generated file; DO NOT EDIT.\n * {prompt}\n */\n")
    } else {
        format!("#\n# Automatically generated file; DO NOT EDIT.\n# {prompt}\n#\n")
    }
}

fn warn_changed(kconf: &Kconfig, ids: &[SymbolId]) {
    if !enabled("KCONFIG_WARN_CHANGED_INPUT") {
        return;
    }
    let mut output = String::new();
    for &id in ids {
        let symbol = &kconf.symbols[id];
        if symbol.kind == SymbolType::Unknown {
            continue;
        }
        let Some(user) = &symbol.user else {
            continue;
        };
        if values_equal(symbol.kind, user, &symbol.current) {
            continue;
        }
        if output.is_empty() {
            output.push_str("warning: user-provided values changed by Kconfig:\n");
        }
        let _ = writeln!(
            output,
            "  {}{}: {} -> {}",
            prefix(),
            symbol.display_name(),
            user.text,
            symbol.current.text
        );
    }
    eprint!("{output}");
}

fn parent_dir(path: &Path) -> io::Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn suffixed(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(suffix);
    name.into()
}

fn atomic_write(path: &Path, text: &str) -> io::Result<()> {
    parent_dir(path)?;
    let temporary = suffixed(path, &format!(".{}.tmp", std::process::id()));
    let result = fs::File::create(&temporary)
        .inspect_err(|error| {
            report_io("fopen", error);
        })
        .and_then(|mut file| file.write_all(text.as_bytes()))
        .and_then(|()| {
            fs::rename(&temporary, path).inspect_err(|error| {
                report_io("rename", error);
            })
        });
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

fn report_io(operation: &str, error: &io::Error) {
    let message = error.to_string();
    let message = message.split(" (os error ").next().unwrap_or(&message);
    eprintln!("{operation}: {message}");
}

fn config_menu(
    kconf: &mut Kconfig,
    menu: MenuId,
    output: &mut String,
    seen: &mut HashSet<SymbolId>,
    written: &mut Vec<SymbolId>,
    newline: &mut bool,
) {
    if let Some(id) = kconf.menus[menu].symbol {
        if kconf.symbols[id].choice_menu.is_none() && seen.insert(id) {
            kconf.calculate(id);
            written.push(id);
            if kconf.symbols[id].write {
                if *newline {
                    output.push('\n');
                    *newline = false;
                }
                output.push_str(&symbol_line(kconf, id, false, false));
            }
        }
    } else if menu != 0 && kconf.menu_visible(menu) {
        if let Some(prompt) = kconf.prompt(menu) {
            let _ = write!(output, "\n#\n# {prompt}\n#\n");
            *newline = false;
        }
    }
    for child in kconf.menus[menu].children.clone() {
        config_menu(kconf, child, output, seen, written, newline);
    }
    if menu != 0
        && kconf.menus[menu].symbol.is_none()
        && kconf.menus[menu].kind == MenuType::Menu
        && kconf.menu_visible(menu)
    {
        if let Some(prompt) = kconf.prompt(menu) {
            let _ = writeln!(output, "# end of {prompt}");
            *newline = true;
        }
    }
}

pub(crate) fn write(kconf: &mut Kconfig, silent: bool) -> io::Result<()> {
    let name = config_name();
    if name.as_os_str().is_empty() {
        eprintln!("config name is empty");
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "config name is empty",
        ));
    }
    if name.is_dir() {
        eprintln!("{}: Is a directory", name.display());
        return Err(io::Error::new(
            io::ErrorKind::IsADirectory,
            format!("{}: Is a directory", name.display()),
        ));
    }
    if !kconf.changed {
        kconf.invalidate();
    }
    let mut output = heading(kconf, false);
    let mut written = Vec::new();
    config_menu(
        kconf,
        0,
        &mut output,
        &mut HashSet::new(),
        &mut written,
        &mut false,
    );
    warn_changed(kconf, &written);
    parent_dir(&name)?;
    if enabled("KCONFIG_OVERWRITECONFIG") {
        fs::write(&name, &output)?;
    } else {
        if fs::read(&name).is_ok_and(|old| old == output.as_bytes()) {
            message(silent, format_args!("No change to {}", name.display()));
            kconf.changed = false;
            return Ok(());
        }
        let temporary = suffixed(&name, &format!(".{}.tmp", std::process::id()));
        fs::write(&temporary, output)?;
        // Preserve the previous configuration before replacing the final pathname.
        if name.symlink_metadata().is_ok() {
            if let Err(error) = fs::rename(&name, suffixed(&name, ".old")) {
                let _ = fs::remove_file(&temporary);
                return Err(error);
            }
        }
        fs::rename(temporary, &name)?;
    }
    message(
        silent,
        format_args!("configuration written to {}", name.display()),
    );
    kconf.changed = false;
    Ok(())
}

pub(crate) fn write_defconfig(kconf: &mut Kconfig, name: &Path) -> io::Result<()> {
    kconf.invalidate();
    let mut seen = HashSet::new();
    let mut written = Vec::new();
    let mut output = String::new();
    for menu in kconf.menu_depth_first(0) {
        let Some(id) = kconf.menus[menu].symbol else {
            continue;
        };
        if kconf.symbols[id].choice_menu.is_some() || !seen.insert(id) {
            continue;
        }
        kconf.calculate(id);
        written.push(id);
        let symbol = &kconf.symbols[id];
        if !symbol.write || symbol.visible <= symbol.selected_value {
            continue;
        }
        let current = symbol.current.clone();
        let kind = symbol.kind;
        if values_equal(kind, &current, &kconf.default_value(id)) {
            continue;
        }
        if let Some(choice) = kconf.symbols[id].choice {
            if current.tri == Tristate::Yes && kconf.choice_default(choice) == Some(id) {
                continue;
            }
        }
        output.push_str(&symbol_line(kconf, id, false, false));
    }
    fs::write(name, output)?;
    warn_changed(kconf, &written);
    Ok(())
}

fn touch_dependencies(kconf: &mut Kconfig, autoconf: &Path) -> io::Result<()> {
    let parent = autoconf.parent().unwrap_or_else(|| Path::new(""));
    for sym in &mut kconf.symbols {
        sym.automatic = None;
    }
    let automatic_text = match source_read(autoconf) {
        Ok(text) => Some(text),
        Err(error) if error.kind() == io::ErrorKind::InvalidData => {
            eprintln!("{}: cannot read configuration: {error}", autoconf.display());
            return Err(error);
        }
        Err(_) => None,
    };
    if let Some(text) = automatic_text {
        kconf.config_warnings = 0;
        for (line, entry) in entries(&text, &prefix()) {
            let (name, text) = match entry {
                Ok(entry) => entry,
                Err(data) => {
                    warning(
                        kconf,
                        autoconf,
                        line,
                        format_args!("unexpected data: {data}"),
                    );
                    continue;
                }
            };
            if let Some(&id) = kconf.names.get(name) {
                if kconf.symbols[id].automatic.is_some() {
                    warning(
                        kconf,
                        autoconf,
                        line,
                        format_args!("override: reassigning to symbol {name}"),
                    );
                }
                if let Ok(Some(value)) = parse_value(kconf.symbols[id].kind, text, true) {
                    kconf.symbols[id].automatic = Some(value);
                }
            } else {
                touch_symbol(parent, name)?;
            }
        }
    }
    for id in symbol_order(kconf) {
        let symbol = &kconf.symbols[id];
        if symbol.choice_menu.is_some() {
            continue;
        }
        let changed = match (&symbol.automatic, symbol.write) {
            (Some(old), true) => !values_equal(symbol.kind, old, &symbol.current),
            (None, true) => {
                !matches!(symbol.kind, SymbolType::Boolean | SymbolType::Tristate)
                    || symbol.current.tri != Tristate::No
            }
            (Some(_), false) => true,
            (None, false) => false,
        };
        if changed {
            if let Some(name) = &symbol.name {
                touch_symbol(parent, name)?;
            }
        }
    }
    Ok(())
}

fn touch_symbol(parent: &Path, name: &str) -> io::Result<()> {
    // Only symbol basenames may be derived from an old, potentially edited config.
    if name.is_empty() || name == "." || name == ".." || name.contains('/') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid symbol name in auto.conf",
        ));
    }
    fs::write(parent.join(name), [])
}

pub(crate) fn write_autoconf(kconf: &mut Kconfig, overwrite: bool) -> io::Result<()> {
    let autoconf = env_path("KCONFIG_AUTOCONFIG", "include/config/auto.conf");
    if !overwrite && autoconf.exists() {
        return Ok(());
    }
    let mut deps = format!(
        "autoconfig := {}\n\ndeps_config := \\\n",
        autoconf.display()
    );
    for file in &kconf.files {
        let _ = writeln!(deps, "\t{file} \\");
    }
    deps.push_str("\n$(autoconfig): $(deps_config)\n$(deps_config): ;\n");
    for (name, value) in &kconf.environment {
        let _ = write!(
            deps,
            "\nifneq \"$({name})\" \"{value}\"\n$(autoconfig): FORCE\nendif\n"
        );
    }
    atomic_write(&suffixed(&autoconf, ".cmd"), &deps)?;
    for id in symbol_order(kconf) {
        kconf.calculate(id);
    }
    touch_dependencies(kconf, &autoconf)?;
    let mut config = heading(kconf, false);
    let mut header = heading(kconf, true);
    let mut rust = String::new();
    let prefix = prefix();
    for id in symbol_order(kconf) {
        let symbol = &kconf.symbols[id];
        let Some(name) = &symbol.name else {
            continue;
        };
        if !symbol.write || symbol.kind == SymbolType::Unknown {
            continue;
        }
        config.push_str(&symbol_line(kconf, id, false, true));
        let mut value = symbol.current.text.clone();
        let mut suffix = "";
        let mut c_value = value.clone();
        match symbol.kind {
            SymbolType::Boolean | SymbolType::Tristate => {
                if symbol.current.tri == Tristate::No {
                    continue;
                }
                if symbol.current.tri == Tristate::Mod {
                    suffix = "_MODULE";
                }
                c_value = "1".into();
                let _ = writeln!(rust, "--cfg={prefix}{name}");
            }
            SymbolType::Hex => {
                if !value.starts_with("0x") && !value.starts_with("0X") {
                    value.insert_str(0, "0x");
                }
                c_value = value.clone();
            }
            SymbolType::String => c_value = escape(&value),
            _ => {}
        }
        let _ = writeln!(header, "#define {prefix}{name}{suffix} {c_value}");
        let _ = writeln!(rust, "--cfg={prefix}{name}={}", escape(&value));
    }
    atomic_write(
        &env_path("KCONFIG_AUTOHEADER", "include/generated/autoconf.h"),
        &header,
    )?;
    atomic_write(
        &env_path("KCONFIG_RUSTCCFG", "include/generated/rustc_cfg"),
        &rust,
    )?;
    // auto.conf is the completion marker consumed by Kbuild: publish it last.
    atomic_write(&autoconf, &config)
}
