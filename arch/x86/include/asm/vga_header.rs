/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	Access to VGA videoram
 *
 *	(c) 1998 Martin Mares <mj@ucw.cz>
 */

// C header guard: _ASM_X86_VGA_H

// Dependency supplied by asm/set_memory.h.

/*
 *	On the PC, we can just recalculate addresses and then
 *	access the videoram directly without any black magic.
 *	To support memory encryption however, we need to access
 *	the videoram as decrypted memory.
 */

macro_rules! VGA_MAP_MEM {
    ($x:expr, $s:expr) => {{
        let start = phys_to_virt($x) as usize;

        // C condition: IS_ENABLED(CONFIG_AMD_MEM_ENCRYPT).
        if cfg!(feature = "CONFIG_AMD_MEM_ENCRYPT") {
            set_memory_decrypted(start, ($s) >> PAGE_SHIFT);
        }

        start
    }};
}

macro_rules! vga_readb {
    ($x:expr) => {{
        unsafe { *($x) }
    }};
}

macro_rules! vga_writeb {
    ($x:expr, $y:expr) => {{
        unsafe { *($y) = $x; }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
