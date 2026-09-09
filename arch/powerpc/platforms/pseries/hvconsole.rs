// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * hvconsole.c
 * Copyright (C) 2004 Hollis Blanchard, IBM Corporation
 * Copyright (C) 2004 IBM Corporation
 *
 * Additional Author(s):
 *  Ryan S. Arnold <rsa@us.ibm.com>
 *
 * LPAR console support.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/export.h, linux/errno.h, asm/hvcall.h,
// asm/hvconsole.h, and asm/plpar_wrappers.h.

extern "C" {
    fn plpar_hcall(opcode: u64, retbuf: *mut usize, arg0: u64) -> isize;
    fn plpar_hcall_norets(
        opcode: u64,
        arg0: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
    ) -> isize;
}

/**
 * hvc_get_chars - retrieve characters from firmware for denoted vterm adapter
 * @vtermno: The vtermno or unit_address of the adapter from which to fetch the
 *\tdata.
 * @buf: The character buffer into which to put the character data fetched from
 *\tfirmware.
 * @count: not used?
 */
pub unsafe fn hvc_get_chars(vtermno: u32, buf: *mut u8, _count: usize) -> isize {
    let mut retbuf: [usize; PLPAR_HCALL_BUFSIZE] = [0; PLPAR_HCALL_BUFSIZE];
    let lbuf = buf as *mut usize;

    let ret = plpar_hcall(H_GET_TERM_CHAR, retbuf.as_mut_ptr(), vtermno as u64);
    *lbuf.add(0) = u64::from_be(retbuf[1] as u64) as usize;
    *lbuf.add(1) = u64::from_be(retbuf[2] as u64) as usize;

    if ret == H_SUCCESS {
        return retbuf[0] as isize;
    }

    0
}

/**
 * hvc_put_chars: send characters to firmware for denoted vterm adapter
 * @vtermno: The vtermno or unit_address of the adapter from which the data
 *\toriginated.
 * @buf: The character buffer that contains the character data to send to
 *\tfirmware. Must be at least 16 bytes, even if count is less than 16.
 * @count: Send this number of characters.
 */
pub unsafe fn hvc_put_chars(vtermno: u32, buf: *const u8, mut count: usize) -> isize {
    let lbuf = buf as *const usize;

    /* hcall will ret H_PARAMETER if 'count' exceeds firmware max. */
    if count > MAX_VIO_PUT_CHARS {
        count = MAX_VIO_PUT_CHARS;
    }

    let ret = plpar_hcall_norets(
        H_PUT_TERM_CHAR,
        vtermno as u64,
        count as u64,
        u64::to_be(*lbuf.add(0) as u64),
        u64::to_be(*lbuf.add(1) as u64),
    );
    if ret == H_SUCCESS {
        return count as isize;
    }
    if ret == H_BUSY {
        return -EAGAIN;
    }
    -EIO
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
