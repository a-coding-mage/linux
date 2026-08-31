/* SPDX-License-Identifier: GPL-2.0-or-later */

/* From asm-compat.h */
/* C preprocessor stringification macros:
 * #define __stringify_in_c(...)    #__VA_ARGS__
 * #define stringify_in_c(...)      __stringify_in_c(__VA_ARGS__) " "
 */

/*
 * Macros taken from arch/powerpc/include/asm/ppc-opcode.h and other
 * header files.
 */
pub const fn ___PPC_RA(a: u32) -> u32 {
    ((a & 0x1f) << 16)
}

pub const fn ___PPC_RB(b: u32) -> u32 {
    ((b & 0x1f) << 11)
}

pub const PPC_INST_COPY: u32 = 0x7c20060c;
pub const PPC_INST_PASTE: u32 = 0x7c20070d;

/* C macros emitted stringified assembly:
 * #define PPC_COPY(a, b)   stringify_in_c(.long PPC_INST_COPY | \
 *                                      ___PPC_RA(a) | ___PPC_RB(b))
 * #define PPC_PASTE(a, b)  stringify_in_c(.long PPC_INST_PASTE | \
 *                                      ___PPC_RA(a) | ___PPC_RB(b))
 */
pub const CR0_SHIFT: u32 = 28;
pub const CR0_MASK: u32 = 0xF;

/*
 * Copy/paste instructions:
 *
 *      copy RA,RB
 *              Copy contents of address (RA) + effective_address(RB)
 *              to internal copy-buffer.
 *
 *      paste RA,RB
 *              Paste contents of internal copy-buffer to the address
 *              (RA) + effective_address(RB)
 */
pub unsafe fn vas_copy(crb: *mut core::ffi::c_void, offset: core::ffi::c_int) -> core::ffi::c_int {
    unsafe {
        core::arch::asm!(
            ".long {inst}",
            inst = const PPC_INST_COPY,
            in("r0") offset,
            in("r0") crb,
            options(nostack, preserves_flags)
        );
    }

    0
}

pub unsafe fn vas_paste(
    paste_address: *mut core::ffi::c_void,
    offset: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cr: u32;

    cr = 0;
    unsafe {
        core::arch::asm!(
            ".long {inst}",
            "mfocrf {cr_out}, 0x80",
            inst = const PPC_INST_PASTE,
            cr_out = out(reg) cr,
            in("r0") offset,
            in("r0") paste_address,
            options(nostack)
        );
    }

    ((cr >> CR0_SHIFT) & CR0_MASK) as core::ffi::c_int
}
