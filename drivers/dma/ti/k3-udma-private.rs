// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 Texas Instruments Incorporated - http://www.ti.com
 * Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// Linux/device-tree and platform-driver declarations are supplied by the
// surrounding kernel translation.

extern "C" {
    fn navss_psil_pair(ud: *mut udma_dev, src_thread: u32, dst_thread: u32) -> i32;
    fn navss_psil_unpair(ud: *mut udma_dev, src_thread: u32, dst_thread: u32) -> i32;
    fn of_parse_phandle(np: *mut device_node, property: *const i8, index: i32) -> *mut device_node;
    fn of_find_device_by_node(np: *mut device_node) -> *mut platform_device;
    fn of_node_put(np: *mut device_node);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut udma_dev;
    fn put_device(dev: *mut device);
    fn __udma_alloc_gp_rflow_range(ud: *mut udma_dev, from: i32, cnt: i32) -> i32;
    fn __udma_free_gp_rflow_range(ud: *mut udma_dev, from: i32, cnt: i32) -> i32;
    fn test_bit(id: i32, map: *const u64) -> bool;
    fn __udma_reserve_tchan(ud: *mut udma_dev, tp: i32, id: i32) -> *mut udma_tchan;
    fn __udma_reserve_rchan(ud: *mut udma_dev, tp: i32, id: i32) -> *mut udma_rchan;
    fn clear_bit(id: i32, map: *mut u64);
    fn __udma_get_rflow(ud: *mut udma_dev, id: i32) -> *mut udma_rflow;
    fn __udma_put_rflow(ud: *mut udma_dev, p: *mut udma_rflow);
    fn udma_read(reg: *mut u8, offset: i32) -> u32;
    fn udma_write(reg: *mut u8, offset: i32, val: u32);
    fn msi_get_virq(dev: *mut device, irq: i32) -> i32;
}

// These layouts and constants are provided by the corresponding kernel headers.
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct k3_ringacc { _private: [u8; 0] }
#[repr(C)] pub struct udma_tisci_rm { _private: [u8; 0] }
#[repr(C)] pub struct udma_tchan { pub id: i32, pub reg_rt: *mut u8 }
#[repr(C)] pub struct udma_rchan { pub id: i32, pub reg_rt: *mut u8 }
#[repr(C)] pub struct udma_rflow { pub id: i32 }
#[repr(C)] pub struct udma_oes_offsets { pub pktdma_tchan_flow: i32, pub pktdma_rchan_flow: i32 }
#[repr(C)] pub struct udma_soc_data { pub oes: udma_oes_offsets }
#[repr(C)] pub struct udma_match_data { pub r#type: i32 }
#[repr(C)] pub struct udma_dev {
    pub dev: *mut device,
    pub ringacc: *mut k3_ringacc,
    pub psil_base: u32,
    pub tisci_rm: udma_tisci_rm,
    pub rflow_gp_map: *mut u64,
    pub tflow_cnt: i32,
    pub match_data: *const udma_match_data,
    pub soc_data: *const udma_soc_data,
    pub tchan_map: *mut u64,
    pub rchan_map: *mut u64,
}

const UDMA_TP_NORMAL: i32 = 0;
const DMA_TYPE_PKTDMA: i32 = 1;

pub unsafe fn xudma_navss_psil_pair(ud: *mut udma_dev, src_thread: u32, dst_thread: u32) -> i32 {
    navss_psil_pair(ud, src_thread, dst_thread)
}

pub unsafe fn xudma_navss_psil_unpair(ud: *mut udma_dev, src_thread: u32, dst_thread: u32) -> i32 {
    navss_psil_unpair(ud, src_thread, dst_thread)
}

pub unsafe fn of_xudma_dev_get(np: *mut device_node, property: *const i8) -> *mut udma_dev {
    let mut udma_node = np;
    let pdev: *mut platform_device;
    let ud: *mut udma_dev;
    if !property.is_null() {
        udma_node = of_parse_phandle(np, property, 0);
        if udma_node.is_null() { return (-19isize) as *mut udma_dev; }
    }
    pdev = of_find_device_by_node(udma_node);
    if np != udma_node { of_node_put(udma_node); }
    if pdev.is_null() { return (-517isize) as *mut udma_dev; }
    ud = platform_get_drvdata(pdev);
    put_device(&mut (*pdev).dev);
    if ud.is_null() { return (-517isize) as *mut udma_dev; }
    ud
}

pub unsafe fn xudma_get_device(ud: *mut udma_dev) -> *mut device { (*ud).dev }
pub unsafe fn xudma_get_ringacc(ud: *mut udma_dev) -> *mut k3_ringacc { (*ud).ringacc }
pub unsafe fn xudma_dev_get_psil_base(ud: *mut udma_dev) -> u32 { (*ud).psil_base }
pub unsafe fn xudma_dev_get_tisci_rm(ud: *mut udma_dev) -> *mut udma_tisci_rm { &mut (*ud).tisci_rm }
pub unsafe fn xudma_alloc_gp_rflow_range(ud: *mut udma_dev, from: i32, cnt: i32) -> i32 { __udma_alloc_gp_rflow_range(ud, from, cnt) }
pub unsafe fn xudma_free_gp_rflow_range(ud: *mut udma_dev, from: i32, cnt: i32) -> i32 { __udma_free_gp_rflow_range(ud, from, cnt) }
pub unsafe fn xudma_rflow_is_gp(ud: *mut udma_dev, id: i32) -> bool {
    if (*ud).rflow_gp_map.is_null() { return false; }
    !test_bit(id, (*ud).rflow_gp_map)
}

pub unsafe fn xudma_tchan_get(ud: *mut udma_dev, id: i32) -> *mut udma_tchan { __udma_reserve_tchan(ud, UDMA_TP_NORMAL, id) }
pub unsafe fn xudma_tchan_put(ud: *mut udma_dev, p: *mut udma_tchan) { clear_bit((*p).id, (*ud).tchan_map); }
pub unsafe fn xudma_rchan_get(ud: *mut udma_dev, id: i32) -> *mut udma_rchan { __udma_reserve_rchan(ud, UDMA_TP_NORMAL, id) }
pub unsafe fn xudma_rchan_put(ud: *mut udma_dev, p: *mut udma_rchan) { clear_bit((*p).id, (*ud).rchan_map); }
pub unsafe fn xudma_rflow_get(ud: *mut udma_dev, id: i32) -> *mut udma_rflow { __udma_get_rflow(ud, id) }
pub unsafe fn xudma_rflow_put(ud: *mut udma_dev, p: *mut udma_rflow) { __udma_put_rflow(ud, p); }
pub unsafe fn xudma_get_rflow_ring_offset(ud: *mut udma_dev) -> i32 { (*ud).tflow_cnt }

pub unsafe fn xudma_tchan_get_id(p: *mut udma_tchan) -> i32 { (*p).id }
pub unsafe fn xudma_rchan_get_id(p: *mut udma_rchan) -> i32 { (*p).id }
pub unsafe fn xudma_rflow_get_id(p: *mut udma_rflow) -> i32 { (*p).id }

pub unsafe fn xudma_tchanrt_read(p: *mut udma_tchan, reg: i32) -> u32 { if p.is_null() { 0 } else { udma_read((*p).reg_rt, reg) } }
pub unsafe fn xudma_tchanrt_write(p: *mut udma_tchan, reg: i32, val: u32) { if !p.is_null() { udma_write((*p).reg_rt, reg, val); } }
pub unsafe fn xudma_rchanrt_read(p: *mut udma_rchan, reg: i32) -> u32 { if p.is_null() { 0 } else { udma_read((*p).reg_rt, reg) } }
pub unsafe fn xudma_rchanrt_write(p: *mut udma_rchan, reg: i32, val: u32) { if !p.is_null() { udma_write((*p).reg_rt, reg, val); } }

pub unsafe fn xudma_is_pktdma(ud: *mut udma_dev) -> i32 { ((*(*ud).match_data).r#type == DMA_TYPE_PKTDMA) as i32 }
pub unsafe fn xudma_pktdma_tflow_get_irq(ud: *mut udma_dev, id: i32) -> i32 { msi_get_virq((*ud).dev, id + (*(*ud).soc_data).oes.pktdma_tchan_flow) }
pub unsafe fn xudma_pktdma_rflow_get_irq(ud: *mut udma_dev, id: i32) -> i32 { msi_get_virq((*ud).dev, id + (*(*ud).soc_data).oes.pktdma_rchan_flow) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
