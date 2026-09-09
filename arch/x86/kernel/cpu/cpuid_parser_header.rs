/* SPDX-License-Identifier: GPL-2.0 */

// Declarations from <asm/cpuid/types.h> are supplied by the surrounding crate.

/*
 * Since accessing the CPUID leaves at `struct cpuid_leaves` requires compile
 * time tokenization, the CPUID parser is split into compile time macros for
 * tokenizing leaf/subleaf output offsets within the table, and generic runtime
 * code to write to the relevant CPUID leaves using such offsets.
 */

/* Compile-time CPUID table offset calculations. */

#[macro_export]
macro_rules! __cpuid_leaves_regs_offset {
    ($leaves:ty, $field:tt) => {
        ::core::mem::offset_of!($leaves, $field)
    };
}

#[macro_export]
macro_rules! __cpuid_leaves_info_offset {
    ($leaves:ty, $field:tt) => {
        ::core::mem::offset_of!($leaves, $field)
    };
}

#[macro_export]
macro_rules! __cpuid_leaves_regs_maxcnt {
    ($leaves:ty, $field:tt) => {
        ::core::mem::size_of_val(&unsafe {
            &(*(core::ptr::null::<$leaves>())).$field
        }) / ::core::mem::size_of::<crate::cpuid_regs>()
    };
}

/* Translation of compile-time offsets to generic runtime pointers. */

#[inline]
pub unsafe fn cpuid_table_regs_p(
    t: *const crate::cpuid_table,
    regs_offset: usize,
) -> *mut crate::cpuid_regs {
    (core::ptr::addr_of!((*t).leaves) as *const u8).add(regs_offset) as *mut crate::cpuid_regs
}

#[inline]
pub unsafe fn cpuid_table_info_p(
    t: *const crate::cpuid_table,
    info_offset: usize,
) -> *mut crate::leaf_parse_info {
    (core::ptr::addr_of!((*t).leaves) as *const u8).add(info_offset) as *mut crate::leaf_parse_info
}

/**
 * struct cpuid_output - Output of a CPUID operation
 * @regs: Pointer to an array of CPUID results.
 * @info: Pointer to query info.
 */
#[repr(C)]
pub struct cpuid_output {
    pub regs: *mut crate::cpuid_regs,
    pub info: *mut crate::leaf_parse_info,
}

/** CPUID parse table entry. */
#[repr(C)]
pub struct cpuid_parse_entry {
    pub leaf: u32,
    pub subleaf: u32,
    pub regs_offs: u32,
    pub info_offs: u32,
    pub maxcnt: u32,
    pub read: Option<unsafe extern "C" fn(*const cpuid_parse_entry, *const cpuid_output)>,
}

/*
 * C token-pasting macros are represented by Rust macro inputs containing the
 * already-resolved field names.  The surrounding CPUID types provide those
 * fields and the reader functions.
 */
#[macro_export]
macro_rules! __CPUID_PARSE_ENTRY {
    ($leaf:expr, $subleaf:expr, $regs_field:tt, $info_field:tt, $maxcnt:expr, $reader:expr) => {
        $crate::cpuid_parse_entry {
            leaf: $leaf,
            subleaf: $subleaf,
            regs_offs: $crate::__cpuid_leaves_regs_offset!($crate::cpuid_leaves, $regs_field) as u32,
            info_offs: $crate::__cpuid_leaves_info_offset!($crate::cpuid_leaves, $info_field) as u32,
            maxcnt: $maxcnt,
            read: Some($reader),
        }
    };
}

#[macro_export]
macro_rules! CPUID_PARSE_ENTRY {
    ($leaf:expr, $subleaf:expr, $regs_field:tt, $info_field:tt, $maxcnt:expr, $reader:expr) => {
        $crate::__CPUID_PARSE_ENTRY!($leaf, $subleaf, $regs_field, $info_field, $maxcnt, $reader)
    };
}

#[macro_export]
macro_rules! CPUID_PARSE_ENTRY_N {
    ($leaf:expr, $first_subleaf:expr, $regs_field:tt, $info_field:tt, $maxcnt:expr, $reader:expr) => {
        $crate::__CPUID_PARSE_ENTRY!($leaf, $first_subleaf, $regs_field, $info_field, $maxcnt, $reader)
    };
}

/* CPUID parser table entries are instantiated by the translation unit. */

#[macro_export]
macro_rules! CPUID_PARSE_ENTRIES {
    ($entry:ident) => {
        $entry!(0x0, 0, generic);
        $entry!(0x1, 0, generic);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
