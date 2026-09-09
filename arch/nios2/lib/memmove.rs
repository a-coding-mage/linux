/*
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

use core::ffi::c_void;

pub unsafe fn memmove(d: *mut c_void, s: *const c_void, mut count: usize) -> *mut c_void {
    let mut dst: usize;
    let mut src: usize;

    if count == 0 {
        return d;
    }

    if (d as usize) < (s as usize) {
        dst = d as usize;
        src = s as usize;

        if count < 8 || ((dst ^ src) & 3) != 0 {
            while count != 0 {
                *(dst as *mut u8) = *(src as *const u8);
                dst = dst.wrapping_add(1);
                src = src.wrapping_add(1);
                count -= 1;
            }
        } else {
            if (dst & 1) != 0 {
                *(dst as *mut u8) = *(src as *const u8);
                dst = dst.wrapping_add(1);
                src = src.wrapping_add(1);
                count -= 1;
            }
            if (dst & 2) != 0 {
                *(dst as *mut u16) = *(src as *const u16);
                src = src.wrapping_add(2);
                dst = dst.wrapping_add(2);
                count -= 2;
            }
            while count > 3 {
                *(dst as *mut u32) = *(src as *const u32);
                src = src.wrapping_add(4);
                dst = dst.wrapping_add(4);
                count -= 4;
            }
            while count != 0 {
                *(dst as *mut u8) = *(src as *const u8);
                dst = dst.wrapping_add(1);
                src = src.wrapping_add(1);
                count -= 1;
            }
        }
    } else {
        dst = (d as usize).wrapping_add(count);
        src = (s as usize).wrapping_add(count);

        if count < 8 || ((dst ^ src) & 3) != 0 {
            while count != 0 {
                src = src.wrapping_sub(1);
                dst = dst.wrapping_sub(1);
                *(dst as *mut u8) = *(src as *const u8);
                count -= 1;
            }
        } else {
            if (dst & 1) != 0 {
                src = src.wrapping_sub(1);
                dst = dst.wrapping_sub(1);
                count -= 1;
                *(dst as *mut u8) = *(src as *const u8);
            }
            if (dst & 2) != 0 {
                src = src.wrapping_sub(2);
                dst = dst.wrapping_sub(2);
                count -= 2;
                *(dst as *mut u16) = *(src as *const u16);
            }
            while count > 3 {
                src = src.wrapping_sub(4);
                dst = dst.wrapping_sub(4);
                count -= 4;
                *(dst as *mut u32) = *(src as *const u32);
            }
            while count != 0 {
                src = src.wrapping_sub(1);
                dst = dst.wrapping_sub(1);
                *(dst as *mut u8) = *(src as *const u8);
                count -= 1;
            }
        }
    }

    d
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
