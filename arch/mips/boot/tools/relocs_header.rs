// SPDX-License-Identifier: GPL-2.0
//! Shared options and target-format definitions for MIPS kernel relocations.

#[derive(Default)]
pub(crate) struct Options {
    pub(crate) text: bool,
    pub(crate) binary: bool,
    pub(crate) info: bool,
    pub(crate) keep: bool,
}

pub(crate) struct Format {
    pub(crate) bits: u32,
    pub(crate) class: u8,
    pub(crate) header_size: usize,
    pub(crate) program_size: u64,
    pub(crate) section_size: u64,
    pub(crate) symbol_size: u64,
    pub(crate) relocation_size: u64,
    pub(crate) relocation_kind: u32,
    pub(crate) machine_name: &'static str,
}

pub(crate) struct Edit {
    pub(crate) offset: u64,
    pub(crate) bytes: Vec<u8>,
}

pub(crate) fn relocation_name(kind: u32) -> &'static str {
    match kind {
        0 => "R_MIPS_NONE",
        1 => "R_MIPS_16",
        2 => "R_MIPS_32",
        3 => "R_MIPS_REL32",
        4 => "R_MIPS_26",
        5 => "R_MIPS_HI16",
        6 => "R_MIPS_LO16",
        7 => "R_MIPS_GPREL16",
        8 => "R_MIPS_LITERAL",
        9 => "R_MIPS_GOT16",
        10 => "R_MIPS_PC16",
        11 => "R_MIPS_CALL16",
        12 => "R_MIPS_GPREL32",
        18 => "R_MIPS_64",
        28 => "R_MIPS_HIGHER",
        29 => "R_MIPS_HIGHEST",
        60 => "R_MIPS_PC21_S2",
        61 => "R_MIPS_PC26_S2",
        248 => "R_MIPS_PC32",
        _ => "unknown type rel type name",
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
