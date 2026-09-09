/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Optimited xor routines
 *
 * Copyright IBM Corp. 2016
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// External dependency supplied by the translated source tree.
#[repr(C)]
pub struct xor_block_template {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut xor_block_xc: xor_block_template;
    fn xor_force(block: *const xor_block_template);
}

// The C __init annotation is a kernel/linker placement attribute.
#[inline(always)]
unsafe fn arch_xor_init() {
    xor_force(&raw const xor_block_xc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
