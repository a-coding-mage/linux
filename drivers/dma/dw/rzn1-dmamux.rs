// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2022 Schneider-Electric
 * Author: Miquel Raynal <miquel.raynal@bootlin.com
 * Based on TI crossbar driver written by Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

pub const RNZ1_DMAMUX_NCELLS: usize = 6;
pub const RZN1_DMAMUX_MAX_LINES: usize = 64;
pub const RZN1_DMAMUX_LINES_PER_CTLR: usize = 16;

#[repr(C)]
pub struct rzn1_dmamux_data {
    pub dmarouter: dma_router,
    pub used_chans: [usize; 1],
}

#[repr(C)]
pub struct rzn1_dmamux_map {
    pub req_idx: u32,
}

unsafe extern "C" {
    pub fn dev_get_drvdata(dev: *mut device) -> *mut rzn1_dmamux_data;
    pub fn dev_dbg(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    pub fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    pub fn clear_bit(nr: u32, addr: *mut usize);
    pub fn kfree(ptr: *mut core::ffi::c_void);
    pub fn of_find_device_by_node(node: *mut device_node) -> *mut platform_device;
    pub fn platform_get_drvdata(pdev: *mut platform_device) -> *mut rzn1_dmamux_data;
    pub fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    pub fn test_and_set_bit(nr: u32, addr: *mut usize) -> bool;
    pub fn r9a06g032_sysctrl_set_dmamux(mask: u32, val: u32) -> i32;
    pub fn put_device(dev: *mut device);
    pub fn of_node_put(node: *mut device_node);
    pub fn of_parse_phandle(node: *mut device_node, name: *const core::ffi::c_char, index: u32) -> *mut device_node;
    pub fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    pub fn dev_err_probe(dev: *mut device, err: i32, fmt: *const core::ffi::c_char, ...) -> i32;
    pub fn of_match_node(matches: *const of_device_id, node: *mut device_node) -> *const of_device_id;
    pub fn platform_set_drvdata(pdev: *mut platform_device, data: *mut rzn1_dmamux_data);
    pub fn of_dma_router_register(node: *mut device_node, allocate: unsafe extern "C" fn(*mut of_phandle_args, *mut of_dma) -> *mut core::ffi::c_void, router: *mut dma_router) -> i32;
}

#[repr(C)] pub struct dma_router { pub dev: *mut device, pub route_free: Option<unsafe extern "C" fn(*mut device, *mut core::ffi::c_void)> }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct of_dma { pub of_node: *mut device_node }
#[repr(C)] pub struct of_phandle_args { pub np: *mut device_node, pub args_count: u32, pub args: [u32; 6] }
#[repr(C)] pub struct of_device_id;

unsafe fn rzn1_dmamux_free(dev: *mut device, route_data: *mut core::ffi::c_void) {
    let dmamux = dev_get_drvdata(dev);
    let map = route_data as *mut rzn1_dmamux_map;
    dev_dbg(dev, b"Unmapping DMAMUX request %u\0".as_ptr() as _, (*map).req_idx);
    clear_bit((*map).req_idx, (*dmamux).used_chans.as_mut_ptr());
    kfree(map as _);
}

unsafe fn rzn1_dmamux_route_allocate(dma_spec: *mut of_phandle_args, ofdma: *mut of_dma) -> *mut core::ffi::c_void {
    let pdev = of_find_device_by_node((*ofdma).of_node);
    let dmamux = platform_get_drvdata(pdev);
    let mut map: *mut rzn1_dmamux_map;
    let (mut dmac_idx, mut chan, mut val): (u32, u32, u32);
    let mut ret: i32;

    if (*dma_spec).args_count != RNZ1_DMAMUX_NCELLS as u32 { ret = -22; goto put_device; }
    map = kzalloc(core::mem::size_of::<rzn1_dmamux_map>(), 0) as *mut rzn1_dmamux_map;
    if map.is_null() { ret = -12; goto put_device; }
    chan = (*dma_spec).args[0]; (*map).req_idx = (*dma_spec).args[4]; val = (*dma_spec).args[5]; (*dma_spec).args_count -= 2;
    if chan >= RZN1_DMAMUX_LINES_PER_CTLR as u32 { dev_err(&mut (*pdev).dev, b"Invalid DMA request line: %u\0".as_ptr() as _, chan); ret = -22; goto free_map; }
    if (*map).req_idx >= RZN1_DMAMUX_MAX_LINES as u32 || ((*map).req_idx as usize % RZN1_DMAMUX_LINES_PER_CTLR) as u32 != chan { dev_err(&mut (*pdev).dev, b"Invalid MUX request line: %u\0".as_ptr() as _, (*map).req_idx); ret = -22; goto free_map; }
    dmac_idx = if (*map).req_idx >= RZN1_DMAMUX_LINES_PER_CTLR as u32 { 1 } else { 0 };
    (*dma_spec).np = of_parse_phandle((*ofdma).of_node, b"dma-masters\0".as_ptr() as _, dmac_idx);
    if (*dma_spec).np.is_null() { dev_err(&mut (*pdev).dev, b"Can't get DMA master\n\0".as_ptr() as _); ret = -22; goto free_map; }
    dev_dbg(&mut (*pdev).dev, b"Mapping DMAMUX request %u to DMAC%u request %u\n\0".as_ptr() as _, (*map).req_idx, dmac_idx, chan);
    if test_and_set_bit((*map).req_idx, (*dmamux).used_chans.as_mut_ptr()) { ret = -16; goto put_dma_spec_np; }
    let mask = 1u32.wrapping_shl((*map).req_idx); ret = r9a06g032_sysctrl_set_dmamux(mask, if val != 0 { mask } else { 0 });
    if ret != 0 { clear_bit((*map).req_idx, (*dmamux).used_chans.as_mut_ptr()); goto put_dma_spec_np; }
    put_device(&mut (*pdev).dev); return map as _;
put_dma_spec_np: of_node_put((*dma_spec).np);
free_map: kfree(map as _);
put_device: put_device(&mut (*pdev).dev); core::ptr::null_mut()
}

#[cfg(feature = "CONFIG_OF")]
static RZN1_DMAC_MATCH: [of_device_id; 2] = [of_device_id, of_device_id];

unsafe extern "C" fn rzn1_dmamux_probe(pdev: *mut platform_device) -> i32 {
    let mux_node = (*pdev).dev.of_node;
    let dmamux = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<rzn1_dmamux_data>(), 0) as *mut rzn1_dmamux_data;
    if dmamux.is_null() { return -12; }
    let dmac_node = of_parse_phandle(mux_node, b"dma-masters\0".as_ptr() as _, 0);
    if dmac_node.is_null() { return dev_err_probe(&mut (*pdev).dev, -19, b"Can't get DMA master node\n\0".as_ptr() as _); }
    let match_ = of_match_node(RZN1_DMAC_MATCH.as_ptr(), dmac_node);
    of_node_put(dmac_node);
    if match_.is_null() { return dev_err_probe(&mut (*pdev).dev, -22, b"DMA master is not supported\n\0".as_ptr() as _); }
    (*dmamux).dmarouter.dev = &mut (*pdev).dev;
    (*dmamux).dmarouter.route_free = Some(rzn1_dmamux_free);
    platform_set_drvdata(pdev, dmamux);
    of_dma_router_register(mux_node, rzn1_dmamux_route_allocate, &mut (*dmamux).dmarouter)
}

static RZN1_DMAMUX_MATCH: [of_device_id; 2] = [of_device_id, of_device_id];

// C metadata: MODULE_DEVICE_TABLE(of, rzn1_dmamux_match)
#[repr(C)]
pub struct platform_driver { pub driver: driver, pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32> }
#[repr(C)] pub struct driver { pub name: *const core::ffi::c_char, pub of_match_table: *const of_device_id }

static mut RZN1_DMAMUX_DRIVER: platform_driver = platform_driver {
    driver: driver { name: b"renesas,rzn1-dmamux\0".as_ptr() as _, of_match_table: RZN1_DMAMUX_MATCH.as_ptr() },
    probe: Some(rzn1_dmamux_probe),
};

// C module registration: module_platform_driver(rzn1_dmamux_driver)
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Miquel Raynal <miquel.raynal@bootlin.com");
// MODULE_DESCRIPTION("Renesas RZ/N1 DMAMUX driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
