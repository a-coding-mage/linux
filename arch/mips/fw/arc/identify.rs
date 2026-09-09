/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * identify.c: identify machine by looking up system identifier
 *
 * Copyright (C) 1998 Thomas Bogendoerfer
 *
 * This code is based on arch/mips/sgi/kernel/system.c, which is
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 */

use core::ffi::{c_char, c_int};

/* Symbols and constants supplied by the surrounding kernel/ARC environment. */
#[allow(non_camel_case_types)]
pub enum pcomponent {}

unsafe extern "C" {
    static PROM_FLAG_ARCS: c_int;
    static PROM_FLAG_DONT_FREE_TEMP: c_int;
    static PROM_NULL_COMPONENT: *mut pcomponent;
    static child_component: usize;
    fn strcmp(left: *const c_char, right: *const c_char) -> c_int;
    fn printk(format: *const c_char, ...);
    fn panic(format: *const c_char, ... ) -> !;
}

#[repr(C)]
struct smatch {
    arcname: *mut c_char,
    liname: *mut c_char,
    flags: c_int,
}

static mut mach_table: [smatch; 8] = [
    smatch {
        arcname: b"SGI-IP22\0".as_ptr() as *mut c_char,
        liname: b"SGI Indy\0".as_ptr() as *mut c_char,
        flags: 0,
    },
    smatch {
        arcname: b"SGI-IP28\0".as_ptr() as *mut c_char,
        liname: b"SGI IP28\0".as_ptr() as *mut c_char,
        flags: 0,
    },
    smatch {
        arcname: b"SGI-IP30\0".as_ptr() as *mut c_char,
        liname: b"SGI Octane\0".as_ptr() as *mut c_char,
        flags: 0,
    },
    smatch {
        arcname: b"SGI-IP32\0".as_ptr() as *mut c_char,
        liname: b"SGI O2\0".as_ptr() as *mut c_char,
        flags: 0,
    },
    smatch {
        arcname: b"Microsoft-Jazz\0".as_ptr() as *mut c_char,
        liname: b"Jazz MIPS_Magnum_4000\0".as_ptr() as *mut c_char,
        flags: 0,
    },
    smatch {
        arcname: b"PICA-61\0".as_ptr() as *mut c_char,
        liname: b"Jazz Acer_PICA_61\0".as_ptr() as *mut c_char,
        flags: 0,
    },
    smatch {
        arcname: b"RM200PCI\0".as_ptr() as *mut c_char,
        liname: b"SNI RM200_PCI\0".as_ptr() as *mut c_char,
        flags: 0,
    },
    smatch {
        arcname: b"RM200PCI-R5K\0".as_ptr() as *mut c_char,
        liname: b"SNI RM200_PCI-R5K\0".as_ptr() as *mut c_char,
        flags: 0,
    },
];

pub static mut prom_flags: c_int = 0;

unsafe fn string_to_mach(s: *const c_char) -> *mut smatch {
    let mut i: usize = 0;

    while i < mach_table.len() {
        if strcmp(s, mach_table[i].arcname) == 0 {
            return &mut mach_table[i];
        }
        i += 1;
    }

    panic(b"Yeee, could not determine architecture type <%s>\0".as_ptr() as *const c_char, s);
}

pub static mut system_type: *mut c_char = core::ptr::null_mut();

pub unsafe fn get_system_type() -> *const c_char {
    system_type
}

unsafe fn ArcGetChild(current: *mut pcomponent) -> *mut pcomponent {
    /* ARC_CALL1(child_component, Current). */
    let function: unsafe extern "C" fn(*mut pcomponent) -> *mut pcomponent =
        core::mem::transmute(child_component);
    function(current)
}

pub unsafe fn prom_identify_arch() {
    let p: *mut pcomponent;
    let mach: *mut smatch;
    let iname: *const c_char;

    /*
     * The root component tells us what machine architecture we have here.
     */
    p = ArcGetChild(PROM_NULL_COMPONENT);
    if p.is_null() {
        iname = b"Unknown\0".as_ptr() as *const c_char;
    } else {
        iname = *(p as *const *const c_char);
    }

    printk(b"ARCH: %s\n\0".as_ptr() as *const c_char, iname);
    mach = string_to_mach(iname);
    system_type = (*mach).liname;

    prom_flags = (*mach).flags;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
