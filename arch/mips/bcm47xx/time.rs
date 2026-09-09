/*
 *  Copyright (C) 2004 Florian Schirmer <jolt@tuxbox.org>
 *
 *  This program is free software; you can redistribute  it and/or modify it
 *  under  the terms of  the GNU General  Public License as published by the
 *  Free Software Foundation; either version 2 of the License, or (at your
 *  option) any later version.
 *
 *  THIS SOFTWARE IS PROVIDED "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 *  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY
 *  AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 */

// External declarations supplied by the Linux/MIPS and BCM47xx dependencies.

pub unsafe fn plat_time_init() {
    let mut hz: ::core::ffi::c_ulong = 0;
    let mut chip_id: u16 = 0;
    let mut buf: [::core::ffi::c_char; 10] = [0; 10];
    let mut len: ::core::ffi::c_int;
    let board: bcm47xx_board = bcm47xx_board_get();

    /*
     * Use deterministic values for initial counter interrupt
     * so that calibrate delay avoids encountering a counter wrap.
     */
    write_c0_count(0);
    write_c0_compare(0xffff);

    match bcm47xx_bus_type {
        // CONFIG_BCM47XX_SSB
        BCM47XX_BUS_TYPE_SSB => {
            hz = ssb_cpu_clock(&bcm47xx_bus.ssb.mipscore) / 2;
            chip_id = bcm47xx_bus.ssb.chip_id;
        }
        // CONFIG_BCM47XX_BCMA
        BCM47XX_BUS_TYPE_BCMA => {
            hz = bcma_cpu_clock(&bcm47xx_bus.bcma.bus.drv_mips) / 2;
            chip_id = bcm47xx_bus.bcma.bus.chipinfo.id;
        }
        _ => {}
    }

    if chip_id == 0x5354 {
        len = bcm47xx_nvram_getenv(
            b"clkfreq\0".as_ptr() as *const ::core::ffi::c_char,
            buf.as_mut_ptr(),
            buf.len(),
        );
        if len >= 0 && strncmp(buf.as_ptr(), b"200\0".as_ptr() as *const _, 4) == 0 {
            hz = 100000000;
        }
    }

    match board {
        BCM47XX_BOARD_ASUS_WL520GC | BCM47XX_BOARD_ASUS_WL520GU => {
            hz = 100000000;
        }
        _ => {}
    }

    if hz == 0 {
        hz = 100000000;
    }

    /* Set MIPS counter frequency for fixed_rate_gettimeoffset() */
    mips_hpt_frequency = hz;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
