/* SPDX-License-Identifier: GPL-2.0-or-later */

use core::ffi::c_void;

// OPSIZ is BITS_PER_LONG / 8 in the C source.  `usize` has the same width as
// the target C `unsigned long` for this low-level implementation.
const OPSIZ: usize = (usize::BITS / 8) as usize;
type OpT = usize;

pub unsafe fn memset(dstpp: *mut c_void, sc: i32, mut len: usize) -> *mut c_void {
    let c = sc as u32;
    let mut dstp = dstpp as isize;

    if len >= 8 {
        let mut xlen: usize;
        let mut cccc: OpT;

        cccc = (c as u8) as OpT;
        cccc |= cccc << 8;
        cccc |= cccc << 16;
        if OPSIZ > 4 {
            /* Do the shift in two steps to avoid warning if long has 32 bits. */
            cccc |= (cccc << 16) << 16;
        }

        /* There are at least some bytes to set.
           No need to test for LEN == 0 in this alignment loop. */
        while (dstp as usize) % OPSIZ != 0 {
            *(dstp as *mut u8) = c as u8;
            dstp += 1;
            len -= 1;
        }

        /* Write 8 `op_t' per iteration until less than 8 `op_t' remain. */
        xlen = len / (OPSIZ * 8);
        while xlen > 0 {
            let p = dstp as *mut OpT;
            *p.add(0) = cccc;
            *p.add(1) = cccc;
            *p.add(2) = cccc;
            *p.add(3) = cccc;
            *p.add(4) = cccc;
            *p.add(5) = cccc;
            *p.add(6) = cccc;
            *p.add(7) = cccc;
            dstp += (8 * OPSIZ) as isize;
            xlen -= 1;
        }
        len %= OPSIZ * 8;

        /* Write 1 `op_t' per iteration until less than OPSIZ bytes remain. */
        xlen = len / OPSIZ;
        while xlen > 0 {
            *(dstp as *mut OpT) = cccc;
            dstp += OPSIZ as isize;
            xlen -= 1;
        }
        len %= OPSIZ;
    }

    /* Write the last few bytes. */
    while len > 0 {
        *(dstp as *mut u8) = c as u8;
        dstp += 1;
        len -= 1;
    }

    dstpp
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
