// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2009 Lemote Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

// C dependencies supplied by the surrounding kernel translation.
use core::ffi::c_char;

extern "C" {
    static mut arcs_cmdline: *mut c_char;
    static mut mips_machtype: i32;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn get_system_type() -> *const c_char;
}

// __init
pub unsafe fn mach_prom_init_machtype() {
    /* We share the same kernel image file among Lemote 2F family
     * of machines, and provide the machtype= kernel command line
     * to users to indicate their machine, this command line will
     * be passed by the latest PMON automatically. and fortunately,
     * up to now, we can get the machine type from the PMON_VER=
     * commandline directly except the NAS machine, In the old
     * machines, this will help the users a lot.
     *
     * If no "machtype=" passed, get machine type from "PMON_VER=".
     *	PMON_VER=LM8089		Lemote 8.9'' netbook
     *		 LM8101		Lemote 10.1'' netbook
     *	(The above two netbooks have the same kernel support)
     *		 LM6XXX		Lemote FuLoong(2F) box series
     *		 LM9XXX		Lemote LynLoong PC series
     */
    if !strstr(arcs_cmdline as *const c_char, b"PMON_VER=LM\0".as_ptr() as *const c_char).is_null() {
        if !strstr(arcs_cmdline as *const c_char, b"PMON_VER=LM8\0".as_ptr() as *const c_char).is_null() {
            mips_machtype = MACH_LEMOTE_YL2F89;
        } else if !strstr(arcs_cmdline as *const c_char, b"PMON_VER=LM6\0".as_ptr() as *const c_char).is_null() {
            mips_machtype = MACH_LEMOTE_FL2F;
        } else if !strstr(arcs_cmdline as *const c_char, b"PMON_VER=LM9\0".as_ptr() as *const c_char).is_null() {
            mips_machtype = MACH_LEMOTE_LL2F;
        } else {
            mips_machtype = MACH_LEMOTE_NAS;
        }

        strcat(arcs_cmdline, b" machtype=\0".as_ptr() as *const c_char);
        strcat(arcs_cmdline, get_system_type());
        strcat(arcs_cmdline, b" \0".as_ptr() as *const c_char);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
