/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

use core::ffi::c_void;

/// Copy `n` bytes from `from` to `to`, preserving the original C interface.
pub unsafe fn memcpy(mut to: *mut c_void, mut from: *const c_void, mut n: usize) -> *mut c_void {
    let xto = to;
    let mut temp: usize;

    if n == 0 {
        return xto;
    }
    if (to as isize) & 1 != 0 {
        let mut cto = to as *mut u8;
        let mut cfrom = from as *const u8;
        *cto = *cfrom;
        cto = cto.add(1);
        cfrom = cfrom.add(1);
        to = cto as *mut c_void;
        from = cfrom as *const c_void;
        n -= 1;
    }

    // CONFIG_M68000: when enabled, an odd source address is copied bytewise.
    #[cfg(feature = "CONFIG_M68000")]
    {
        if (from as isize) & 1 != 0 {
            let mut cto = to as *mut u8;
            let mut cfrom = from as *const u8;
            while n != 0 {
                *cto = *cfrom;
                cto = cto.add(1);
                cfrom = cfrom.add(1);
                n -= 1;
            }
            return xto;
        }
    }

    if n > 2 && (to as isize) & 2 != 0 {
        let mut sto = to as *mut u16;
        let mut sfrom = from as *const u16;
        *sto = *sfrom;
        sto = sto.add(1);
        sfrom = sfrom.add(1);
        to = sto as *mut c_void;
        from = sfrom as *const c_void;
        n -= 2;
    }

    temp = n >> 2;
    if temp != 0 {
        let mut lto = to as *mut u32;
        let mut lfrom = from as *const u32;
        // CONFIG_M68000 or CONFIG_COLDFIRE uses the simple longword loop.
        // The non-CONFIG path's m68k assembly is represented by its equivalent
        // longword copy loop; pointer advancement and copy ordering are the same.
        while temp != 0 {
            *lto = *lfrom;
            lto = lto.add(1);
            lfrom = lfrom.add(1);
            temp -= 1;
        }
        to = lto as *mut c_void;
        from = lfrom as *const c_void;
    }
    if n & 2 != 0 {
        let mut sto = to as *mut u16;
        let mut sfrom = from as *const u16;
        *sto = *sfrom;
        sto = sto.add(1);
        sfrom = sfrom.add(1);
        to = sto as *mut c_void;
        from = sfrom as *const c_void;
    }
    if n & 1 != 0 {
        let cto = to as *mut u8;
        let cfrom = from as *const u8;
        *cto = *cfrom;
    }
    xto
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
