/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * RapidIO driver services
 *
 * Copyright 2005 MontaVista Software, Inc.
 * Matt Porter <mporter@kernel.crashing.org>
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    pub fn __rio_local_read_config_32(port: *mut rio_mport, offset: u32, data: *mut u32) -> i32;
    pub fn __rio_local_write_config_32(port: *mut rio_mport, offset: u32, data: u32) -> i32;
    pub fn __rio_local_read_config_16(port: *mut rio_mport, offset: u32, data: *mut u16) -> i32;
    pub fn __rio_local_write_config_16(port: *mut rio_mport, offset: u32, data: u16) -> i32;
    pub fn __rio_local_read_config_8(port: *mut rio_mport, offset: u32, data: *mut u8) -> i32;
    pub fn __rio_local_write_config_8(port: *mut rio_mport, offset: u32, data: u8) -> i32;

    pub fn rio_mport_read_config_32(port: *mut rio_mport, destid: u16, hopcount: u8, offset: u32, data: *mut u32) -> i32;
    pub fn rio_mport_write_config_32(port: *mut rio_mport, destid: u16, hopcount: u8, offset: u32, data: u32) -> i32;
    pub fn rio_mport_read_config_16(port: *mut rio_mport, destid: u16, hopcount: u8, offset: u32, data: *mut u16) -> i32;
    pub fn rio_mport_write_config_16(port: *mut rio_mport, destid: u16, hopcount: u8, offset: u32, data: u16) -> i32;
    pub fn rio_mport_read_config_8(port: *mut rio_mport, destid: u16, hopcount: u8, offset: u32, data: *mut u8) -> i32;
    pub fn rio_mport_write_config_8(port: *mut rio_mport, destid: u16, hopcount: u8, offset: u32, data: u8) -> i32;

    pub fn rio_mport_send_doorbell(mport: *mut rio_mport, destid: u16, data: u16) -> i32;

    pub fn rio_request_outb_mbox(mport: *mut rio_mport, dev_id: *mut core::ffi::c_void, mbox: i32, entries: i32, callback: Option<unsafe extern "C" fn(*mut rio_mport, *mut core::ffi::c_void, i32, i32)> ) -> i32;
    pub fn rio_release_outb_mbox(mport: *mut rio_mport, mbox: i32) -> i32;
    pub fn rio_request_inb_mbox(mport: *mut rio_mport, dev_id: *mut core::ffi::c_void, mbox: i32, entries: i32, callback: Option<unsafe extern "C" fn(*mut rio_mport, *mut core::ffi::c_void, i32, i32)> ) -> i32;
    pub fn rio_release_inb_mbox(mport: *mut rio_mport, mbox: i32) -> i32;
    pub fn rio_request_inb_dbell(mport: *mut rio_mport, dev_id: *mut core::ffi::c_void, start: u16, end: u16, callback: Option<unsafe extern "C" fn(*mut rio_mport, *mut core::ffi::c_void, u16, u16, u16)> ) -> i32;
    pub fn rio_release_inb_dbell(mport: *mut rio_mport, start: u16, end: u16) -> i32;
    pub fn rio_request_outb_dbell(rdev: *mut rio_dev, start: u16, end: u16) -> *mut resource;
    pub fn rio_release_outb_dbell(rdev: *mut rio_dev, res: *mut resource) -> i32;

    pub fn rio_claim_resource(rdev: *mut rio_dev, resource: i32) -> i32;
    pub fn rio_request_regions(rdev: *mut rio_dev, name: *mut u8) -> i32;
    pub fn rio_release_regions(rdev: *mut rio_dev);
    pub fn rio_request_region(rdev: *mut rio_dev, resource: i32, name: *mut u8) -> i32;
    pub fn rio_release_region(rdev: *mut rio_dev, resource: i32);

    pub fn rio_map_inb_region(mport: *mut rio_mport, local: dma_addr_t, rbase: u64, size: u32, rflags: u32) -> i32;
    pub fn rio_unmap_inb_region(mport: *mut rio_mport, lstart: dma_addr_t);
    pub fn rio_map_outb_region(mport: *mut rio_mport, destid: u16, rbase: u64, size: u32, rflags: u32, local: *mut dma_addr_t) -> i32;
    pub fn rio_unmap_outb_region(mport: *mut rio_mport, destid: u16, rstart: u64);

    pub fn rio_request_inb_pwrite(rdev: *mut rio_dev, callback: Option<unsafe extern "C" fn(*mut rio_dev, *mut rio_pw_msg, i32)> ) -> i32;
    pub fn rio_release_inb_pwrite(rdev: *mut rio_dev) -> i32;
    pub fn rio_add_mport_pw_handler(mport: *mut rio_mport, dev_id: *mut core::ffi::c_void, callback: Option<unsafe extern "C" fn(*mut rio_mport, *mut core::ffi::c_void, *mut rio_pw_msg, i32)> ) -> i32;
    pub fn rio_del_mport_pw_handler(mport: *mut rio_mport, dev_id: *mut core::ffi::c_void, callback: Option<unsafe extern "C" fn(*mut rio_mport, *mut core::ffi::c_void, *mut rio_pw_msg, i32)> ) -> i32;
    pub fn rio_inb_pwrite_handler(mport: *mut rio_mport, pw_msg: *mut rio_pw_msg) -> i32;
    pub fn rio_pw_enable(mport: *mut rio_mport, enable: i32);

    pub fn rio_register_driver(driver: *mut rio_driver) -> i32;
    pub fn rio_unregister_driver(driver: *mut rio_driver);
    pub fn rio_dev_get(rdev: *mut rio_dev) -> *mut rio_dev;
    pub fn rio_dev_put(rdev: *mut rio_dev);
    pub fn rio_local_get_device_id(port: *mut rio_mport) -> u16;
    pub fn rio_local_set_device_id(port: *mut rio_mport, did: u16);
    pub fn rio_init_mports() -> i32;
}

// Preserved from #ifdef CONFIG_RAPIDIO_DMA_ENGINE.
#[cfg(CONFIG_RAPIDIO_DMA_ENGINE)]
extern "C" {
    pub fn rio_request_mport_dma(mport: *mut rio_mport) -> *mut dma_chan;
    pub fn rio_release_dma(dchan: *mut dma_chan);
    pub fn rio_dma_prep_xfer(dchan: *mut dma_chan, destid: u16, data: *mut rio_dma_data, direction: dma_transfer_direction, flags: usize) -> *mut dma_async_tx_descriptor;
}

pub unsafe fn rio_local_read_config_32(port: *mut rio_mport, offset: u32, data: *mut u32) -> i32 { __rio_local_read_config_32(port, offset, data) }
pub unsafe fn rio_local_write_config_32(port: *mut rio_mport, offset: u32, data: u32) -> i32 { __rio_local_write_config_32(port, offset, data) }
pub unsafe fn rio_local_read_config_16(port: *mut rio_mport, offset: u32, data: *mut u16) -> i32 { __rio_local_read_config_16(port, offset, data) }
pub unsafe fn rio_local_write_config_16(port: *mut rio_mport, offset: u32, data: u16) -> i32 { __rio_local_write_config_16(port, offset, data) }
pub unsafe fn rio_local_read_config_8(port: *mut rio_mport, offset: u32, data: *mut u8) -> i32 { __rio_local_read_config_8(port, offset, data) }
pub unsafe fn rio_local_write_config_8(port: *mut rio_mport, offset: u32, data: u8) -> i32 { __rio_local_write_config_8(port, offset, data) }

pub unsafe fn rio_read_config_32(rdev: *mut rio_dev, offset: u32, data: *mut u32) -> i32 { rio_mport_read_config_32((*rdev).net.hport, (*rdev).destid, (*rdev).hopcount, offset, data) }
pub unsafe fn rio_write_config_32(rdev: *mut rio_dev, offset: u32, data: u32) -> i32 { rio_mport_write_config_32((*rdev).net.hport, (*rdev).destid, (*rdev).hopcount, offset, data) }
pub unsafe fn rio_read_config_16(rdev: *mut rio_dev, offset: u32, data: *mut u16) -> i32 { rio_mport_read_config_16((*rdev).net.hport, (*rdev).destid, (*rdev).hopcount, offset, data) }
pub unsafe fn rio_write_config_16(rdev: *mut rio_dev, offset: u32, data: u16) -> i32 { rio_mport_write_config_16((*rdev).net.hport, (*rdev).destid, (*rdev).hopcount, offset, data) }
pub unsafe fn rio_read_config_8(rdev: *mut rio_dev, offset: u32, data: *mut u8) -> i32 { rio_mport_read_config_8((*rdev).net.hport, (*rdev).destid, (*rdev).hopcount, offset, data) }
pub unsafe fn rio_write_config_8(rdev: *mut rio_dev, offset: u32, data: u8) -> i32 { rio_mport_write_config_8((*rdev).net.hport, (*rdev).destid, (*rdev).hopcount, offset, data) }
pub unsafe fn rio_send_doorbell(rdev: *mut rio_dev, data: u16) -> i32 { rio_mport_send_doorbell((*rdev).net.hport, (*rdev).destid, data) }

pub unsafe fn rio_init_mbox_res(res: *mut resource, start: i32, end: i32) { core::ptr::write_bytes(res, 0, 1); (*res).start = start; (*res).end = end; (*res).flags = RIO_RESOURCE_MAILBOX; }
pub unsafe fn rio_init_dbell_res(res: *mut resource, start: u16, end: u16) { core::ptr::write_bytes(res, 0, 1); (*res).start = start as _; (*res).end = end as _; (*res).flags = RIO_RESOURCE_DOORBELL; }

// RIO_DEVICE(dev, ven): .did = dev, .vid = ven, .asm_did = RIO_ANY_ID, .asm_vid = RIO_ANY_ID

pub unsafe fn rio_add_outb_message(mport: *mut rio_mport, rdev: *mut rio_dev, mbox: i32, buffer: *mut core::ffi::c_void, len: usize) -> i32 { ((*(*mport).ops).add_outb_message)(mport, rdev, mbox, buffer, len) }
pub unsafe fn rio_add_inb_buffer(mport: *mut rio_mport, mbox: i32, buffer: *mut core::ffi::c_void) -> i32 { ((*(*mport).ops).add_inb_buffer)(mport, mbox, buffer) }
pub unsafe fn rio_get_inb_message(mport: *mut rio_mport, mbox: i32) -> *mut core::ffi::c_void { ((*(*mport).ops).get_inb_message)(mport, mbox) }

pub unsafe fn rio_name(rdev: *mut rio_dev) -> *const u8 { dev_name(&(*rdev).dev) }
pub unsafe fn rio_get_drvdata(rdev: *mut rio_dev) -> *mut core::ffi::c_void { dev_get_drvdata(&(*rdev).dev) }
pub unsafe fn rio_set_drvdata(rdev: *mut rio_dev, data: *mut core::ffi::c_void) { dev_set_drvdata(&mut (*rdev).dev, data); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
