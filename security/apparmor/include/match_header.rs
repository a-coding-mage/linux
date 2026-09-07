// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor policy dfa matching engine definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2012 Canonical Ltd.

// Dependency: linux/kref.h (kref struct)

pub const DFA_NOMATCH: u32 = 0;
pub const DFA_START: u32 = 1;

// The format used for transition tables is based on the GNU flex table
// file format (--tables-file option; see Table File Format in the flex
// info pages and the flex sources for documentation). The magic number
// used in the header is 0x1B5E783D instead of 0xF13C57B1 though, because
// new tables have been defined and others YY_ID_CHK (check) and YY_ID_DEF
// (default) tables are used slightly differently (see the apparmor-parser
// package).
//
//
// The data in the packed dfa is stored in network byte order, and the tables
// are arranged for flexibility.  We convert the table data to host native
// byte order.
//
// The dfa begins with a table set header, and is followed by the actual
// tables.

pub const YYTH_MAGIC: u32 = 0x1B5E783D;
pub const YYTH_FLAG_DIFF_ENCODE: u16 = 1;
pub const YYTH_FLAG_OOB_TRANS: u16 = 2;
pub const YYTH_FLAGS: u16 = YYTH_FLAG_DIFF_ENCODE | YYTH_FLAG_OOB_TRANS;

pub const MAX_OOB_SUPPORTED: u32 = 1;

#[repr(C)]
pub struct table_set_header {
    pub th_magic: u32,       // YYTH_MAGIC
    pub th_hsize: u32,
    pub th_ssize: u32,
    pub th_flags: u16,
    pub th_version: [u8; 0], // Flexible array member (zero-sized array)
}

// The YYTD_ID are one less than flex table mappings.  The flex id
// has 1 subtracted at table load time, this allows us to directly use the
// ID's as indexes.
pub const YYTD_ID_ACCEPT: u32 = 0;
pub const YYTD_ID_BASE: u32 = 1;
pub const YYTD_ID_CHK: u32 = 2;
pub const YYTD_ID_DEF: u32 = 3;
pub const YYTD_ID_EC: u32 = 4;
pub const YYTD_ID_META: u32 = 5;
pub const YYTD_ID_ACCEPT2: u32 = 6;
pub const YYTD_ID_NXT: u32 = 7;
pub const YYTD_ID_TSIZE: u32 = 8;
pub const YYTD_ID_MAX: u32 = 8;

pub const YYTD_DATA8: u32 = 1;
pub const YYTD_DATA16: u32 = 2;
pub const YYTD_DATA32: u32 = 4;
pub const YYTD_DATA64: u32 = 8;

// ACCEPT & ACCEPT2 tables gets 6 dedicated flags, YYTD_DATAX define the
// first flags
#[inline]
pub fn ACCEPT1_FLAGS(x: u32) -> u32 {
    x & 0x3f
}

#[inline]
pub fn ACCEPT2_FLAGS(x: u32) -> u32 {
    ACCEPT1_FLAGS(x >> YYTD_ID_ACCEPT2)
}

#[inline]
pub fn TO_ACCEPT1_FLAG(x: u32) -> u32 {
    ACCEPT1_FLAGS(x)
}

#[inline]
pub fn TO_ACCEPT2_FLAG(x: u32) -> u32 {
    ACCEPT1_FLAGS(x) << YYTD_ID_ACCEPT2
}

pub const DFA_FLAG_VERIFY_STATES: u32 = 0x1000;

#[repr(C)]
pub struct table_header {
    pub td_id: u16,
    pub td_flags: u16,
    pub td_hilen: u32,
    pub td_lolen: u32,
    pub td_data: [u8; 0], // Flexible array member (zero-sized array)
}

// Helper macros converted to inline functions
#[inline]
pub unsafe fn TABLE_DATAU16(table: *const table_header) -> *const u16 {
    (*table).td_data.as_ptr() as *const u16
}

#[inline]
pub unsafe fn TABLE_DATAU32(table: *const table_header) -> *const u32 {
    (*table).td_data.as_ptr() as *const u32
}

#[inline]
pub unsafe fn DEFAULT_TABLE(dfa: *const aa_dfa) -> *const u32 {
    ((*dfa).tables[YYTD_ID_DEF as usize] as *const table_header)
        .as_ref()
        .map(|t| t.td_data.as_ptr() as *const u32)
        .unwrap_or(std::ptr::null())
}

#[inline]
pub unsafe fn BASE_TABLE(dfa: *const aa_dfa) -> *const u32 {
    ((*dfa).tables[YYTD_ID_BASE as usize] as *const table_header)
        .as_ref()
        .map(|t| t.td_data.as_ptr() as *const u32)
        .unwrap_or(std::ptr::null())
}

#[inline]
pub unsafe fn NEXT_TABLE(dfa: *const aa_dfa) -> *const u32 {
    ((*dfa).tables[YYTD_ID_NXT as usize] as *const table_header)
        .as_ref()
        .map(|t| t.td_data.as_ptr() as *const u32)
        .unwrap_or(std::ptr::null())
}

#[inline]
pub unsafe fn CHECK_TABLE(dfa: *const aa_dfa) -> *const u32 {
    ((*dfa).tables[YYTD_ID_CHK as usize] as *const table_header)
        .as_ref()
        .map(|t| t.td_data.as_ptr() as *const u32)
        .unwrap_or(std::ptr::null())
}

#[inline]
pub unsafe fn EQUIV_TABLE(dfa: *const aa_dfa) -> *const u8 {
    ((*dfa).tables[YYTD_ID_EC as usize] as *const table_header)
        .as_ref()
        .map(|t| t.td_data.as_ptr() as *const u8)
        .unwrap_or(std::ptr::null())
}

#[inline]
pub unsafe fn ACCEPT_TABLE(dfa: *const aa_dfa) -> *const u32 {
    ((*dfa).tables[YYTD_ID_ACCEPT as usize] as *const table_header)
        .as_ref()
        .map(|t| t.td_data.as_ptr() as *const u32)
        .unwrap_or(std::ptr::null())
}

#[inline]
pub unsafe fn ACCEPT_TABLE2(dfa: *const aa_dfa) -> *const u32 {
    ((*dfa).tables[YYTD_ID_ACCEPT2 as usize] as *const table_header)
        .as_ref()
        .map(|t| t.td_data.as_ptr() as *const u32)
        .unwrap_or(std::ptr::null())
}

// Dependency: struct kref from linux/kref.h
#[repr(C)]
pub struct aa_dfa {
    pub count: std::ffi::c_void, // kref (external dependency)
    pub flags: u16,
    pub max_oob: u32,
    pub tables: [*mut table_header; YYTD_ID_TSIZE as usize],
}

// UNPACK_ARRAY macro - complex C macro with conditional compilation
#[macro_export]
macro_rules! UNPACK_ARRAY {
    ($table:expr, $blob:expr, $len:expr, $ttype:ty, $btype:ty, $ntohx:expr) => {{
        let __t: *mut $ttype = $table as *mut $ttype;
        let __b: *const $btype = $blob as *const $btype;
        debug_assert_eq!(std::mem::size_of::<$ttype>(), std::mem::size_of::<$btype>());
        #[cfg(target_endian = "big")]
        unsafe { std::ptr::copy_nonoverlapping(__b, __t, $len); }
        #[cfg(target_endian = "little")]
        unsafe { for __i in 0..$len { *__t.add(__i) = ($ntohx)(&*__b.add(__i)); } }
    }};
}

// This macro unpacks an array from network byte order (big-endian) to host order
// Parameters: TABLE (target), BLOB (source), LEN (length), TTYPE (target type),
//             BTYPE (blob type), NTOHX (conversion function)
// Behavior: If CONFIG_CPU_BIG_ENDIAN is enabled, does direct memcpy.
//           Otherwise, converts each element using NTOHX from big-endian to native order.
// Note: Direct Rust macro-based translation would require generic specialization.
// The conversion logic should be implemented at call sites with explicit type handling.

#[inline]
pub fn table_size(len: usize, el_size: usize) -> usize {
    const ALIGNMENT: usize = 8;
    let size_without_align = std::mem::size_of::<table_header>() + len * el_size;
    (size_without_align + (ALIGNMENT - 1)) & !(ALIGNMENT - 1)
}

pub type aa_state_t = u32;

// External function declarations
extern "C" {
    pub fn aa_dfa_unpack(blob: *const std::ffi::c_void, size: usize, flags: i32) -> *mut aa_dfa;
    pub fn aa_dfa_match_len(
        dfa: *const aa_dfa,
        start: aa_state_t,
        str_: *const u8,
        len: i32,
    ) -> aa_state_t;
    pub fn aa_dfa_match(dfa: *const aa_dfa, start: aa_state_t, str_: *const u8) -> aa_state_t;
    pub fn aa_dfa_next(dfa: *const aa_dfa, state: aa_state_t, c: u8) -> aa_state_t;
    pub fn aa_dfa_outofband_transition(dfa: *const aa_dfa, state: aa_state_t) -> aa_state_t;
    pub fn aa_dfa_match_until(
        dfa: *const aa_dfa,
        start: aa_state_t,
        str_: *const u8,
        retpos: *mut *const u8,
    ) -> aa_state_t;
    pub fn aa_dfa_matchn_until(
        dfa: *const aa_dfa,
        start: aa_state_t,
        str_: *const u8,
        n: i32,
        retpos: *mut *const u8,
    ) -> aa_state_t;
    pub fn aa_dfa_free_kref(kref: *mut std::ffi::c_void);
}

// This needs to be a power of 2
pub const WB_HISTORY_SIZE: usize = 32;

#[repr(C)]
pub struct match_workbuf {
    pub pos: u32,
    pub len: u32,
    pub history: [aa_state_t; WB_HISTORY_SIZE],
}

// Helper macro to create and initialize a match_workbuf
#[macro_export]
macro_rules! DEFINE_MATCH_WB {
    ($name:ident) => {
        let mut $name = match_workbuf { pos: 0, len: 0, history: [0; WB_HISTORY_SIZE] };
    };
}

// Original: #define DEFINE_MATCH_WB(N) struct match_workbuf N = { .pos = 0, .len = 0, }
// In Rust, this can be expressed as a function that creates the struct
#[inline]
pub fn define_match_wb() -> match_workbuf {
    match_workbuf {
        pos: 0,
        len: 0,
        history: [0; WB_HISTORY_SIZE],
    }
}

extern "C" {
    pub fn aa_dfa_leftmatch(
        dfa: *const aa_dfa,
        start: aa_state_t,
        str_: *const u8,
        count: *mut u32,
    ) -> aa_state_t;
}

// aa_get_dfa - increment refcount on dfa @p
// @dfa: dfa  (MAYBE NULL)
//
// Returns: pointer to @dfa if @dfa is NULL will return NULL
// Requires: @dfa must be held with valid refcount when called
#[inline]
pub unsafe fn aa_get_dfa(dfa: *mut aa_dfa) -> *mut aa_dfa {
    if !dfa.is_null() {
        // kref_get(&(dfa->count)) - external dependency call
        // This would need to be: extern "C" { fn kref_get(kref: *mut kref); }
        extern "C" {
            fn kref_get(kref: *mut std::ffi::c_void);
        }
        kref_get(&mut (*dfa).count);
    }
    dfa
}

// aa_put_dfa - put a dfa refcount
// @dfa: dfa to put refcount   (MAYBE NULL)
//
// Requires: if @dfa != NULL that a valid refcount be held
#[inline]
pub unsafe fn aa_put_dfa(dfa: *mut aa_dfa) {
    if !dfa.is_null() {
        extern "C" {
            fn kref_put(kref: *mut std::ffi::c_void, release: unsafe extern "C" fn(*mut std::ffi::c_void));
        }
        kref_put(&mut (*dfa).count, aa_dfa_free_kref);
    }
}

pub const MATCH_FLAG_DIFF_ENCODE: u32 = 0x80000000;
pub const MARK_DIFF_ENCODE: u32 = 0x40000000;
pub const MATCH_FLAG_OOB_TRANSITION: u32 = 0x20000000;
pub const MARK_DIFF_ENCODE_VERIFIED: u32 = 0x10000000;
pub const MATCH_FLAGS_MASK: u32 = 0xff000000;
pub const MATCH_FLAGS_VALID: u32 = MATCH_FLAG_DIFF_ENCODE | MATCH_FLAG_OOB_TRANSITION;
pub const MATCH_FLAGS_INVALID: u32 = MATCH_FLAGS_MASK & !MATCH_FLAGS_VALID;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
