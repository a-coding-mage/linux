// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Kernel and UML dependencies supplied by other translation units.

extern "C" {
    static mut initrd_start: libc::c_ulong;
    static mut initrd_end: libc::c_ulong;

    fn uml_load_file(
        path: *mut libc::c_char,
        size: *mut libc::c_ulonglong,
    ) -> *mut libc::c_void;
}

/* Changed by uml_initrd_setup, which is a setup */
static mut initrd: *mut libc::c_char = core::ptr::null_mut();

pub unsafe extern "C" fn read_initrd() -> libc::c_int
{
    let mut size = core::mem::MaybeUninit::<libc::c_ulonglong>::uninit();
    let area: *mut libc::c_void;

    if initrd.is_null() {
        return 0;
    }

    area = uml_load_file(initrd, size.as_mut_ptr());
    if area.is_null() {
        return 0;
    }

    initrd_start = area as libc::c_ulong;
    initrd_end = initrd_start.wrapping_add(size.assume_init() as libc::c_ulong);
    0
}

unsafe extern "C" fn uml_initrd_setup(
    line: *mut libc::c_char,
    add: *mut libc::c_int,
) -> libc::c_int
{
    *add = 0;
    initrd = line;
    0
}

// __uml_setup("initrd=", uml_initrd_setup,
// "initrd=<initrd image>\n"
// "    This is used to boot UML from an initrd image.  The argument is the\n"
// "    name of the file containing the image.\n\n"
// );

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
