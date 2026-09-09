// SPDX-License-Identifier: GPL-2.0-only
/*
 * Marvell Bluetooth driver
 *
 * Copyright (C) 2009, Marvell International Ltd.
 */

// Linux and driver headers are supplied by the surrounding translation unit.

const VERSION: &str = "1.0";

/*
 * This function is called by interface specific interrupt handler.
 * It updates Power Save & Host Sleep states, and wakes up the main
 * thread.
 */
#[no_mangle]
pub unsafe extern "C" fn btmrvl_interrupt(priv_: *mut btmrvl_private) {
    (*(*priv_).adapter).ps_state = PS_AWAKE;
    (*(*priv_).adapter).wakeup_tries = 0;
    (*(*priv_).adapter).int_count += 1;

    if (*(*priv_).adapter).hs_state == HS_ACTIVATED {
        BT_DBG!("BT: HS DEACTIVATED in ISR!");
        (*(*priv_).adapter).hs_state = HS_DEACTIVATED;
    }
    wake_up_interruptible(&mut (*priv_).main_thread.wait_q);
}

#[no_mangle]
pub unsafe extern "C" fn btmrvl_check_evtpkt(
    priv_: *mut btmrvl_private,
    skb: *mut sk_buff,
) -> bool {
    let hdr = (*skb).data as *mut hci_event_hdr;
    if (*skb).len < core::mem::size_of::<hci_event_hdr>() { return true; }

    if (*hdr).evt == HCI_EV_CMD_COMPLETE {
        if (*hdr).plen < core::mem::size_of::<hci_ev_cmd_complete>()
            || (*skb).len < HCI_EVENT_HDR_SIZE + core::mem::size_of::<hci_ev_cmd_complete>() { return true; }
        let ec = ((*skb).data.add(HCI_EVENT_HDR_SIZE)) as *mut hci_ev_cmd_complete;
        let opcode = __le16_to_cpu((*ec).opcode);
        if (*priv_).btmrvl_dev.sendcmdflag {
            (*priv_).btmrvl_dev.sendcmdflag = false;
            (*(*priv_).adapter).cmd_complete = true;
            wake_up_interruptible(&mut (*(*priv_).adapter).cmd_wait_q);
            if hci_opcode_ogf(opcode) == 0x3f {
                BT_DBG!("vendor event skipped: opcode=%#4.4x", opcode);
                kfree_skb(skb);
                return false;
            }
        }
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn btmrvl_process_event(priv_: *mut btmrvl_private, skb: *mut sk_buff) -> i32 {
    let adapter = (*priv_).adapter;
    let event = (*skb).data as *mut btmrvl_event;
    let mut ret: i32 = 0;
    if (*skb).len <= core::mem::offset_of!(btmrvl_event, data) { return -EINVAL; }
    if (*event).ec != 0xff {
        BT_DBG!("Not Marvell Event=%x", (*event).ec);
        ret = -EINVAL;
    } else {
        match (*event).data[0] {
            BT_EVENT_AUTO_SLEEP_MODE => {
                if (*skb).len <= core::mem::offset_of!(btmrvl_event, data) + 2 { return -EINVAL; }
                if (*event).data[2] == 0 {
                    (*adapter).psmode = if (*event).data[1] == BT_PS_ENABLE { 1 } else { 0 };
                    BT_DBG!("PS Mode:%s", str_enable_disable((*adapter).psmode));
                } else { BT_DBG!("PS Mode command failed"); }
            }
            BT_EVENT_HOST_SLEEP_CONFIG => {
                if (*skb).len <= core::mem::offset_of!(btmrvl_event, data) + 3 { return -EINVAL; }
                if (*event).data[3] == 0 { BT_DBG!("gpio=%x, gap=%x", (*event).data[1], (*event).data[2]); }
                else { BT_DBG!("HSCFG command failed"); }
            }
            BT_EVENT_HOST_SLEEP_ENABLE => {
                if (*skb).len <= core::mem::offset_of!(btmrvl_event, data) + 1 { return -EINVAL; }
                if (*event).data[1] == 0 {
                    (*adapter).hs_state = HS_ACTIVATED;
                    if (*adapter).psmode != 0 { (*adapter).ps_state = PS_SLEEP; }
                    wake_up_interruptible(&mut (*adapter).event_hs_wait_q);
                    BT_DBG!("HS ACTIVATED!");
                } else { BT_DBG!("HS Enable failed"); }
            }
            BT_EVENT_MODULE_CFG_REQ => {
                if (*skb).len <= core::mem::offset_of!(btmrvl_event, data) + 2 { return -EINVAL; }
                if (*priv_).btmrvl_dev.sendcmdflag && (*event).data[1] == MODULE_BRINGUP_REQ {
                    BT_DBG!("EVENT:%s", if (*event).data[2] == MODULE_BROUGHT_UP || (*event).data[2] == MODULE_ALREADY_UP { "Bring-up succeed" } else { "Bring-up failed" });
                } else if (*priv_).btmrvl_dev.sendcmdflag && (*event).data[1] == MODULE_SHUTDOWN_REQ {
                    BT_DBG!("EVENT:%s", if (*event).data[2] != 0 { "Shutdown failed" } else { "Shutdown succeed" });
                } else { BT_DBG!("BT_CMD_MODULE_CFG_REQ resp for APP"); ret = -EINVAL; }
            }
            BT_EVENT_POWER_STATE => {
                if (*skb).len <= core::mem::offset_of!(btmrvl_event, data) + 1 { return -EINVAL; }
                if (*event).data[1] == BT_PS_SLEEP { (*adapter).ps_state = PS_SLEEP; }
                BT_DBG!("EVENT:%s", if (*adapter).ps_state != 0 { "PS_SLEEP" } else { "PS_AWAKE" });
            }
            _ => { BT_DBG!("Unknown Event=%d", (*event).data[0]); ret = -EINVAL; }
        }
    }
    if ret == 0 { kfree_skb(skb); }
    ret
}

unsafe fn btmrvl_send_sync_cmd(priv_: *mut btmrvl_private, opcode: u16, param: *const core::ffi::c_void, len: u8) -> i32 {
    if (*priv_).surprise_removed { BT_ERR!("Card is removed"); return -EFAULT; }
    let skb = bt_skb_alloc(HCI_COMMAND_HDR_SIZE + len as usize, GFP_KERNEL);
    if skb.is_null() { BT_ERR!("No free skb"); return -ENOMEM; }
    let hdr = skb_put(skb, HCI_COMMAND_HDR_SIZE) as *mut hci_command_hdr;
    (*hdr).opcode = cpu_to_le16(opcode); (*hdr).plen = len;
    if len != 0 { skb_put_data(skb, param, len as usize); }
    *hci_skb_pkt_type(skb) = MRVL_VENDOR_PKT;
    skb_queue_head(&mut (*(*priv_).adapter).tx_queue, skb);
    (*priv_).btmrvl_dev.sendcmdflag = true; (*(*priv_).adapter).cmd_complete = false;
    wake_up_interruptible(&mut (*priv_).main_thread.wait_q);
    if wait_event_interruptible_timeout(&mut (*(*priv_).adapter).cmd_wait_q, (*(*priv_).adapter).cmd_complete || (*priv_).surprise_removed, WAIT_UNTIL_CMD_RESP) == 0 { return -ETIMEDOUT; }
    if (*priv_).surprise_removed { return -EFAULT; } 0
}

#[no_mangle]
pub unsafe extern "C" fn btmrvl_send_module_cfg_cmd(priv_: *mut btmrvl_private, subcmd: u8) -> i32 {
    let ret = btmrvl_send_sync_cmd(priv_, BT_CMD_MODULE_CFG_REQ, &subcmd as *const _ as *const _, 1);
    if ret != 0 { BT_ERR!("module_cfg_cmd(%x) failed", subcmd); } ret
}

unsafe fn btmrvl_enable_sco_routing_to_host(priv_: *mut btmrvl_private) -> i32 {
    let subcmd: u8 = 0;
    let ret = btmrvl_send_sync_cmd(priv_, BT_CMD_ROUTE_SCO_TO_HOST, &subcmd as *const _ as *const _, 1);
    if ret != 0 { BT_ERR!("BT_CMD_ROUTE_SCO_TO_HOST command failed: %#x", ret); } ret
}

#[no_mangle]
pub unsafe extern "C" fn btmrvl_pscan_window_reporting(priv_: *mut btmrvl_private, subcmd: u8) -> i32 {
    let card = (*priv_).btmrvl_dev.card;
    if !(*card).support_pscan_win_report { return 0; }
    let ret = btmrvl_send_sync_cmd(priv_, BT_CMD_PSCAN_WIN_REPORT_ENABLE, &subcmd as *const _ as *const _, 1);
    if ret != 0 { BT_ERR!("PSCAN_WIN_REPORT_ENABLE command failed: %#x", ret); } ret
}

#[no_mangle]
pub unsafe extern "C" fn btmrvl_send_hscfg_cmd(priv_: *mut btmrvl_private) -> i32 {
    let gap = (*priv_).btmrvl_dev.gpio_gap;
    let param = [(gap & 0xff00) >> 8 as u16 as u8, (gap & 0xff) as u8];
    BT_DBG!("Sending HSCFG Command, gpio=0x%x, gap=0x%x", param[0], param[1]);
    let ret = btmrvl_send_sync_cmd(priv_, BT_CMD_HOST_SLEEP_CONFIG, param.as_ptr() as *const _, 2);
    if ret != 0 { BT_ERR!("HSCFG command failed"); } ret
}

#[no_mangle]
pub unsafe extern "C" fn btmrvl_enable_ps(priv_: *mut btmrvl_private) -> i32 {
    let param = if (*priv_).btmrvl_dev.psmode { BT_PS_ENABLE } else { BT_PS_DISABLE };
    let ret = btmrvl_send_sync_cmd(priv_, BT_CMD_AUTO_SLEEP_MODE, &param as *const _ as *const _, 1);
    if ret != 0 { BT_ERR!("PSMODE command failed"); } 0
}

#[no_mangle]
pub unsafe extern "C" fn btmrvl_enable_hs(priv_: *mut btmrvl_private) -> i32 {
    let adapter = (*priv_).adapter;
    let mut ret = btmrvl_send_sync_cmd(priv_, BT_CMD_HOST_SLEEP_ENABLE, core::ptr::null(), 0);
    if ret != 0 { BT_ERR!("Host sleep enable command failed"); return ret; }
    ret = wait_event_interruptible_timeout(&mut (*adapter).event_hs_wait_q, (*adapter).hs_state != 0 || (*priv_).surprise_removed, WAIT_UNTIL_HS_STATE_CHANGED);
    if ret < 0 || (*priv_).surprise_removed { BT_ERR!("event_hs_wait_q terminated (%d): %d,%d,%d", ret, (*adapter).hs_state, (*adapter).ps_state, (*adapter).wakeup_tries); }
    else if ret == 0 { BT_ERR!("hs_enable timeout: %d,%d,%d", (*adapter).hs_state, (*adapter).ps_state, (*adapter).wakeup_tries); ret = -ETIMEDOUT; }
    else { BT_DBG!("host sleep enabled: %d,%d,%d", (*adapter).hs_state, (*adapter).ps_state, (*adapter).wakeup_tries); ret = 0; } ret
}

#[no_mangle]
pub unsafe extern "C" fn btmrvl_prepare_command(priv_: *mut btmrvl_private) -> i32 {
    let mut ret = 0;
    if (*priv_).btmrvl_dev.hscfgcmd { (*priv_).btmrvl_dev.hscfgcmd = 0; btmrvl_send_hscfg_cmd(priv_); }
    if (*priv_).btmrvl_dev.pscmd { (*priv_).btmrvl_dev.pscmd = 0; btmrvl_enable_ps(priv_); }
    if (*priv_).btmrvl_dev.hscmd { (*priv_).btmrvl_dev.hscmd = 0; if (*priv_).btmrvl_dev.hsmode { ret = btmrvl_enable_hs(priv_); } else { ret = (*priv_).hw_wakeup_firmware.unwrap()(priv_); (*(*priv_).adapter).hs_state = HS_DEACTIVATED; BT_DBG!("BT: HS DEACTIVATED due to host activity!"); } }
    ret
}

unsafe fn btmrvl_tx_pkt(priv_: *mut btmrvl_private, skb: *mut sk_buff) -> i32 {
    if skb.is_null() || (*skb).data.is_null() { return -EINVAL; }
    if (*skb).len == 0 || (*skb).len + BTM_HEADER_LEN > BTM_UPLD_SIZE { BT_ERR!("Tx Error: Bad skb length %d : %d", (*skb).len, BTM_UPLD_SIZE); return -EINVAL; }
    skb_push(skb, BTM_HEADER_LEN); (*skb).data[0] = ((*skb).len & 0xff) as u8; (*skb).data[1] = ((*skb).len >> 8) as u8; (*skb).data[2] = ((*skb).len >> 16) as u8; (*skb).data[3] = *hci_skb_pkt_type(skb);
    if let Some(f) = (*priv_).hw_host_to_card { f(priv_, (*skb).data, (*skb).len) } else { 0 }
}

unsafe fn btmrvl_init_adapter(priv_: *mut btmrvl_private) {
    skb_queue_head_init(&mut (*(*priv_).adapter).tx_queue); (*(*priv_).adapter).ps_state = PS_AWAKE;
    let buf_size = ALIGN_SZ(SDIO_BLOCK_SIZE, BTSDIO_DMA_ALIGN); (*(*priv_).adapter).hw_regs_buf = kzalloc(buf_size, GFP_KERNEL);
    if (*(*priv_).adapter).hw_regs_buf.is_null() { (*(*priv_).adapter).hw_regs = core::ptr::null_mut(); BT_ERR!("Unable to allocate buffer for hw_regs."); } else { (*(*priv_).adapter).hw_regs = ALIGN_ADDR((*(*priv_).adapter).hw_regs_buf, BTSDIO_DMA_ALIGN) as *mut u8; BT_DBG!("hw_regs_buf=%p hw_regs=%p", (*(*priv_).adapter).hw_regs_buf, (*(*priv_).adapter).hw_regs); }
    init_waitqueue_head(&mut (*(*priv_).adapter).cmd_wait_q); init_waitqueue_head(&mut (*(*priv_).adapter).event_hs_wait_q);
}

unsafe fn btmrvl_free_adapter(priv_: *mut btmrvl_private) { skb_queue_purge(&mut (*(*priv_).adapter).tx_queue); kfree((*(*priv_).adapter).hw_regs_buf); kfree((*priv_).adapter as *mut _); (*priv_).adapter = core::ptr::null_mut(); }

unsafe fn btmrvl_service_main_thread(_data: *mut core::ffi::c_void) -> i32 { 0 }

unsafe fn btmrvl_open(_hdev: *mut hci_dev) -> i32 { 0 }
unsafe fn btmrvl_close(hdev: *mut hci_dev) -> i32 { let p = hci_get_drvdata(hdev); skb_queue_purge(&mut (*(*p).adapter).tx_queue); 0 }
unsafe fn btmrvl_flush(hdev: *mut hci_dev) -> i32 { btmrvl_close(hdev) }

#[no_mangle]
pub unsafe extern "C" fn btmrvl_remove_card(priv_: *mut btmrvl_private) -> i32 { let hdev = (*priv_).btmrvl_dev.hcidev; wake_up_interruptible(&mut (*(*priv_).adapter).cmd_wait_q); wake_up_interruptible(&mut (*(*priv_).adapter).event_hs_wait_q); kthread_stop((*priv_).main_thread.task); hci_unregister_dev(hdev); hci_free_dev(hdev); (*priv_).btmrvl_dev.hcidev = core::ptr::null_mut(); btmrvl_free_adapter(priv_); kfree(priv_ as *mut _); 0 }

unsafe fn btmrvl_send_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> i32 { let p = hci_get_drvdata(hdev); if (*(*p).adapter).is_suspending || (*(*p).adapter).is_suspended { return -EBUSY; } skb_queue_tail(&mut (*(*p).adapter).tx_queue, skb); if !(*(*p).adapter).is_suspended { wake_up_interruptible(&mut (*p).main_thread.wait_q); } 0 }

unsafe fn btmrvl_setup(hdev: *mut hci_dev) -> i32 { let p = hci_get_drvdata(hdev); let r = btmrvl_send_module_cfg_cmd(p, MODULE_BRINGUP_REQ); if r != 0 { return r; } (*p).btmrvl_dev.gpio_gap = 0xfffe; btmrvl_enable_sco_routing_to_host(p); btmrvl_pscan_window_reporting(p, 1); (*p).btmrvl_dev.psmode = true; btmrvl_enable_ps(p); btmrvl_send_hscfg_cmd(p); 0 }

unsafe fn btmrvl_set_bdaddr(_hdev: *mut hci_dev, _bdaddr: *const bdaddr_t) -> i32 { 0 }
unsafe fn btmrvl_wakeup(hdev: *mut hci_dev) -> bool { let p = hci_get_drvdata(hdev); device_may_wakeup(&(*(*p).btmrvl_dev.card).func.dev) }

#[no_mangle]
pub unsafe extern "C" fn btmrvl_register_hdev(priv_: *mut btmrvl_private) -> i32 { let hdev = hci_alloc_dev(); if hdev.is_null() { BT_ERR!("Can not allocate HCI device"); kthread_stop((*priv_).main_thread.task); btmrvl_free_adapter(priv_); kfree(priv_ as *mut _); return -ENOMEM; } (*priv_).btmrvl_dev.hcidev = hdev; hci_set_drvdata(hdev, priv_); (*hdev).bus = HCI_SDIO; (*hdev).open = Some(btmrvl_open); (*hdev).close = Some(btmrvl_close); (*hdev).flush = Some(btmrvl_flush); (*hdev).send = Some(btmrvl_send_frame); (*hdev).setup = Some(btmrvl_setup); (*hdev).set_bdaddr = Some(btmrvl_set_bdaddr); (*hdev).wakeup = Some(btmrvl_wakeup); let r = hci_register_dev(hdev); if r < 0 { hci_free_dev(hdev); kthread_stop((*priv_).main_thread.task); btmrvl_free_adapter(priv_); kfree(priv_ as *mut _); return -ENOMEM; } 0 }

#[no_mangle]
pub unsafe extern "C" fn btmrvl_add_card(card: *mut core::ffi::c_void) -> *mut btmrvl_private {
    let priv_ = kzalloc_obj::<btmrvl_private>();
    if priv_.is_null() { BT_ERR!("Can not allocate priv"); return core::ptr::null_mut(); }
    (*priv_).adapter = kzalloc_obj::<btmrvl_adapter>();
    if (*priv_).adapter.is_null() { BT_ERR!("Allocate buffer for btmrvl_adapter failed!"); kfree(priv_ as *mut _); return core::ptr::null_mut(); }
    btmrvl_init_adapter(priv_); BT_DBG!("Starting kthread..."); (*priv_).main_thread.priv_ = priv_; spin_lock_init(&mut (*priv_).driver_lock);
    init_waitqueue_head(&mut (*priv_).main_thread.wait_q);
    (*priv_).main_thread.task = kthread_run(btmrvl_service_main_thread, &mut (*priv_).main_thread as *mut _, "btmrvl_main_service");
    if IS_ERR((*priv_).main_thread.task) { btmrvl_free_adapter(priv_); kfree(priv_ as *mut _); return core::ptr::null_mut(); }
    (*priv_).btmrvl_dev.card = card as *mut _; (*priv_).btmrvl_dev.tx_dnld_rdy = true; priv_
}

// Direct translations of the remaining static helpers and exported lifecycle
// entry points are declared against the same external definitions.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
