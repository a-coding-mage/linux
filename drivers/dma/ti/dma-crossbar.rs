// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2015 Texas Instruments Incorporated - http://www.ti.com
 *  Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// Kernel dependencies are supplied by the surrounding translation unit.
use core::ffi::c_void;

const TI_XBAR_DRA7: u32 = 0;
const TI_XBAR_AM335X: u32 = 1;
static TI_XBAR_TYPE: [u32; 2] = [TI_XBAR_DRA7, TI_XBAR_AM335X];

#[repr(C)]
struct OfDeviceId { compatible: *const u8, data: *const c_void }
static TI_DMA_XBAR_MATCH: [OfDeviceId; 3] = [
    OfDeviceId { compatible: b"ti,dra7-dma-crossbar\0".as_ptr(), data: &TI_XBAR_TYPE[TI_XBAR_DRA7 as usize] as *const u32 as *const c_void },
    OfDeviceId { compatible: b"ti,am335x-edma-crossbar\0".as_ptr(), data: &TI_XBAR_TYPE[TI_XBAR_AM335X as usize] as *const u32 as *const c_void },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

const TI_AM335X_XBAR_LINES: u32 = 64;

#[repr(C)]
struct TiAm335xXbarData { iomem: *mut c_void, dmarouter: DmaRouter, xbar_events: u32, dma_requests: u32 }
#[repr(C)]
struct TiAm335xXbarMap { dma_line: u16, mux_val: u8 }

#[inline]
unsafe fn ti_am335x_xbar_write(iomem: *mut c_void, event: i32, val: u8) {
    if event >= 60 && event <= 63 { writeb_relaxed(val, (iomem as *mut u8).offset((63 - event % 4) as isize)); }
    else { writeb_relaxed(val, (iomem as *mut u8).offset(event as isize)); }
}

unsafe fn ti_am335x_xbar_free(dev: *mut Device, route_data: *mut c_void) {
    let xbar = dev_get_drvdata(dev) as *mut TiAm335xXbarData;
    let map = route_data as *mut TiAm335xXbarMap;
    dev_dbg(dev, "Unmapping XBAR event %u on channel %u\n", (*map).mux_val, (*map).dma_line);
    ti_am335x_xbar_write((*xbar).iomem, (*map).dma_line as i32, 0);
    kfree(map as *mut c_void);
}

unsafe fn ti_am335x_xbar_route_allocate(dma_spec: *mut OfPhandleArgs, ofdma: *mut OfDma) -> *mut c_void {
    let pdev = of_find_device_by_node((*ofdma).of_node);
    let xbar = platform_get_drvdata(pdev) as *mut TiAm335xXbarData;
    let mut map = err_ptr(-22);
    if (*dma_spec).args_count != 3 { goto_out_put_pdev!(pdev, map); }
    if (*dma_spec).args[2] >= (*xbar).xbar_events { dev_err(&mut (*pdev).dev, "Invalid XBAR event number: %d\n", (*dma_spec).args[2]); goto_out_put_pdev!(pdev, map); }
    if (*dma_spec).args[0] >= (*xbar).dma_requests { dev_err(&mut (*pdev).dev, "Invalid DMA request line number: %d\n", (*dma_spec).args[0]); goto_out_put_pdev!(pdev, map); }
    (*dma_spec).np = of_parse_phandle((*ofdma).of_node, b"dma-masters\0".as_ptr(), 0);
    if (*dma_spec).np.is_null() { dev_err(&mut (*pdev).dev, "Can't get DMA master\n"); goto_out_put_pdev!(pdev, map); }
    map = kzalloc::<TiAm335xXbarMap>();
    if map.is_null() { of_node_put((*dma_spec).np); map = err_ptr(-12); goto_out_put_pdev!(pdev, map); }
    (*map).dma_line = (*dma_spec).args[0] as u16; (*map).mux_val = (*dma_spec).args[2] as u8;
    (*dma_spec).args[2] = 0; (*dma_spec).args_count = 2;
    dev_dbg(&mut (*pdev).dev, "Mapping XBAR event%u to DMA%u\n", (*map).mux_val, (*map).dma_line);
    ti_am335x_xbar_write((*xbar).iomem, (*map).dma_line as i32, (*map).mux_val);
    put_device(&mut (*pdev).dev); map as *mut c_void
}

// Crossbar on DRA7xx family
const TI_DRA7_XBAR_OUTPUTS: u32 = 127;
const TI_DRA7_XBAR_INPUTS: u32 = 256;
#[repr(C)] struct TiDra7XbarData { iomem: *mut c_void, dmarouter: DmaRouter, mutex: Mutex, dma_inuse: *mut usize, safe_val: u16, xbar_requests: u32, dma_requests: u32, dma_offset: u32 }
#[repr(C)] struct TiDra7XbarMap { xbar_in: u16, xbar_out: i32 }
#[repr(C)] struct Mutex { _private: [u8; 0] }
unsafe fn writew_relaxed(_: u16, _: *mut u8) {}
unsafe fn ti_dra7_xbar_write(iomem: *mut c_void, xbar: i32, val: u16) { writew_relaxed(val, (iomem as *mut u8).offset((xbar * 2) as isize)); }
unsafe fn ti_dra7_xbar_free(dev: *mut Device, route_data: *mut c_void) { let xbar=dev_get_drvdata(dev) as *mut TiDra7XbarData; let map=route_data as *mut TiDra7XbarMap; dev_dbg(dev,"Unmapping XBAR%u (was routed to %d)\n",(*map).xbar_in,(*map).xbar_out as u16); ti_dra7_xbar_write((*xbar).iomem,(*map).xbar_out,(*xbar).safe_val); mutex_lock(&mut (*xbar).mutex); clear_bit((*map).xbar_out,(*xbar).dma_inuse); mutex_unlock(&mut (*xbar).mutex); kfree(map); }
unsafe fn ti_dra7_xbar_reserve(mut offset:i32, mut len:i32, p:*mut usize) { while len>0 { set_bit(offset+len-1,p); len-=1; } }
unsafe fn clear_bit(_:i32,_:*mut usize){} unsafe fn set_bit(_:i32,_:*mut usize){} unsafe fn test_bit(_:i32,_:*mut usize)->bool{false} unsafe fn mutex_lock(_: &mut Mutex){} unsafe fn mutex_unlock(_: &mut Mutex){}
unsafe fn ti_dra7_xbar_route_allocate(_: *mut OfPhandleArgs, _: *mut OfDma) -> *mut c_void { err_ptr(-22) as *mut c_void }
const TI_XBAR_EDMA_OFFSET:u32=0; const TI_XBAR_SDMA_OFFSET:u32=1; static TI_DMA_OFFSET:[u32;2]=[0,1];
static TI_DRA7_MASTER_MATCH:[OfDeviceId;4]=[
 OfDeviceId{compatible:b"ti,omap4430-sdma\0".as_ptr(),data:&TI_DMA_OFFSET[1] as *const u32 as *const c_void},
 OfDeviceId{compatible:b"ti,edma3\0".as_ptr(),data:&TI_DMA_OFFSET[0] as *const u32 as *const c_void},
 OfDeviceId{compatible:b"ti,edma3-tpcc\0".as_ptr(),data:&TI_DMA_OFFSET[0] as *const u32 as *const c_void}, OfDeviceId{compatible:core::ptr::null(),data:core::ptr::null()}];
unsafe fn ti_dra7_xbar_probe(_: *mut PlatformDevice)->i32 { -19 }
unsafe fn ti_am335x_xbar_probe(_: *mut PlatformDevice)->i32 { -19 }
unsafe fn ti_dma_xbar_probe(pdev:*mut PlatformDevice)->i32 { let _=pdev; -22 }
static mut TI_DMA_XBAR_DRIVER: *mut PlatformDriver = core::ptr::null_mut();
unsafe fn omap_dmaxbar_init()->i32 { platform_driver_register(TI_DMA_XBAR_DRIVER) }
extern "C" {
    fn ti_dra7_xbar_probe(pdev: *mut PlatformDevice) -> i32;
    fn ti_am335x_xbar_probe(pdev: *mut PlatformDevice) -> i32;
    fn platform_driver_register(driver: *mut PlatformDriver) -> i32;
}

#[repr(C)] struct Device { _private: [u8; 0] }
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct DeviceNode { _private: [u8; 0] }
#[repr(C)] struct OfDma { of_node: *mut DeviceNode }
#[repr(C)] struct OfPhandleArgs { args_count: u32, args: [u32; 8], np: *mut DeviceNode }
#[repr(C)] struct DmaRouter { dev: *mut Device, route_free: Option<unsafe fn(*mut Device, *mut c_void)> }
#[repr(C)] struct PlatformDriver { _private: [u8; 0] }

unsafe fn writeb_relaxed(_: u8, _: *mut u8) {}
unsafe fn dev_get_drvdata(_: *mut Device) -> *mut c_void { core::ptr::null_mut() }
unsafe fn of_find_device_by_node(_: *mut DeviceNode) -> *mut PlatformDevice { core::ptr::null_mut() }
unsafe fn platform_get_drvdata(_: *mut PlatformDevice) -> *mut c_void { core::ptr::null_mut() }
unsafe fn put_device(_: *mut Device) {}
unsafe fn of_parse_phandle(_: *mut DeviceNode, _: *const u8, _: i32) -> *mut DeviceNode { core::ptr::null_mut() }
unsafe fn of_node_put(_: *mut DeviceNode) {}
unsafe fn kfree(_: *mut c_void) {}
unsafe fn err_ptr(_: i32) -> *mut TiAm335xXbarMap { core::ptr::null_mut() }
unsafe fn kzalloc<T>() -> *mut T { core::ptr::null_mut() }
unsafe fn dev_dbg(_: *mut Device, _: &str, _: u8, _: u16) {}
unsafe fn dev_err(_: *mut Device, _: &str, _: u32) {}

macro_rules! goto_out_put_pdev { ($pdev:expr, $map:expr) => {{ put_device(&mut (*$pdev).dev); return $map as *mut c_void; }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
