/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/mm.h, linux/highmem.h, and asm/setup.h.

#[cfg(CONFIG_MMU)]
// Dependency supplied by asm/virtconvert.h.
// When CONFIG_COLDFIRE is enabled, use asm/mcf_pgalloc.h.
// Otherwise, when CONFIG_SUN3 is enabled, use asm/sun3_pgalloc.h.
// Otherwise, use asm/motorola_pgalloc.h.

#[cfg(CONFIG_MMU)]
extern "C" {
    pub fn m68k_setup_node(node: ::core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
