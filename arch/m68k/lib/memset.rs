/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Linux kernel module and string declarations are supplied by the surrounding
// crate.

pub unsafe fn memset(mut s: *mut core::ffi::c_void, mut c: i32, mut count: usize) -> *mut core::ffi::c_void {
    let xs = s;
    let mut temp: usize;

    if count == 0 {
        return xs;
    }
    c &= 0xff;
    c |= c << 8;
    c |= c << 16;
    if (s as isize & 1) != 0 {
        let cs = s as *mut u8;
        *cs = c as u8;
        s = cs.add(1) as *mut core::ffi::c_void;
        count -= 1;
    }
    if count > 2 && (s as isize & 2) != 0 {
        let ss = s as *mut u16;
        *ss = c as u16;
        s = ss.add(1) as *mut core::ffi::c_void;
        count -= 2;
    }
    temp = count >> 2;
    if temp != 0 {
        let mut ls = s as *mut i32;
        // The CONFIG_M68000/CONFIG_COLDFIRE branch uses this direct loop.
        // The other branch is an architecture-specific m68k inline-assembly
        // unrolled loop; its source-level memory effect is equivalent here.
        while temp != 0 {
            *ls = c;
            ls = ls.add(1);
            temp -= 1;
        }
        s = ls as *mut core::ffi::c_void;
    }
    if (count & 2) != 0 {
        let ss = s as *mut u16;
        *ss = c as u16;
        s = ss.add(1) as *mut core::ffi::c_void;
    }
    if (count & 1) != 0 {
        let cs = s as *mut u8;
        *cs = c as u8;
    }
    xs
}

// EXPORT_SYMBOL(memset);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
