// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *  Copyright (C) 2010 John Crispin <john@phrozen.org>
 */

// Dependencies supplied by the surrounding platform/kernel translation.

const ASC_BUF: usize = 1024;
const TXMASK: u32 = 0x3F00;
const TXOFFSET: u32 = 8;

// C: LTQ_ASC_FSTAT ((u32 *)(LTQ_EARLY_ASC + 0x0048))
// C: LTQ_ASC_TBUF is endian-dependent: +0x0020 + 3 on big-endian,
// otherwise +0x0020.

extern "C" {
    static LTQ_EARLY_ASC: usize;
    fn ltq_r32(addr: *const u32) -> u32;
    fn ltq_w8(value: u8, addr: *mut u8);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
}

#[inline]
pub unsafe fn prom_putchar(c: core::ffi::c_char) {
    let mut flags: usize = 0;
    local_irq_save(&mut flags as *mut usize);

    let fstat = (LTQ_EARLY_ASC + 0x0048) as *const u32;
    loop {
        if ((ltq_r32(fstat) & TXMASK) >> TXOFFSET) == 0 {
            break;
        }
    }

    #[cfg(target_endian = "big")]
    let tbuf = (LTQ_EARLY_ASC + 0x0020 + 3) as *mut u8;
    #[cfg(not(target_endian = "big"))]
    let tbuf = (LTQ_EARLY_ASC + 0x0020) as *mut u8;

    if c == b'\n' as core::ffi::c_char {
        ltq_w8(b'\r', tbuf);
    }
    ltq_w8(c as u8, tbuf);
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
