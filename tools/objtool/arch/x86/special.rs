// SPDX-License-Identifier: GPL-2.0-or-later

// C includes translated as external dependency intent:
// <string.h>, <arch/special.h>, <objtool/special.h>, <objtool/builtin.h>,
// <objtool/warn.h>, <asm/cpufeatures.h>
//
// cpu feature name array generated from cpufeatures.h:
// #include "cpu-feature-names.c"

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    static cpu_feature_names: [*const c_char; 0];

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn find_reloc_by_dest_range(
        elf: *mut elf,
        sec: *mut section,
        offset: c_ulong,
        len: c_ulong,
    ) -> *mut reloc;
    fn find_reloc_by_dest(elf: *mut elf, sec: *mut section, offset: c_ulong) -> *mut reloc;
    fn is_sec_sym(sym: *mut symbol) -> bool;
    fn reloc_addend(reloc: *mut reloc) -> c_ulong;
    fn reloc_type(reloc: *mut reloc) -> c_int;
    fn find_symbol_containing(sec: *mut section, offset: c_ulong) -> *mut symbol;
    fn WARN_INSN(insn: *mut instruction, format: *const c_char, ...);
}

extern "Rust" {
    fn list_next_entry(iter: *mut special_alt, member: list_head) -> *mut special_alt;
}

#[repr(C)]
pub struct elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct data {
    pub d_buf: *mut c_void,
}

#[repr(C)]
pub struct section {
    pub name: *const c_char,
    pub data: *mut data,
    pub rodata: bool,
}

#[repr(C)]
pub struct symbol {
    pub sec: *mut section,
}

#[repr(C)]
pub struct reloc {
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct instruction {
    pub sec: *mut section,
    pub offset: c_ulong,
    pub len: c_ulong,
}

#[repr(C)]
pub struct special_alt {
    pub orig_sec: *mut section,
    pub orig_off: c_ulong,
    pub orig_len: c_ulong,
    pub list: list_head,
}

#[repr(C)]
pub struct objtool_file {
    pub elf: *mut elf,
    pub ignore_unreachables: bool,
}

const ALT_ENTRY_SIZE: c_ulong = 13;
const ALT_NEW_OFFSET: c_ulong = 4;
const ALT_NEW_LEN_OFFSET: c_ulong = 11;
const R_X86_64_PC32: c_int = 2;

extern "C" {
    static C_JUMP_TABLE_SECTION: *const c_char;
}

/*
 * An alternative with an empty replacement, e.g. the second entry of
 *
 *   ALTERNATIVE_2("orig", "repl", ft1, "", ft2)
 *
 * still gets a relocation for its replacement offset.  But the label it points
 * at is the end of the previous entry's replacement, which is also the
 * beginning of the *next* entry's replacement.  The value is meaningless: it's
 * only ever used with a length of zero.
 */
#[no_mangle]
pub unsafe extern "C" fn arch_alt_ignore_new_reloc(
    sec: *mut section,
    offset: c_ulong,
) -> bool {
    let entry_off: c_ulong;

    if strcmp((*sec).name, b".altinstructions\0".as_ptr() as *const c_char) != 0 {
        return false;
    }

    entry_off = offset.wrapping_sub(offset % ALT_ENTRY_SIZE);

    if offset.wrapping_sub(entry_off) != ALT_NEW_OFFSET {
        return false;
    }

    *(*(*sec).data)
        .d_buf
        .cast::<u8>()
        .add((entry_off + ALT_NEW_LEN_OFFSET) as usize)
        == 0
}

static mut GROUP: *mut special_alt = core::ptr::null_mut();
static mut PREV: *mut special_alt = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn arch_handle_alternative(alt: *mut special_alt) {
    /*
     * Recompute orig_len for nested ALTERNATIVE()s.
     */
    if !GROUP.is_null()
        && (*GROUP).orig_sec == (*alt).orig_sec
        && (*GROUP).orig_off == (*alt).orig_off
    {
        let mut iter: *mut special_alt = GROUP;
        loop {
            let len: c_ulong = if (*iter).orig_len > (*alt).orig_len {
                (*iter).orig_len
            } else {
                (*alt).orig_len
            };
            (*iter).orig_len = len;
            (*alt).orig_len = len;

            if iter == PREV {
                break;
            }

            iter = list_next_entry(iter, (*iter).list);
        }
    } else {
        GROUP = alt;
    }

    PREV = alt;
}

#[no_mangle]
pub unsafe extern "C" fn arch_support_alt_relocation(
    _special_alt: *mut special_alt,
    _insn: *mut instruction,
    _reloc: *mut reloc,
) -> bool {
    true
}

/*
 * There are 3 basic jump table patterns:
 *
 * 1. jmpq *[rodata addr](,%reg,8)
 *
 *    This is the most common case by far.  It jumps to an address in a simple
 *    jump table which is stored in .rodata.
 *
 * 2. jmpq *[rodata addr](%rip)
 *
 *    This is caused by a rare GCC quirk, currently only seen in three driver
 *    functions in the kernel, only with certain obscure non-distro configs.
 *
 *    As part of an optimization, GCC makes a copy of an existing switch jump
 *    table, modifies it, and then hard-codes the jump (albeit with an indirect
 *    jump) to use a single entry in the table.  The rest of the jump table and
 *    some of its jump targets remain as dead code.
 *
 *    In such a case we can just crudely ignore all unreachable instruction
 *    warnings for the entire object file.  Ideally we would just ignore them
 *    for the function, but that would require redesigning the code quite a
 *    bit.  And honestly that's just not worth doing: unreachable instruction
 *    warnings are of questionable value anyway, and this is such a rare issue.
 *
 * 3. mov [rodata addr],%reg1
 *    ... some instructions ...
 *    jmpq *(%reg1,%reg2,8)
 *
 *    This is a fairly uncommon pattern which is new for GCC 6.  As of this
 *    writing, there are 11 occurrences of it in the allmodconfig kernel.
 *
 *    As of GCC 7 there are quite a few more of these and the 'in between' code
 *    is significant. Esp. with KASAN enabled some of the code between the mov
 *    and jmpq uses .rodata itself, which can confuse things.
 *
 *    TODO: Once we have DWARF CFI and smarter instruction decoding logic,
 *    ensure the same register is used in the mov and jump instructions.
 *
 *    NOTE: MITIGATION_RETPOLINE made it harder still to decode dynamic jumps.
 */
#[no_mangle]
pub unsafe extern "C" fn arch_find_switch_table(
    file: *mut objtool_file,
    insn: *mut instruction,
    table_size: *mut c_ulong,
) -> *mut reloc {
    let text_reloc: *mut reloc;
    let rodata_reloc: *mut reloc;
    let table_sec: *mut section;
    let mut table_offset: c_ulong;

    /* look for a relocation which references .rodata */
    text_reloc = find_reloc_by_dest_range((*file).elf, (*insn).sec, (*insn).offset, (*insn).len);
    if text_reloc.is_null()
        || !is_sec_sym((*text_reloc).sym)
        || !(*(*(*text_reloc).sym).sec).rodata
    {
        return core::ptr::null_mut();
    }

    table_offset = reloc_addend(text_reloc);
    table_sec = (*(*text_reloc).sym).sec;

    if reloc_type(text_reloc) == R_X86_64_PC32 {
        table_offset = table_offset.wrapping_add(4);
    }

    /*
     * Make sure the .rodata address isn't associated with a
     * symbol.  GCC jump tables are anonymous data.
     *
     * Also support C jump tables which are in the same format as
     * switch jump tables.  For objtool to recognize them, they
     * need to be placed in the C_JUMP_TABLE_SECTION section.  They
     * have symbols associated with them.
     */
    if !find_symbol_containing(table_sec, table_offset).is_null()
        && strcmp((*table_sec).name, C_JUMP_TABLE_SECTION) != 0
    {
        return core::ptr::null_mut();
    }

    /*
     * Each table entry has a rela associated with it.  The rela
     * should reference text in the same function as the original
     * instruction.
     */
    rodata_reloc = find_reloc_by_dest((*file).elf, table_sec, table_offset);
    if rodata_reloc.is_null() {
        return core::ptr::null_mut();
    }

    /*
     * Use of RIP-relative switch jumps is quite rare, and
     * indicates a rare GCC quirk/bug which can leave dead
     * code behind.
     */
    if !(*file).ignore_unreachables && reloc_type(text_reloc) == R_X86_64_PC32 {
        WARN_INSN(
            insn,
            b"ignoring unreachables due to jump table quirk\0".as_ptr() as *const c_char,
        );
        (*file).ignore_unreachables = true;
    }

    *table_size = 0;
    rodata_reloc
}

#[no_mangle]
pub unsafe extern "C" fn arch_cpu_feature_name(feature_number: c_int) -> *const c_char {
    if feature_number >= 0 && (feature_number as usize) < cpu_feature_names.len() {
        cpu_feature_names[feature_number as usize]
    } else {
        core::ptr::null()
    }
}
