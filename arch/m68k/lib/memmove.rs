/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

pub unsafe fn memmove(mut dest: *mut core::ffi::c_void,
                      mut src: *const core::ffi::c_void,
                      mut n: usize) -> *mut core::ffi::c_void
{
    let xdest = dest;
    let mut temp: usize;

    if n == 0 {
        return xdest;
    }

    if (dest as usize) < (src as usize) {
        if (dest as usize) & 1 != 0 {
            let mut cdest = dest as *mut u8;
            let mut csrc = src as *const u8;
            *cdest = *csrc;
            cdest = cdest.add(1);
            csrc = csrc.add(1);
            dest = cdest as *mut core::ffi::c_void;
            src = csrc as *const core::ffi::c_void;
            n -= 1;
        }
        #[cfg(feature = "CONFIG_M68000")]
        if (src as usize) & 1 != 0 {
            let mut cdest = dest as *mut u8;
            let mut csrc = src as *const u8;
            while n != 0 {
                *cdest = *csrc;
                cdest = cdest.add(1);
                csrc = csrc.add(1);
                n -= 1;
            }
            return xdest;
        }
        if n > 2 && (dest as usize) & 2 != 0 {
            let mut sdest = dest as *mut i16;
            let mut ssrc = src as *const i16;
            *sdest = *ssrc;
            sdest = sdest.add(1);
            ssrc = ssrc.add(1);
            dest = sdest as *mut core::ffi::c_void;
            src = ssrc as *const core::ffi::c_void;
            n -= 2;
        }
        temp = n >> 2;
        if temp != 0 {
            let mut ldest = dest as *mut i32;
            let mut lsrc = src as *const i32;
            temp -= 1;
            loop {
                *ldest = *lsrc;
                ldest = ldest.add(1);
                lsrc = lsrc.add(1);
                if temp == 0 { break; }
                temp -= 1;
            }
            dest = ldest as *mut core::ffi::c_void;
            src = lsrc as *const core::ffi::c_void;
        }
        if n & 2 != 0 {
            let mut sdest = dest as *mut i16;
            let mut ssrc = src as *const i16;
            *sdest = *ssrc;
            dest = sdest.add(1) as *mut core::ffi::c_void;
            src = ssrc.add(1) as *const core::ffi::c_void;
        }
        if n & 1 != 0 {
            *(dest as *mut u8) = *(src as *const u8);
        }
    } else {
        dest = (dest as *mut u8).add(n) as *mut core::ffi::c_void;
        src = (src as *const u8).add(n) as *const core::ffi::c_void;
        if (dest as usize) & 1 != 0 {
            let cdest = (dest as *mut u8).sub(1);
            let csrc = (src as *const u8).sub(1);
            *cdest = *csrc;
            dest = cdest as *mut core::ffi::c_void;
            src = csrc as *const core::ffi::c_void;
            n -= 1;
        }
        #[cfg(feature = "CONFIG_M68000")]
        if (src as usize) & 1 != 0 {
            let mut cdest = dest as *mut u8;
            let mut csrc = src as *const u8;
            while n != 0 {
                cdest = cdest.sub(1);
                csrc = csrc.sub(1);
                *cdest = *csrc;
                n -= 1;
            }
            return xdest;
        }
        if n > 2 && (dest as usize) & 2 != 0 {
            let sdest = (dest as *mut i16).sub(1);
            let ssrc = (src as *const i16).sub(1);
            *sdest = *ssrc;
            dest = sdest as *mut core::ffi::c_void;
            src = ssrc as *const core::ffi::c_void;
            n -= 2;
        }
        temp = n >> 2;
        if temp != 0 {
            let mut ldest = dest as *mut i32;
            let mut lsrc = src as *const i32;
            temp -= 1;
            loop {
                ldest = ldest.sub(1);
                lsrc = lsrc.sub(1);
                *ldest = *lsrc;
                if temp == 0 { break; }
                temp -= 1;
            }
            dest = ldest as *mut core::ffi::c_void;
            src = lsrc as *const core::ffi::c_void;
        }
        if n & 2 != 0 {
            let sdest = (dest as *mut i16).sub(1);
            let ssrc = (src as *const i16).sub(1);
            *sdest = *ssrc;
            dest = sdest as *mut core::ffi::c_void;
            src = ssrc as *const core::ffi::c_void;
        }
        if n & 1 != 0 {
            let cdest = (dest as *mut u8).sub(1);
            let csrc = (src as *const u8).sub(1);
            *cdest = *csrc;
        }
    }
    xdest
}

// EXPORT_SYMBOL(memmove);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
