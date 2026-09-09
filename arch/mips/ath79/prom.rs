// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Atheros AR71XX/AR724X/AR913X specific prom routines
 *
 *  Copyright (C) 2015 Laurent Fasnacht <l@libres.ch>
 *  Copyright (C) 2008-2010 Gabor Juhos <juhosg@openwrt.org>
 *  Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 */

// External declarations supplied by the kernel and architecture dependencies.
unsafe extern "C" {
    fn fw_init_cmdline();
    fn fw_getenvl(name: *const core::ffi::c_char) -> usize;
    static mut initrd_start: usize;
    static mut initrd_end: usize;
    fn KSEG0ADDR(address: usize) -> usize;
}

pub unsafe extern "C" fn prom_init() {
    fw_init_cmdline();

    // CONFIG_BLK_DEV_INITRD: read the initrd address from the firmware environment.
    #[cfg(feature = "CONFIG_BLK_DEV_INITRD")]
    {
        initrd_start = fw_getenvl(b"initrd_start\0".as_ptr() as *const core::ffi::c_char);
        if initrd_start != 0 {
            initrd_start = KSEG0ADDR(initrd_start);
            initrd_end = initrd_start.wrapping_add(
                fw_getenvl(b"initrd_size\0".as_ptr() as *const core::ffi::c_char),
            );
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
