// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-omap2/usb-tusb6010.c
 *
 * Copyright (C) 2006 Nokia Corporation
 */

// Declarations supplied by the Linux and OMAP dependencies are intentionally
// left external to this translation unit.

static mut async_cs: u8 = 0;
static mut sync_cs: u8 = 0;
static mut refclk_psec: usize = 0;

static mut tusb_async: gpmc_settings = gpmc_settings {
    wait_on_read: true,
    wait_on_write: true,
    device_width: GPMC_DEVWIDTH_16BIT,
    mux_add_data: GPMC_MUX_AD,
    ..unsafe { core::mem::zeroed() }
};

static mut tusb_sync: gpmc_settings = gpmc_settings {
    burst_read: true,
    burst_write: true,
    sync_read: true,
    sync_write: true,
    wait_on_read: true,
    wait_on_write: true,
    burst_len: GPMC_BURST_16,
    device_width: GPMC_DEVWIDTH_16BIT,
    mux_add_data: GPMC_MUX_AD,
    ..unsafe { core::mem::zeroed() }
};

/* NOTE: timings are from tusb 6010 datasheet Rev 1.8, 12-Sept 2006 */

unsafe fn tusb_set_async_mode(sysclk_ps: u32) -> i32 {
    let mut dev_t: gpmc_device_timings = core::mem::zeroed();
    let mut t: gpmc_timings = core::mem::zeroed();
    let t_acsnh_advnh = sysclk_ps + 3000;

    dev_t.t_ceasu = 8 * 1000;
    dev_t.t_avdasu = t_acsnh_advnh - 7000;
    dev_t.t_ce_avd = 1000;
    dev_t.t_avdp_r = t_acsnh_advnh;
    dev_t.t_oeasu = t_acsnh_advnh + 1000;
    dev_t.t_oe = 300;
    dev_t.t_cez_r = 7000;
    dev_t.t_cez_w = dev_t.t_cez_r;
    dev_t.t_avdp_w = t_acsnh_advnh;
    dev_t.t_weasu = t_acsnh_advnh + 1000;
    dev_t.t_wpl = 300;
    dev_t.cyc_aavdh_we = 1;

    gpmc_calc_timings(&mut t, &tusb_async, &dev_t);
    gpmc_cs_set_timings(async_cs, &t, &tusb_async)
}

unsafe fn tusb_set_sync_mode(sysclk_ps: u32) -> i32 {
    let mut dev_t: gpmc_device_timings = core::mem::zeroed();
    let mut t: gpmc_timings = core::mem::zeroed();
    let t_scsnh_advnh = sysclk_ps + 3000;

    dev_t.clk = 11100;
    dev_t.t_bacc = 1000;
    dev_t.t_ces = 1000;
    dev_t.t_ceasu = 8 * 1000;
    dev_t.t_avdasu = t_scsnh_advnh - 7000;
    dev_t.t_ce_avd = 1000;
    dev_t.t_avdp_r = t_scsnh_advnh;
    dev_t.cyc_aavdh_oe = 3;
    dev_t.cyc_oe = 5;
    dev_t.t_ce_rdyz = 7000;
    dev_t.t_avdp_w = t_scsnh_advnh;
    dev_t.cyc_aavdh_we = 3;
    dev_t.cyc_wpl = 6;

    gpmc_calc_timings(&mut t, &tusb_sync, &dev_t);
    gpmc_cs_set_timings(sync_cs, &t, &tusb_sync)
}

/* tusb driver calls this when it changes the chip's clocking */
unsafe fn tusb6010_platform_retime(is_refclk: u32) -> i32 {
    if refclk_psec == 0 {
        return -ENODEV;
    }

    let sysclk_ps = if is_refclk != 0 { refclk_psec as u32 } else { TUSB6010_OSCCLK_60 };
    let mut status = tusb_set_async_mode(sysclk_ps);
    if status < 0 {
        printk(KERN_ERR "tusb6010 async retime error %d\n", status);
        return status;
    }
    status = tusb_set_sync_mode(sysclk_ps);
    if status < 0 {
        printk(KERN_ERR "tusb6010 sync retime error %d\n", status);
    }
    status
}

static mut tusb_resources: [resource; 2] = [
    resource { flags: IORESOURCE_MEM, ..unsafe { core::mem::zeroed() } },
    resource { flags: IORESOURCE_MEM, ..unsafe { core::mem::zeroed() } },
];

static mut tusb_dmamask: u64 = !(0u32) as u64;

static mut tusb_device: platform_device = platform_device {
    name: "musb-tusb\0".as_ptr() as *const i8,
    id: -1,
    dev: device {
        dma_mask: &mut tusb_dmamask,
        coherent_dma_mask: 0xffffffff,
        ..unsafe { core::mem::zeroed() }
    },
    num_resources: 2,
    resource: tusb_resources.as_mut_ptr(),
    ..unsafe { core::mem::zeroed() }
};

/* this may be called only from board-*.c setup code */
unsafe fn tusb6010_setup_interface(
    data: *mut musb_hdrc_platform_data,
    ps_refclk: u32,
    waitpin: u32,
    async_: u32,
    sync: u32,
    _dmachan: u32,
) -> i32 {
    let mut status: i32;

    /* ASYNC region, primarily for PIO */
    status = gpmc_cs_request(async_, SZ_16M, &mut tusb_resources[0].start as *mut _ as *mut usize);
    if status < 0 { return status; }
    tusb_resources[0].end = tusb_resources[0].start + 0x9ff;
    tusb_async.wait_pin = waitpin;
    async_cs = async_ as u8;
    status = gpmc_cs_program_settings(async_cs, &tusb_async);
    if status < 0 { return status; }

    /* SYNC region, primarily for DMA */
    status = gpmc_cs_request(sync, SZ_16M, &mut tusb_resources[1].start as *mut _ as *mut usize);
    if status < 0 { return status; }
    tusb_resources[1].end = tusb_resources[1].start + 0x9ff;
    tusb_sync.wait_pin = waitpin;
    sync_cs = sync as u8;
    status = gpmc_cs_program_settings(sync_cs, &tusb_sync);
    if status < 0 { return status; }

    /* set up memory timings ... can speed them up later */
    if ps_refclk == 0 { return -ENODEV; }
    refclk_psec = ps_refclk as usize;
    status = tusb6010_platform_retime(1);
    if status < 0 { return status; }

    /* finish device setup ... */
    if data.is_null() { return -ENODEV; }
    tusb_device.dev.platform_data = data as *mut _;
    /* so far so good ... register the device */
    status = platform_device_register(&mut tusb_device);
    if status < 0 { return status; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
