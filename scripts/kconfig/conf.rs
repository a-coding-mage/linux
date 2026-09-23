// SPDX-License-Identifier: GPL-2.0
//! Line-oriented Kconfig front end, implemented with owned Rust state.
// Original Kconfig implementation: Copyright (C) 2002 Roman Zippel.

mod confdata;
mod expr;
mod menu;
mod model;
mod parser;
mod preprocess;
mod symbol;

use std::env;
use std::io::{self, IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use expr::Tristate;
use model::{Kconfig, MenuId, MenuType, SymbolId, SymbolType, Value};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Mode {
    OldAsk,
    Sync,
    Old,
    AllNo,
    AllYes,
    AllMod,
    AllDef,
    Random,
    Default,
    SaveDefault,
    ListNew,
    HelpNew,
    OldDefault,
    YesToMod,
    ModToYes,
    ModToNo,
}

impl Mode {
    fn parse(name: &str) -> Option<Self> {
        Some(match name {
            "oldaskconfig" => Self::OldAsk,
            "syncconfig" => Self::Sync,
            "oldconfig" => Self::Old,
            "allnoconfig" => Self::AllNo,
            "allyesconfig" => Self::AllYes,
            "allmodconfig" => Self::AllMod,
            "alldefconfig" => Self::AllDef,
            "randconfig" => Self::Random,
            "defconfig" => Self::Default,
            "savedefconfig" => Self::SaveDefault,
            "listnewconfig" => Self::ListNew,
            "helpnewconfig" => Self::HelpNew,
            "olddefconfig" => Self::OldDefault,
            "yes2modconfig" => Self::YesToMod,
            "mod2yesconfig" => Self::ModToYes,
            "mod2noconfig" => Self::ModToNo,
            _ => return None,
        })
    }
}

fn usage(program: &str) {
    println!("Usage: {program} [options] kconfig_file\n\nGeneric options:\n  -h, --help              Print this message and exit.\n  -s, --silent            Do not print log.\n\nMode options:\n  --listnewconfig         List new options\n  --helpnewconfig         List new options and help text\n  --oldaskconfig          Start a new configuration using a line-oriented program\n  --oldconfig             Update a configuration using a provided .config as base\n  --syncconfig            Similar to oldconfig but generates configuration in\n                          include/{{generated/,config/}}\n  --olddefconfig          Same as oldconfig but sets new symbols to their default value\n  --defconfig <file>      New config with default defined in <file>\n  --savedefconfig <file>  Save the minimal current configuration to <file>\n  --allnoconfig           New config where all options are answered with no\n  --allyesconfig          New config where all options are answered with yes\n  --allmodconfig          New config where all options are answered with mod\n  --alldefconfig          New config with all symbols set to default\n  --randconfig            New config with random answer to all options\n  --yes2modconfig         Change answers from yes to mod if possible\n  --mod2yesconfig         Change answers from mod to yes if possible\n  --mod2noconfig          Change answers from mod to no if possible\n  (If none of the above is given, --oldaskconfig is the default)\n\nArguments:\n  kconfig_file            Top-level Kconfig file.");
}

// The additive-feedback generator used by the Linux host's srand()/rand().
// Keeping its sequence makes a recorded KCONFIG_SEED reproducible after migration.
struct Random {
    state: [u32; 31],
    front: usize,
    rear: usize,
}

impl Random {
    fn new(seed: u32) -> Self {
        let mut state = [0; 31];
        state[0] = if seed == 0 { 1 } else { seed };
        let mut word = state[0] as i32 as i64;
        for value in state.iter_mut().skip(1) {
            word = 16_807 * (word % 127_773) - 2_836 * (word / 127_773);
            if word < 0 {
                word += 2_147_483_647;
            }
            *value = word as u32;
        }
        let mut random = Self {
            state,
            front: 3,
            rear: 0,
        };
        for _ in 0..310 {
            random.next();
        }
        random
    }

    fn next(&mut self) -> u32 {
        let value = self.state[self.front].wrapping_add(self.state[self.rear]);
        self.state[self.front] = value;
        self.front = (self.front + 1) % 31;
        self.rear = (self.rear + 1) % 31;
        value >> 1
    }

    fn from_environment() -> Self {
        let seed = env::var("KCONFIG_SEED")
            .ok()
            .and_then(|text| {
                let text = text.trim_start();
                let negative = text.starts_with('-');
                let text = text.strip_prefix(['-', '+']).unwrap_or(text);
                let (radix, digits) = if let Some(digits) =
                    text.strip_prefix("0x").or_else(|| text.strip_prefix("0X"))
                {
                    (16, digits)
                } else if text.starts_with('0') {
                    (8, text)
                } else {
                    (10, text)
                };
                u64::from_str_radix(digits, radix).ok().map(|seed| {
                    if negative {
                        (seed as u32).wrapping_neg()
                    } else {
                        seed as u32
                    }
                })
            })
            .unwrap_or_else(|| {
                let time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default();
                (time.as_secs() as u32)
                    .wrapping_add(1)
                    .wrapping_mul(time.subsec_micros() + 1)
            });
        println!("KCONFIG_SEED=0x{seed:X}");
        Self::new(seed)
    }
}

fn probabilities() -> Result<(u32, u32, u32), String> {
    let text = env::var("KCONFIG_PROBABILITY").unwrap_or_default();
    if text.is_empty() {
        return Ok((50, 33, 33));
    }
    let mut values = Vec::new();
    for text in text.split(':').take(3) {
        if text.is_empty() {
            break;
        }
        let value = text
            .trim_start()
            .parse::<u32>()
            .map_err(|_| "KCONFIG_PROBABILITY: Numerical result out of range".to_owned())?;
        if value > 100 {
            return Err("KCONFIG_PROBABILITY: Numerical result out of range".into());
        }
        values.push(value);
    }
    let result = match values.as_slice() {
        [value] => (*value, value - value / 2, value / 2),
        [yes, module] => (yes + module, *yes, *module),
        [boolean, yes, module] => (*boolean, *yes, *module),
        _ => (50, 33, 33),
    };
    if result.1 + result.2 > 100 {
        return Err("KCONFIG_PROBABILITY: Numerical result out of range".into());
    }
    Ok(result)
}

fn set_all(kconf: &mut Kconfig, mode: Mode, random: &mut Random) -> Result<(), String> {
    let (boolean, yes, module) = if mode == Mode::Random {
        probabilities()?
    } else {
        (50, 33, 33)
    };
    for menu in kconf.menu_depth_first(0) {
        let Some(id) = kconf.menus[menu].symbol else {
            continue;
        };
        let symbol = &kconf.symbols[id];
        if kconf.menus[menu].prompt.is_none()
            || symbol.user.is_some()
            || !matches!(symbol.kind, SymbolType::Boolean | SymbolType::Tristate)
            || symbol.choice.is_some()
        {
            continue;
        }
        if symbol.choice_menu.is_some() {
            if mode == Mode::Random {
                let mut members: Vec<_> = kconf
                    .menu_depth_first(menu)
                    .into_iter()
                    .skip(1)
                    .filter_map(|entry| kconf.menus[entry].symbol)
                    .filter(|&member| kconf.symbols[member].user.is_none())
                    .collect();
                while !members.is_empty() {
                    let index = random.next() as usize % members.len();
                    let member = members.remove(index);
                    kconf.symbols[member].user = Some(Value::tristate(Tristate::Yes));
                    kconf.menus[menu].members.retain(|&id| id != member);
                    kconf.menus[menu].members.push(member);
                }
            }
            continue;
        }
        let tri = match mode {
            Mode::AllYes => Tristate::Yes,
            Mode::AllMod => Tristate::Mod,
            Mode::AllNo => Tristate::No,
            Mode::Random => {
                let value = random.next() % 100;
                if symbol.kind == SymbolType::Tristate {
                    if value < yes {
                        Tristate::Yes
                    } else if value < yes + module {
                        Tristate::Mod
                    } else {
                        Tristate::No
                    }
                } else if value < boolean {
                    Tristate::Yes
                } else {
                    Tristate::No
                }
            }
            _ => continue,
        };
        kconf.symbols[id].user = Some(Value::tristate(tri));
    }
    kconf.invalidate();
    Ok(())
}

struct Frontend {
    mode: Mode,
    indent: usize,
    root: MenuId,
    count: usize,
    tty: bool,
}

impl Frontend {
    fn help(&self, kconf: &mut Kconfig, menu: MenuId) {
        println!("\n{}", kconf.extended_help(menu));
    }

    fn input(&self) -> io::Result<(String, bool)> {
        io::stdout().flush()?;
        let mut line = String::new();
        let eof = io::stdin().read_line(&mut line)? == 0;
        if eof {
            eprintln!("\nError in reading or end of file.");
            line.push('\n');
        }
        if !self.tty {
            print!("{line}");
        }
        Ok((line, eof))
    }

    fn ask(&self, kconf: &Kconfig, id: SymbolId) -> io::Result<Option<(String, bool)>> {
        let symbol = &kconf.symbols[id];
        if symbol.user.is_none() {
            print!("(NEW) ");
        }
        if symbol.visible <= symbol.selected_value
            || (matches!(self.mode, Mode::Old | Mode::Sync) && symbol.user.is_some())
        {
            println!("{}", symbol.current.text);
            return Ok(None);
        }
        self.input().map(Some)
    }

    fn string(&self, kconf: &mut Kconfig, menu: MenuId, id: SymbolId) -> Result<(), String> {
        loop {
            kconf.calculate(id);
            let default = kconf.symbols[id].current.text.clone();
            print!(
                "{}{} ({}) [{}] ",
                " ".repeat(self.indent - 1),
                kconf.prompt(menu).unwrap_or(""),
                kconf.symbols[id].display_name(),
                default
            );
            let Some((line, eof)) = self.ask(kconf, id).map_err(|e| e.to_string())? else {
                return Ok(());
            };
            let line = line.strip_suffix('\n').unwrap_or(&line);
            if line == "?" {
                self.help(kconf, menu);
            } else if kconf.set_string(id, if line.is_empty() { &default } else { line }) {
                return Ok(());
            }
            if eof {
                return Err(format!(
                    "\nerror: no value for new symbol '{}' at end of input",
                    kconf.symbols[id].display_name()
                ));
            }
        }
    }

    fn boolean(&self, kconf: &mut Kconfig, menu: MenuId, id: SymbolId) -> Result<(), String> {
        loop {
            kconf.calculate(id);
            let old = kconf.symbols[id].current.tri;
            print!(
                "{}{} ",
                " ".repeat(self.indent - 1),
                kconf.prompt(menu).unwrap_or("")
            );
            if let Some(name) = &kconf.symbols[id].name {
                print!("({name}) ");
            }
            print!("[{}", old.to_string().to_ascii_uppercase());
            for value in [Tristate::No, Tristate::Mod, Tristate::Yes] {
                if value != old && kconf.within_range(id, value) {
                    print!("/{value}");
                }
            }
            print!("/?] ");
            let Some((line, _)) = self.ask(kconf, id).map_err(|e| e.to_string())? else {
                return Ok(());
            };
            let line = line.trim();
            let value = match line {
                "n" | "N" | "no" | "No" => Tristate::No,
                "m" | "M" => Tristate::Mod,
                "y" | "Y" | "yes" | "Yes" => Tristate::Yes,
                "" => old,
                _ if line.starts_with('?') => {
                    self.help(kconf, menu);
                    continue;
                }
                _ => continue,
            };
            if kconf.set_tristate(id, value) {
                return Ok(());
            }
            self.help(kconf, menu);
        }
    }

    fn choice(&self, kconf: &mut Kconfig, menu: MenuId) -> Result<(), String> {
        let mut is_new = false;
        loop {
            println!(
                "{}{}",
                " ".repeat(self.indent - 1),
                kconf.prompt(menu).unwrap_or("")
            );
            let default = kconf.calculate_choice(menu);
            let mut children = Vec::new();
            let mut default_index = 0;
            for child in kconf.menus[menu].children.clone() {
                if !kconf.menu_visible(child) {
                    continue;
                }
                let Some(id) = kconf.menus[child].symbol else {
                    println!(
                        "{:>width$} {}",
                        '*',
                        kconf.prompt(child).unwrap_or(""),
                        width = self.indent
                    );
                    continue;
                };
                children.push((child, id));
                let marker = if Some(id) == default {
                    default_index = children.len();
                    '>'
                } else {
                    ' '
                };
                print!(
                    "{:>width$} {}. {} ({})",
                    marker,
                    children.len(),
                    kconf.prompt(child).unwrap_or(""),
                    kconf.symbols[id].display_name(),
                    width = self.indent
                );
                if kconf.symbols[id].user.is_none() {
                    is_new = true;
                    print!(" (NEW)");
                }
                println!();
            }
            if children.is_empty() {
                return Ok(());
            }
            print!("{}choice", " ".repeat(self.indent - 1));
            let (index, help) = if children.len() == 1 {
                println!("[1]: 1");
                (1, false)
            } else {
                print!("[1-{}?]: ", children.len());
                if matches!(self.mode, Mode::Old | Mode::Sync) && !is_new {
                    println!("{default_index}");
                    (default_index, false)
                } else {
                    let (line, _) = self.input().map_err(|e| e.to_string())?;
                    let line = line.trim();
                    if line.starts_with('?') {
                        self.help(kconf, menu);
                        continue;
                    }
                    if line.is_empty() {
                        (default_index, false)
                    } else {
                        let number: String =
                            line.chars().take_while(|ch| ch.is_ascii_digit()).collect();
                        (number.parse::<usize>().unwrap_or(0), line.ends_with('?'))
                    }
                }
            };
            let Some(&(child, id)) = index.checked_sub(1).and_then(|index| children.get(index))
            else {
                continue;
            };
            if help {
                self.help(kconf, child);
                continue;
            }
            kconf.set_choice(menu, id);
            return Ok(());
        }
    }

    fn configure(&mut self, kconf: &mut Kconfig, menu: MenuId) -> Result<(), String> {
        if !kconf.menu_visible(menu) {
            return Ok(());
        }
        let symbol = kconf.menus[menu].symbol;
        if kconf.menus[menu].prompt.is_some() {
            if kconf.menus[menu].kind == MenuType::Menu
                && self.mode != Mode::OldAsk
                && self.root != menu
            {
                return self.check(kconf, menu);
            }
            if matches!(kconf.menus[menu].kind, MenuType::Menu | MenuType::Comment) {
                if let Some(prompt) = kconf.prompt(menu) {
                    println!(
                        "{:>width$}\n{:>width$} {}\n{:>width$}",
                        '*',
                        '*',
                        prompt,
                        '*',
                        width = self.indent
                    );
                }
            }
        }
        if let Some(id) = symbol {
            if kconf.symbols[id].choice_menu.is_some() {
                return self.choice(kconf, menu);
            }
            match kconf.symbols[id].kind {
                SymbolType::Int | SymbolType::Hex | SymbolType::String => {
                    self.string(kconf, menu, id)?
                }
                _ => self.boolean(kconf, menu, id)?,
            }
            self.indent += 2;
        }
        for child in kconf.menus[menu].children.clone() {
            self.configure(kconf, child)?;
        }
        if symbol.is_some() {
            self.indent -= 2;
        }
        Ok(())
    }

    fn check(&mut self, kconf: &mut Kconfig, menu: MenuId) -> Result<(), String> {
        if !kconf.menu_visible(menu) {
            return Ok(());
        }
        if let Some(id) = kconf.menus[menu].symbol {
            let symbol = &kconf.symbols[id];
            if symbol.choice_menu.is_none()
                && symbol.user.is_none()
                && symbol.visible > symbol.selected_value
            {
                match self.mode {
                    Mode::ListNew => print!("{}", confdata::symbol_line(kconf, id, true, false)),
                    Mode::HelpNew => {
                        println!("-----");
                        self.help(kconf, menu);
                        println!("-----");
                    }
                    _ => {
                        if self.count == 0 {
                            println!("*\n* Restart config...\n*");
                        }
                        self.count += 1;
                        let mut root = menu;
                        while root != 0 && kconf.menus[root].kind != MenuType::Menu {
                            root = kconf.menus[root].parent.unwrap_or(0);
                        }
                        self.root = root;
                        self.configure(kconf, root)?;
                    }
                }
            }
        }
        for child in kconf.menus[menu].children.clone() {
            self.check(kconf, child)?;
        }
        Ok(())
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args_os();
    let program = args
        .next()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let mut mode = Mode::OldAsk;
    let mut silent = false;
    let mut sync = false;
    let mut config = None;
    let mut kconfig = None;
    let mut random = Random::new(1);
    let mut options = true;
    while let Some(argument) = args.next() {
        let text = argument.to_string_lossy();
        if options && text == "--" {
            options = false;
            continue;
        }
        if options && matches!(text.as_ref(), "-h" | "--help") {
            usage(&program);
            return Err(String::new());
        }
        if options && matches!(text.as_ref(), "-s" | "--silent") {
            silent = true;
            continue;
        }
        if options && text.starts_with('-') {
            let option = text
                .strip_prefix("--")
                .ok_or_else(|| format!("{program}: invalid option: {text}"))?;
            let (name, value) = option
                .split_once('=')
                .map_or((option, None), |(name, value)| (name, Some(value)));
            mode = Mode::parse(name)
                .ok_or_else(|| format!("{program}: unrecognized option '--{name}'"))?;
            if matches!(mode, Mode::Default | Mode::SaveDefault) {
                config = Some(match value {
                    Some(value) => PathBuf::from(value),
                    None => PathBuf::from(args.next().ok_or_else(|| {
                        format!("{program}: option '--{name}' requires an argument")
                    })?),
                });
            } else if value.is_some() {
                return Err(format!(
                    "{program}: option '--{name}' doesn't allow an argument"
                ));
            }
            if mode == Mode::Sync {
                silent = true;
                sync = true;
            }
            if mode == Mode::Random {
                random = Random::from_environment();
            }
        } else if kconfig.is_none() {
            kconfig = Some(argument);
        }
    }
    let Some(kconfig) = kconfig else {
        eprintln!("{program}: Kconfig file missing");
        usage(&program);
        return Err(String::new());
    };
    let filename = kconfig
        .to_str()
        .ok_or("Kconfig filename is not valid UTF-8")?;
    let mut kconf = parser::parse(filename)?;
    kconf.finalize()?;
    match mode {
        Mode::Default => {
            let path = config.as_deref().expect("--defconfig requires a pathname");
            confdata::read(&mut kconf, Some(path), silent).map_err(|_| {
                format!(
                    "***\n*** Can't find default configuration \"{}\"!\n***",
                    path.display()
                )
            })?;
        }
        Mode::AllNo | Mode::AllYes | Mode::AllMod | Mode::AllDef | Mode::Random => {
            if let Some(name) = env::var_os("KCONFIG_ALLCONFIG") {
                if !name.is_empty() && name != "1" {
                    confdata::read_simple(&mut kconf, Some(Path::new(&name)), silent).map_err(
                        |_| {
                            format!(
                                "*** Can't read seed configuration \"{}\"!",
                                name.to_string_lossy()
                            )
                        },
                    )?;
                } else {
                    let name = match mode {
                        Mode::AllNo => "allno.config",
                        Mode::AllYes => "allyes.config",
                        Mode::AllMod => "allmod.config",
                        Mode::AllDef => "alldef.config",
                        _ => "allrandom.config",
                    };
                    if confdata::read_simple(&mut kconf, Some(Path::new(name)), silent).is_err()
                        && confdata::read_simple(&mut kconf, Some(Path::new("all.config")), silent)
                            .is_err()
                    {
                        return Err(format!("*** KCONFIG_ALLCONFIG set, but no \"{name}\" or \"all.config\" file found"));
                    }
                }
            }
        }
        _ => {
            if let Err(error) = confdata::read(&mut kconf, None, silent) {
                if error.kind() == io::ErrorKind::InvalidData {
                    return Err(format!(
                        "{}: cannot read configuration: {error}",
                        confdata::config_name().display()
                    ));
                }
            }
        }
    }
    if kconf.config_warnings != 0 && env::var_os("KCONFIG_WERROR").is_some() {
        return Err(String::new());
    }
    let no_write = sync && confdata::enabled("KCONFIG_NOSILENTUPDATE");
    if no_write && kconf.changed {
        return Err("\n*** The configuration requires explicit update.\n".into());
    }
    match mode {
        Mode::AllNo | Mode::AllYes | Mode::AllMod | Mode::AllDef | Mode::Random | Mode::Default => {
            set_all(&mut kconf, mode, &mut random)?
        }
        Mode::YesToMod | Mode::ModToYes | Mode::ModToNo => {
            let (old, new) = match mode {
                Mode::YesToMod => (Tristate::Yes, Tristate::Mod),
                Mode::ModToYes => (Tristate::Mod, Tristate::Yes),
                _ => (Tristate::Mod, Tristate::No),
            };
            for id in confdata::symbol_order(&kconf) {
                if kconf.effective_type(id) == SymbolType::Tristate {
                    if let Some(value) = &mut kconf.symbols[id].user {
                        if value.tri == old {
                            *value = Value::tristate(new);
                        }
                    }
                }
            }
            kconf.invalidate();
        }
        Mode::OldAsk | Mode::Old | Mode::Sync | Mode::ListNew | Mode::HelpNew => {
            let mut frontend = Frontend {
                mode,
                indent: 1,
                root: 0,
                count: 0,
                tty: io::stdin().is_terminal() && io::stdout().is_terminal(),
            };
            if mode == Mode::OldAsk {
                frontend.configure(&mut kconf, 0)?;
                frontend.mode = Mode::Old;
            }
            loop {
                frontend.count = 0;
                frontend.check(&mut kconf, 0)?;
                if frontend.count == 0 {
                    break;
                }
            }
        }
        _ => {}
    }
    if kconf.errors != 0
        || (kconf.dependency_warnings != 0 && env::var_os("KCONFIG_WERROR").is_some())
    {
        return Err(String::new());
    }
    if mode == Mode::SaveDefault {
        let path = config
            .as_deref()
            .expect("--savedefconfig requires a pathname");
        confdata::write_defconfig(&mut kconf, path)
            .map_err(|_| format!("n*** Error while saving defconfig to: {}\n", path.display()))?;
    } else if !matches!(mode, Mode::ListNew | Mode::HelpNew) {
        if !no_write {
            confdata::write(&mut kconf, silent)
                .map_err(|_| "\n*** Error during writing of the configuration.\n".to_owned())?;
        }
        if confdata::write_autoconf(&mut kconf, sync).is_err() {
            if sync {
                return Err("\n*** Error during sync of the configuration.\n".into());
            }
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            if !error.is_empty() {
                eprintln!("{error}");
            }
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Random;

    #[test]
    fn random_sequence_matches_linux_host() {
        let mut random = Random::new(1);
        assert_eq!(
            (0..5).map(|_| random.next()).collect::<Vec<_>>(),
            [
                1_804_289_383,
                846_930_886,
                1_681_692_777,
                1_714_636_915,
                1_957_747_793
            ]
        );
    }
}
