/*
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/// Set `count` bytes at `s` to the low byte of `c`.
pub unsafe fn memset(
    s: *mut core::ffi::c_void,
    mut c: i32,
    mut count: usize,
) -> *mut core::ffi::c_void {
    if count == 0 {
        return s;
    }

    c &= 0xff;

    if count <= 8 {
        let mut xs = s as *mut u8;
        while count != 0 {
            *xs = c as u8;
            xs = xs.add(1);
            count -= 1;
        }
        return s;
    }

    // The original implementation uses Nios II inline assembly.  The
    // operations below preserve its byte/halfword/word alignment sequence.
    let fill8reg = (c as u32)
        | ((c as u32) << 8)
        | ((c as u32) << 16)
        | ((c as u32) << 24);
    let mut destptr = s as *mut u8;
    let mut charcnt = count;

    if (destptr as usize & 0x01) != 0 {
        charcnt -= 1;
        *destptr = fill8reg as u8;
        destptr = destptr.add(1);
    }

    if (destptr as usize & 0x02) != 0 {
        charcnt -= 2;
        *(destptr as *mut u16) = fill8reg as u16;
        destptr = destptr.add(2);
    }

    let mut dwordcnt = charcnt >> 2;
    while dwordcnt != 0 {
        *(destptr as *mut u32) = fill8reg;
        destptr = destptr.add(4);
        dwordcnt -= 1;
    }

    if (charcnt & 0x02) != 0 {
        *(destptr as *mut u16) = fill8reg as u16;
        destptr = destptr.add(2);
    }

    if (charcnt & 0x01) != 0 {
        *destptr = fill8reg as u8;
    }

    s
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
