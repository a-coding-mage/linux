/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Build-time condition preserved from CONFIG_HAVE_FUNCTION_DESCRIPTORS.
 * The C header includes <asm/elf.h> when this configuration is enabled.
 */
#[cfg(CONFIG_HAVE_FUNCTION_DESCRIPTORS)]
pub type func_desc_t = Elf64_Fdesc;

/* The C header includes <asm-generic/sections.h>. */

/* nothing to see, move along */
unsafe extern "C" {
    pub static mut __alt_instructions: [core::ffi::c_char; 0];
    pub static mut __alt_instructions_end: [core::ffi::c_char; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
