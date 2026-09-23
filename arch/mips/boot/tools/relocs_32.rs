// SPDX-License-Identifier: GPL-2.0
//! ELF32 MIPS uses REL records and modulo-32-bit image offsets.
use crate::relocs_header::Format;

pub(crate) const FORMAT: Format = Format {
    bits: 32,
    class: 1,
    header_size: 52,
    program_size: 32,
    section_size: 40,
    symbol_size: 16,
    relocation_size: 8,
    relocation_kind: 9,
    machine_name: "MIPS",
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
