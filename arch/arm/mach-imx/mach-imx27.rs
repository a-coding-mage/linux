// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Sascha Hauer, Pengutronix
 */

// C dependencies: <linux/init.h>, <asm/mach/arch.h>, <asm/mach/map.h>,
// "common.h", "hardware.h", and "mx27.h".

/* MX27 memory map definition */
static mut imx27_io_desc: [map_desc; 3] = [
    /*
     * this fixed mapping covers:
     * - AIPI1
     * - AIPI2
     * - AITC
     * - ROM Patch
     * - and some reserved space
     */
    imx_map_entry(MX27, AIPI, MT_DEVICE),
    /*
     * this fixed mapping covers:
     * - CSI
     * - ATA
     */
    imx_map_entry(MX27, SAHB1, MT_DEVICE),
    /*
     * this fixed mapping covers:
     * - EMI
     */
    imx_map_entry(MX27, X_MEMC, MT_DEVICE),
];

/*
 * Initialize the memory map. It is called during the
 * system startup to create static physical to virtual
 * memory map for the IO modules.
 */
unsafe fn mx27_map_io() {
    iotable_init(
        imx27_io_desc.as_ptr(),
        core::mem::size_of_val(&imx27_io_desc) / core::mem::size_of::<map_desc>(),
    );
}

unsafe fn imx27_init_early() {
    mxc_set_cpu_type(MXC_CPU_MX27);
}

static imx27_dt_board_compat: [*const core::ffi::c_char; 2] = [
    b"fsl,imx27\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

// Translation of:
// DT_MACHINE_START(IMX27_DT, "Freescale i.MX27 (Device Tree Support)")
//     .map_io      = mx27_map_io,
//     .init_early  = imx27_init_early,
//     .init_late   = imx27_pm_init,
//     .dt_compat   = imx27_dt_board_compat,
// MACHINE_END
static IMX27_DT: machine_desc = machine_desc {
    map_io: Some(mx27_map_io),
    init_early: Some(imx27_init_early),
    init_late: Some(imx27_pm_init),
    dt_compat: imx27_dt_board_compat.as_ptr(),
    name: b"Freescale i.MX27 (Device Tree Support)\0".as_ptr()
        as *const core::ffi::c_char,
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
