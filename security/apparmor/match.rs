// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor dfa based regular expression matching engine
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2012 Canonical Ltd.
 */

// Linux kernel headers would be included here: linux/errno.h, linux/kernel.h, etc.
// Custom headers: include/lib.h, include/match.h

// External types and constants (from include/match.h and other headers)
// These would be defined in dependency crates or header translations

// Type aliases for external types that need to be declared
#[repr(C)]
pub struct TableHeader {
    pub td_id: u16,
    pub td_flags: u16,
    pub td_lolen: u32,
    pub td_data: [u8; 0],
}

#[repr(C)]
pub struct TableSetHeader {
    // Fields from the binary format
}

#[repr(C)]
pub struct AaDfa {
    pub count: KRef,
    pub flags: u16,
    pub max_oob: u32,
    pub tables: [*mut TableHeader; 6],
}

#[repr(C)]
pub struct KRef {
    // Reference count structure
}

#[repr(C)]
pub struct MatchWorkbuf {
    pub pos: usize,
    pub len: usize,
    pub history: [u32; 256],
}

pub type AaState = u32;

// Constants
const EPROTO: i32 = 71;
const ENOMEM: i32 = 12;
const YYTD_ID_MAX: u16 = 255;
const YYTD_DATA8: u16 = 1;
const YYTD_DATA16: u16 = 2;
const YYTD_DATA32: u16 = 4;
const YYTD_ID_ACCEPT: usize = 0;
const YYTD_ID_ACCEPT2: usize = 1;
const YYTD_ID_BASE: usize = 2;
const YYTD_ID_DEF: usize = 3;
const YYTD_ID_NXT: usize = 4;
const YYTD_ID_CHK: usize = 5;
const YYTD_ID_EC: usize = 6;
const YYTH_MAGIC: u32 = 0x1B1F;
const YYTH_FLAG_DIFF_ENCODE: u16 = 0x1;
const YYTH_FLAG_OOB_TRANS: u16 = 0x2;
const YYTH_FLAGS: u16 = YYTH_FLAG_DIFF_ENCODE | YYTH_FLAG_OOB_TRANS;
const MATCH_FLAGS_INVALID: u32 = 0xFF000000;
const MATCH_FLAG_DIFF_ENCODE: u32 = 0x80000000;
const MATCH_FLAG_OOB_TRANSITION: u32 = 0x40000000;
const MARK_DIFF_ENCODE: u32 = 0x20000000;
const MARK_DIFF_ENCODE_VERIFIED: u32 = 0x10000000;
const DFA_NOMATCH: u32 = 0;
const DFA_FLAG_VERIFY_STATES: i32 = 0x1;
const WB_HISTORY_SIZE: usize = 256;

const fn base_idx(x: u32) -> u32 {
    x & 0xffffff
}

// Forward declarations of external functions and macros
extern "C" {
    fn get_unaligned_be16(p: *const u8) -> u16;
    fn get_unaligned_be32(p: *const u8) -> u32;
    fn kvzalloc(size: usize, flags: i32) -> *mut u8;
    fn kvfree(addr: *mut u8);
    fn kzalloc_obj() -> *mut AaDfa;
    fn kref_init(kref: *mut KRef);
    fn memcpy(dest: *mut u8, src: *const u8, n: usize);
    fn is_vmalloc_addr(addr: *const u8) -> bool;
    fn vm_unmap_aliases();
    fn pr_err(fmt: *const u8, ...);
}

const GFP_KERNEL: i32 = 0;

// Helper macros/functions
fn is_err_ptr(ptr: *const u8) -> bool {
    (ptr as usize) > (usize::MAX - 4096)
}

fn ptr_err(ptr: *const u8) -> i32 {
    -(ptr as i32)
}

fn err_ptr(err: i32) -> *mut u8 {
    (-(err as usize)) as *mut u8
}

fn table_size(lolen: u32, flags: u16) -> usize {
    let entry_size = match flags {
        YYTD_DATA8 => 1,
        YYTD_DATA16 => 2,
        YYTD_DATA32 => 4,
        _ => 0,
    };
    std::mem::size_of::<TableHeader>() + (lolen as usize) * entry_size
}

// Placeholder for UNPACK_ARRAY macro
unsafe fn unpack_array_u16(dest: *mut u8, src: *const u8, lolen: u32) {
    let dest_u16 = dest as *mut u16;
    for i in 0..lolen {
        *dest_u16.add(i as usize) = get_unaligned_be16(src.add((i * 2) as usize) as *const u8);
    }
}

unsafe fn unpack_array_u32(dest: *mut u8, src: *const u8, lolen: u32) {
    let dest_u32 = dest as *mut u32;
    for i in 0..lolen {
        *dest_u32.add(i as usize) = get_unaligned_be32(src.add((i * 4) as usize) as *const u8);
    }
}

fn accept1_flags(flags: i32) -> bool {
    (flags & 0x1) != 0
}

fn accept2_flags(flags: i32) -> bool {
    (flags & 0x2) != 0
}

unsafe fn default_table(dfa: *const AaDfa) -> *const u32 {
    let table = (*dfa).tables[YYTD_ID_DEF];
    ((*table).td_data.as_ptr() as *const u32)
}

unsafe fn base_table(dfa: *const AaDfa) -> *const u32 {
    let table = (*dfa).tables[YYTD_ID_BASE];
    ((*table).td_data.as_ptr() as *const u32)
}

unsafe fn next_table(dfa: *const AaDfa) -> *const u32 {
    let table = (*dfa).tables[YYTD_ID_NXT];
    ((*table).td_data.as_ptr() as *const u32)
}

unsafe fn check_table(dfa: *const AaDfa) -> *const u32 {
    let table = (*dfa).tables[YYTD_ID_CHK];
    ((*table).td_data.as_ptr() as *const u32)
}

unsafe fn accept_table(dfa: *const AaDfa) -> *const u32 {
    let table = (*dfa).tables[YYTD_ID_ACCEPT];
    ((*table).td_data.as_ptr() as *const u32)
}

unsafe fn equiv_table(dfa: *const AaDfa) -> *const u8 {
    let table = (*dfa).tables[YYTD_ID_EC];
    (*table).td_data.as_ptr()
}

unsafe fn table_datau16(table: *mut TableHeader) -> *mut u16 {
    (*table).td_data.as_mut_ptr() as *mut u16
}

unsafe fn table_datau32(table: *mut TableHeader) -> *mut u32 {
    (*table).td_data.as_mut_ptr() as *mut u32
}

macro_rules! match_char {
    ($state:expr, $def:expr, $base:expr, $next:expr, $check:expr, $c:expr) => {
        loop {
            let b = *$base.add($state as usize);
            let pos = (base_idx(b) + $c as u32) as usize;
            if *$check.add(pos) != $state {
                $state = *$def.add($state as usize);
                if (b & MATCH_FLAG_DIFF_ENCODE) != 0 {
                    continue;
                }
                break;
            }
            $state = *$next.add(pos);
            break;
        }
    };
}

/**
 * unpack_table - unpack a dfa table (one of accept, default, base, next check)
 * @blob: data to unpack (NOT NULL)
 * @bsize: size of blob
 *
 * Returns: pointer to table else ERR_PTR on failure
 *
 * NOTE: must be freed by kvfree (not kfree)
 */
unsafe fn unpack_table(blob: *const u8, bsize: usize) -> *mut TableHeader {
    let mut table: *mut TableHeader = err_ptr(-EPROTO);
    let mut th: TableHeader;
    let mut tsize: usize;

    if bsize < std::mem::size_of::<TableHeader>() {
        return table;
    }

    th.td_id = get_unaligned_be16(blob).wrapping_sub(1);
    if th.td_id > YYTD_ID_MAX {
        return table;
    }
    th.td_flags = get_unaligned_be16(blob.add(2));
    th.td_lolen = get_unaligned_be32(blob.add(8));
    let blob_data = blob.add(std::mem::size_of::<TableHeader>());

    if !(th.td_flags == YYTD_DATA16 || th.td_flags == YYTD_DATA32 || th.td_flags == YYTD_DATA8) {
        return table;
    }

    if th.td_lolen == 0 {
        return table;
    }
    tsize = table_size(th.td_lolen, th.td_flags);
    if bsize < tsize {
        return table;
    }

    table = kvzalloc(tsize, GFP_KERNEL) as *mut TableHeader;
    if !table.is_null() {
        (*table).td_id = th.td_id;
        (*table).td_flags = th.td_flags;
        (*table).td_lolen = th.td_lolen;
        if th.td_flags == YYTD_DATA8 {
            memcpy((*table).td_data.as_mut_ptr(), blob_data, th.td_lolen as usize);
        } else if th.td_flags == YYTD_DATA16 {
            unpack_array_u16((*table).td_data.as_mut_ptr(), blob_data, th.td_lolen);
        } else if th.td_flags == YYTD_DATA32 {
            unpack_array_u32((*table).td_data.as_mut_ptr(), blob_data, th.td_lolen);
        } else {
            kvfree(table as *mut u8);
            table = err_ptr(-EPROTO);
            return table;
        }
        if is_vmalloc_addr(table as *const u8) {
            vm_unmap_aliases();
        }
    } else {
        table = err_ptr(-ENOMEM);
    }

    table
}

/**
 * verify_table_headers - verify that the tables headers are as expected
 * @tables: array of dfa tables to check (NOT NULL)
 * @flags: flags controlling what type of accept table are acceptable
 *
 * Assumes dfa has gone through the first pass verification done by unpacking
 * NOTE: this does not valid accept table values
 *
 * Returns: %0 else error code on failure to verify
 */
unsafe fn verify_table_headers(tables: *const *mut TableHeader, flags: i32) -> i32 {
    let mut state_count: usize;
    let mut trans_count: usize;
    let mut error = -EPROTO;

    if (*tables.add(YYTD_ID_DEF)).is_null()
        || (*tables.add(YYTD_ID_BASE)).is_null()
        || (*tables.add(YYTD_ID_NXT)).is_null()
        || (*tables.add(YYTD_ID_CHK)).is_null()
    {
        return error;
    }

    state_count = (*(*tables.add(YYTD_ID_BASE))).td_lolen as usize;
    if accept1_flags(flags) {
        if (*tables.add(YYTD_ID_ACCEPT)).is_null() {
            return error;
        }
        if state_count != (*(*tables.add(YYTD_ID_ACCEPT))).td_lolen as usize {
            return error;
        }
    }
    if accept2_flags(flags) {
        if (*tables.add(YYTD_ID_ACCEPT2)).is_null() {
            return error;
        }
        if state_count != (*(*tables.add(YYTD_ID_ACCEPT2))).td_lolen as usize {
            return error;
        }
    }
    if state_count != (*(*tables.add(YYTD_ID_DEF))).td_lolen as usize {
        return error;
    }

    trans_count = (*(*tables.add(YYTD_ID_NXT))).td_lolen as usize;
    if trans_count != (*(*tables.add(YYTD_ID_CHK))).td_lolen as usize {
        return error;
    }

    if !(*tables.add(YYTD_ID_EC)).is_null()
        && (*(*tables.add(YYTD_ID_EC))).td_lolen != 256
    {
        return error;
    }

    error = 0;
    error
}

/**
 * verify_dfa - verify that transitions and states in the tables are in bounds.
 * @dfa: dfa to test  (NOT NULL)
 *
 * Assumes dfa has gone through the first pass verification done by unpacking
 * NOTE: this does not valid accept table values
 *
 * Returns: %0 else error code on failure to verify
 */
unsafe fn verify_dfa(dfa: *const AaDfa) -> i32 {
    let mut i: usize;
    let mut state_count: usize;
    let mut trans_count: usize;
    let mut error = -EPROTO;

    state_count = (*(*(*dfa).tables[YYTD_ID_BASE])).td_lolen as usize;
    trans_count = (*(*(*dfa).tables[YYTD_ID_NXT])).td_lolen as usize;
    if state_count < 2 {
        return error;
    }

    let def_table = default_table(dfa);
    let base_table_ptr = base_table(dfa);
    let next_table_ptr = next_table(dfa);
    let check_table_ptr = check_table(dfa);

    i = 0;
    while i < state_count {
        if *def_table.add(i) >= state_count as u32 {
            pr_err(b"AppArmor DFA default state out of bounds\0".as_ptr());
            return error;
        }
        if (*base_table_ptr.add(i) & MATCH_FLAGS_INVALID) != 0 {
            pr_err(b"AppArmor DFA state with invalid match flags\0".as_ptr());
            return error;
        }
        if (*base_table_ptr.add(i) & MATCH_FLAG_DIFF_ENCODE) != 0 {
            if ((*dfa).flags & YYTH_FLAG_DIFF_ENCODE as u16) == 0 {
                pr_err(b"AppArmor DFA diff encoded transition state without header flag\0".as_ptr());
                return error;
            }
        }
        if (*base_table_ptr.add(i) & MATCH_FLAG_OOB_TRANSITION) != 0 {
            if base_idx(*base_table_ptr.add(i)) < (*dfa).max_oob {
                pr_err(b"AppArmor DFA out of bad transition out of range\0".as_ptr());
                return error;
            }
            if ((*dfa).flags & YYTH_FLAG_OOB_TRANS as u16) == 0 {
                pr_err(b"AppArmor DFA out of bad transition state without header flag\0".as_ptr());
                return error;
            }
        }
        if base_idx(*base_table_ptr.add(i)) + 255 >= trans_count as u32 {
            pr_err(b"AppArmor DFA next/check upper bounds error\n\0".as_ptr());
            return error;
        }
        i += 1;
    }

    i = 0;
    while i < trans_count {
        if *next_table_ptr.add(i) >= state_count as u32 {
            return error;
        }
        if *check_table_ptr.add(i) >= state_count as u32 {
            return error;
        }
        i += 1;
    }

    i = 0;
    while i < state_count {
        let mut j: usize;
        let mut k: usize;

        j = i;
        loop {
            if (*base_table_ptr.add(j) & MATCH_FLAG_DIFF_ENCODE) == 0
                || (*base_table_ptr.add(j) & MARK_DIFF_ENCODE_VERIFIED) != 0
            {
                break;
            }
            if (*base_table_ptr.add(j) & MARK_DIFF_ENCODE) != 0 {
                return error;
            }
            k = *def_table.add(j) as usize;
            if j == k {
                return error;
            }
            *base_table_ptr.add(j) |= MARK_DIFF_ENCODE;
            j = k;
        }

        j = i;
        loop {
            if (*base_table_ptr.add(j) & MATCH_FLAG_DIFF_ENCODE) == 0 {
                break;
            }
            k = *def_table.add(j) as usize;
            if j < i {
                break;
            }
            *base_table_ptr.add(j) &= !MARK_DIFF_ENCODE;
            *base_table_ptr.add(j) |= MARK_DIFF_ENCODE_VERIFIED;
            j = k;
        }
        i += 1;
    }

    error = 0;
    error
}

/**
 * dfa_free - free a dfa allocated by aa_dfa_unpack
 * @dfa: the dfa to free  (MAYBE NULL)
 *
 * Requires: reference count to dfa == 0
 */
unsafe fn dfa_free(dfa: *mut AaDfa) {
    if !dfa.is_null() {
        for i in 0..(*dfa).tables.len() {
            kvfree((*dfa).tables[i] as *mut u8);
            (*dfa).tables[i] = std::ptr::null_mut();
        }
        // kfree(dfa) would go here but we're using Rust allocation
    }
}

/**
 * aa_dfa_free_kref - free aa_dfa by kref (called by aa_put_dfa)
 * @kref: kref callback for freeing of a dfa  (NOT NULL)
 */
pub unsafe extern "C" fn aa_dfa_free_kref(kref: *mut KRef) {
    let dfa = (kref as *mut u8 as *mut AaDfa)
        .offset(-(std::mem::offset_of!(AaDfa, count) as isize));
    dfa_free(dfa);
}

/**
 * remap_data16_to_data32 - remap u16 @old table to a u32 based table
 * @old: table to remap
 *
 * Returns: new table with u32 entries instead of u16.
 *
 * Note: will free @old so caller does not have to
 */
unsafe fn remap_data16_to_data32(old: *mut TableHeader) -> *mut TableHeader {
    let mut new: *mut TableHeader;
    let mut tsize: usize;
    let mut i: u32;

    tsize = table_size((*old).td_lolen, YYTD_DATA32);
    new = kvzalloc(tsize, GFP_KERNEL) as *mut TableHeader;
    if new.is_null() {
        kvfree(old as *mut u8);
        return std::ptr::null_mut();
    }
    (*new).td_id = (*old).td_id;
    (*new).td_flags = YYTD_DATA32;
    (*new).td_lolen = (*old).td_lolen;

    i = 0;
    while i < (*old).td_lolen {
        *table_datau32(new).add(i as usize) =
            *table_datau16(old).add(i as usize) as u32;
        i += 1;
    }

    kvfree(old as *mut u8);
    if is_vmalloc_addr(new as *const u8) {
        vm_unmap_aliases();
    }

    new
}

/**
 * aa_dfa_unpack - unpack the binary tables of a serialized dfa
 * @blob: aligned serialized stream of data to unpack  (NOT NULL)
 * @size: size of data to unpack
 * @flags: flags controlling what type of accept tables are acceptable
 *
 * Unpack a dfa that has been serialized.  To find information on the dfa
 * format look in Documentation/admin-guide/LSM/apparmor.rst
 * Assumes the dfa @blob stream has been aligned on a 8 byte boundary
 *
 * Returns: an unpacked dfa ready for matching or ERR_PTR on failure
 */
pub unsafe fn aa_dfa_unpack(blob: *const u8, size: usize, flags: i32) -> *mut AaDfa {
    let mut hsize: usize;
    let mut error = -ENOMEM;
    let mut data = blob;
    let mut table: *mut TableHeader = std::ptr::null_mut();
    let mut dfa = kzalloc_obj();
    if dfa.is_null() {
        return err_ptr(error);
    }

    kref_init(&mut (*dfa).count);

    error = -EPROTO;

    if size < std::mem::size_of::<TableSetHeader>() {
        dfa_free(dfa);
        return err_ptr(error);
    }

    if get_unaligned_be32(data) != YYTH_MAGIC {
        dfa_free(dfa);
        return err_ptr(error);
    }

    hsize = get_unaligned_be32(data.add(4)) as usize;
    if size < hsize {
        dfa_free(dfa);
        return err_ptr(error);
    }

    (*dfa).flags = get_unaligned_be16(data.add(12));
    if ((*dfa).flags as u16 & !YYTH_FLAGS) != 0 {
        dfa_free(dfa);
        return err_ptr(error);
    }

    (*dfa).max_oob = 1;

    data = data.add(hsize);
    let mut remaining_size = size - hsize;

    while remaining_size > 0 {
        table = unpack_table(data, remaining_size);
        if is_err_ptr(table as *const u8) {
            error = ptr_err(table as *const u8);
            table = std::ptr::null_mut();
            dfa_free(dfa);
            return err_ptr(error);
        }

        match (*table).td_id {
            0 => {
                if ((*table).td_flags as i32 & accept1_flags(flags) as i32) == 0 {
                    kvfree(table as *mut u8);
                    dfa_free(dfa);
                    return err_ptr(-EPROTO);
                }
            }
            1 => {
                if ((*table).td_flags as i32 & accept2_flags(flags) as i32) == 0 {
                    kvfree(table as *mut u8);
                    dfa_free(dfa);
                    return err_ptr(-EPROTO);
                }
            }
            2 => {
                if (*table).td_flags != YYTD_DATA32 {
                    kvfree(table as *mut u8);
                    dfa_free(dfa);
                    return err_ptr(-EPROTO);
                }
            }
            3 | 4 | 5 => {
                if !((*table).td_flags == YYTD_DATA16 || (*table).td_flags == YYTD_DATA32) {
                    kvfree(table as *mut u8);
                    dfa_free(dfa);
                    return err_ptr(-EPROTO);
                }
            }
            6 => {
                if (*table).td_flags != YYTD_DATA8 {
                    kvfree(table as *mut u8);
                    dfa_free(dfa);
                    return err_ptr(-EPROTO);
                }
            }
            _ => {
                kvfree(table as *mut u8);
                dfa_free(dfa);
                return err_ptr(-EPROTO);
            }
        }

        if !(*dfa).tables[(*table).td_id as usize].is_null() {
            kvfree(table as *mut u8);
            dfa_free(dfa);
            return err_ptr(-EPROTO);
        }
        (*dfa).tables[(*table).td_id as usize] = table;
        let tsize = table_size((*table).td_lolen, (*table).td_flags);
        data = data.add(tsize);
        remaining_size -= tsize;

        match (*table).td_id {
            3 | 4 | 5 => {
                if (*table).td_flags == YYTD_DATA16 {
                    table = remap_data16_to_data32(table);
                    if table.is_null() {
                        dfa_free(dfa);
                        return err_ptr(-ENOMEM);
                    }
                }
                (*dfa).tables[(*table).td_id as usize] = table;
            }
            _ => {}
        }
        table = std::ptr::null_mut();
    }

    error = verify_table_headers((*dfa).tables.as_ptr(), flags);
    if error != 0 {
        dfa_free(dfa);
        return err_ptr(error);
    }

    if (flags & DFA_FLAG_VERIFY_STATES) != 0 {
        error = verify_dfa(dfa);
        if error != 0 {
            dfa_free(dfa);
            return err_ptr(error);
        }
    }

    dfa
}

/**
 * aa_dfa_match_len - traverse @dfa to find state @str stops at
 * @dfa: the dfa to match @str against  (NOT NULL)
 * @start: the state of the dfa to start matching in
 * @str: the string of bytes to match against the dfa  (NOT NULL)
 * @len: length of the string of bytes to match
 *
 * aa_dfa_match_len will match @str against the dfa and return the state it
 * finished matching in. The final state can be used to look up the accepting
 * label, or as the start state of a continuing match.
 *
 * This function will happily match again the 0 byte and only finishes
 * when @len input is consumed.
 *
 * Returns: final state reached after input is consumed
 */
pub unsafe fn aa_dfa_match_len(
    dfa: *const AaDfa,
    start: AaState,
    str: *const u8,
    len: i32,
) -> AaState {
    let def = default_table(dfa);
    let base = base_table(dfa);
    let next = next_table(dfa);
    let check = check_table(dfa);
    let mut state = start;
    let mut remaining = len;

    if state == DFA_NOMATCH {
        return DFA_NOMATCH;
    }

    let mut str_ptr = str;
    if !(*dfa).tables[YYTD_ID_EC].is_null() {
        let equiv = equiv_table(dfa);
        while remaining > 0 {
            let c = *equiv.add(*str_ptr as usize);
            match_char!(state, def, base, next, check, c);
            str_ptr = str_ptr.add(1);
            remaining -= 1;
        }
    } else {
        while remaining > 0 {
            match_char!(state, def, base, next, check, *str_ptr);
            str_ptr = str_ptr.add(1);
            remaining -= 1;
        }
    }

    state
}

/**
 * aa_dfa_match - traverse @dfa to find state @str stops at
 * @dfa: the dfa to match @str against  (NOT NULL)
 * @start: the state of the dfa to start matching in
 * @str: the null terminated string of bytes to match against the dfa (NOT NULL)
 *
 * aa_dfa_match will match @str against the dfa and return the state it
 * finished matching in. The final state can be used to look up the accepting
 * label, or as the start state of a continuing match.
 *
 * Returns: final state reached after input is consumed
 */
pub unsafe fn aa_dfa_match(
    dfa: *const AaDfa,
    start: AaState,
    str: *const u8,
) -> AaState {
    let def = default_table(dfa);
    let base = base_table(dfa);
    let next = next_table(dfa);
    let check = check_table(dfa);
    let mut state = start;

    if state == DFA_NOMATCH {
        return DFA_NOMATCH;
    }

    let mut str_ptr = str;
    if !(*dfa).tables[YYTD_ID_EC].is_null() {
        let equiv = equiv_table(dfa);
        while *str_ptr != 0 {
            let c = *equiv.add(*str_ptr as usize);
            match_char!(state, def, base, next, check, c);
            str_ptr = str_ptr.add(1);
        }
    } else {
        while *str_ptr != 0 {
            match_char!(state, def, base, next, check, *str_ptr);
            str_ptr = str_ptr.add(1);
        }
    }

    state
}

/**
 * aa_dfa_next - step one character to the next state in the dfa
 * @dfa: the dfa to traverse (NOT NULL)
 * @state: the state to start in
 * @c: the input character to transition on
 *
 * aa_dfa_match will step through the dfa by one input character @c
 *
 * Returns: state reach after input @c
 */
pub unsafe fn aa_dfa_next(dfa: *const AaDfa, mut state: AaState, c: u8) -> AaState {
    let def = default_table(dfa);
    let base = base_table(dfa);
    let next = next_table(dfa);
    let check = check_table(dfa);

    if !(*dfa).tables[YYTD_ID_EC].is_null() {
        let equiv = equiv_table(dfa);
        match_char!(state, def, base, next, check, *equiv.add(c as usize));
    } else {
        match_char!(state, def, base, next, check, c);
    }

    state
}

pub unsafe fn aa_dfa_outofband_transition(dfa: *const AaDfa, mut state: AaState) -> AaState {
    let def = default_table(dfa);
    let base = base_table(dfa);
    let next = next_table(dfa);
    let check = check_table(dfa);
    let b = *base.add(state as usize);

    if (b & MATCH_FLAG_OOB_TRANSITION) == 0 {
        return DFA_NOMATCH;
    }

    match_char!(state, def, base, next, check, (-1i32) as u8);

    state
}

/**
 * aa_dfa_match_until - traverse @dfa until accept state or end of input
 * @dfa: the dfa to match @str against  (NOT NULL)
 * @start: the state of the dfa to start matching in
 * @str: the null terminated string of bytes to match against the dfa (NOT NULL)
 * @retpos: first character in str after match OR end of string
 *
 * aa_dfa_match will match @str against the dfa and return the state it
 * finished matching in. The final state can be used to look up the accepting
 * label, or as the start state of a continuing match.
 *
 * Returns: final state reached after input is consumed
 */
pub unsafe fn aa_dfa_match_until(
    dfa: *const AaDfa,
    start: AaState,
    str: *const u8,
    retpos: *mut *const u8,
) -> AaState {
    let def = default_table(dfa);
    let base = base_table(dfa);
    let next = next_table(dfa);
    let check = check_table(dfa);
    let accept = accept_table(dfa);
    let mut state = start;
    let mut pos: AaState;
    let mut str_ptr = str;

    if state == DFA_NOMATCH {
        return DFA_NOMATCH;
    }

    if !(*dfa).tables[YYTD_ID_EC].is_null() {
        let equiv = equiv_table(dfa);
        while *str_ptr != 0 {
            pos = base_idx(*base.add(state as usize)) + *equiv.add(*str_ptr as usize) as u32;
            str_ptr = str_ptr.add(1);
            if *check.add(pos as usize) == state {
                state = *next.add(pos as usize);
            } else {
                state = *def.add(state as usize);
            }
            if *accept.add(state as usize) != 0 {
                break;
            }
        }
    } else {
        while *str_ptr != 0 {
            pos = base_idx(*base.add(state as usize)) + *str_ptr as u32;
            str_ptr = str_ptr.add(1);
            if *check.add(pos as usize) == state {
                state = *next.add(pos as usize);
            } else {
                state = *def.add(state as usize);
            }
            if *accept.add(state as usize) != 0 {
                break;
            }
        }
    }

    *retpos = str_ptr;
    state
}

/**
 * aa_dfa_matchn_until - traverse @dfa until accept or @n bytes consumed
 * @dfa: the dfa to match @str against  (NOT NULL)
 * @start: the state of the dfa to start matching in
 * @str: the string of bytes to match against the dfa  (NOT NULL)
 * @n: length of the string of bytes to match
 * @retpos: first character in str after match OR str + n
 *
 * aa_dfa_match_len will match @str against the dfa and return the state it
 * finished matching in. The final state can be used to look up the accepting
 * label, or as the start state of a continuing match.
 *
 * This function will happily match again the 0 byte and only finishes
 * when @n input is consumed.
 *
 * Returns: final state reached after input is consumed
 */
pub unsafe fn aa_dfa_matchn_until(
    dfa: *const AaDfa,
    start: AaState,
    str: *const u8,
    n: i32,
    retpos: *mut *const u8,
) -> AaState {
    let def = default_table(dfa);
    let base = base_table(dfa);
    let next = next_table(dfa);
    let check = check_table(dfa);
    let accept = accept_table(dfa);
    let mut state = start;
    let mut pos: AaState;
    let mut str_ptr = str;
    let mut remaining = n;

    *retpos = std::ptr::null();
    if state == DFA_NOMATCH {
        return DFA_NOMATCH;
    }

    if !(*dfa).tables[YYTD_ID_EC].is_null() {
        let equiv = equiv_table(dfa);
        while remaining > 0 {
            pos = base_idx(*base.add(state as usize)) + *equiv.add(*str_ptr as usize) as u32;
            str_ptr = str_ptr.add(1);
            if *check.add(pos as usize) == state {
                state = *next.add(pos as usize);
            } else {
                state = *def.add(state as usize);
            }
            if *accept.add(state as usize) != 0 {
                break;
            }
            remaining -= 1;
        }
    } else {
        while remaining > 0 {
            pos = base_idx(*base.add(state as usize)) + *str_ptr as u32;
            str_ptr = str_ptr.add(1);
            if *check.add(pos as usize) == state {
                state = *next.add(pos as usize);
            } else {
                state = *def.add(state as usize);
            }
            if *accept.add(state as usize) != 0 {
                break;
            }
            remaining -= 1;
        }
    }

    *retpos = str_ptr;
    state
}

fn inc_wb_pos(wb: *mut MatchWorkbuf) {
    unsafe {
        (*wb).pos = ((*wb).pos + 1) & (WB_HISTORY_SIZE - 1);
        (*wb).len = if ((*wb).len + 1) > WB_HISTORY_SIZE {
            WB_HISTORY_SIZE
        } else {
            (*wb).len + 1
        };
    }
}

unsafe fn is_loop(wb: *mut MatchWorkbuf, state: AaState, adjust: *mut u32) -> bool {
    let mut pos = (*wb).pos;
    let mut i: i32;

    if (*wb).history[pos] < state {
        return false;
    }

    i = 0;
    while i < (*wb).len as i32 {
        if (*wb).history[pos] == state {
            *adjust = i as u32;
            return true;
        }
        pos = (pos as i32 - 1) as usize & (WB_HISTORY_SIZE - 1);
        i += 1;
    }

    false
}

unsafe fn leftmatch_fb(
    dfa: *const AaDfa,
    start: AaState,
    str: *const u8,
    wb: *mut MatchWorkbuf,
    count: *mut u32,
) -> AaState {
    let def = default_table(dfa);
    let base = base_table(dfa);
    let next = next_table(dfa);
    let check = check_table(dfa);
    let mut state = start;
    let mut pos: AaState;
    let mut str_ptr = str;

    *count = 0;
    if state == DFA_NOMATCH {
        return DFA_NOMATCH;
    }

    if !(*dfa).tables[YYTD_ID_EC].is_null() {
        let equiv = equiv_table(dfa);
        while *str_ptr != 0 {
            let mut adjust: u32 = 0;

            (*wb).history[(*wb).pos] = state;
            pos = base_idx(*base.add(state as usize)) + *equiv.add(*str_ptr as usize) as u32;
            str_ptr = str_ptr.add(1);
            if *check.add(pos as usize) == state {
                state = *next.add(pos as usize);
            } else {
                state = *def.add(state as usize);
            }
            if is_loop(wb, state, &mut adjust) {
                state = aa_dfa_match(dfa, state, str_ptr);
                *count = (*count).wrapping_sub(adjust);
                goto_out(wb, state, count);
                return state;
            }
            inc_wb_pos(wb);
            *count = (*count).wrapping_add(1);
        }
    } else {
        while *str_ptr != 0 {
            let mut adjust: u32 = 0;

            (*wb).history[(*wb).pos] = state;
            pos = base_idx(*base.add(state as usize)) + *str_ptr as u32;
            str_ptr = str_ptr.add(1);
            if *check.add(pos as usize) == state {
                state = *next.add(pos as usize);
            } else {
                state = *def.add(state as usize);
            }
            if is_loop(wb, state, &mut adjust) {
                state = aa_dfa_match(dfa, state, str_ptr);
                *count = (*count).wrapping_sub(adjust);
                goto_out(wb, state, count);
                return state;
            }
            inc_wb_pos(wb);
            *count = (*count).wrapping_add(1);
        }
    }

    goto_out(wb, state, count);
    state
}

unsafe fn goto_out(wb: *mut MatchWorkbuf, state: AaState, count: *mut u32) {
    if state == 0 {
        *count = 0;
    }
}

/**
 * aa_dfa_leftmatch - traverse @dfa to find state @str stops at
 * @dfa: the dfa to match @str against  (NOT NULL)
 * @start: the state of the dfa to start matching in
 * @str: the null terminated string of bytes to match against the dfa (NOT NULL)
 * @count: current count of longest left.
 *
 * aa_dfa_match will match @str against the dfa and return the state it
 * finished matching in. The final state can be used to look up the accepting
 * label, or as the start state of a continuing match.
 *
 * Returns: final state reached after input is consumed
 */
pub unsafe fn aa_dfa_leftmatch(
    dfa: *const AaDfa,
    start: AaState,
    str: *const u8,
    count: *mut u32,
) -> AaState {
    let mut wb: MatchWorkbuf = MatchWorkbuf {
        pos: 0,
        len: 0,
        history: [0; 256],
    };

    leftmatch_fb(dfa, start, str, &mut wb, count)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
