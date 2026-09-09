/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2007 John Williams
 *
 * Reasonably optimised generic C-code for memset on Microblaze
 * This is generic C code to do efficient, alignment-aware memcpy.
 *
 * It is based on demo code originally Copyright 2001 by Intel Corp, taken from
 * http://www.embedded.com/showArticle.jhtml?articleID=19205567
 *
 * Attempts were made, unsuccessfully, to contact the original
 * author of this code (Michael Morrow, Intel).  Below is the original
 * copyright notice.
 *
 * This software has been developed by Intel Corporation.
 * Intel specifically disclaims all warranties, express or
 * implied, and all liability, including consequential and
 * other indirect damages, for the use of this program, including
 * liability for infringement of any proprietary rights,
 * and including the warranties of merchantability and fitness
 * for a particular purpose. Intel does not assume any
 * responsibility for and errors which may appear in this program
 * not any responsibility to update it.
 */

// CONFIG_OPT_LIB_FUNCTION is a build-time condition from the C source.

#[no_mangle]
pub unsafe extern "C" fn memset(
    v_src: *mut core::ffi::c_void,
    mut c: i32,
    mut n: usize,
) -> *mut core::ffi::c_void {
    let mut src = v_src as *mut i8;
    let mut i_src: *mut u32;
    let mut w32: u32 = 0;

    /* Truncate c to 8 bits */
    c &= 0xFF;

    if c != 0 {
        /* Make a repeating word out of it */
        w32 = c as u32;
        w32 |= w32 << 8;
        w32 |= w32 << 16;
    }

    if n >= 4 {
        /* Align the destination to a word boundary */
        /* This is done in an endian independent manner */
        match (src as usize) & 3 {
            1 => {
                *src = c as i8;
                src = src.add(1);
                n -= 1;
                *src = c as i8;
                src = src.add(1);
                n -= 1;
                *src = c as i8;
                src = src.add(1);
                n -= 1;
            }
            2 => {
                *src = c as i8;
                src = src.add(1);
                n -= 1;
                *src = c as i8;
                src = src.add(1);
                n -= 1;
            }
            3 => {
                *src = c as i8;
                src = src.add(1);
                n -= 1;
            }
            _ => {}
        }

        i_src = src as *mut u32;

        /* Do as many full-word copies as we can */
        while n >= 4 {
            *i_src = w32;
            i_src = i_src.add(1);
            n -= 4;
        }

        src = i_src as *mut i8;
    }

    /* Simple, byte oriented memset or the rest of count. */
    match n {
        3 => {
            *src = c as i8;
            src = src.add(1);
            *src = c as i8;
            src = src.add(1);
            *src = c as i8;
        }
        2 => {
            *src = c as i8;
            src = src.add(1);
            *src = c as i8;
        }
        1 => {
            *src = c as i8;
        }
        _ => {}
    }

    v_src
}

// EXPORT_SYMBOL(memset);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
