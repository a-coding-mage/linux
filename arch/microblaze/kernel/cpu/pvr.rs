/*
 * Support for MicroBlaze PVR (processor version register)
 *
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2007 John Williams <john.williams@petalogix.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

use core::ffi::{c_char, c_ulong};

/* Values supplied by the MicroBlaze architecture headers. */
pub const PVR_MSR_BIT: c_ulong = 0; // build-time architecture constant
pub const PVR0_PVR_FULL_MASK: u32 = 0; // build-time architecture constant

#[repr(C)]
pub struct pvr_s {
    pub pvr: [u32; 12],
}

extern "C" {
    fn local_save_flags(flags: *mut c_ulong);
    fn pr_debug(fmt: *const c_char, ...);
}

/*
 * Until we get an assembler that knows about the pvr registers,
 * this horrible cruft will have to do.
 * That hardcoded opcode is mfs r3, rpvrNN
 */
macro_rules! get_single_pvr {
    ($pvrid:literal, $val:expr) => {{
        let mut tmp: u32 = 0;
        unsafe {
            core::arch::asm!(
                concat!("mfs {0}, rpvr", $pvrid, ";"),
                out(reg) tmp,
                options(nostack, preserves_flags)
            );
        }
        $val = tmp;
    }};
}

/*
 * Does the CPU support the PVR register?
 * return value:
 * 0: no PVR
 * 1: simple PVR
 * 2: full PVR
 *
 * This must work on all CPU versions, including those before the
 * PVR was even an option.
 */
pub unsafe fn cpu_has_pvr() -> i32 {
    let mut flags: c_ulong = 0;
    let mut pvr0: u32 = 0;

    local_save_flags(&mut flags);

    /* PVR bit in MSR tells us if there is any support */
    if (flags & PVR_MSR_BIT) == 0 {
        return 0;
    }

    get_single_pvr!(0, pvr0);
    pr_debug(b"%s: pvr0 is 0x%08x\0".as_ptr() as *const c_char, b"cpu_has_pvr\0".as_ptr(), pvr0);

    if (pvr0 & PVR0_PVR_FULL_MASK) != 0 {
        return 1;
    }

    /* for partial PVR use static cpuinfo */
    2
}

pub unsafe fn get_pvr(p: *mut pvr_s) {
    get_single_pvr!(0, (*p).pvr[0]);
    get_single_pvr!(1, (*p).pvr[1]);
    get_single_pvr!(2, (*p).pvr[2]);
    get_single_pvr!(3, (*p).pvr[3]);
    get_single_pvr!(4, (*p).pvr[4]);
    get_single_pvr!(5, (*p).pvr[5]);
    get_single_pvr!(6, (*p).pvr[6]);
    get_single_pvr!(7, (*p).pvr[7]);
    get_single_pvr!(8, (*p).pvr[8]);
    get_single_pvr!(9, (*p).pvr[9]);
    get_single_pvr!(10, (*p).pvr[10]);
    get_single_pvr!(11, (*p).pvr[11]);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
