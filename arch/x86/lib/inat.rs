// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * x86 instruction attribute tables
 *
 * Written by Masami Hiramatsu <mhiramat@redhat.com>
 */
// Dependency declarations from <asm/insn.h> and the generated "inat-tables.c"
// are supplied by other translation units.

/* Attribute search APIs */
pub unsafe fn inat_get_opcode_attribute(opcode: insn_byte_t) -> insn_attr_t {
    inat_primary_table[opcode as usize]
}

pub unsafe fn inat_get_last_prefix_id(last_pfx: insn_byte_t) -> i32 {
    let lpfx_attr: insn_attr_t;

    lpfx_attr = inat_get_opcode_attribute(last_pfx);
    inat_last_prefix_id(lpfx_attr)
}

pub unsafe fn inat_get_escape_attribute(
    opcode: insn_byte_t,
    lpfx_id: i32,
    esc_attr: insn_attr_t,
) -> insn_attr_t {
    let mut table: *const insn_attr_t;
    let n: i32;

    n = inat_escape_id(esc_attr);

    table = inat_escape_tables[n as usize][0];
    if table.is_null() {
        return 0 as insn_attr_t;
    }
    if inat_has_variant(*table.add(opcode as usize)) && lpfx_id != 0 {
        table = inat_escape_tables[n as usize][lpfx_id as usize];
        if table.is_null() {
            return 0 as insn_attr_t;
        }
    }
    *table.add(opcode as usize)
}

pub unsafe fn inat_get_group_attribute(
    modrm: insn_byte_t,
    lpfx_id: i32,
    grp_attr: insn_attr_t,
) -> insn_attr_t {
    let mut table: *const insn_attr_t;
    let n: i32;

    n = inat_group_id(grp_attr);

    table = inat_group_tables[n as usize][0];
    if table.is_null() {
        return inat_group_common_attribute(grp_attr);
    }
    let reg = ((modrm >> 3) & 7) as usize;
    if inat_has_variant(*table.add(reg)) && lpfx_id != 0 {
        table = inat_group_tables[n as usize][lpfx_id as usize];
        if table.is_null() {
            return inat_group_common_attribute(grp_attr);
        }
    }
    *table.add(reg) | inat_group_common_attribute(grp_attr)
}

pub unsafe fn inat_get_avx_attribute(
    opcode: insn_byte_t,
    vex_m: insn_byte_t,
    vex_p: insn_byte_t,
) -> insn_attr_t {
    let mut table: *const insn_attr_t;
    if vex_m > X86_VEX_M_MAX || vex_p > INAT_LSTPFX_MAX {
        return 0 as insn_attr_t;
    }
    /* At first, this checks the master table */
    table = inat_avx_tables[vex_m as usize][0];
    if table.is_null() {
        return 0 as insn_attr_t;
    }
    if !inat_is_group(*table.add(opcode as usize)) && vex_p != 0 {
        /* If this is not a group, get attribute directly */
        table = inat_avx_tables[vex_m as usize][vex_p as usize];
        if table.is_null() {
            return 0 as insn_attr_t;
        }
    }
    *table.add(opcode as usize)
}

pub unsafe fn inat_get_xop_attribute(
    opcode: insn_byte_t,
    mut map_select: insn_byte_t,
) -> insn_attr_t {
    let table: *const insn_attr_t;

    if map_select < X86_XOP_M_MIN || map_select > X86_XOP_M_MAX {
        return 0 as insn_attr_t;
    }
    map_select -= X86_XOP_M_MIN;
    /* At first, this checks the master table */
    table = inat_xop_tables[map_select as usize];
    if table.is_null() {
        return 0 as insn_attr_t;
    }
    *table.add(opcode as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
