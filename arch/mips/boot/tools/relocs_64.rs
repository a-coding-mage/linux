// SPDX-License-Identifier: GPL-2.0
//! ELF64 MIPS uses RELA with a target-endian symbol word and four type bytes.
use crate::relocs_header::Format;

pub(crate) const FORMAT: Format = Format {
    bits: 64,
    class: 2,
    header_size: 64,
    program_size: 56,
    section_size: 64,
    symbol_size: 24,
    relocation_size: 24,
    relocation_kind: 4,
    machine_name: "MIPS64",
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
