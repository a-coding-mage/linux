// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2012  Intel Corporation. All rights reserved.
 */

// The declarations referenced below are supplied by the surrounding HCI implementation.

use core::ffi::c_void;

const MAX_FWI: i32 = 4949;

#[repr(C)]
pub struct sk_buff {
    pub data: *mut u8,
}

#[repr(C)]
pub struct nfc_hci_dev {
    pub gate2pipe: [u8; 256],
    pub quirks: usize,
    pub pipes: [nfc_hci_pipe; 256],
}

#[repr(C)]
pub struct nfc_hci_pipe {
    pub gate: u8,
    pub dest_host: u8,
}

#[repr(C)]
pub struct hcp_exec_waiter {
    pub wq: *mut c_void,
    pub exec_complete: bool,
    pub result_skb: *mut sk_buff,
    pub exec_result: i32,
}

#[repr(C)]
pub struct hci_create_pipe_params {
    pub src_gate: u8,
    pub dest_host: u8,
    pub dest_gate: u8,
}

#[repr(C)]
pub struct hci_create_pipe_resp {
    pub pipe: u8,
}

pub type data_exchange_cb_t = unsafe extern "C" fn(*mut c_void, *mut sk_buff, i32);

extern "C" {
    fn nfc_hci_hcp_message_tx(
        hdev: *mut nfc_hci_dev, pipe: u8, typ: u8, cmd: u8, param: *const u8,
        param_len: usize, cb: Option<data_exchange_cb_t>, cb_context: *mut c_void,
        max_fwi: i32,
    ) -> i32;
    fn kfree_skb(skb: *mut sk_buff);
    fn kmalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn memcpy(dest: *mut u8, src: *const u8, n: usize);
    fn wake_up(wq: *mut c_void);
    fn wait_event(wq: *mut c_void, condition: bool);
    fn test_bit(nr: usize, addr: *const usize) -> bool;
    fn nfc_hci_reset_pipes(hdev: *mut nfc_hci_dev);
}

const NFC_HCI_HCP_COMMAND: u8 = 0;
const NFC_HCI_HCP_EVENT: u8 = 1;
const NFC_HCI_INVALID_PIPE: u8 = 0xff;
const NFC_HCI_LINK_MGMT_PIPE: u8 = 0;
const NFC_HCI_ADMIN_PIPE: u8 = 1;
const NFC_HCI_LINK_MGMT_GATE: u8 = 0x01;
const NFC_HCI_ADMIN_GATE: u8 = 0x02;
const NFC_HCI_DO_NOT_CREATE_PIPE: u8 = 0xfe;
const NFC_HCI_ANY_SET_PARAMETER: u8 = 0x01;
const NFC_HCI_ANY_GET_PARAMETER: u8 = 0x02;
const NFC_HCI_ANY_OPEN_PIPE: u8 = 0x03;
const NFC_HCI_ANY_CLOSE_PIPE: u8 = 0x04;
const NFC_HCI_ADM_CREATE_PIPE: u8 = 0x10;
const NFC_HCI_ADM_DELETE_PIPE: u8 = 0x11;
const NFC_HCI_ADM_CLEAR_ALL_PIPE: u8 = 0x12;
const NFC_HCI_QUIRK_SHORT_CLEAR: usize = 0;
const GFP_KERNEL: u32 = 0;

unsafe extern "C" fn nfc_hci_execute_cb(context: *mut c_void, skb: *mut sk_buff, err: i32) {
    let hcp_ew = &mut *(context as *mut hcp_exec_waiter);

    hcp_ew.exec_result = err;
    if hcp_ew.exec_result == 0 {
        hcp_ew.result_skb = skb;
    } else {
        kfree_skb(skb);
    }
    hcp_ew.exec_complete = true;

    wake_up(hcp_ew.wq);
}

unsafe fn nfc_hci_execute_cmd_async(
    hdev: *mut nfc_hci_dev, pipe: u8, cmd: u8, param: *const u8, param_len: usize,
    cb: Option<data_exchange_cb_t>, cb_context: *mut c_void,
) -> i32 {
    nfc_hci_hcp_message_tx(hdev, pipe, NFC_HCI_HCP_COMMAND, cmd, param, param_len,
                           cb, cb_context, MAX_FWI)
}

unsafe fn nfc_hci_execute_cmd(
    hdev: *mut nfc_hci_dev, pipe: u8, cmd: u8, param: *const u8, param_len: usize,
    skb: *mut *mut sk_buff,
) -> i32 {
    let mut ew_wq: c_void = c_void::default();
    let mut hcp_ew = hcp_exec_waiter {
        wq: &mut ew_wq,
        exec_complete: false,
        result_skb: core::ptr::null_mut(),
        exec_result: 0,
    };

    hcp_ew.exec_result = nfc_hci_hcp_message_tx(
        hdev, pipe, NFC_HCI_HCP_COMMAND, cmd, param, param_len,
        Some(nfc_hci_execute_cb), &mut hcp_ew as *mut _ as *mut c_void, MAX_FWI,
    );
    if hcp_ew.exec_result < 0 {
        return hcp_ew.exec_result;
    }

    wait_event(&mut ew_wq, hcp_ew.exec_complete == true);

    if hcp_ew.exec_result == 0 {
        if !skb.is_null() {
            *skb = hcp_ew.result_skb;
        } else {
            kfree_skb(hcp_ew.result_skb);
        }
    }

    hcp_ew.exec_result
}

pub unsafe fn nfc_hci_send_event(
    hdev: *mut nfc_hci_dev, gate: u8, event: u8, param: *const u8, param_len: usize,
) -> i32 {
    let pipe = (*hdev).gate2pipe[gate as usize];
    if pipe == NFC_HCI_INVALID_PIPE { return -99; }
    nfc_hci_hcp_message_tx(hdev, pipe, NFC_HCI_HCP_EVENT, event, param, param_len,
                           None, core::ptr::null_mut(), 0)
}

pub unsafe fn nfc_hci_send_cmd(
    hdev: *mut nfc_hci_dev, gate: u8, cmd: u8, param: *const u8, param_len: usize,
    skb: *mut *mut sk_buff,
) -> i32 {
    let pipe = (*hdev).gate2pipe[gate as usize];
    if pipe == NFC_HCI_INVALID_PIPE { return -99; }
    nfc_hci_execute_cmd(hdev, pipe, cmd, param, param_len, skb)
}

pub unsafe fn nfc_hci_send_cmd_async(
    hdev: *mut nfc_hci_dev, gate: u8, cmd: u8, param: *const u8, param_len: usize,
    cb: Option<data_exchange_cb_t>, cb_context: *mut c_void,
) -> i32 {
    let pipe = (*hdev).gate2pipe[gate as usize];
    if pipe == NFC_HCI_INVALID_PIPE { return -99; }
    nfc_hci_execute_cmd_async(hdev, pipe, cmd, param, param_len, cb, cb_context)
}

pub unsafe fn nfc_hci_set_param(
    hdev: *mut nfc_hci_dev, gate: u8, idx: u8, param: *const u8, param_len: usize,
) -> i32 {
    let tmp = kmalloc(1 + param_len, GFP_KERNEL);
    if tmp.is_null() { return -12; }
    *tmp = idx;
    memcpy(tmp.add(1), param, param_len);
    let r = nfc_hci_send_cmd(hdev, gate, NFC_HCI_ANY_SET_PARAMETER, tmp,
                             param_len + 1, core::ptr::null_mut());
    kfree(tmp);
    r
}

pub unsafe fn nfc_hci_get_param(
    hdev: *mut nfc_hci_dev, gate: u8, idx: u8, skb: *mut *mut sk_buff,
) -> i32 {
    nfc_hci_send_cmd(hdev, gate, NFC_HCI_ANY_GET_PARAMETER, &idx, 1, skb)
}

unsafe fn nfc_hci_open_pipe(hdev: *mut nfc_hci_dev, pipe: u8) -> i32 {
    let mut skb = core::ptr::null_mut();
    let r = nfc_hci_execute_cmd(hdev, pipe, NFC_HCI_ANY_OPEN_PIPE,
                                 core::ptr::null(), 0, &mut skb);
    if r == 0 { kfree_skb(skb); }
    r
}

unsafe fn nfc_hci_close_pipe(hdev: *mut nfc_hci_dev, pipe: u8) -> i32 {
    nfc_hci_execute_cmd(hdev, pipe, NFC_HCI_ANY_CLOSE_PIPE, core::ptr::null(), 0,
                        core::ptr::null_mut())
}

unsafe fn nfc_hci_create_pipe(hdev: *mut nfc_hci_dev, dest_host: u8, dest_gate: u8,
                              result: *mut i32) -> u8 {
    let params = hci_create_pipe_params { src_gate: NFC_HCI_ADMIN_GATE, dest_host, dest_gate };
    let mut skb = core::ptr::null_mut();
    *result = nfc_hci_execute_cmd(hdev, NFC_HCI_ADMIN_PIPE, NFC_HCI_ADM_CREATE_PIPE,
                                  &params as *const _ as *const u8, core::mem::size_of_val(&params),
                                  &mut skb);
    if *result < 0 { return NFC_HCI_INVALID_PIPE; }
    let pipe = (*(skb as *mut sk_buff)).data as *const hci_create_pipe_resp;
    let pipe = (*pipe).pipe;
    kfree_skb(skb);
    pipe
}

unsafe fn nfc_hci_delete_pipe(hdev: *mut nfc_hci_dev, pipe: u8) -> i32 {
    nfc_hci_execute_cmd(hdev, NFC_HCI_ADMIN_PIPE, NFC_HCI_ADM_DELETE_PIPE, &pipe, 1,
                        core::ptr::null_mut())
}

unsafe fn nfc_hci_clear_all_pipes(hdev: *mut nfc_hci_dev) -> i32 {
    let param = [0u8; 2];
    let param_len = if test_bit(NFC_HCI_QUIRK_SHORT_CLEAR, &(*hdev).quirks) { 0 } else { 2 };
    nfc_hci_execute_cmd(hdev, NFC_HCI_ADMIN_PIPE, NFC_HCI_ADM_CLEAR_ALL_PIPE,
                        param.as_ptr(), param_len, core::ptr::null_mut())
}

pub unsafe fn nfc_hci_disconnect_gate(hdev: *mut nfc_hci_dev, gate: u8) -> i32 {
    let pipe = (*hdev).gate2pipe[gate as usize];
    if pipe == NFC_HCI_INVALID_PIPE { return -99; }
    let mut r = nfc_hci_close_pipe(hdev, pipe);
    if r < 0 { return r; }
    if pipe != NFC_HCI_LINK_MGMT_PIPE && pipe != NFC_HCI_ADMIN_PIPE {
        r = nfc_hci_delete_pipe(hdev, pipe);
        if r < 0 { return r; }
    }
    (*hdev).gate2pipe[gate as usize] = NFC_HCI_INVALID_PIPE;
    0
}

pub unsafe fn nfc_hci_disconnect_all_gates(hdev: *mut nfc_hci_dev) -> i32 {
    let r = nfc_hci_clear_all_pipes(hdev);
    if r < 0 { return r; }
    nfc_hci_reset_pipes(hdev);
    0
}

pub unsafe fn nfc_hci_connect_gate(hdev: *mut nfc_hci_dev, dest_host: u8, dest_gate: u8,
                                   mut pipe: u8) -> i32 {
    let mut pipe_created = false;
    let mut r: i32;
    if pipe == NFC_HCI_DO_NOT_CREATE_PIPE { return 0; }
    if (*hdev).gate2pipe[dest_gate as usize] != NFC_HCI_INVALID_PIPE { return -98; }
    if pipe == NFC_HCI_INVALID_PIPE {
        pipe = match dest_gate {
            NFC_HCI_LINK_MGMT_GATE => NFC_HCI_LINK_MGMT_PIPE,
            NFC_HCI_ADMIN_GATE => NFC_HCI_ADMIN_PIPE,
            _ => { let p = nfc_hci_create_pipe(hdev, dest_host, dest_gate, &mut r);
                   if p == NFC_HCI_INVALID_PIPE { return r; }
                   pipe_created = true; p }
        };
    }
    r = nfc_hci_open_pipe(hdev, pipe);
    if r < 0 {
        if pipe_created { let _ = nfc_hci_delete_pipe(hdev, pipe); }
        return r;
    }
    (*hdev).pipes[pipe as usize].gate = dest_gate;
    (*hdev).pipes[pipe as usize].dest_host = dest_host;
    (*hdev).gate2pipe[dest_gate as usize] = pipe;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
