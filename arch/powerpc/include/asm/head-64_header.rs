/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of powerpc/include/asm/head-64.h.
// The source is assembler-only; the macros below preserve its names and
// expansion intent for the assembler/linker layer.

/// Define an executable fixed-location text section.
#[macro_export]
macro_rules! define_ftsec {
    ($name:ident) => { /* .section ".head.text.$name", "ax", @progbits */ };
}

/// Define a fixed-location data section.
#[macro_export]
macro_rules! define_data_ftsec {
    ($name:ident) => { /* .section ".head.data.$name", "a", @progbits */ };
}

/// Select an executable fixed-location text section.
#[macro_export]
macro_rules! use_ftsec {
    ($name:ident) => { /* .section ".head.text.$name", "ax", @progbits */ };
}

// Fixed sections are opened, populated in source order, and then closed.
// Their linker-script locations and entry placement remain assembler/linker
// responsibilities.  The following macros retain the original operations.

#[macro_export]
macro_rules! open_fixed_section {
    ($sname:ident, $start:expr, $end:expr) => {
        /* sname_start = start; sname_end = end; sname_len = end - start;
           define_ftsec sname; . = 0x0; start_sname: */
    };
}

// CONFIG_LD_HEAD_STUB_CATCH selects whether the linker-stub catch section is
// emitted before .text and whether text_start is offset by 0x100.
#[cfg(feature = "CONFIG_LD_HEAD_STUB_CATCH")]
#[macro_export]
macro_rules! open_text_section {
    ($start:expr) => {
        /* .section ".linker_stub_catch", "ax", @progbits;
           linker_stub_catch:; . = 0x4; text_start = start + 0x100;
           .section ".text", "ax", @progbits; .balign 0x100; start_text: */
    };
}

#[cfg(not(feature = "CONFIG_LD_HEAD_STUB_CATCH"))]
#[macro_export]
macro_rules! open_text_section {
    ($start:expr) => {
        /* text_start = start; .section ".text", "ax", @progbits;
           . = 0x0; start_text: */
    };
}

#[macro_export]
macro_rules! zero_fixed_section {
    ($sname:ident, $start:expr, $end:expr) => {
        /* sname_start = start; sname_end = end; sname_len = end - start;
           define_data_ftsec sname; . = 0x0; . = sname_len; */
    };
}

#[macro_export]
macro_rules! use_fixed_section {
    ($sname:ident) => { /* use_ftsec sname; */ };
}

#[macro_export]
macro_rules! use_text_section {
    () => { /* .text */ };
}

#[macro_export]
macro_rules! close_fixed_section {
    ($sname:ident) => { /* use_fixed_section sname; . = sname_len; end_sname: */ };
}

#[macro_export]
macro_rules! __fixed_section_entry_begin {
    ($sname:ident, $name:ident, $align:expr) => {
        /* use_fixed_section sname; .balign align; .global name; name: */
    };
}

// IFETCH_ALIGN_BYTES is supplied by asm/cache.h.
#[macro_export]
macro_rules! fixed_section_entry_begin {
    ($sname:ident, $name:ident) => {
        $crate::__fixed_section_entry_begin!($sname, $name, IFETCH_ALIGN_BYTES)
    };
}

#[macro_export]
macro_rules! fixed_section_entry_begin_location {
    ($sname:ident, $name:ident, $start:expr, $size:expr) => {
        /* use_fixed_section sname; name_start = start;
           require start % size == 0; require size in {0x20,0x80,0x100,0x1000};
           require start >= sname_start; . = start - sname_start;
           .global name; name: */
    };
}

#[macro_export]
macro_rules! fixed_section_entry_end_location {
    ($sname:ident, $name:ident, $start:expr, $size:expr) => {
        /* require start + size <= sname_end;
           require . - name <= start + size - name_start;
           . = start + size - sname_start; */
    };
}

// Define a symbol as being in a fixed section.
#[macro_export]
macro_rules! define_fixed_symbol {
    ($label:ident, $sname:ident) => {
        /* label_absolute = label - start_sname + sname_start */
    };
}

/// Absolute address of a symbol previously made fixed-section absolute.
#[macro_export]
macro_rules! fixed_symbol_abs_addr {
    ($label:ident) => { $label##_absolute };
}

/// Find a label from within a fixed section.
#[macro_export]
macro_rules! abs_addr {
    ($label:ident, $sname:ident) => {
        /* label - start_sname + sname_start */
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
