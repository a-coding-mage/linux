/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2005 Thiemo Seufer
 * Copyright (C) 2005  MIPS Technologies, Inc.\tAll rights reserved.
 *\tAuthor: Maciej W. Rozycki <macro@mips.com>
 */

// The address-space, bug, and cache-flush definitions are supplied externally.

#[cfg(not(any()))]
const CKSEG2: isize = CKSSEG;
#[cfg(not(any()))]
const TO_PHYS_MASK: isize = -1;

extern "C" {
    fn BUG();
}

/*
 * FUNC is executed in one of the uncached segments, depending on its
 * original address as follows:
 *
 * 1. If the original address is in CKSEG0 or CKSEG1, then the uncached
 *    segment used is CKSEG1.
 * 2. If the original address is in XKPHYS, then the uncached segment
 *    used is XKPHYS(2).
 * 3. Otherwise it's a bug.
 *
 * The same remapping is done with the stack pointer.  Stack handling
 * works because we don't handle stack arguments or more complex return
 * values, so we can avoid sharing the same stack area between a cached
 * and the uncached mode.
 */
pub unsafe fn run_uncached(func: *mut core::ffi::c_void) -> libc::c_ulong {
    let mut ret: libc::c_long;
    let lfunc = func as libc::c_long;
    let ufunc: libc::c_long;
    let usp: libc::c_long;
    let sp: libc::c_long;

    core::arch::asm!("move {0}, $sp", out(reg) sp);

    if sp >= CKSEG0 as libc::c_long && sp < CKSEG2 as libc::c_long {
        usp = CKSEG1ADDR(sp) as libc::c_long;
    }
    #[cfg(CONFIG_64BIT)]
    else if (sp as libc::c_longlong) >= PHYS_TO_XKPHYS(0, 0) as libc::c_longlong
        && (sp as libc::c_longlong) < PHYS_TO_XKPHYS(8, 0) as libc::c_longlong
    {
        usp = PHYS_TO_XKPHYS(
            K_CALG_UNCACHED,
            XKPHYS_TO_PHYS(sp as libc::c_longlong),
        ) as libc::c_long;
    }
    else {
        BUG();
        usp = sp;
    }

    if lfunc >= CKSEG0 as libc::c_long && lfunc < CKSEG2 as libc::c_long {
        ufunc = CKSEG1ADDR(lfunc) as libc::c_long;
    }
    #[cfg(CONFIG_64BIT)]
    else if (lfunc as libc::c_longlong) >= PHYS_TO_XKPHYS(0, 0) as libc::c_longlong
        && (lfunc as libc::c_longlong) < PHYS_TO_XKPHYS(8, 0) as libc::c_longlong
    {
        ufunc = PHYS_TO_XKPHYS(
            K_CALG_UNCACHED,
            XKPHYS_TO_PHYS(lfunc as libc::c_longlong),
        ) as libc::c_long;
    }
    else {
        BUG();
        ufunc = lfunc;
    }

    core::arch::asm!(
        "move $16, $sp",
        "move $sp, {usp}",
        "jalr {ufunc}",
        "move $sp, $16",
        usp = in(reg) usp,
        ufunc = in(reg) ufunc,
        lateout("$2") ret,
        clobber_abi("C"),
    );

    ret as libc::c_ulong
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
