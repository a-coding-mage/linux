/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016-17 IBM Corp.
 */

// Dependency intent preserved from <asm/ppc-opcode.h> and <asm/reg.h>.

/*
 * Copy/paste instructions:
 *
 *     copy RA,RB
 *             Copy contents of address (RA) + effective_address(RB)
 *             to internal copy-buffer.
 *
 *     paste RA,RB
 *             Paste contents of internal copy-buffer to the address
 *             (RA) + effective_address(RB).
 */

/// Execute the PowerPC copy instruction.
#[inline]
pub unsafe fn vas_copy(crb: *mut core::ffi::c_void, offset: i32) -> i32 {
    // PPC_COPY(%0, %1) is supplied by the architecture opcode definitions.
    core::arch::asm!(
        "copy {offset}, {crb}",
        offset = in(reg) offset,
        crb = in(reg) crb,
        options(nostack, preserves_flags)
    );

    0
}

/// Execute the PowerPC paste instruction and return the CR0 status bits.
#[inline]
pub unsafe fn vas_paste(paste_address: *mut core::ffi::c_void, offset: i32) -> i32 {
    let mut cr: u32;

    core::arch::asm!(
        "paste {offset}, {paste_address}",
        "mfocrf {cr}, 0x80",
        offset = in(reg) offset,
        paste_address = in(reg) paste_address,
        cr = lateout(reg) cr,
        options(nostack)
    );

    /* We mask with 0xE to ignore SO */
    ((cr >> CR0_SHIFT) & 0xE) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
