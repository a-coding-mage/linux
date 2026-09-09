// SPDX-License-Identifier: GPL-2.0
/*
 * System Control and Management Interface (SCMI) Message Mailbox Transport
 * driver.
 *
 * Copyright (C) 2019-2024 ARM Ltd.
 */

// Linux/kernel dependencies and ../common.h are supplied by the surrounding
// translation unit.

#[repr(C)]
pub struct scmi_mailbox {
    pub cl: mbox_client,
    pub chan: *mut mbox_chan,
    pub chan_receiver: *mut mbox_chan,
    pub chan_platform_receiver: *mut mbox_chan,
    pub cinfo: *mut scmi_chan_info,
    pub shmem: *mut scmi_shared_mem,
    pub chan_lock: mutex,
    pub io_ops: *mut scmi_shmem_io_ops,
}

static mut core: *mut scmi_transport_core_operations = core::ptr::null_mut();

unsafe fn tx_prepare(cl: *mut mbox_client, m: *mut core::ffi::c_void) {
    let smbox = container_of!(cl, scmi_mailbox, cl);
    ((*core).shmem).as_ref().unwrap().tx_prepare((*smbox).shmem, m, (*smbox).cinfo, (*(*smbox).io_ops).toio);
}

unsafe fn rx_callback(cl: *mut mbox_client, _m: *mut core::ffi::c_void) {
    let smbox = container_of!(cl, scmi_mailbox, cl);
    if (*cl).knows_txdone && !((*(*core).shmem).channel_free)((*smbox).shmem) {
        dev_warn!((*smbox).cinfo, "Ignoring spurious A2P IRQ !\n");
        ((*core).bad_message_trace)((*smbox).cinfo,
            ((*core).shmem).as_ref().unwrap().read_header((*smbox).shmem), MSG_MBOX_SPURIOUS);
        return;
    }
    ((*core).rx_callback)((*smbox).cinfo,
        ((*core).shmem).as_ref().unwrap().read_header((*smbox).shmem), core::ptr::null_mut());
}

unsafe fn mailbox_chan_available(of_node: *mut device_node, mut idx: i32) -> bool {
    let num_mb = of_count_phandle_with_args(of_node, c"mboxes".as_ptr(), c"#mbox-cells".as_ptr());
    if num_mb == 3 && idx == 1 { idx = 2; }
    !of_parse_phandle_with_args(of_node, c"mboxes".as_ptr(), c"#mbox-cells".as_ptr(), idx, core::ptr::null_mut()).is_null()
}

unsafe fn mailbox_chan_validate(cdev: *mut device, a2p_rx_chan: *mut i32,
    p2a_chan: *mut i32, p2a_rx_chan: *mut i32) -> i32 {
    let np = (*cdev).of_node;
    let num_mb = of_count_phandle_with_args(np, c"mboxes".as_ptr(), c"#mbox-cells".as_ptr());
    let num_sh = of_count_phandle_with_args(np, c"shmem".as_ptr(), core::ptr::null());
    dev_dbg!(cdev, "Found %d mboxes and %d shmems !\n", num_mb, num_sh);
    if num_mb <= 0 || num_sh <= 0 || num_sh > 2 || num_mb > 4 ||
       (num_mb == 1 && num_sh != 1) || (num_mb == 3 && num_sh != 2) || (num_mb == 4 && num_sh != 2) {
        dev_warn!(cdev, "Invalid channel descriptor for '%pOF' - mbs:%d  shm:%d\n", np, num_mb, num_sh);
        return -EINVAL;
    }
    if num_sh > 1 {
        let np_tx = of_parse_phandle(np, c"shmem".as_ptr(), 0);
        let np_rx = of_parse_phandle(np, c"shmem".as_ptr(), 1);
        if np_tx.is_null() || np_rx.is_null() || np_tx == np_rx {
            dev_warn!(cdev, "Invalid shmem descriptor for '%pOF'\n", np);
            return -EINVAL;
        }
    }
    match num_mb {
        1 => { *a2p_rx_chan = 0; *p2a_chan = 0; *p2a_rx_chan = 0; }
        2 => { if num_sh == 2 { *a2p_rx_chan = 0; *p2a_chan = 1; } else { *a2p_rx_chan = 1; *p2a_chan = 0; } *p2a_rx_chan = 0; }
        3 => { *a2p_rx_chan = 1; *p2a_chan = 2; *p2a_rx_chan = 0; }
        4 => { *a2p_rx_chan = 1; *p2a_chan = 2; *p2a_rx_chan = 3; }
        _ => {}
    }
    0
}

// The remaining transport operations retain the C ABI and are supplied by the
// kernel-facing Rust bindings used by this translation.
unsafe fn mailbox_chan_setup(cinfo: *mut scmi_chan_info, dev: *mut device, tx: bool) -> i32 {
    let mut a2p_rx_chan = 0; let mut p2a_chan = 0; let mut p2a_rx_chan = 0;
    let ret = mailbox_chan_validate((*cinfo).dev, &mut a2p_rx_chan, &mut p2a_chan, &mut p2a_rx_chan);
    if ret != 0 { return ret; }
    if !tx && p2a_chan == 0 { return -ENODEV; }
    let smbox = devm_kzalloc(dev, core::mem::size_of::<scmi_mailbox>(), GFP_KERNEL) as *mut scmi_mailbox;
    if smbox.is_null() { return -ENOMEM; }
    (*smbox).shmem = ((*core).shmem).as_ref().unwrap().setup_iomap(cinfo, dev, tx, core::ptr::null_mut(), &mut (*smbox).io_ops);
    if IS_ERR((*smbox).shmem) { return PTR_ERR((*smbox).shmem); }
    (*smbox).cl.dev = (*cinfo).dev; (*smbox).cl.tx_prepare = if tx { Some(tx_prepare) } else { None };
    (*smbox).cl.rx_callback = Some(rx_callback); (*smbox).cl.tx_block = false; (*smbox).cl.knows_txdone = tx;
    (*cinfo).transport_info = smbox as *mut core::ffi::c_void; (*smbox).cinfo = cinfo; mutex_init(&mut (*smbox).chan_lock);
    (*smbox).chan = mbox_request_channel(&mut (*smbox).cl, if tx { 0 } else { p2a_chan });
    if IS_ERR((*smbox).chan) { let r = PTR_ERR((*smbox).chan); (*smbox).chan = core::ptr::null_mut(); (*cinfo).transport_info = core::ptr::null_mut(); devm_iounmap(dev, (*smbox).shmem); devm_kfree(dev, smbox as *mut _); return r; }
    if tx && a2p_rx_chan != 0 { (*smbox).chan_receiver = mbox_request_channel(&mut (*smbox).cl, a2p_rx_chan); if IS_ERR((*smbox).chan_receiver) { mbox_free_channel((*smbox).chan); return PTR_ERR((*smbox).chan_receiver); } }
    if !tx && p2a_rx_chan != 0 { (*smbox).chan_platform_receiver = mbox_request_channel(&mut (*smbox).cl, p2a_rx_chan); if IS_ERR((*smbox).chan_platform_receiver) { mbox_free_channel((*smbox).chan); return PTR_ERR((*smbox).chan_platform_receiver); } }
    0
}
unsafe fn mailbox_chan_free(_id: i32, p: *mut core::ffi::c_void, _data: *mut core::ffi::c_void) -> i32 { let cinfo = p as *mut scmi_chan_info; let smbox = (*cinfo).transport_info as *mut scmi_mailbox; if !smbox.is_null() { mbox_free_channel((*smbox).chan); mbox_free_channel((*smbox).chan_receiver); mbox_free_channel((*smbox).chan_platform_receiver); (*cinfo).transport_info = core::ptr::null_mut(); } 0 }
unsafe fn mailbox_send_message(cinfo: *mut scmi_chan_info, xfer: *mut scmi_xfer) -> i32 { let smbox = (*cinfo).transport_info as *mut scmi_mailbox; mutex_lock(&mut (*smbox).chan_lock); let ret = mbox_send_message((*smbox).chan, xfer as *mut _); if ret < 0 { mutex_unlock(&mut (*smbox).chan_lock); } if ret < 0 { ret } else { 0 } }
unsafe fn mailbox_mark_txdone(cinfo: *mut scmi_chan_info, ret: i32, _unused: *mut scmi_xfer) { let smbox = (*cinfo).transport_info as *mut scmi_mailbox; mbox_client_txdone((*smbox).chan, ret); mutex_unlock(&mut (*smbox).chan_lock); }
unsafe fn mailbox_fetch_response(cinfo: *mut scmi_chan_info, xfer: *mut scmi_xfer) { let s = (*cinfo).transport_info as *mut scmi_mailbox; ((*core).shmem).as_ref().unwrap().fetch_response((*s).shmem, xfer, (*(*s).io_ops).fromio); }
unsafe fn mailbox_fetch_notification(cinfo: *mut scmi_chan_info, max_len: usize, xfer: *mut scmi_xfer) { let s = (*cinfo).transport_info as *mut scmi_mailbox; ((*core).shmem).as_ref().unwrap().fetch_notification((*s).shmem, max_len, xfer, (*(*s).io_ops).fromio); }
unsafe fn mailbox_clear_channel(cinfo: *mut scmi_chan_info) { let s = (*cinfo).transport_info as *mut scmi_mailbox; ((*core).shmem).as_ref().unwrap().clear_channel((*s).shmem); }
unsafe fn mailbox_poll_done(cinfo: *mut scmi_chan_info, xfer: *mut scmi_xfer) -> bool { let s = (*cinfo).transport_info as *mut scmi_mailbox; ((*core).shmem).as_ref().unwrap().poll_done((*s).shmem, xfer) }

// Metadata and driver registration correspond to the C definitions below.
static mut scmi_mailbox_ops: scmi_transport_ops = scmi_transport_ops { chan_available: Some(mailbox_chan_available), chan_setup: Some(mailbox_chan_setup), chan_free: Some(mailbox_chan_free), send_message: Some(mailbox_send_message), mark_txdone: Some(mailbox_mark_txdone), fetch_response: Some(mailbox_fetch_response), fetch_notification: Some(mailbox_fetch_notification), clear_channel: Some(mailbox_clear_channel), poll_done: Some(mailbox_poll_done) };
static mut scmi_mailbox_desc: scmi_desc = scmi_desc { ops: &raw mut scmi_mailbox_ops, max_rx_timeout_ms: 30, max_msg: 20, max_msg_size: SCMI_SHMEM_MAX_PAYLOAD_SIZE };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
