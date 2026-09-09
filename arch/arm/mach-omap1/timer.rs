// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP1 Dual-Mode Timers - platform device registration
 *
 * Contains first level initialization routines which internally
 * generates timer device information and registers with linux
 * device model. It also has a low level function to change the timer
 * input clock source.
 *
 * Copyright (C) 2011 Texas Instruments Incorporated - https://www.ti.com/
 * Tarun Kanti DebBarma <tarun.kanti@ti.com>
 * Thara Gopinath <thara@ti.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const OMAP1610_GPTIMER1_BASE: u32 = 0xfffb1400;
const OMAP1610_GPTIMER2_BASE: u32 = 0xfffb1c00;
const OMAP1610_GPTIMER3_BASE: u32 = 0xfffb2400;
const OMAP1610_GPTIMER4_BASE: u32 = 0xfffb2c00;
const OMAP1610_GPTIMER5_BASE: u32 = 0xfffb3400;
const OMAP1610_GPTIMER6_BASE: u32 = 0xfffb3c00;
const OMAP1610_GPTIMER7_BASE: u32 = 0xfffb7400;
const OMAP1610_GPTIMER8_BASE: u32 = 0xfffbd400;

const OMAP1_DM_TIMER_COUNT: i32 = 8;

unsafe fn omap1_dm_timer_set_src(pdev: *mut platform_device, source: i32) -> i32 {
    let n = ((*pdev).id - 1) << 1;
    let mut l: u32;

    l = omap_readl(MOD_CONF_CTRL_1) & !(0x03u32 << n);
    l |= (source as u32) << n;
    omap_writel(l, MOD_CONF_CTRL_1);

    0
}

unsafe fn omap1_dm_timer_init() -> i32 {
    let mut i: i32;
    let mut ret: i32;
    let mut pdata: *mut dmtimer_platform_data;
    let mut pdev: *mut platform_device;

    if !cpu_is_omap16xx() {
        return 0;
    }

    i = 1;
    while i <= OMAP1_DM_TIMER_COUNT {
        let mut res: [resource; 2] = [core::mem::zeroed(), core::mem::zeroed()];
        let base: u32;
        let irq: u32;

        match i {
            1 => { base = OMAP1610_GPTIMER1_BASE; irq = INT_1610_GPTIMER1; }
            2 => { base = OMAP1610_GPTIMER2_BASE; irq = INT_1610_GPTIMER2; }
            3 => { base = OMAP1610_GPTIMER3_BASE; irq = INT_1610_GPTIMER3; }
            4 => { base = OMAP1610_GPTIMER4_BASE; irq = INT_1610_GPTIMER4; }
            5 => { base = OMAP1610_GPTIMER5_BASE; irq = INT_1610_GPTIMER5; }
            6 => { base = OMAP1610_GPTIMER6_BASE; irq = INT_1610_GPTIMER6; }
            7 => { base = OMAP1610_GPTIMER7_BASE; irq = INT_1610_GPTIMER7; }
            8 => { base = OMAP1610_GPTIMER8_BASE; irq = INT_1610_GPTIMER8; }
            _ => {
                // not supposed to reach here; this is to remove warning.
                return -EINVAL;
            }
        }

        pdev = platform_device_alloc("omap_timer", i);
        if pdev.is_null() {
            pr_err!("{}: Failed to device alloc for dmtimer{}\n", "omap1_dm_timer_init", i);
            return -ENOMEM;
        }

        res[0].start = base;
        res[0].end = base + 0x46;
        res[0].flags = IORESOURCE_MEM;
        res[1].start = irq;
        res[1].end = irq;
        res[1].flags = IORESOURCE_IRQ;
        ret = platform_device_add_resources(pdev, res.as_mut_ptr(), res.len());
        if ret != 0 {
            dev_err!(&mut (*pdev).dev, "{}: Failed to add resources.\n", "omap1_dm_timer_init");
            goto_err_free_pdev!(pdev);
        }

        pdata = kzalloc_obj!(*pdata);
        if pdata.is_null() {
            ret = -ENOMEM;
            goto_err_free_pdata!(pdata, pdev);
        }

        (*pdata).set_timer_src = Some(omap1_dm_timer_set_src);
        (*pdata).timer_capability = OMAP_TIMER_ALWON |
            OMAP_TIMER_NEEDS_RESET | OMAP_TIMER_HAS_DSP_IRQ;

        ret = platform_device_add_data(pdev, pdata, core::mem::size_of::<dmtimer_platform_data>());
        if ret != 0 {
            dev_err!(&mut (*pdev).dev, "{}: Failed to add platform data.\n", "omap1_dm_timer_init");
            goto_err_free_pdata!(pdata, pdev);
        }

        ret = platform_device_add(pdev);
        if ret != 0 {
            dev_err!(&mut (*pdev).dev, "{}: Failed to add platform device.\n", "omap1_dm_timer_init");
            goto_err_free_pdata!(pdata, pdev);
        }

        dev_dbg!(&mut (*pdev).dev, " Registered.\n");
        i += 1;
    }

    0
}

// Equivalent of arch_initcall(omap1_dm_timer_init).
arch_initcall!(omap1_dm_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
