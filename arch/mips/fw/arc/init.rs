/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * PROM library initialisation code.
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 */

// Dependencies corresponding to linux/init.h, linux/kernel.h,
// asm/bootinfo.h, asm/sgialib.h, and asm/smp-ops.h are supplied externally.

// #undef DEBUG_PROM_INIT

/* Master romvec interface. */
#[allow(non_camel_case_types)]
pub struct linux_romvec {
    _private: [u8; 0],
}

#[allow(non_upper_case_globals)]
pub static mut romvec: *mut linux_romvec = core::ptr::null_mut();

#[cfg(all(CONFIG_64BIT, CONFIG_FW_ARC32))]
/* stack for calling 32bit ARC prom */
#[allow(non_upper_case_globals)]
pub static mut o32_stk: [u64; 4096] = [0; 4096];

#[allow(non_snake_case)]
pub unsafe fn prom_init() {
    let pb: PSYSTEM_PARAMETER_BLOCK = PROMBLOCK;

    romvec = ROMVECTOR;

    if (*pb).magic != 0x53435241 {
        printk(
            KERN_CRIT,
            "Aieee, bad prom vector magic %08lx\n",
            (*pb).magic as c_ulong,
        );
        loop {}
    }

    prom_init_cmdline(fw_arg0, fw_arg1 as *mut LONG);
    prom_identify_arch();
    printk(
        KERN_INFO,
        "PROMLIB: ARC firmware Version %d Revision %d\n",
        (*pb).ver,
        (*pb).rev,
    );
    prom_meminit();

    // #ifdef DEBUG_PROM_INIT
    // pr_info("Press a key to reboot\n");
    // ArcRead(0, &c, 1, &cnt);
    // ArcEnterInteractiveMode();
    // #endif
}

// External types, globals, constants, and functions supplied by the firmware
// and architecture headers.
extern "C" {
    static mut PROMBLOCK: PSYSTEM_PARAMETER_BLOCK;
    static mut ROMVECTOR: *mut linux_romvec;
    static mut fw_arg0: c_ulong;
    static mut fw_arg1: c_ulong;

    fn printk(level: *const core::ffi::c_char, fmt: *const core::ffi::c_char, ...);
    fn prom_init_cmdline(arg0: c_ulong, arg1: *mut LONG);
    fn prom_identify_arch();
    fn prom_meminit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
