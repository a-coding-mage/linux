// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Freescale Lite5200 board support
 *
 * Written by: Grant Likely <grant.likely@secretlab.ca>
 *
 * Copyright (C) Secret Lab Technologies Ltd. 2006. All rights reserved.
 * Copyright 2006 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Description:
 */

// Translated from the C implementation; kernel and architecture dependencies
// are supplied by the surrounding source tree.

static const mpc5200_cdm_ids: [of_device_id; 3] = [
    of_device_id { compatible: c"fsl,mpc5200-cdm", ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: c"mpc5200-cdm", ..unsafe { core::mem::zeroed() } },
    unsafe { core::mem::zeroed() },
];

static const mpc5200_gpio_ids: [of_device_id; 3] = [
    of_device_id { compatible: c"fsl,mpc5200-gpio", ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: c"mpc5200-gpio", ..unsafe { core::mem::zeroed() } },
    unsafe { core::mem::zeroed() },
];

unsafe fn lite5200_fix_clock_config() {
    let np: *mut device_node;
    let cdm: *mut mpc52xx_cdm;

    np = of_find_matching_node(core::ptr::null_mut(), mpc5200_cdm_ids.as_ptr());
    cdm = of_iomap(np, 0) as *mut mpc52xx_cdm;
    of_node_put(np);
    if cdm.is_null() {
        printk!(KERN_ERR, "{}() failed; expect abnormal behaviour\n", "lite5200_fix_clock_config");
        return;
    }

    out_8(core::ptr::addr_of_mut!((*cdm).ext_48mhz_en), 0x00);
    out_8(core::ptr::addr_of_mut!((*cdm).fd_enable), 0x01);
    if in_be32(core::ptr::addr_of!((*cdm).rstcfg)) & 0x40 != 0 {
        out_be16(core::ptr::addr_of_mut!((*cdm).fd_counters), 0x0001);
    } else {
        out_be16(core::ptr::addr_of_mut!((*cdm).fd_counters), 0x5555);
    }
    iounmap(cdm as *mut core::ffi::c_void);
}

unsafe fn lite5200_fix_port_config() {
    let np: *mut device_node;
    let gpio: *mut mpc52xx_gpio;
    let mut port_config: u32;

    np = of_find_matching_node(core::ptr::null_mut(), mpc5200_gpio_ids.as_ptr());
    gpio = of_iomap(np, 0) as *mut mpc52xx_gpio;
    of_node_put(np);
    if gpio.is_null() {
        printk!(KERN_ERR, "{}() failed. expect abnormal behavior\n", "lite5200_fix_port_config");
        return;
    }

    port_config = in_be32(core::ptr::addr_of!((*gpio).port_config));
    port_config &= !0x00800000;
    port_config &= !0x00007000;
    port_config |= 0x00001000;
    port_config &= !0x03000000;
    port_config |= 0x01000000;

    pr_debug!("port_config: old:%x new:%x\n", in_be32(core::ptr::addr_of!((*gpio).port_config)), port_config);
    out_be32(core::ptr::addr_of_mut!((*gpio).port_config), port_config);
    iounmap(gpio as *mut core::ffi::c_void);
}

#[cfg(CONFIG_PM)]
unsafe fn lite5200_suspend_prepare(mbar: *mut u8) {
    let pin: u8 = 1;
    let level: u8 = 0;
    mpc52xx_set_wakeup_gpio(pin, level);
    out_be32(mbar.add(0x1048) as *mut u32, in_be32(mbar.add(0x1048) as *const u32) & !0x300);
    out_be32(mbar.add(0x1050) as *mut u32, 0x00000001);
}

#[cfg(CONFIG_PM)]
unsafe fn lite5200_resume_finish(mbar: *mut u8) {
    out_be32(mbar.add(0x1050) as *mut u32, 0x00010000);
}

unsafe fn lite5200_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress(c"lite5200_setup_arch()".as_ptr(), 0);
    }
    mpc52xx_map_common_devices();
    mpc5200_setup_xlb_arbiter();
    lite5200_fix_clock_config();
    lite5200_fix_port_config();

    #[cfg(CONFIG_PM)]
    {
        mpc52xx_suspend.board_suspend_prepare = Some(lite5200_suspend_prepare);
        mpc52xx_suspend.board_resume_finish = Some(lite5200_resume_finish);
        lite5200_pm_init();
    }
}

static board: [*const core::ffi::c_char; 3] = [c"fsl,lite5200".as_ptr(), c"fsl,lite5200b".as_ptr(), core::ptr::null()];

define_machine!(lite5200 {
    name: c"lite5200".as_ptr(),
    compatibles: board.as_ptr(),
    setup_arch: Some(lite5200_setup_arch),
    discover_phbs: Some(mpc52xx_setup_pci),
    init: Some(mpc52xx_declare_of_platform_devices),
    init_IRQ: Some(mpc52xx_init_irq),
    get_irq: Some(mpc52xx_get_irq),
    restart: Some(mpc52xx_restart),
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
