// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018, NVIDIA CORPORATION.
 */

// Linux kernel dependencies supplied by the surrounding repository.

const TRIGGER_OFFSET: usize = 0x000;
const TRIGGER_ID_SHIFT: u32 = 16;
const TRIGGER_CMD_GET: u32 = 4;

const STA_OFFSET: usize = 0;
const SET_OFFSET: usize = 4;
const CLR_OFFSET: usize = 8;

#[inline]
const fn result_offset(id: usize) -> usize { 0xc00 + id * 4 }
#[inline]
const fn ch_mask(ch: u32) -> u32 { 0x3u32 << (ch * 2) }
#[inline]
const fn sl_sigl(ch: u32) -> u32 { 0x0u32 << (ch * 2) }
#[inline]
const fn sl_qued(ch: u32) -> u32 { 0x1u32 << (ch * 2) }
#[inline]
const fn ma_free(ch: u32) -> u32 { 0x2u32 << (ch * 2) }
#[inline]
const fn ma_ackd(ch: u32) -> u32 { 0x3u32 << (ch * 2) }

#[repr(C)]
struct Tegra210Bpmp {
    atomics: *mut core::ffi::c_void,
    arb_sema: *mut core::ffi::c_void,
    tx_irq_data: *mut irq_data,
}

unsafe fn bpmp_channel_status(bpmp: *mut tegra_bpmp, index: u32) -> u32 {
    let priv_data = (*bpmp).priv_data as *mut Tegra210Bpmp;
    core::ptr::read_volatile((*priv_data).arb_sema.cast::<u32>().add(STA_OFFSET / 4))
        & ch_mask(index)
}

unsafe fn tegra210_bpmp_is_response_ready(channel: *mut tegra_bpmp_channel) -> bool {
    let index = (*channel).index;
    bpmp_channel_status((*channel).bpmp, index) == ma_ackd(index)
}

unsafe fn tegra210_bpmp_is_request_ready(channel: *mut tegra_bpmp_channel) -> bool {
    let index = (*channel).index;
    bpmp_channel_status((*channel).bpmp, index) == sl_sigl(index)
}

unsafe fn tegra210_bpmp_is_request_channel_free(channel: *mut tegra_bpmp_channel) -> bool {
    let index = (*channel).index;
    bpmp_channel_status((*channel).bpmp, index) == ma_free(index)
}

unsafe fn tegra210_bpmp_is_response_channel_free(channel: *mut tegra_bpmp_channel) -> bool {
    let index = (*channel).index;
    bpmp_channel_status((*channel).bpmp, index) == sl_qued(index)
}

unsafe fn tegra210_bpmp_post_request(channel: *mut tegra_bpmp_channel) -> i32 {
    let priv_data = (*(*channel).bpmp).priv_data as *mut Tegra210Bpmp;
    core::ptr::write_volatile(
        (*priv_data).arb_sema.cast::<u32>().add(CLR_OFFSET / 4),
        ch_mask((*channel).index),
    );
    0
}

unsafe fn tegra210_bpmp_post_response(channel: *mut tegra_bpmp_channel) -> i32 {
    let priv_data = (*(*channel).bpmp).priv_data as *mut Tegra210Bpmp;
    core::ptr::write_volatile(
        (*priv_data).arb_sema.cast::<u32>().add(SET_OFFSET / 4),
        ma_ackd((*channel).index),
    );
    0
}

unsafe fn tegra210_bpmp_ack_response(channel: *mut tegra_bpmp_channel) -> i32 {
    let priv_data = (*(*channel).bpmp).priv_data as *mut Tegra210Bpmp;
    core::ptr::write_volatile(
        (*priv_data).arb_sema.cast::<u32>().add(CLR_OFFSET / 4),
        ma_ackd((*channel).index) ^ ma_free((*channel).index),
    );
    0
}

unsafe fn tegra210_bpmp_ack_request(channel: *mut tegra_bpmp_channel) -> i32 {
    let priv_data = (*(*channel).bpmp).priv_data as *mut Tegra210Bpmp;
    core::ptr::write_volatile(
        (*priv_data).arb_sema.cast::<u32>().add(SET_OFFSET / 4),
        sl_qued((*channel).index),
    );
    0
}

unsafe fn tegra210_bpmp_ring_doorbell(bpmp: *mut tegra_bpmp) -> i32 {
    let priv_data = (*bpmp).priv_data as *mut Tegra210Bpmp;
    let irq_data = (*priv_data).tx_irq_data;

    /* Tegra Legacy Interrupt Controller (LIC) is used to notify BPMP of
     * available messages. */
    if !irq_data.is_null() && !(*irq_data).chip.is_null() {
        if let Some(retrigger) = (*(*irq_data).chip).irq_retrigger {
            return retrigger(irq_data);
        }
    }
    -22
}

unsafe extern "C" fn rx_irq(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    tegra_bpmp_handle_rx(data as *mut tegra_bpmp);
    IRQ_HANDLED
}

unsafe fn tegra210_bpmp_channel_init(
    channel: *mut tegra_bpmp_channel,
    bpmp: *mut tegra_bpmp,
    index: u32,
) -> i32 {
    let priv_data = (*bpmp).priv_data as *mut Tegra210Bpmp;
    core::ptr::write_volatile(
        (*priv_data).atomics.cast::<u32>().add(TRIGGER_OFFSET / 4),
        (index << TRIGGER_ID_SHIFT) | TRIGGER_CMD_GET,
    );
    let address = core::ptr::read_volatile(
        (*priv_data).atomics.cast::<u32>().add(result_offset(index as usize) / 4),
    );
    let p = devm_ioremap((*bpmp).dev, address as usize, 0x80);
    if p.is_null() { return -12; }

    iosys_map_set_vaddr_iomem(&mut (*channel).ib, p);
    iosys_map_set_vaddr_iomem(&mut (*channel).ob, p);
    (*channel).index = index;
    init_completion(&mut (*channel).completion);
    (*channel).bpmp = bpmp;
    0
}

unsafe fn tegra210_bpmp_init(bpmp: *mut tegra_bpmp) -> i32 {
    let pdev = to_platform_device((*bpmp).dev);
    let priv_data = devm_kzalloc((*pdev).dev, core::mem::size_of::<Tegra210Bpmp>(), GFP_KERNEL);
    if priv_data.is_null() { return -12; }
    (*bpmp).priv_data = priv_data as *mut core::ffi::c_void;
    let priv_data = priv_data as *mut Tegra210Bpmp;

    (*priv_data).atomics = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*priv_data).atomics) { return ptr_err((*priv_data).atomics); }
    (*priv_data).arb_sema = devm_platform_ioremap_resource(pdev, 1);
    if is_err((*priv_data).arb_sema) { return ptr_err((*priv_data).arb_sema); }

    let mut err = tegra210_bpmp_channel_init((*bpmp).tx_channel, bpmp, (*(*bpmp).soc).channels.cpu_tx.offset);
    if err < 0 { return err; }
    err = tegra210_bpmp_channel_init((*bpmp).rx_channel, bpmp, (*(*bpmp).soc).channels.cpu_rx.offset);
    if err < 0 { return err; }
    for i in 0..(*bpmp).threaded.count {
        let index = (*(*bpmp).soc).channels.thread.offset + i;
        err = tegra210_bpmp_channel_init((*bpmp).threaded_channels.add(i as usize), bpmp, index);
        if err < 0 { return err; }
    }
    err = platform_get_irq_byname(pdev, b"tx\0".as_ptr() as *const i8);
    if err < 0 { return err; }
    (*priv_data).tx_irq_data = irq_get_irq_data(err);
    if (*priv_data).tx_irq_data.is_null() { return -2; }
    err = platform_get_irq_byname(pdev, b"rx\0".as_ptr() as *const i8);
    if err < 0 { return err; }
    err = devm_request_irq((*pdev).dev, err, rx_irq, IRQF_NO_SUSPEND, dev_name((*pdev).dev), bpmp as *mut core::ffi::c_void);
    if err < 0 { return err; }
    0
}

// Corresponds to tegra210_bpmp_ops; its concrete definition is supplied by
// the BPMP interface declarations.
extern "C" {
    static tegra210_bpmp_ops: tegra_bpmp_ops;
}

extern "C" {
    fn tegra_bpmp_handle_rx(bpmp: *mut tegra_bpmp);
    fn to_platform_device(dev: *mut core::ffi::c_void) -> *mut platform_device;
    fn devm_kzalloc(dev: *mut core::ffi::c_void, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut core::ffi::c_void;
    fn is_err(ptr: *mut core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
    fn platform_get_irq_byname(pdev: *mut platform_device, name: *const i8) -> i32;
    fn irq_get_irq_data(irq: i32) -> *mut irq_data;
    fn devm_request_irq(dev: *mut core::ffi::c_void, irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t, flags: u32, name: *const i8, data: *mut core::ffi::c_void) -> i32;
    fn dev_name(dev: *mut core::ffi::c_void) -> *const i8;
    fn devm_ioremap(dev: *mut core::ffi::c_void, address: usize, size: usize) -> *mut core::ffi::c_void;
    fn iosys_map_set_vaddr_iomem(map: *mut core::ffi::c_void, addr: *mut core::ffi::c_void);
    fn init_completion(completion: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
