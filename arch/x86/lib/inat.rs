// SPDX-License-Identifier: GPL-2.0-or-later
// Written by Masami Hiramatsu <mhiramat@redhat.com>.
//! Immutable, checked x86 instruction attribute lookup for kernel and host decoders.

#[path = "../include/asm/inat_header.rs"]
mod attributes;
pub(crate) use attributes::*;
#[path = "inat_tables.rs"]
#[rustfmt::skip]
mod tables;

pub(crate) fn inat_get_opcode_attribute(opcode: Byte) -> Attr {
    tables::INAT_PRIMARY_TABLE[opcode as usize]
}

pub(crate) fn inat_get_last_prefix_id(last_prefix: Byte) -> u8 {
    inat_last_prefix_id(inat_get_opcode_attribute(last_prefix))
}

pub(crate) fn inat_get_escape_attribute(opcode: Byte, prefix: u8, escape: Attr) -> Attr {
    let row = &tables::INAT_ESCAPE_TABLES[inat_escape_id(escape) as usize];
    let Some(mut table) = row[0] else { return 0 };
    if inat_has_variant(table[opcode as usize]) && prefix != 0 {
        let Some(Some(variant)) = row.get(prefix as usize) else {
            return 0;
        };
        table = variant;
    }
    table[opcode as usize]
}

pub(crate) fn inat_get_group_attribute(modrm: Byte, prefix: u8, group: Attr) -> Attr {
    let common = inat_group_common_attribute(group);
    let row = &tables::INAT_GROUP_TABLES[inat_group_id(group) as usize];
    let Some(mut table) = row[0] else {
        return common;
    };
    let reg = ((modrm >> 3) & 7) as usize;
    if inat_has_variant(table[reg]) && prefix != 0 {
        let Some(Some(variant)) = row.get(prefix as usize) else {
            return common;
        };
        table = variant;
    }
    table[reg] | common
}

pub(crate) fn inat_get_avx_attribute(opcode: Byte, map: Byte, prefix: Byte) -> Attr {
    if prefix > INAT_LSTPFX_MAX {
        return 0;
    }
    let Some(row) = tables::INAT_AVX_TABLES.get(map as usize) else {
        return 0;
    };
    let Some(mut table) = row[0] else { return 0 };
    if !inat_is_group(table[opcode as usize]) && prefix != 0 {
        let Some(variant) = row[prefix as usize] else {
            return 0;
        };
        table = variant;
    }
    table[opcode as usize]
}

pub(crate) fn inat_get_xop_attribute(opcode: Byte, map: Byte) -> Attr {
    let Some(index) = map.checked_sub(8) else {
        return 0;
    };
    let Some(Some(table)) = tables::INAT_XOP_TABLES.get(index as usize) else {
        return 0;
    };
    table[opcode as usize]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
