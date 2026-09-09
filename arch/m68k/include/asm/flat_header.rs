/* SPDX-License-Identifier: GPL-2.0 */
/*
 * flat_header.rs -- uClinux flat-format executables
 *
 * Translated from m68k/include/asm/flat.h.
 */

// Dependency corresponding to <asm-generic/flat.h> is supplied externally.

/*
 * C equivalent:
 *
 * #define FLAT_PLAT_INIT(regs) \
 *     do { \
 *         if (current->mm) \
 *             (regs)->d5 = current->mm->start_data; \
 *     } while (0)
 *
 * `current` and the pointed-to register and memory-management structures are
 * supplied by the surrounding kernel translation unit.
 */
#[macro_export]
macro_rules! FLAT_PLAT_INIT {
    ($regs:expr) => {{
        unsafe {
            if !(*current).mm.is_null() {
                (*$regs).d5 = (*(*current).mm).start_data;
            }
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
