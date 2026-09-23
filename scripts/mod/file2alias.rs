// SPDX-License-Identifier: GPL-2.0
//! Device-table aliases, decoded using the target compiler's generated offsets.
// Original implementation: Copyright 2002-2003 Rusty Russell, IBM Corporation;
// Copyright 2003 Kai Germaschewski.

use std::collections::HashMap;
use std::fmt::Write as _;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Alias {
    pub(crate) text: String,
    pub(crate) builtin_modname: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Severity {
    Warning,
    Error,
    Fatal,
}

#[derive(Debug)]
pub(crate) struct Diagnostic {
    pub(crate) severity: Severity,
    pub(crate) message: String,
}

impl Diagnostic {
    fn new(severity: Severity, message: impl Into<String>) -> Self {
        Self {
            severity,
            message: message.into(),
        }
    }
}

#[derive(Clone, Copy)]
struct Target {
    big_endian: bool,
    word_bits: u32,
}

#[derive(Default)]
pub(crate) struct Tables {
    offsets: HashMap<String, usize>,
}

impl Tables {
    pub(crate) fn parse(header: &str) -> Result<Self, String> {
        let mut offsets = HashMap::new();
        for line in header.lines() {
            let mut words = line.split_whitespace();
            if words.next() != Some("#define") {
                continue;
            }
            let Some(name) = words.next() else { continue };
            if !name.starts_with("SIZE_") && !name.starts_with("OFF_") {
                continue;
            }
            let value = words
                .next()
                .ok_or_else(|| format!("missing value for {name}"))?;
            let value = value
                .parse()
                .map_err(|_| format!("invalid offset for {name}: {value}"))?;
            if offsets.insert(name.into(), value).is_some() {
                return Err(format!("duplicate device-table offset {name}"));
            }
        }
        if offsets.is_empty() {
            return Err("empty device-table offsets".into());
        }
        Ok(Self { offsets })
    }

    fn offset(&self, key: &str) -> Result<usize, Diagnostic> {
        self.offsets.get(key).copied().ok_or_else(|| {
            Diagnostic::new(
                Severity::Fatal,
                if self.offsets.is_empty() {
                    "device-table layout metadata is unavailable; build modpost through configured Kbuild or set MODPOST_DEVICETABLE_OFFSETS\n".into()
                } else { format!("missing device-table offset {key}\n") },
            )
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn handle(
        &self,
        module: &str,
        is_vmlinux: bool,
        symbol: &str,
        bytes: &[u8],
        little_endian: bool,
        word_bits: u32,
        aliases: &mut Vec<Alias>,
    ) -> Vec<Diagnostic> {
        let target = Target {
            big_endian: !little_endian,
            word_bits,
        };
        let mut diagnostics = Vec::new();
        if !symbol.starts_with("__mod_device_table__") {
            return diagnostics;
        }
        let Some((_, suffix)) = symbol.split_once("__kmod_") else {
            return diagnostics;
        };
        let Some((builtin, suffix)) = suffix.split_once("__") else {
            return diagnostics;
        };
        let Some((bus, name)) = suffix.split_once("__") else {
            return diagnostics;
        };
        let Some(structure) = structure(bus) else {
            return diagnostics;
        };
        let size = match self.offset(&format!("SIZE_{structure}")) {
            Ok(size) => size,
            Err(error) => return vec![error],
        };
        if size == 0 || bytes.len() < size || bytes.len() % size != 0 {
            return vec![Diagnostic::new(
                Severity::Error,
                format!(
                "{module}: type mismatch between {name}[] and MODULE_DEVICE_TABLE({bus}, ...)\n"
            ),
            )];
        }
        if bytes[bytes.len() - size..].iter().any(|&byte| byte != 0) {
            return vec![Diagnostic::new(
                Severity::Error,
                format!("{module}: {name}[] is not terminated with a NULL entry\n"),
            )];
        }
        if !matches!(target.word_bits, 32 | 64) {
            return vec![Diagnostic::new(
                Severity::Fatal,
                "invalid device-table word size\n",
            )];
        }
        for data in bytes[..bytes.len() - size].chunks_exact(size) {
            let entry = Entry {
                tables: self,
                structure,
                data,
                target,
            };
            match generate(bus, module, &entry) {
                Ok(generated) => {
                    for text in generated {
                        if !aliases.iter().any(|alias| alias.text == text) {
                            aliases.push(Alias {
                                text,
                                builtin_modname: is_vmlinux.then(|| builtin.to_owned()),
                            });
                        }
                    }
                }
                Err(error) => {
                    let fatal = error.severity == Severity::Fatal;
                    diagnostics.push(error);
                    if fatal {
                        break;
                    }
                }
            }
        }
        diagnostics
    }
}

fn structure(bus: &str) -> Option<&'static str> {
    Some(match bus {
        "hid" => "hid_device_id",
        "ieee1394" => "ieee1394_device_id",
        "pci" => "pci_device_id",
        "ccw" => "ccw_device_id",
        "ap" => "ap_device_id",
        "css" => "css_device_id",
        "serio" => "serio_device_id",
        "acpi" => "acpi_device_id",
        "pcmcia" => "pcmcia_device_id",
        "vio" => "vio_device_id",
        "input" => "input_device_id",
        "eisa" => "eisa_device_id",
        "parisc" => "parisc_device_id",
        "sdio" => "sdio_device_id",
        "ssb" => "ssb_device_id",
        "bcma" => "bcma_device_id",
        "virtio" => "virtio_device_id",
        "vmbus" => "hv_vmbus_device_id",
        "rpmsg" => "rpmsg_device_id",
        "i2c" => "i2c_device_id",
        "i3c" => "i3c_device_id",
        "slim" => "slim_device_id",
        "spi" => "spi_device_id",
        "dmi" => "dmi_system_id",
        "platform" => "platform_device_id",
        "mdio" => "mdio_device_id",
        "zorro" => "zorro_device_id",
        "isapnp" => "isapnp_device_id",
        "ipack" => "ipack_device_id",
        "amba" => "amba_id",
        "mipscdmm" => "mips_cdmm_device_id",
        "x86cpu" => "x86_cpu_id",
        "cpu" => "cpu_feature",
        "mcb" => "mcb_device_id",
        "mei" => "mei_cl_device_id",
        "rapidio" => "rio_device_id",
        "ulpi" => "ulpi_device_id",
        "hdaudio" => "hda_device_id",
        "sdw" => "sdw_device_id",
        "fslmc" => "fsl_mc_device_id",
        "tbsvc" => "tb_service_id",
        "typec" => "typec_device_id",
        "tee" => "tee_client_device_id",
        "wmi" => "wmi_device_id",
        "mhi" | "mhi_ep" => "mhi_device_id",
        "auxiliary" => "auxiliary_device_id",
        "ssam" => "ssam_device_id",
        "dfl" => "dfl_device_id",
        "ishtp" => "ishtp_device_id",
        "cdx" => "cdx_device_id",
        "vchiq" => "vchiq_device_id",
        "coreboot" => "coreboot_device_id",
        "of" => "of_device_id",
        "usb" => "usb_device_id",
        "pnp" => "pnp_device_id",
        "pnp_card" => "pnp_card_device_id",
        _ => return None,
    })
}

struct Entry<'a> {
    tables: &'a Tables,
    structure: &'static str,
    data: &'a [u8],
    target: Target,
}

impl Entry<'_> {
    fn offset(&self, field: &str) -> Result<usize, Diagnostic> {
        self.tables
            .offset(&format!("OFF_{}_{field}", self.structure))
    }

    fn bytes(&self, offset: usize, count: usize) -> Result<&[u8], Diagnostic> {
        offset
            .checked_add(count)
            .and_then(|end| self.data.get(offset..end))
            .ok_or_else(|| {
                Diagnostic::new(
                    Severity::Fatal,
                    format!("{}: device-table field outside entry\n", self.structure),
                )
            })
    }

    fn number_at(&self, offset: usize, width: usize) -> Result<u64, Diagnostic> {
        let bytes = self.bytes(offset, width)?;
        let mut value = 0;
        if self.target.big_endian {
            for &byte in bytes {
                value = (value << 8) | u64::from(byte);
            }
        } else {
            for &byte in bytes.iter().rev() {
                value = (value << 8) | u64::from(byte);
            }
        }
        Ok(value)
    }

    fn number(&self, field: &str, width: usize) -> Result<u64, Diagnostic> {
        self.number_at(self.offset(field)?, width)
    }

    fn string_bytes_at(&self, offset: usize, width: usize) -> Result<&[u8], Diagnostic> {
        let bytes = self.bytes(offset, width)?;
        let length = bytes.iter().position(|&byte| byte == 0).ok_or_else(|| {
            Diagnostic::new(
                Severity::Error,
                format!("{}: unterminated device-table string\n", self.structure),
            )
        })?;
        Ok(&bytes[..length])
    }

    fn string_at(&self, offset: usize, width: usize) -> Result<&str, Diagnostic> {
        std::str::from_utf8(self.string_bytes_at(offset, width)?).map_err(|_| {
            Diagnostic::new(
                Severity::Error,
                format!("{}: non-UTF-8 device-table string\n", self.structure),
            )
        })
    }

    fn string(&self, field: &str, width: usize) -> Result<&str, Diagnostic> {
        self.string_at(self.offset(field)?, width)
    }
}

#[derive(Clone, Copy)]
enum Match {
    Not(u64),
    Flags(u64),
}

type Part = (&'static str, usize, &'static str, Match);

fn add(output: &mut String, separator: &str, matches: bool, value: u64, width: usize) {
    output.push_str(separator);
    if matches {
        let _ = write!(output, "{value:0digits$X}", digits = width * 2);
    } else {
        output.push('*');
    }
}

fn wildcard(mut text: String, append: bool) -> String {
    if append && !text.ends_with('*') {
        text.push('*');
    }
    text
}

fn simple(bus: &str, entry: &Entry<'_>) -> Result<Option<String>, Diagnostic> {
    use Match::{Flags as F, Not as N};
    let (prefix, parts, flags_width, append): (&str, &[Part], usize, bool) = match bus {
        "hid" => (
            "hid:",
            &[
                ("bus", 2, "b", N(0xffff)),
                ("group", 2, "g", N(0)),
                ("vendor", 4, "v", N(0xffffffff)),
                ("product", 4, "p", N(0xffffffff)),
            ],
            0,
            false,
        ),
        "ieee1394" => (
            "ieee1394:",
            &[
                ("vendor_id", 4, "ven", F(1)),
                ("model_id", 4, "mo", F(2)),
                ("specifier_id", 4, "sp", F(4)),
                ("version", 4, "ver", F(8)),
            ],
            4,
            true,
        ),
        "ccw" => (
            "ccw:",
            &[
                ("cu_type", 2, "t", F(1)),
                ("cu_model", 1, "m", F(2)),
                ("dev_type", 2, "dt", F(4)),
                ("dev_model", 1, "dm", F(8)),
            ],
            2,
            true,
        ),
        "serio" => (
            "serio:",
            &[
                ("type", 1, "ty", N(0xff)),
                ("proto", 1, "pr", N(0xff)),
                ("id", 1, "id", N(0xff)),
                ("extra", 1, "ex", N(0xff)),
            ],
            0,
            true,
        ),
        "parisc" => (
            "parisc:",
            &[
                ("hw_type", 1, "t", N(0xff)),
                ("hversion", 2, "hv", N(0xffff)),
                ("hversion_rev", 1, "rev", N(0xff)),
                ("sversion", 4, "sv", N(0xffffffff)),
            ],
            0,
            true,
        ),
        "sdio" => (
            "sdio:",
            &[
                ("class", 1, "c", N(0xff)),
                ("vendor", 2, "v", N(0xffff)),
                ("device", 2, "d", N(0xffff)),
            ],
            0,
            true,
        ),
        "ssb" => (
            "ssb:",
            &[
                ("vendor", 2, "v", N(0xffff)),
                ("coreid", 2, "id", N(0xffff)),
                ("revision", 1, "rev", N(0xff)),
            ],
            0,
            true,
        ),
        "bcma" => (
            "bcma:",
            &[
                ("manuf", 2, "m", N(0xffff)),
                ("id", 2, "id", N(0xffff)),
                ("rev", 1, "rev", N(0xff)),
                ("class", 1, "cl", N(0xff)),
            ],
            0,
            true,
        ),
        "virtio" => (
            "virtio:",
            &[
                ("device", 4, "d", N(0xffffffff)),
                ("vendor", 4, "v", N(0xffffffff)),
            ],
            0,
            true,
        ),
        "i3c" => (
            "i3c:",
            &[
                ("dcr", 1, "dcr", F(1)),
                ("manuf_id", 2, "manuf", F(2)),
                ("part_id", 2, "part", F(4)),
                ("extra_info", 2, "ext", F(8)),
            ],
            1,
            false,
        ),
        "zorro" => ("zorro:", &[("id", 4, "i", N(0xffffffff))], 0, false),
        "ipack" => (
            "ipack:",
            &[
                ("format", 1, "f", N(0xff)),
                ("vendor", 4, "v", N(0xffffffff)),
                ("device", 4, "d", N(0xffffffff)),
            ],
            0,
            true,
        ),
        "rapidio" => (
            "rapidio:",
            &[
                ("vid", 2, "v", N(0xffff)),
                ("did", 2, "d", N(0xffff)),
                ("asm_vid", 2, "av", N(0xffff)),
                ("asm_did", 2, "ad", N(0xffff)),
            ],
            0,
            true,
        ),
        "hdaudio" => (
            "hdaudio:",
            &[
                ("vendor_id", 4, "v", N(0)),
                ("rev_id", 4, "r", N(0)),
                ("api_version", 1, "a", N(0)),
            ],
            0,
            true,
        ),
        "sdw" => (
            "sdw:",
            &[
                ("mfg_id", 2, "m", N(0)),
                ("part_id", 2, "p", N(0)),
                ("sdw_version", 1, "v", N(0)),
                ("class_id", 1, "c", N(0)),
            ],
            0,
            true,
        ),
        _ => return Ok(None),
    };
    let flags = if flags_width == 0 {
        0
    } else {
        entry.number("match_flags", flags_width)?
    };
    let mut alias = prefix.to_owned();
    for &(field, width, separator, condition) in parts {
        let value = entry.number(field, width)?;
        let matches = match condition {
            N(any) => value != any,
            F(mask) => flags & mask != 0,
        };
        add(&mut alias, separator, matches, value, width);
    }
    Ok(Some(wildcard(alias, append)))
}

fn generate(bus: &str, module: &str, e: &Entry<'_>) -> Result<Vec<String>, Diagnostic> {
    if let Some(alias) = simple(bus, e)? {
        return Ok(vec![alias]);
    }
    let number = |field, width| e.number(field, width);
    let string = |field, width| e.string(field, width);
    let alias = match bus {
        "ap" => format!("ap:t{:02X}*", number("dev_type", 1)?),
        "css" => format!("css:t{:01X}", number("type", 1)?),
        "mipscdmm" => format!("mipscdmm:t{:02X}*", number("type", 1)?),
        "cpu" => format!("cpu:type:*:feature:*{:04X}*", number("feature", 2)?),
        "mcb" => format!("mcb:16z{:03}", number("device", 2)?),
        "slim" => format!(
            "slim:{:x}:{:x}:*",
            number("manf_id", 2)?,
            number("prod_code", 2)?
        ),
        "ulpi" => format!(
            "ulpi:v{:04x}p{:04x}",
            number("vendor", 2)?,
            number("product", 2)?
        ),
        "typec" => format!("typec:id{:04X}", number("svid", 2)?),
        "coreboot" => format!("coreboot:t{:08X}", number("tag", 4)?),
        "fslmc" => format!(
            "fsl-mc:v{:08X}d{}",
            number("vendor", 2)?,
            string("obj_type", 16)?
        ),
        "dfl" => format!(
            "dfl:t{:04X}f{:04X}*",
            number("type", 2)?,
            number("feature_id", 2)?
        ),
        "eisa" => format!("eisa:s{}*", string("sig", 8)?),
        "rpmsg" => format!("rpmsg:{}", string("name", 32)?),
        "i2c" => format!("i2c:{}", string("name", 20)?),
        "spi" => format!("spi:{}", string("name", 32)?),
        "platform" => format!("platform:{}", string("name", 24)?),
        "auxiliary" => format!("auxiliary:{}", string("name", 40)?),
        "vchiq" => format!("vchiq:{}", string("name", 32)?),
        "mhi" | "mhi_ep" => format!("{bus}:{}", string("chan", 32)?),
        "pci" | "cdx" => return pci(bus, module, e),
        "usb" => return usb(e),
        "input" => input(e)?,
        "dmi" => dmi(e)?,
        "wmi" => return wmi(module, e),
        "of" => {
            let name = string("name", 32)?;
            let kind = string("type", 32)?;
            let compatible = string("compatible", 128)?;
            let mut alias = format!("of:N{}T{}", star_if_empty(name), star_if_empty(kind));
            if !compatible.is_empty() {
                if !kind.is_empty() {
                    alias.push('*');
                }
                alias.push('C');
                alias.push_str(compatible);
            }
            alias = whitespace(&alias);
            return Ok(vec![alias.clone(), format!("{alias}C*")]);
        }
        "vio" => wildcard(
            whitespace(&format!(
                "vio:T{}S{}",
                star_if_empty(string("type", 32)?),
                star_if_empty(string("compat", 32)?)
            )),
            true,
        ),
        "acpi" => {
            let id = string("id", 16)?;
            let mut alias = id.to_owned();
            if id.is_empty() {
                let class = number("cls", 4)?;
                let mask = number("cls_msk", 4)?;
                for shift in [16, 8, 0] {
                    if mask >> shift & 0xff != 0 {
                        let _ = write!(alias, "{:02x}", class >> shift & 0xff);
                    } else {
                        alias.push_str("??");
                    }
                }
            }
            format!("acpi*:{alias}:*")
        }
        "pnp" => return Ok(pnp(string("id", 8)?)),
        "pnp_card" => {
            let mut aliases = Vec::new();
            let offset = e.offset("devs")?;
            for index in 0..8 {
                let id = e.string_at(offset + index * 8, 8)?;
                if id.is_empty() {
                    break;
                }
                aliases.extend(pnp(id));
            }
            return Ok(aliases);
        }
        "pcmcia" => {
            let flags = number("match_flags", 2)?;
            let mut alias = String::from("pcmcia:");
            for (field, width, separator, flag) in [
                ("manf_id", 2, "m", 1),
                ("card_id", 2, "c", 2),
                ("func_id", 1, "f", 4),
                ("function", 1, "fn", 8),
                ("device_no", 1, "pfn", 0x100),
            ] {
                add(
                    &mut alias,
                    separator,
                    flags & flag != 0,
                    number(field, width)?,
                    width,
                );
            }
            let hashes = e.offset("prod_id_hash")?;
            for (index, separator) in ["pa", "pb", "pc", "pd"].iter().enumerate() {
                add(
                    &mut alias,
                    separator,
                    flags & (0x10 << index) != 0,
                    e.number_at(hashes + index * 4, 4)?,
                    4,
                );
            }
            wildcard(alias, true)
        }
        "vmbus" => {
            let bytes = e.bytes(e.offset("guid")?, 16)?;
            let mut alias = String::from("vmbus:");
            for byte in bytes {
                let _ = write!(alias, "{byte:02x}");
            }
            alias
        }
        "tee" => format!(
            "tee:{}*",
            uuid(e.bytes(e.offset("uuid")?, 16)?, false, false)
        ),
        "ishtp" => format!(
            "ishtp:{{{}}}",
            uuid(e.bytes(e.offset("guid")?, 16)?, true, true)
        ),
        "mei" => {
            let mut alias = format!(
                "mei:{}:{}",
                star_if_empty(string("name", 32)?),
                uuid(e.bytes(e.offset("uuid")?, 16)?, true, false)
            );
            let version = number("version", 1)?;
            add(&mut alias, ":", version != 0xff, version, 1);
            alias.push_str(":*");
            alias
        }
        "mdio" => {
            let id = number("phy_id", 4)?;
            let mask = number("phy_id_mask", 4)?;
            let mut alias = String::from("mdio:");
            for shift in (0..32).rev() {
                alias.push(if mask >> shift & 1 == 0 {
                    '?'
                } else if id >> shift & 1 != 0 {
                    '1'
                } else {
                    '0'
                });
            }
            alias
        }
        "isapnp" => {
            let vendor = number("vendor", 2)?;
            let function = number("function", 2)?;
            format!(
                "pnp:d{}{}{}{:x}{:x}{:x}{:x}*",
                (b'A' - 1 + ((vendor >> 2) & 0x3f) as u8) as char,
                (b'A' - 1 + (((vendor & 3) << 3) | ((vendor >> 13) & 7)) as u8) as char,
                (b'A' - 1 + ((vendor >> 8) & 0x1f) as u8) as char,
                function >> 4 & 15,
                function & 15,
                function >> 12 & 15,
                function >> 8 & 15
            )
        }
        "amba" => {
            let id = number("id", 4)?;
            let mask = number("mask", 4)?;
            if id & mask != id {
                return Err(Diagnostic::new(Severity::Fatal, format!(
                    "{module}: Masked-off bit(s) of AMBA device ID are non-zero: id=0x{id:08X}, mask=0x{mask:08X}.  Please fix this driver.\n"
                )));
            }
            let mut alias = String::from("amba:d");
            for shift in (0..8).rev().map(|digit| digit * 4) {
                let nibble = id >> shift & 15;
                match mask >> shift & 15 {
                    0 => alias.push('?'),
                    15 => {
                        let _ = write!(alias, "{nibble:X}");
                    }
                    part => {
                        alias.push('[');
                        for digit in 0..16 {
                            if digit & part == nibble {
                                let _ = write!(alias, "{digit:X}");
                            }
                        }
                        alias.push(']');
                    }
                }
            }
            alias
        }
        "x86cpu" => {
            let mut alias = String::from("cpu:type:x86,");
            for (field, separator, any) in [
                ("vendor", "ven", 0xffff),
                ("family", "fam", 0),
                ("model", "mod", 0),
            ] {
                let value = number(field, 2)?;
                add(&mut alias, separator, value != any, value, 2);
            }
            alias.push_str(":feature:*");
            let feature = number("feature", 2)?;
            if feature != 0 {
                let _ = write!(alias, "{feature:04X}*");
            }
            alias
        }
        "tbsvc" => {
            let flags = number("match_flags", 4)?;
            let key = if flags & 1 != 0 {
                string("protocol_key", 9)?
            } else {
                "*"
            };
            let mut alias = format!("tbsvc:k{key}");
            for (field, separator, flag) in [
                ("protocol_id", "p", 2),
                ("protocol_version", "v", 4),
                ("protocol_revision", "r", 8),
            ] {
                add(
                    &mut alias,
                    separator,
                    flags & flag != 0,
                    number(field, 4)?,
                    4,
                );
            }
            wildcard(alias, true)
        }
        "ssam" => {
            let flags = number("match_flags", 1)?;
            let mut alias = format!(
                "ssam:d{:02X}c{:02X}",
                number("domain", 1)?,
                number("category", 1)?
            );
            for (field, separator, flag) in [
                ("target", "t", 1),
                ("instance", "i", 2),
                ("function", "f", 4),
            ] {
                add(
                    &mut alias,
                    separator,
                    flags & flag != 0,
                    number(field, 1)?,
                    1,
                );
            }
            alias
        }
        _ => unreachable!("all recognized device tables have a formatter"),
    };
    Ok(vec![alias])
}

fn star_if_empty(text: &str) -> &str {
    if text.is_empty() {
        "*"
    } else {
        text
    }
}

fn whitespace(text: &str) -> String {
    text.chars()
        .map(|ch| {
            if matches!(ch, ' ' | '\t'..='\r') {
                '_'
            } else {
                ch
            }
        })
        .collect()
}

fn pnp(id: &str) -> Vec<String> {
    vec![
        format!("pnp:d{id}*"),
        format!("acpi*:{}:*", id.to_ascii_uppercase()),
    ]
}

fn uuid(bytes: &[u8], little_fields: bool, uppercase: bool) -> String {
    let order = if little_fields {
        [3, 2, 1, 0, 5, 4, 7, 6, 8, 9, 10, 11, 12, 13, 14, 15]
    } else {
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    };
    let mut text = String::new();
    for (index, offset) in order.into_iter().enumerate() {
        if [4, 6, 8, 10].contains(&index) {
            text.push('-');
        }
        let _ = write!(text, "{:02x}", bytes[offset]);
    }
    if uppercase {
        text.make_ascii_uppercase();
    }
    text
}

fn pci(bus: &str, module: &str, e: &Entry<'_>) -> Result<Vec<String>, Diagnostic> {
    let override_only = e.number("override_only", 4)?;
    let mut alias = match override_only {
        0 => format!("{bus}:"),
        1 => format!("vfio_{bus}:"),
        _ => {
            return Err(Diagnostic::new(
                Severity::Warning,
                format!(
                    "Unknown {} driver_override alias {override_only:08X}\n",
                    bus.to_ascii_uppercase()
                ),
            ))
        }
    };
    let width = if bus == "pci" { 4 } else { 2 };
    let any = if width == 4 { 0xffffffff } else { 0xffff };
    for (field, separator) in [
        ("vendor", "v"),
        ("device", "d"),
        ("subvendor", "sv"),
        ("subdevice", "sd"),
    ] {
        let value = e.number(field, width)?;
        add(&mut alias, separator, value != any, value, width);
    }
    let class = e.number("class", 4)?;
    let mask = e.number("class_mask", 4)?;
    if bus == "cdx" {
        add(&mut alias, "c", mask == 0xffffff, class, 4);
    } else {
        for shift in [16, 8, 0] {
            let part = mask >> shift & 255;
            if part != 0 && part != 255 {
                return Err(Diagnostic::new(
                    Severity::Warning,
                    format!("Can't handle masks in {module}:{mask:04X}\n"),
                ));
            }
        }
        for (separator, shift) in [("bc", 16), ("sc", 8), ("i", 0)] {
            add(
                &mut alias,
                separator,
                mask >> shift & 255 == 255,
                class >> shift & 255,
                1,
            );
        }
    }
    Ok(vec![wildcard(alias, bus == "pci")])
}

fn wmi(module: &str, e: &Entry<'_>) -> Result<Vec<String>, Diagnostic> {
    let guid = e.string("guid_string", 37)?;
    if guid.len() != 36 {
        return Err(Diagnostic::new(
            Severity::Warning,
            format!("Invalid WMI device id 'wmi:{guid}' in '{module}'\n"),
        ));
    }
    for (index, ch) in guid.bytes().enumerate() {
        let valid = if [8, 13, 18, 23].contains(&index) {
            ch == b'-'
        } else {
            ch.is_ascii_hexdigit()
        };
        if !valid {
            return Err(Diagnostic::new(
                Severity::Warning,
                format!(
                    "Invalid character {} inside WMI GUID string '{guid}' in '{module}'\n",
                    ch as char
                ),
            ));
        }
    }
    Ok(vec![format!("wmi:{}", guid.to_ascii_uppercase())])
}

fn checked_append(alias: &mut String, text: &str) -> Result<(), Diagnostic> {
    if alias.len() >= 256 {
        return Err(Diagnostic::new(
            Severity::Fatal,
            "alias buffer (256) overflow before append\n",
        ));
    }
    if text.len() >= 256 - alias.len() {
        return Err(Diagnostic::new(
            Severity::Fatal,
            format!(
                "alias buffer (256) overflow on append (need {}, have {})\n",
                text.len(),
                256 - alias.len()
            ),
        ));
    }
    alias.push_str(text);
    Ok(())
}

fn input(e: &Entry<'_>) -> Result<String, Diagnostic> {
    let width = (e.target.word_bits / 8) as usize;
    let flags = e.number("flags", width)?;
    let mut alias = String::new();
    for (field, separator, flag) in [
        ("bustype", "b", 1),
        ("vendor", "v", 2),
        ("product", "p", 4),
        ("version", "e", 8),
    ] {
        add(
            &mut alias,
            separator,
            flags & flag != 0,
            e.number(field, 2)?,
            2,
        );
    }
    for (field, separator, flag, min, max) in [
        ("evbit", "-e*", 0x10, 0, 0x1f),
        ("keybit", "k*", 0x20, 0x71, 0x2ff),
        ("relbit", "r*", 0x40, 0, 0xf),
        ("absbit", "a*", 0x80, 0, 0x3f),
        ("mscbit", "m*", 0x100, 0, 7),
        ("ledbit", "l*", 0x200, 0, 0xf),
        ("sndbit", "s*", 0x400, 0, 7),
        ("ffbit", "f*", 0x800, 0, 0x7f),
        ("swbit", "w*", 0x1000, 0, 0x11),
    ] {
        checked_append(&mut alias, separator)?;
        if flags & flag == 0 {
            continue;
        }
        let offset = e.offset(field)?;
        for bit in min..=max {
            let word = e.number_at(offset + bit / (width * 8) * width, width)?;
            if word >> (bit % (width * 8)) & 1 != 0 {
                checked_append(&mut alias, &format!("{bit:X},*"))?;
            }
        }
    }
    Ok(format!("input:{alias}"))
}

fn dmi(e: &Entry<'_>) -> Result<String, Diagnostic> {
    let matches = e.offset("matches")?;
    let mut alias = String::new();
    for (prefix, slot) in [
        ("bvn", 1),
        ("bvr", 2),
        ("bd", 3),
        ("br", 4),
        ("efr", 5),
        ("svn", 6),
        ("pn", 7),
        ("pvr", 8),
        ("rvn", 13),
        ("rn", 14),
        ("rvr", 15),
        ("cvn", 18),
        ("ct", 19),
        ("cvr", 20),
    ] {
        for index in 0..4 {
            let offset = matches + index * 80;
            let bits = e.number_at(offset, 1)?;
            // C bitfield allocation follows the target, not the build host.
            let field = if e.target.big_endian {
                bits >> 1
            } else {
                bits & 0x7f
            };
            if field != slot {
                continue;
            }
            checked_append(&mut alias, &format!(":{prefix}*"))?;
            for &byte in e.string_bytes_at(offset + 1, 79)? {
                if byte > b' ' && byte < 127 && byte != b':' {
                    if alias.len() >= 255 {
                        return Err(Diagnostic::new(
                            Severity::Fatal,
                            "dmi_ascii_filter: alias buffer overflow\n",
                        ));
                    }
                    alias.push(byte as char);
                }
            }
            checked_append(&mut alias, "*")?;
        }
    }
    Ok(format!("dmi*{alias}:"))
}

fn inc_bcd(value: &mut u32, increment: i32, max: u8) -> u32 {
    let previous = *value;
    if max > 9 {
        *value = value.wrapping_add(increment as u32);
    } else {
        let mut decimal = 0u64;
        for digit in 0..4 {
            decimal += u64::from(((*value >> (digit * 4)) & 15).min(9)) * 10u64.pow(digit);
        }
        decimal = decimal.wrapping_add(increment as u64);
        *value = 0;
        for digit in 0..4 {
            *value |= ((decimal / 10u64.pow(digit) % 10) as u32) << (digit * 4);
        }
    }
    previous
}

fn usb(e: &Entry<'_>) -> Result<Vec<String>, Diagnostic> {
    let flags = e.number("match_flags", 2)?;
    let mut low = if flags & 4 != 0 {
        e.number("bcdDevice_lo", 2)? as u32
    } else {
        0
    };
    let mut high = if flags & 8 != 0 {
        e.number("bcdDevice_hi", 2)? as u32
    } else {
        u32::MAX
    };
    if e.number("idVendor", 2)?
        | e.number("idProduct", 2)?
        | e.number("bDeviceClass", 1)?
        | e.number("bInterfaceClass", 1)?
        == 0
    {
        return Ok(Vec::new());
    }
    let max = if (0..4)
        .any(|digit| (low >> (digit * 4) & 15) > 9 || (high.min(0x9999) >> (digit * 4) & 15) > 9)
    {
        15
    } else {
        9
    };
    let mut aliases = Vec::new();
    for digits in (0..4).rev() {
        if low > high {
            break;
        }
        let lo = (low & 15) as u8;
        let hi = ((high & 15) as u8).min(max);
        low >>= 4;
        high >>= 4;
        if low == high || digits == 0 {
            aliases.push(usb_range(e, flags, low, digits, lo, hi, max)?);
            break;
        }
        if lo > 0 {
            aliases.push(usb_range(
                e,
                flags,
                inc_bcd(&mut low, 1, max),
                digits,
                lo,
                max,
                max,
            )?);
        }
        if hi < max {
            aliases.push(usb_range(
                e,
                flags,
                inc_bcd(&mut high, -1, max),
                digits,
                0,
                hi,
                max,
            )?);
        }
    }
    Ok(aliases)
}

fn usb_range(
    e: &Entry<'_>,
    flags: u64,
    initial: u32,
    digits: usize,
    lo: u8,
    hi: u8,
    max: u8,
) -> Result<String, Diagnostic> {
    let mut alias = String::from("usb:");
    add(&mut alias, "v", flags & 1 != 0, e.number("idVendor", 2)?, 2);
    add(
        &mut alias,
        "p",
        flags & 2 != 0,
        e.number("idProduct", 2)?,
        2,
    );
    alias.push('d');
    if digits > 0 {
        let _ = write!(alias, "{initial:0digits$X}");
    }
    if lo == hi {
        let _ = write!(alias, "{lo:X}");
    } else if lo > 0 || hi < max {
        if lo > 9 || hi < 10 {
            let _ = write!(alias, "[{lo:X}-{hi:X}]");
        } else {
            if lo < 9 {
                let _ = write!(alias, "[{lo:X}-9");
            } else {
                let _ = write!(alias, "[{lo:X}");
            }
            if hi > 10 {
                let _ = write!(alias, "A-{hi:X}]");
            } else {
                let _ = write!(alias, "{hi:X}]");
            }
        }
    }
    if digits < 3 {
        alias.push('*');
    }
    for (field, separator, flag) in [
        ("bDeviceClass", "dc", 0x10),
        ("bDeviceSubClass", "dsc", 0x20),
        ("bDeviceProtocol", "dp", 0x40),
        ("bInterfaceClass", "ic", 0x80),
        ("bInterfaceSubClass", "isc", 0x100),
        ("bInterfaceProtocol", "ip", 0x200),
        ("bInterfaceNumber", "in", 0x400),
    ] {
        add(
            &mut alias,
            separator,
            flags & flag != 0,
            e.number(field, 1)?,
            1,
        );
    }
    Ok(wildcard(alias, true))
}
