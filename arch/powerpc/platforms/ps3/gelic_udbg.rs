// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * udbg debug output routine via GELIC UDP broadcasts
 *
 * Copyright (C) 2007 Sony Computer Entertainment Inc.
 * Copyright 2006, 2007 Sony Corporation
 * Copyright (C) 2010 Hector Martin <hector@marcansoft.com>
 * Copyright (C) 2011 Andre Heider <aheider@gmail.com>
 */

const GELIC_BUS_ID: i32 = 1;
const GELIC_DEVICE_ID: i32 = 0;
const GELIC_DEBUG_PORT: u16 = 18194;
const GELIC_MAX_MESSAGE_SIZE: usize = 1000;

const GELIC_LV1_GET_MAC_ADDRESS: u64 = 1;
const GELIC_LV1_GET_VLAN_ID: u64 = 4;
const GELIC_LV1_VLAN_TX_ETHERNET_0: u64 = 2;

const GELIC_DESCR_DMA_STAT_MASK: u32 = 0xf0000000;
const GELIC_DESCR_DMA_CARDOWNED: u32 = 0xa0000000;
const GELIC_DESCR_TX_DMA_IKE: u32 = 0x00080000;
const GELIC_DESCR_TX_DMA_NO_CHKSUM: u32 = 0x00000000;
const GELIC_DESCR_TX_DMA_FRAME_TAIL: u32 = 0x00040000;
const GELIC_DESCR_DMA_CMD_NO_CHKSUM: u32 =
    GELIC_DESCR_DMA_CARDOWNED | GELIC_DESCR_TX_DMA_IKE | GELIC_DESCR_TX_DMA_NO_CHKSUM;

#[repr(C, align(32))]
struct GelicDescr {
    buf_addr: u32,
    buf_size: u32,
    next_descr_addr: u32,
    dmac_cmd_status: u32,
    result_size: u32,
    valid_size: u32,
    data_status: u32,
    data_error: u32,
}

#[repr(C, align(32))]
struct DebugBlock {
    descr: GelicDescr,
    pkt: [u8; 1520],
}

#[repr(C)]
struct EthHdr { h_dest: [u8; 6], h_source: [u8; 6], h_proto: u16 }
#[repr(C)]
struct VlanHdr { h_vlan_TCI: u16, h_vlan_encapsulated_proto: u16 }
#[repr(C)]
struct IpHdr {
    ihl: u8, version: u8, tos: u8, tot_len: u16, id: u16, frag_off: u16,
    ttl: u8, protocol: u8, check: u16, saddr: u32, daddr: u32,
}
#[repr(C)]
struct UdpHdr { source: u16, dest: u16, len: u16, check: u16 }

extern "C" {
    fn lv1_allocate_device_dma_region(bus_id: i32, dev_id: i32, len: u64, page_size: u64,
                                       flags: u64, bus_addr: *mut u64) -> i64;
    fn lv1_map_device_dma_region(bus_id: i32, dev_id: i32, real_addr: u64, bus_addr: u64,
                                 len: u64, flags: u64) -> i64;
    fn lv1_unmap_device_dma_region(bus_id: i32, dev_id: i32, bus_addr: u64, len: u64) -> i64;
    fn lv1_free_device_dma_region(bus_id: i32, dev_id: i32, bus_addr: u64) -> i64;
    fn lv1_panic(code: u64) -> !;
    fn lv1_open_device(bus_id: i32, dev_id: i32, flags: u64) -> i64;
    fn lv1_close_device(bus_id: i32, dev_id: i32) -> i64;
    fn lv1_net_control(bus_id: i32, dev_id: i32, command: u64, a3: u64, a4: u64, a5: u64,
                       out1: *mut u64, out2: *mut u64) -> i64;
    fn lv1_net_start_tx_dma(bus_id: i32, dev_id: i32, bus_addr: u64, flags: u64) -> i64;
    fn wmb();
    fn cpu_relax();
    static mut udbg_putc: Option<unsafe extern "C" fn(char)>;
}

static mut bus_addr: u64 = 0;
static mut h_eth: *mut EthHdr = core::ptr::null_mut();
static mut h_vlan: *mut VlanHdr = core::ptr::null_mut();
static mut h_ip: *mut IpHdr = core::ptr::null_mut();
static mut h_udp: *mut UdpHdr = core::ptr::null_mut();
static mut pmsg: *mut u8 = core::ptr::null_mut();
static mut pmsgc: *mut u8 = core::ptr::null_mut();
static mut dbg: DebugBlock = DebugBlock { descr: GelicDescr { buf_addr: 0, buf_size: 0, next_descr_addr: 0, dmac_cmd_status: 0, result_size: 0, valid_size: 0, data_status: 0, data_error: 0 }, pkt: [0; 1520] };
static mut header_size: usize = 0;

unsafe fn map_dma_mem(bus_id: i32, dev_id: i32, start: *mut core::ffi::c_void, len: usize, real_bus_addr: *mut u64) {
    let real_addr = (start as u64) & 0x0fffffffffffffff;
    let real_end = real_addr + len as u64;
    let map_start = real_addr & !0xfff;
    let map_end = (real_end + 0xfff) & !0xfff;
    let mut dma_bus_addr = 0u64;
    let flags = 0xf800000000000000u64;
    if lv1_allocate_device_dma_region(bus_id, dev_id, map_end - map_start, 12, 0, &mut dma_bus_addr) != 0 { lv1_panic(0); }
    if lv1_map_device_dma_region(bus_id, dev_id, map_start, dma_bus_addr, map_end - map_start, flags) != 0 { lv1_panic(0); }
    *real_bus_addr = dma_bus_addr + real_addr - map_start;
}

unsafe fn unmap_dma_mem(bus_id: i32, dev_id: i32, dma_bus_addr: u64, mut len: usize) -> i64 {
    let real_bus_addr = dma_bus_addr & !0xfff;
    len += (dma_bus_addr - real_bus_addr) as usize;
    len = (len + 0xfff) & !0xfff;
    let result = lv1_unmap_device_dma_region(bus_id, dev_id, real_bus_addr, len as u64);
    if result != 0 { return result; }
    lv1_free_device_dma_region(bus_id, dev_id, real_bus_addr)
}

unsafe fn gelic_debug_init() {
    let mut v2 = 0u64; let mut mac = 0u64; let mut vlan_id = 0u64;
    if lv1_open_device(GELIC_BUS_ID, GELIC_DEVICE_ID, 0) != 0 { lv1_panic(0); }
    map_dma_mem(GELIC_BUS_ID, GELIC_DEVICE_ID, core::ptr::addr_of_mut!(dbg).cast(), core::mem::size_of::<DebugBlock>(), &mut bus_addr);
    core::ptr::write_bytes(core::ptr::addr_of_mut!(dbg).cast::<u8>(), 0, core::mem::size_of::<DebugBlock>());
    dbg.descr.buf_addr = (bus_addr + core::mem::offset_of!(DebugBlock, pkt) as u64) as u32;
    wmb();
    if lv1_net_control(GELIC_BUS_ID, GELIC_DEVICE_ID, GELIC_LV1_GET_MAC_ADDRESS, 0, 0, 0, &mut mac, &mut v2) != 0 { lv1_panic(0); }
    mac <<= 16;
    h_eth = dbg.pkt.as_mut_ptr().cast(); (*h_eth).h_dest = [0xff; 6]; (*h_eth).h_source.copy_from_slice(&mac.to_be_bytes()[2..]);
    header_size = core::mem::size_of::<EthHdr>();
    if lv1_net_control(GELIC_BUS_ID, GELIC_DEVICE_ID, GELIC_LV1_GET_VLAN_ID, GELIC_LV1_VLAN_TX_ETHERNET_0, 0, 0, &mut vlan_id, &mut v2) == 0 {
        (*h_eth).h_proto = 0x8100u16.to_be(); header_size += core::mem::size_of::<VlanHdr>(); h_vlan = h_eth.add(1).cast(); (*h_vlan).h_vlan_TCI = vlan_id as u16; (*h_vlan).h_vlan_encapsulated_proto = 0x0800u16.to_be(); h_ip = h_vlan.add(1).cast();
    } else { (*h_eth).h_proto = 0x0800u16.to_be(); h_ip = h_eth.add(1).cast(); }
    header_size += core::mem::size_of::<IpHdr>(); (*h_ip).version = 4; (*h_ip).ihl = 5; (*h_ip).ttl = 10; (*h_ip).protocol = 0x11; (*h_ip).saddr = 0; (*h_ip).daddr = 0xffffffff;
    header_size += core::mem::size_of::<UdpHdr>(); h_udp = h_ip.add(1).cast(); (*h_udp).source = GELIC_DEBUG_PORT.to_be(); (*h_udp).dest = GELIC_DEBUG_PORT.to_be(); pmsgc = h_udp.add(1).cast(); pmsg = pmsgc;
}

unsafe fn gelic_debug_shutdown() { if bus_addr != 0 { unmap_dma_mem(GELIC_BUS_ID, GELIC_DEVICE_ID, bus_addr, core::mem::size_of::<DebugBlock>()); } lv1_close_device(GELIC_BUS_ID, GELIC_DEVICE_ID); }

unsafe fn gelic_sendbuf(msgsize: usize) {
    dbg.descr.buf_size = (header_size + msgsize) as u32; (*h_ip).tot_len = (msgsize + core::mem::size_of::<UdpHdr>() + core::mem::size_of::<IpHdr>()) as u16; (*h_udp).len = (msgsize + core::mem::size_of::<UdpHdr>()) as u16; (*h_ip).check = 0;
    let mut sum = 0u32; let p = h_ip.cast::<u16>(); for i in 0..5 { sum += *p.add(i) as u32; } (*h_ip).check = (!(sum + (sum >> 16))) as u16;
    dbg.descr.dmac_cmd_status = GELIC_DESCR_DMA_CMD_NO_CHKSUM | GELIC_DESCR_TX_DMA_FRAME_TAIL; dbg.descr.result_size = 0; dbg.descr.data_status = 0; wmb(); lv1_net_start_tx_dma(GELIC_BUS_ID, GELIC_DEVICE_ID, bus_addr, 0);
    while (dbg.descr.dmac_cmd_status & GELIC_DESCR_DMA_STAT_MASK) == GELIC_DESCR_DMA_CARDOWNED { cpu_relax(); }
}

unsafe extern "C" fn ps3gelic_udbg_putc(ch: char) { *pmsgc = ch as u8; pmsgc = pmsgc.add(1); if ch == '\n' || pmsgc.offset_from(pmsg) as usize >= GELIC_MAX_MESSAGE_SIZE { gelic_sendbuf(pmsgc.offset_from(pmsg) as usize); pmsgc = pmsg; } }

pub unsafe extern "C" fn udbg_init_ps3gelic() { gelic_debug_init(); udbg_putc = Some(ps3gelic_udbg_putc); }
#[no_mangle]
pub unsafe extern "C" fn udbg_shutdown_ps3gelic() { udbg_putc = None; gelic_debug_shutdown(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
