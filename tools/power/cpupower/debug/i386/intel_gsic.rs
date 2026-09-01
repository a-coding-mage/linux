// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2003  Bruno Ducrot
 *  (C) 2004  Dominik Brodowski <linux@dominikbrodowski.de>
 *
 * Based on code found in
 * linux/include/asm-i386/ist.h and linux/arch/i386/kernel/setup.c
 * and originally developed by Andy Grover <andrew.grover@intel.com>
 */

use std::ffi::c_char;
use std::mem;
use std::os::raw::c_int;

// C dependencies: <stdio.h>, <string.h>, <lrmi.h>
#[repr(C)]
pub struct LRMI_regs {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn LRMI_init() -> c_int;
    fn LRMI_int(num: c_int, r: *mut LRMI_regs) -> c_int;
}

fn main() {
    std::process::exit(unsafe { main_0() });
}

unsafe fn main_0() -> c_int {
    let mut r: LRMI_regs;
    let retval: c_int;

    if unsafe { LRMI_init() } == 0 {
        return 0;
    }

    r = unsafe { mem::zeroed() };

    r.eax = 0x0000E980;
    r.edx = 0x47534943;

    retval = unsafe { LRMI_int(0x15, &mut r) };

    if retval == 0 {
        unsafe {
            printf(c"Failed!\n".as_ptr());
        }
        return 0;
    }
    if r.eax == 0x47534943 {
        unsafe {
            printf(c"BIOS supports GSIC call:\n".as_ptr());
            printf(
                c"\tsignature: %c%c%c%c\n".as_ptr(),
                ((r.eax >> 24) & 0xff) as c_int,
                ((r.eax >> 16) & 0xff) as c_int,
                ((r.eax >> 8) & 0xff) as c_int,
                (r.eax & 0xff) as c_int,
            );
            printf(c"\tcommand port = 0x%.4x\n".as_ptr(), (r.ebx & 0xffff) as u32);
            printf(
                c"\tcommand =      0x%.4x\n".as_ptr(),
                ((r.ebx >> 16) & 0xffff) as u32,
            );
            printf(c"\tevent port =   0x%.8x\n".as_ptr(), r.ecx);
            printf(c"\tflags =        0x%.8x\n".as_ptr(), r.edx);
        }
        if ((r.ebx >> 16) & 0xffff) != 0x82 {
            unsafe {
                printf(
                    c"non-default command value. If speedstep-smi doesn't work out of the box,\nyou may want to try out the default value by passing smi_cmd=0x82 to the module\n ON YOUR OWN RISK.\n"
                        .as_ptr(),
                );
            }
        }
        if (r.ebx & 0xffff) != 0xb2 {
            unsafe {
                printf(
                    c"non-default command port. If speedstep-smi doesn't work out of the box,\nyou may want to try out the default value by passing smi_port=0x82 to the module\n ON YOUR OWN RISK.\n"
                        .as_ptr(),
                );
            }
        }
    } else {
        unsafe {
            printf(c"BIOS DOES NOT support GSIC call.  Dumping registers anyway:\n".as_ptr());
            printf(c"eax = 0x%.8x\n".as_ptr(), r.eax);
            printf(c"ebx = 0x%.8x\n".as_ptr(), r.ebx);
            printf(c"ecx = 0x%.8x\n".as_ptr(), r.ecx);
            printf(c"edx = 0x%.8x\n".as_ptr(), r.edx);
            printf(
                c"Note also that some BIOS do not support the initial GSIC call, but the newer\nspeedstep-smi driver may work.\nFor this, you need to pass some arguments to the speedstep-smi driver:\n"
                    .as_ptr(),
            );
            printf(c"\tsmi_cmd=0x?? smi_port=0x?? smi_sig=1\n".as_ptr());
            printf(
                c"\nUnfortunately, you have to know what exactly are smi_cmd and smi_port, and this\nis system dependent.\n"
                    .as_ptr(),
            );
        }
    }
    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
