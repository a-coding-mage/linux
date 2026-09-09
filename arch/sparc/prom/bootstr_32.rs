// SPDX-License-Identifier: GPL-2.0
/*
 * bootstr.c:  Boot string/argument acquisition from the PROM.
 *
 * Copyright(C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

// Dependency declarations corresponding to <linux/string.h>, <asm/oplib.h>,
// and <linux/init.h> are supplied by the surrounding translation unit.

const BARG_LEN: usize = 256;
static mut barg_buf: [u8; BARG_LEN] = [0; BARG_LEN];
static mut fetched: i8 = 0;

#[repr(C)]
pub struct PromV0BootArgs {
    pub argv: [*mut u8; 8],
}

#[repr(C)]
pub struct PromV2BootArgs {
    pub bootargs: *mut u8,
}

#[repr(C)]
pub struct RomVec {
    pub pv_v0bootargs: *mut *mut PromV0BootArgs,
    pub pv_v2bootargs: PromV2BootArgs,
}

extern "C" {
    static mut prom_vers: i32;
    static mut romvec: *mut RomVec;
}

extern "C" {
    fn strscpy(dst: *mut u8, src: *const u8, count: usize) -> isize;
}

const PROM_V0: i32 = 0;
const PROM_V2: i32 = 2;
const PROM_V3: i32 = 3;

pub unsafe fn prom_getbootargs() -> *mut u8 {
    let mut iter: i32;
    let mut cp: *mut u8;
    let mut arg: *mut u8;

    /* This check saves us from a panic when bootfd patches args. */
    if fetched != 0 {
        return barg_buf.as_mut_ptr();
    }

    match prom_vers {
        PROM_V0 => {
            cp = barg_buf.as_mut_ptr();
            /* Start from 1 and go over fd(0,0,0)kernel */
            iter = 1;
            while iter < 8 {
                arg = (*(*romvec).pv_v0bootargs).argv[iter as usize];
                if arg.is_null() {
                    break;
                }
                while *arg != 0 {
                    /* Leave place for space and null. */
                    if cp >= barg_buf.as_mut_ptr().add(BARG_LEN - 2) {
                        /* We might issue a warning here. */
                        break;
                    }
                    *cp = *arg;
                    cp = cp.add(1);
                    arg = arg.add(1);
                }
                *cp = b' ';
                cp = cp.add(1);
                if cp >= barg_buf.as_mut_ptr().add(BARG_LEN - 1) {
                    /* We might issue a warning here. */
                    break;
                }
                iter += 1;
            }
            *cp = 0;
        }
        PROM_V2 | PROM_V3 => {
            /*
             * V3 PROM cannot supply as with more than 128 bytes
             * of an argument. But a smart bootstrap loader can.
             */
            strscpy(
                barg_buf.as_mut_ptr(),
                (*romvec).pv_v2bootargs.bootargs,
                core::mem::size_of_val(&barg_buf),
            );
        }
        _ => {}
    }

    fetched = 1;
    barg_buf.as_mut_ptr()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
