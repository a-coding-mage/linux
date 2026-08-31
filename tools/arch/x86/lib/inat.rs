// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * x86 instruction attribute tables
 *
 * Written by Masami Hiramatsu <mhiramat@redhat.com>
 */

/* Dependency intent from C:
 * #include "../include/asm/insn.h" // __ignore_sync_check__
 */

/* Attribute tables are generated from opcode map.
 * Dependency intent from C:
 * #include "inat-tables.c"
 */

#[allow(non_camel_case_types)]
pub type insn_attr_t = u32;
#[allow(non_camel_case_types)]
pub type insn_byte_t = u8;

unsafe extern "C" {
    static inat_primary_table: [insn_attr_t; 256];
    static inat_escape_tables: [[*const insn_attr_t; (INAT_LSTPFX_MAX as usize) + 1]; 0];
    static inat_group_tables: [[*const insn_attr_t; (INAT_LSTPFX_MAX as usize) + 1]; 0];
    static inat_avx_tables: [[*const insn_attr_t; (INAT_LSTPFX_MAX as usize) + 1]; (X86_VEX_M_MAX as usize) + 1];
    static inat_xop_tables: [*const insn_attr_t; ((X86_XOP_M_MAX - X86_XOP_M_MIN) as usize) + 1];

    fn inat_last_prefix_id(attr: insn_attr_t) -> i32;
    fn inat_escape_id(attr: insn_attr_t) -> i32;
    fn inat_group_id(attr: insn_attr_t) -> i32;
    fn inat_has_variant(attr: insn_attr_t) -> i32;
    fn inat_group_common_attribute(attr: insn_attr_t) -> insn_attr_t;
    fn inat_is_group(attr: insn_attr_t) -> i32;
    fn X86_MODRM_REG(modrm: insn_byte_t) -> i32;
}

unsafe extern "C" {
    static X86_VEX_M_MAX: insn_byte_t;
    static X86_XOP_M_MIN: insn_byte_t;
    static X86_XOP_M_MAX: insn_byte_t;
    static INAT_LSTPFX_MAX: insn_byte_t;
}

/* Attribute search APIs */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn inat_get_opcode_attribute(opcode: insn_byte_t) -> insn_attr_t {
    unsafe { inat_primary_table[opcode as usize] }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inat_get_last_prefix_id(last_pfx: insn_byte_t) -> i32 {
    let lpfx_attr: insn_attr_t;

    lpfx_attr = unsafe { inat_get_opcode_attribute(last_pfx) };
    unsafe { inat_last_prefix_id(lpfx_attr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inat_get_escape_attribute(
    opcode: insn_byte_t,
    lpfx_id: i32,
    esc_attr: insn_attr_t,
) -> insn_attr_t {
    let mut table: *const insn_attr_t;
    let n: i32;

    n = unsafe { inat_escape_id(esc_attr) };

    table = unsafe { inat_escape_tables[n as usize][0] };
    if table.is_null() {
        return 0;
    }
    if unsafe { inat_has_variant(*table.add(opcode as usize)) != 0 } && lpfx_id != 0 {
        table = unsafe { inat_escape_tables[n as usize][lpfx_id as usize] };
        if table.is_null() {
            return 0;
        }
    }
    unsafe { *table.add(opcode as usize) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inat_get_group_attribute(
    modrm: insn_byte_t,
    lpfx_id: i32,
    grp_attr: insn_attr_t,
) -> insn_attr_t {
    let mut table: *const insn_attr_t;
    let n: i32;

    n = unsafe { inat_group_id(grp_attr) };

    table = unsafe { inat_group_tables[n as usize][0] };
    if table.is_null() {
        return unsafe { inat_group_common_attribute(grp_attr) };
    }
    if unsafe { inat_has_variant(*table.add(X86_MODRM_REG(modrm) as usize)) != 0 } && lpfx_id != 0 {
        table = unsafe { inat_group_tables[n as usize][lpfx_id as usize] };
        if table.is_null() {
            return unsafe { inat_group_common_attribute(grp_attr) };
        }
    }
    unsafe {
        *table.add(X86_MODRM_REG(modrm) as usize) | inat_group_common_attribute(grp_attr)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inat_get_avx_attribute(
    opcode: insn_byte_t,
    vex_m: insn_byte_t,
    vex_p: insn_byte_t,
) -> insn_attr_t {
    let mut table: *const insn_attr_t;
    if unsafe { vex_m > X86_VEX_M_MAX || vex_p > INAT_LSTPFX_MAX } {
        return 0;
    }
    /* At first, this checks the master table */
    table = unsafe { inat_avx_tables[vex_m as usize][0] };
    if table.is_null() {
        return 0;
    }
    if unsafe { inat_is_group(*table.add(opcode as usize)) == 0 } && vex_p != 0 {
        /* If this is not a group, get attribute directly */
        table = unsafe { inat_avx_tables[vex_m as usize][vex_p as usize] };
        if table.is_null() {
            return 0;
        }
    }
    unsafe { *table.add(opcode as usize) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inat_get_xop_attribute(
    opcode: insn_byte_t,
    mut map_select: insn_byte_t,
) -> insn_attr_t {
    let table: *const insn_attr_t;

    if unsafe { map_select < X86_XOP_M_MIN || map_select > X86_XOP_M_MAX } {
        return 0;
    }
    unsafe {
        map_select = map_select.wrapping_sub(X86_XOP_M_MIN);
    }
    /* At first, this checks the master table */
    table = unsafe { inat_xop_tables[map_select as usize] };
    if table.is_null() {
        return 0;
    }
    unsafe { *table.add(opcode as usize) }
}
