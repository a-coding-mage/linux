// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2024 Timesys Corporation <piotr.wojtaszczyk@timesys.com>
//
// Based on TI DMA Crossbar driver by:
//   Copyright (C) 2015 Texas Instruments Incorporated - http://www.ti.com
//   Author: Peter Ujfalusi <peter.ujfalusi@ti.com>

// Dependencies supplied by the surrounding kernel translation.

const LPC32XX_SSP_CLK_CTRL: u32 = 0x78;
const LPC32XX_I2S_CLK_CTRL: u32 = 0x7c;

#[repr(C)]
struct lpc32xx_dmamux {
    signal: i32,
    name_sel0: *mut i8,
    name_sel1: *mut i8,
    muxval: i32,
    muxreg: i32,
    bit: i32,
    busy: bool,
}

#[repr(C)]
struct lpc32xx_dmamux_data {
    dmarouter: dma_router,
    reg: *mut regmap,
    lock: spinlock_t, // protects busy status flag
}

/* From LPC32x0 User manual "3.2.1 DMA request signals" */
static mut lpc32xx_muxes: [lpc32xx_dmamux; 5] = [
    lpc32xx_dmamux { signal: 3, name_sel0: b"spi2-rx-tx\0" as *const u8 as *mut i8, name_sel1: b"ssp1-rx\0" as *const u8 as *mut i8, muxval: 0, muxreg: LPC32XX_SSP_CLK_CTRL as i32, bit: 5, busy: false },
    lpc32xx_dmamux { signal: 10, name_sel0: b"uart7-rx\0" as *const u8 as *mut i8, name_sel1: b"i2s1-dma1\0" as *const u8 as *mut i8, muxval: 0, muxreg: LPC32XX_I2S_CLK_CTRL as i32, bit: 4, busy: false },
    lpc32xx_dmamux { signal: 11, name_sel0: b"spi1-rx-tx\0" as *const u8 as *mut i8, name_sel1: b"ssp1-tx\0" as *const u8 as *mut i8, muxval: 0, muxreg: LPC32XX_SSP_CLK_CTRL as i32, bit: 4, busy: false },
    lpc32xx_dmamux { signal: 14, name_sel0: b"none\0" as *const u8 as *mut i8, name_sel1: b"ssp0-rx\0" as *const u8 as *mut i8, muxval: 0, muxreg: LPC32XX_SSP_CLK_CTRL as i32, bit: 3, busy: false },
    lpc32xx_dmamux { signal: 15, name_sel0: b"none\0" as *const u8 as *mut i8, name_sel1: b"ssp0-tx\0" as *const u8 as *mut i8, muxval: 0, muxreg: LPC32XX_SSP_CLK_CTRL as i32, bit: 2, busy: false },
];

unsafe fn lpc32xx_dmamux_release(dev: *mut device, route_data: *mut c_void) {
    let dmamux = dev_get_drvdata(dev) as *mut lpc32xx_dmamux_data;
    let mux = route_data as *mut lpc32xx_dmamux;

    dev_dbg(dev, "releasing dma request signal %d routed to %s\n", (*mux).signal, if (*mux).muxval != 0 { (*mux).name_sel1 } else { (*mux).name_sel1 });

    let _guard = guard_spinlock(&mut (*dmamux).lock);
    (*mux).busy = false;
}

unsafe fn lpc32xx_dmamux_reserve(dma_spec: *mut of_phandle_args, ofdma: *mut of_dma) -> *mut c_void {
    let pdev = of_find_device_by_node((*ofdma).of_node);
    let dev = &mut (*pdev).dev;
    let dmamux = platform_get_drvdata(pdev) as *mut lpc32xx_dmamux_data;
    let mut flags: c_ulong = 0;
    let mut mux: *mut lpc32xx_dmamux = core::ptr::null_mut();
    let mut ret: i32 = -EINVAL;

    if (*dma_spec).args_count != 3 {
        dev_err(&mut (*pdev).dev, "invalid number of dma mux args\n");
        return err_put_pdev(pdev, ret);
    }

    for i in 0..lpc32xx_muxes.len() {
        if lpc32xx_muxes[i].signal == (*dma_spec).args[0] {
            mux = &mut lpc32xx_muxes[i];
            break;
        }
    }
    if mux.is_null() {
        dev_err(&mut (*pdev).dev, "invalid mux request number: %d\n", (*dma_spec).args[0]);
        return err_put_pdev(pdev, ret);
    }

    if (*dma_spec).args[2] > 1 {
        dev_err(&mut (*pdev).dev, "invalid dma mux value: %d\n", (*dma_spec).args[1]);
        return err_put_pdev(pdev, ret);
    }

    (*dma_spec).np = of_parse_phandle((*ofdma).of_node, b"dma-masters\0" as *const u8 as *const i8, 0);
    if (*dma_spec).np.is_null() {
        dev_err(&mut (*pdev).dev, "can't get dma master\n");
        return err_put_pdev(pdev, ret);
    }

    spin_lock_irqsave(&mut (*dmamux).lock, &mut flags);
    if (*mux).busy {
        spin_unlock_irqrestore(&mut (*dmamux).lock, flags);
        dev_err(dev, "dma request signal %d busy, routed to %s\n", (*mux).signal, if (*mux).muxval != 0 { (*mux).name_sel1 } else { (*mux).name_sel1 });
        of_node_put((*dma_spec).np);
        ret = -EBUSY;
        return err_put_pdev(pdev, ret);
    }

    (*mux).busy = true;
    (*mux).muxval = if (*dma_spec).args[2] != 0 { 1 << (*mux).bit } else { 0 };
    regmap_update_bits((*dmamux).reg, (*mux).muxreg as u32, 1 << (*mux).bit, (*mux).muxval as u32);
    spin_unlock_irqrestore(&mut (*dmamux).lock, flags);

    (*dma_spec).args[2] = 0;
    (*dma_spec).args_count = 2;
    dev_dbg(dev, "dma request signal %d routed to %s\n", (*mux).signal, if (*mux).muxval != 0 { (*mux).name_sel1 } else { (*mux).name_sel1 });
    put_device(&mut (*pdev).dev);
    mux as *mut c_void
}

unsafe fn err_put_pdev(pdev: *mut platform_device, ret: i32) -> *mut c_void {
    put_device(&mut (*pdev).dev);
    ERR_PTR(ret)
}

unsafe fn lpc32xx_dmamux_probe(pdev: *mut platform_device) -> i32 {
    let np = (*pdev).dev.of_node;
    let dmamux = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<lpc32xx_dmamux_data>(), GFP_KERNEL) as *mut lpc32xx_dmamux_data;
    if dmamux.is_null() { return -ENOMEM; }
    (*dmamux).reg = syscon_node_to_regmap((*np).parent);
    if IS_ERR((*dmamux).reg) { dev_err(&mut (*pdev).dev, "syscon lookup failed\n"); return PTR_ERR((*dmamux).reg); }
    spin_lock_init(&mut (*dmamux).lock);
    platform_set_drvdata(pdev, dmamux as *mut c_void);
    (*dmamux).dmarouter.dev = &mut (*pdev).dev;
    (*dmamux).dmarouter.route_free = Some(lpc32xx_dmamux_release);
    of_dma_router_register(np, Some(lpc32xx_dmamux_reserve), &mut (*dmamux).dmarouter)
}

static lpc32xx_dmamux_match: [of_device_id; 2] = [of_device_id { compatible: b"nxp,lpc3220-dmamux\0" as *const u8 as *const i8 }, of_device_id { compatible: core::ptr::null() }];
static mut lpc32xx_dmamux_driver: platform_driver = platform_driver { probe: Some(lpc32xx_dmamux_probe), driver: driver { name: b"lpc32xx-dmamux\0" as *const u8 as *const i8, of_match_table: lpc32xx_dmamux_match.as_ptr() } };

unsafe fn lpc32xx_dmamux_init() -> i32 {
    platform_driver_register(&mut lpc32xx_dmamux_driver)
}

// arch_initcall(lpc32xx_dmamux_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
