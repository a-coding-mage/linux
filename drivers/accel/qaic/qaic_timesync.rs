// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries. */

// Kernel dependencies supplied by the surrounding QAIC/MHI Rust environment.

const QTIMER_REG_OFFSET: usize = 0xa28;
const QAIC_TIMESYNC_SIGNATURE: u16 = 0x55aa;

static mut timesync_delay_ms: u32 = 1000; // 1 sec default

#[repr(u8)]
enum qts_msg_type {
    QAIC_TS_CMD_TO_HOST,
    QAIC_TS_SYNC_REQ,
    QAIC_TS_ACK_TO_HOST,
    QAIC_TS_MSG_TYPE_MAX,
}

#[repr(C, packed)]
struct qts_hdr {
    signature: u16,
    reserved_1: u16,
    reserved_2: u8,
    msg_type: u8,
    reserved_3: u16,
}

#[repr(C, packed)]
struct qts_timeval {
    tv_sec: u64,
    tv_usec: u64,
}

#[repr(C, packed)]
struct qts_host_time_sync_msg_data {
    header: qts_hdr,
    data: qts_timeval,
}

#[repr(C)]
struct mqts_dev {
    qdev: *mut qaic_device,
    mhi_dev: *mut mhi_device,
    timer: timer_list,
    qtimer_addr: *mut core::ffi::c_void,
    buff_in_use: atomic_t,
    dev: *mut device,
    sync_msg: *mut qts_host_time_sync_msg_data,
}

#[repr(C, packed)]
struct qts_resp_msg { hdr: qts_hdr }

#[repr(C)]
struct qts_resp {
    data: qts_resp_msg,
    work: work_struct,
    qdev: *mut qaic_device,
}

unsafe fn read_qtimer(addr: *const core::ffi::c_void) -> u64 {
    // The C implementation selects readq when available; the portable form is
    // retained here as two 32-bit volatile register reads.
    let low = readl(addr);
    let high = readl((addr as usize + core::mem::size_of::<u32>()) as *const core::ffi::c_void);
    (low as u64) | ((high as u64) << 32)
}

unsafe extern "C" fn qaic_timesync_ul_xfer_cb(mhi_dev: *mut mhi_device, mhi_result: *mut mhi_result) {
    let mqtsdev = dev_get_drvdata((&mut (*mhi_dev).dev) as *mut device);
    dev_dbg((*mqtsdev).dev, "%s status: %d xfer_len: %zu\n", "qaic_timesync_ul_xfer_cb", (*mhi_result).transaction_status, (*mhi_result).bytes_xferd);
    atomic_set(&mut (*mqtsdev).buff_in_use, 0);
}

unsafe extern "C" fn qaic_timesync_dl_xfer_cb(mhi_dev: *mut mhi_device, _mhi_result: *mut mhi_result) {
    let mqtsdev = dev_get_drvdata((&mut (*mhi_dev).dev) as *mut device);
    dev_err((*mqtsdev).dev, "%s no data expected on dl channel\n", "qaic_timesync_dl_xfer_cb");
}

unsafe extern "C" fn qaic_timesync_timer(t: *mut timer_list) {
    let mqtsdev = timer_container_of(t, mqts_dev, timer);
    if atomic_read(&mut (*mqtsdev).buff_in_use) != 0 {
        dev_dbg((*mqtsdev).dev, "%s buffer not free, schedule next cycle\n", "qaic_timesync_timer");
    } else {
        atomic_set(&mut (*mqtsdev).buff_in_use, 1);
        let sync_msg = (*mqtsdev).sync_msg;
        (*sync_msg).header.signature = QAIC_TIMESYNC_SIGNATURE.to_le();
        (*sync_msg).header.msg_type = QAIC_TS_SYNC_REQ as u8;
        let host_time_us = div_u64(ktime_get_real_ns(), NSEC_PER_USEC);
        let device_qtimer_us = div_u64(read_qtimer((*mqtsdev).qtimer_addr), 192) * 10;
        let offset_us = host_time_us.wrapping_sub(device_qtimer_us);
        let host_sec = div_u64(offset_us, USEC_PER_SEC);
        (*sync_msg).data.tv_usec = (offset_us - host_sec * USEC_PER_SEC).to_le();
        (*sync_msg).data.tv_sec = host_sec.to_le();
        let ret = mhi_queue_buf((*mqtsdev).mhi_dev, DMA_TO_DEVICE, sync_msg as *mut _, core::mem::size_of::<qts_host_time_sync_msg_data>(), MHI_EOT);
        if ret != 0 && ret != -EAGAIN { dev_err((*mqtsdev).dev, "%s unable to queue to mhi:%d\n", "qaic_timesync_timer", ret); return; }
        if ret == -EAGAIN { atomic_set(&mut (*mqtsdev).buff_in_use, 0); }
    }
    let ret = mod_timer(t, jiffies + msecs_to_jiffies(timesync_delay_ms));
    if ret != 0 { dev_err((*mqtsdev).dev, "%s mod_timer error:%d\n", "qaic_timesync_timer", ret); }
}

pub unsafe extern "C" fn qaic_mqts_ch_stop_timer(mhi_dev: *mut mhi_device) {
    let mqtsdev = dev_get_drvdata((&mut (*mhi_dev).dev) as *mut device);
    timer_delete_sync(&mut (*mqtsdev).timer);
}

unsafe extern "C" fn qaic_timesync_probe(mhi_dev: *mut mhi_device, _id: *const mhi_device_id) -> i32 {
    let qdev = pci_get_drvdata(to_pci_dev((*(*mhi_dev).mhi_cntrl).cntrl_dev));
    let mqtsdev = kzalloc::<mqts_dev>();
    if mqtsdev.is_null() { return -ENOMEM; }
    (*mqtsdev).mhi_dev = mhi_dev; (*mqtsdev).qdev = qdev; (*mqtsdev).dev = &mut (*(*qdev).pdev).dev;
    (*mqtsdev).sync_msg = kzalloc::<qts_host_time_sync_msg_data>();
    if (*mqtsdev).sync_msg.is_null() { kfree(mqtsdev); return -ENOMEM; }
    atomic_set(&mut (*mqtsdev).buff_in_use, 0);
    let ret = mhi_prepare_for_transfer(mhi_dev);
    if ret != 0 { kfree((*mqtsdev).sync_msg); kfree(mqtsdev); return ret; }
    (*mqtsdev).qtimer_addr = ((*qdev).bar_mhi as usize + QTIMER_REG_OFFSET) as *mut _;
    timer_setup(&mut (*mqtsdev).timer, qaic_timesync_timer, 0);
    (*mqtsdev).timer.expires = jiffies + msecs_to_jiffies(timesync_delay_ms);
    add_timer(&mut (*mqtsdev).timer); dev_set_drvdata(&mut (*mhi_dev).dev, mqtsdev); (*qdev).mqts_ch = mhi_dev; 0
}

unsafe extern "C" fn qaic_timesync_remove(mhi_dev: *mut mhi_device) {
    let mqtsdev = dev_get_drvdata((&mut (*mhi_dev).dev) as *mut device);
    (*(*mqtsdev).qdev).mqts_ch = core::ptr::null_mut(); timer_delete_sync(&mut (*mqtsdev).timer);
    mhi_unprepare_from_transfer((*mqtsdev).mhi_dev); kfree((*mqtsdev).sync_msg); kfree(mqtsdev);
}

// The following boot-timesync callbacks and driver registration preserve the
// external kernel interfaces; their dependent kernel types/functions are supplied elsewhere.
unsafe extern "C" fn qaic_boot_timesync_worker(work: *mut work_struct) {
    let resp = container_of(work, qts_resp, work);
    let qdev = (*resp).qdev;
    let mhi_dev = (*qdev).qts_ch;
    let ret = mhi_queue_buf(mhi_dev, DMA_FROM_DEVICE, &mut (*resp).data as *mut _, core::mem::size_of::<qts_resp_msg>(), MHI_EOT);
    if ret != 0 { kfree(resp); dev_warn(&mut (*mhi_dev).dev, "Failed to re-queue response buffer %d\n", ret); return; }
    let data = (*resp).data;
    match data.hdr.msg_type {
        x if x == QAIC_TS_CMD_TO_HOST as u8 => {
            let req = kzalloc::<qts_host_time_sync_msg_data>(); if req.is_null() { return; }
            (*req).header = data.hdr; (*req).header.msg_type = QAIC_TS_SYNC_REQ as u8;
            let mut ts = timespec64 { ..core::mem::zeroed() }; ktime_get_real_ts64(&mut ts);
            (*req).data.tv_sec = (ts.tv_sec as u64).to_le(); (*req).data.tv_usec = div_u64(ts.tv_nsec as u64, NSEC_PER_USEC).to_le();
            let r = mhi_queue_buf(mhi_dev, DMA_TO_DEVICE, req as *mut _, core::mem::size_of::<qts_host_time_sync_msg_data>(), MHI_EOT);
            if r != 0 { kfree(req); dev_dbg(&mut (*mhi_dev).dev, "Failed to send request message. Error %d\n", r); }
        },
        x if x == QAIC_TS_ACK_TO_HOST as u8 => dev_dbg(&mut (*mhi_dev).dev, "ACK received from device\n"),
        _ => dev_err(&mut (*mhi_dev).dev, "Invalid message type %u.\n", data.hdr.msg_type),
    }
}
unsafe extern "C" fn qaic_boot_timesync_remove(mhi_dev: *mut mhi_device) { let qdev = dev_get_drvdata(&mut (*mhi_dev).dev); mhi_unprepare_from_transfer((*qdev).qts_ch); (*qdev).qts_ch = core::ptr::null_mut(); }
unsafe extern "C" fn qaic_boot_timesync_probe(mhi_dev: *mut mhi_device, _id: *const mhi_device_id) -> i32 { let qdev = pci_get_drvdata(to_pci_dev((*(*mhi_dev).mhi_cntrl).cntrl_dev)); let ret = mhi_prepare_for_transfer(mhi_dev); if ret != 0 { return ret; } (*qdev).qts_ch = mhi_dev; dev_set_drvdata(&mut (*mhi_dev).dev, qdev); 0 }
unsafe extern "C" fn qaic_boot_timesync_ul_xfer_cb(_mhi_dev: *mut mhi_device, mhi_result: *mut mhi_result) { kfree((*mhi_result).buf_addr); }
unsafe extern "C" fn qaic_boot_timesync_dl_xfer_cb(_mhi_dev: *mut mhi_device, _mhi_result: *mut mhi_result) {}

pub unsafe extern "C" fn qaic_timesync_init() -> i32 {
    let ret = mhi_driver_register(&mut qaic_timesync_driver);
    if ret != 0 { return ret; }
    mhi_driver_register(&mut qaic_boot_timesync_driver)
}

pub unsafe extern "C" fn qaic_timesync_deinit() {
    mhi_driver_unregister(&mut qaic_boot_timesync_driver);
    mhi_driver_unregister(&mut qaic_timesync_driver);
}

// Driver tables and boot-timesync message body are supplied by the kernel-facing bindings.
static mut qaic_timesync_driver: mhi_driver = mhi_driver { ..mhi_driver::ZERO };
static mut qaic_boot_timesync_driver: mhi_driver = mhi_driver { ..mhi_driver::ZERO };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
