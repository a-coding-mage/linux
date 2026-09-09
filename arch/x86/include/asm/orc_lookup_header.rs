/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2017 Josh Poimboeuf <jpoimboe@redhat.com>
 */

/*
 * This is a lookup table for speeding up access to the .orc_unwind table.
 * Given an input address offset, the corresponding lookup table entry
 * specifies a subset of the .orc_unwind table to search.
 *
 * Each block represents the end of the previous range and the start of the
 * next range.  An extra block is added to give the last range an end.
 *
 * The block size should be a power of 2 to avoid a costly 'div' instruction.
 *
 * A block size of 256 was chosen because it roughly doubles unwinder
 * performance while only adding ~5% to the ORC data footprint.
 */
pub const LOOKUP_BLOCK_ORDER: u32 = 8;
pub const LOOKUP_BLOCK_SIZE: u32 = 1 << LOOKUP_BLOCK_ORDER;

/* The linker script configuration supplies these symbols. */
unsafe extern "C" {
    pub static mut orc_lookup: [::core::ffi::c_uint; 0];
    pub static mut orc_lookup_end: [::core::ffi::c_uint; 0];
    pub static _stext: u8;
    pub static _etext: u8;
}

#[macro_export]
macro_rules! LOOKUP_START_IP {
    () => {
        (&raw const $crate::_stext as usize)
    };
}

#[macro_export]
macro_rules! LOOKUP_STOP_IP {
    () => {
        (&raw const $crate::_etext as usize)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
