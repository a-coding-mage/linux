// SPDX-License-Identifier: GPL-2.0-only
/*
 * DMA Router driver for LPC18xx/43xx DMA MUX
 *
 * Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
 *
 * Based on TI DMA Crossbar driver by:
 *   Copyright (C) 2015 Texas Instruments Incorporated - http://www.ti.com
 *   Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

/* CREG register offset and macros for mux manipulation */
const LPC18XX_CREG_DMAMUX: u32 = 0x11c;
const LPC18XX_DMAMUX_MAX_VAL: u32 = 0x3;

#[inline]
const fn lpc18xx_dmamux_val(v: u32, n: u32) -> u32 { v << (n * 2) }

#[inline]
const fn lpc18xx_dmamux_mask(n: u32) -> u32 { 0x3 << (n * 2) }

#[repr(C)]
struct lpc18xx_dmamux {
    value: u32,
    busy: bool,
}

#[repr(C)]
struct lpc18xx_dmamux_data {
    dmarouter: dma_router,
    muxes: *mut lpc18xx_dmamux,
    dma_master_requests: u32,
    dma_mux_requests: u32,
    reg: *mut regmap,
    lock: spinlock_t,
}

/* External kernel types and functions supplied by other translation units. */
#[repr(C)] struct device;
#[repr(C)] struct device_node;
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct of_dma { of_node: *mut device_node }
#[repr(C)] struct of_phandle_args { args_count: u32, args: [u32; 3], np: *mut device_node }
#[repr(C)] struct dma_router { dev: *mut device, route_free: Option<unsafe extern "C" fn(*mut device, *mut core::ffi::c_void)> }
#[repr(C)] struct regmap;
#[repr(C)] struct spinlock_t;

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn of_find_device_by_node(node: *mut device_node) -> *mut platform_device;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut core::ffi::c_void;
    fn of_parse_phandle(node: *mut device_node, name: *const i8, index: i32) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn regmap_update_bits(reg: *mut regmap, regnum: u32, mask: u32, val: u32) -> i32;
    fn put_device(dev: *mut device);
    fn syscon_regmap_lookup_by_compatible(compatible: *const i8) -> *mut regmap;
    fn of_property_read_u32(node: *mut device_node, name: *const i8, value: *mut u32) -> i32;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut lpc18xx_dmamux_data);
    fn of_dma_router_register(node: *mut device_node, reserve: unsafe extern "C" fn(*mut of_phandle_args, *mut of_dma) -> *mut core::ffi::c_void, router: *mut dma_router) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_kcalloc(dev: *mut device, n: u32, size: usize, flags: u32) -> *mut core::ffi::c_void;
}

#[repr(C)] struct of_device_id { compatible: *const i8 }
#[repr(C)] struct driver { name: *const i8, of_match_table: *const of_device_id }
#[repr(C)] struct platform_driver { probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>, driver: driver }

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const ENODEV: i32 = 19;
const EBUSY: i32 = 16;
const GFP_KERNEL: u32 = 0;

unsafe extern "C" fn lpc18xx_dmamux_free(dev: *mut device, route_data: *mut core::ffi::c_void) {
    let dmamux = dev_get_drvdata(dev) as *mut lpc18xx_dmamux_data;
    let mux = route_data as *mut lpc18xx_dmamux;
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*dmamux).lock, &mut flags);
    (*mux).busy = false;
    spin_unlock_irqrestore(&mut (*dmamux).lock, flags);
}

unsafe extern "C" fn lpc18xx_dmamux_reserve(dma_spec: *mut of_phandle_args, ofdma: *mut of_dma) -> *mut core::ffi::c_void {
    let pdev = of_find_device_by_node((*ofdma).of_node);
    let dmamux = platform_get_drvdata(pdev) as *mut lpc18xx_dmamux_data;
    let mut flags = 0usize;
    let mux = (*dma_spec).args[0] as usize;
    let mut ret = -EINVAL;
    if (*dma_spec).args_count != 3 || mux >= (*dmamux).dma_master_requests as usize || (*dma_spec).args[1] > LPC18XX_DMAMUX_MAX_VAL { put_device(&mut (*pdev).dev); return ret as isize as *mut core::ffi::c_void; }
    (*dma_spec).np = of_parse_phandle((*ofdma).of_node, b"dma-masters\0".as_ptr() as *const i8, 0);
    if (*dma_spec).np.is_null() { put_device(&mut (*pdev).dev); return ret as isize as *mut core::ffi::c_void; }
    spin_lock_irqsave(&mut (*dmamux).lock, &mut flags);
    let mux_ptr = (*dmamux).muxes.add(mux);
    if (*mux_ptr).busy { spin_unlock_irqrestore(&mut (*dmamux).lock, flags); of_node_put((*dma_spec).np); ret = -EBUSY; put_device(&mut (*pdev).dev); return ret as isize as *mut core::ffi::c_void; }
    (*mux_ptr).busy = true;
    (*mux_ptr).value = (*dma_spec).args[1];
    regmap_update_bits((*dmamux).reg, LPC18XX_CREG_DMAMUX, lpc18xx_dmamux_mask(mux as u32), lpc18xx_dmamux_val((*mux_ptr).value, mux as u32));
    spin_unlock_irqrestore(&mut (*dmamux).lock, flags);
    (*dma_spec).args[1] = (*dma_spec).args[2];
    (*dma_spec).args_count = 2;
    put_device(&mut (*pdev).dev);
    mux_ptr as *mut core::ffi::c_void
}

unsafe extern "C" fn lpc18xx_dmamux_probe(pdev: *mut platform_device) -> i32 {
    let np = (*pdev).dev.of_node;
    let dmamux = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<lpc18xx_dmamux_data>(), GFP_KERNEL) as *mut lpc18xx_dmamux_data;
    if dmamux.is_null() { return -ENOMEM; }
    (*dmamux).reg = syscon_regmap_lookup_by_compatible(b"nxp,lpc1850-creg\0".as_ptr() as *const i8);
    if (*dmamux).reg.is_null() { return -ENODEV; }
    let mut ret = of_property_read_u32(np, b"dma-requests\0".as_ptr() as *const i8, &mut (*dmamux).dma_mux_requests);
    if ret != 0 { return ret; }
    let dma_np = of_parse_phandle(np, b"dma-masters\0".as_ptr() as *const i8, 0);
    if dma_np.is_null() { return -ENODEV; }
    ret = of_property_read_u32(dma_np, b"dma-requests\0".as_ptr() as *const i8, &mut (*dmamux).dma_master_requests);
    of_node_put(dma_np);
    if ret != 0 { return ret; }
    (*dmamux).muxes = devm_kcalloc(&mut (*pdev).dev, (*dmamux).dma_master_requests, core::mem::size_of::<lpc18xx_dmamux>(), GFP_KERNEL) as *mut lpc18xx_dmamux;
    if (*dmamux).muxes.is_null() { return -ENOMEM; }
    spin_lock_init(&mut (*dmamux).lock);
    platform_set_drvdata(pdev, dmamux);
    (*dmamux).dmarouter.dev = &mut (*pdev).dev;
    (*dmamux).dmarouter.route_free = Some(lpc18xx_dmamux_free);
    of_dma_router_register(np, lpc18xx_dmamux_reserve, &mut (*dmamux).dmarouter)
}

static Lpc18xx_dmamux_match: [of_device_id; 2] = [
    of_device_id { compatible: b"nxp,lpc1850-dmamux\0".as_ptr() as *const i8 },
    of_device_id { compatible: core::ptr::null() },
];

static mut Lpc18xx_dmamux_driver: platform_driver = platform_driver { probe: Some(lpc18xx_dmamux_probe), driver: driver { name: b"lpc18xx-dmamux\0".as_ptr() as *const i8, of_match_table: Lpc18xx_dmamux_match.as_ptr() } };

unsafe extern "C" fn lpc18xx_dmamux_init() -> i32 { platform_driver_register(&mut Lpc18xx_dmamux_driver) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
