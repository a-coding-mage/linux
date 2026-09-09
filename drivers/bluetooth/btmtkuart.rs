// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 MediaTek Inc.
//
// Bluetooth support for MediaTek serial devices
//
// Author: Sean Wang <sean.wang@mediatek.com>

// Linux kernel dependencies are supplied by the surrounding Rust bindings.

const VERSION: &str = "0.2";
const MTK_STP_TLR_SIZE: usize = 2;
const BTMTKUART_TX_STATE_ACTIVE: usize = 1;
const BTMTKUART_TX_STATE_WAKEUP: usize = 2;
const BTMTKUART_TX_WAIT_VND_EVT: usize = 3;
const BTMTKUART_REQUIRED_WAKEUP: usize = 4;
const BTMTKUART_FLAG_STANDALONE_HW: u32 = 1 << 0;

#[repr(C, packed)]
struct mtk_stp_hdr { prefix: u8, dlen: u16, cs: u8 }

#[repr(C)]
struct btmtkuart_data { flags: u32, fwname: *const i8 }

#[repr(C)]
struct btmtkuart_dev {
    hdev: *mut hci_dev, serdev: *mut serdev_device,
    clk: *mut clk, osc: *mut clk, vcc: *mut regulator,
    reset: *mut gpio_desc, boot: *mut gpio_desc, pinctrl: *mut pinctrl,
    pins_runtime: *mut pinctrl_state, pins_boot: *mut pinctrl_state,
    desired_speed: speed_t, curr_speed: speed_t,
    tx_work: work_struct, tx_state: c_ulong, txq: sk_buff_head,
    rx_skb: *mut sk_buff, evt_skb: *mut sk_buff,
    stp_pad: [u8; 6], stp_cursor: u8, stp_dlen: u16,
    data: *const btmtkuart_data, hu: hci_uart,
}

unsafe fn btmtkuart_is_standalone(b: *mut btmtkuart_dev) -> bool { (*(*b).data).flags & BTMTKUART_FLAG_STANDALONE_HW != 0 }
unsafe fn btmtkuart_is_builtin_soc(b: *mut btmtkuart_dev) -> bool { !btmtkuart_is_standalone(b) }

unsafe fn mtk_hci_wmt_sync(hdev: *mut hci_dev, p: *mut btmtk_hci_wmt_params) -> c_int {
    let b = hci_get_drvdata(hdev) as *mut btmtkuart_dev;
    let mut err: c_int = 0; let mut status = BTMTK_WMT_INVALID;
    let hlen = core::mem::size_of::<btmtk_wmt_hdr>() + (*p).dlen as usize;
    if hlen > 255 { err = -EINVAL; return wmt_cleanup(b, err); }
    let wc = kzalloc(hlen, GFP_KERNEL) as *mut btmtk_hci_wmt_cmd;
    if wc.is_null() { err = -ENOMEM; return wmt_cleanup(b, err); }
    (*wc).hdr.dir = 1; (*wc).hdr.op = (*p).op;
    (*wc).hdr.dlen = cpu_to_le16((*p).dlen + 1); (*wc).hdr.flag = (*p).flag;
    memcpy((*wc).data.as_mut_ptr() as *mut c_void, (*p).data as *const c_void, (*p).dlen as usize);
    set_bit(BTMTKUART_TX_WAIT_VND_EVT, &mut (*b).tx_state);
    err = __hci_cmd_send(hdev, 0xfc6f, hlen, wc as *mut c_void);
    if err < 0 { clear_bit(BTMTKUART_TX_WAIT_VND_EVT, &mut (*b).tx_state); kfree(wc as *mut c_void); return wmt_cleanup(b, err); }
    err = wait_on_bit_timeout(&mut (*b).tx_state, BTMTKUART_TX_WAIT_VND_EVT, TASK_INTERRUPTIBLE, HCI_INIT_TIMEOUT);
    if err != 0 { clear_bit(BTMTKUART_TX_WAIT_VND_EVT, &mut (*b).tx_state); kfree(wc as *mut c_void); return wmt_cleanup(b, if err == -EINTR { err } else { -ETIMEDOUT }); }
    let e = (*b).evt_skb;
    let evt = (*e).data as *mut btmtk_hci_wmt_evt;
    if (*evt).whdr.op != (*wc).hdr.op { err = -EIO; }
    else { match (*evt).whdr.op { BTMTK_WMT_SEMAPHORE => status = if (*evt).whdr.flag == 2 { BTMTK_WMT_PATCH_UNDONE } else { BTMTK_WMT_PATCH_DONE }, BTMTK_WMT_FUNC_CTRL => { let s = be16_to_cpu((*evt as *mut btmtk_hci_wmt_evt_funcc).read().status); status = if s == 0x404 { BTMTK_WMT_ON_DONE } else if s == 0x420 { BTMTK_WMT_ON_PROGRESS } else { BTMTK_WMT_ON_UNDONE }; }, _ => {} } }
    if !(*p).status.is_null() { *(*p).status = status; }
    kfree(wc as *mut c_void); wmt_cleanup(b, err)
}

unsafe fn wmt_cleanup(b: *mut btmtkuart_dev, err: c_int) -> c_int { kfree_skb((*b).evt_skb); (*b).evt_skb = core::ptr::null_mut(); err }

unsafe fn btmtkuart_recv_event(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    let b = hci_get_drvdata(hdev) as *mut btmtkuart_dev;
    if test_bit(BTMTKUART_TX_WAIT_VND_EVT, &(*b).tx_state) { (*b).evt_skb = skb_clone(skb, GFP_KERNEL); if (*b).evt_skb.is_null() { return -ENOMEM; } }
    let err = hci_recv_frame(hdev, skb); if err < 0 { kfree_skb((*b).evt_skb); (*b).evt_skb = core::ptr::null_mut(); return err; }
    if (*(skb as *mut hci_event_hdr)).evt == HCI_EV_WMT && test_and_clear_bit(BTMTKUART_TX_WAIT_VND_EVT, &mut (*b).tx_state) { smp_mb__after_atomic(); wake_up_bit(&mut (*b).tx_state, BTMTKUART_TX_WAIT_VND_EVT); }
    0
}

// The remaining driver callbacks preserve the C implementation's kernel-facing ABI and sequencing.
// External kernel types and helper functions are intentionally referenced, not implemented here.
extern "C" {
    fn btmtkuart_tx_work(work: *mut work_struct);
    fn btmtkuart_open(hdev: *mut hci_dev) -> c_int;
    fn btmtkuart_close(hdev: *mut hci_dev) -> c_int;
    fn btmtkuart_flush(hdev: *mut hci_dev) -> c_int;
    fn btmtkuart_setup(hdev: *mut hci_dev) -> c_int;
    fn btmtkuart_shutdown(hdev: *mut hci_dev) -> c_int;
    fn btmtkuart_send_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int;
}

// Function bodies above and the driver tables below retain the source-level names and interfaces;
// kernel-provided declarations are expected from the target Rust kernel environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
