// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 Google Corporation
 */

// Linux Bluetooth and HCI declarations are supplied by the surrounding kernel
// translation unit.  The C includes are intentionally not emitted here.

use core::ffi::{c_int, c_void};

extern "C" {
    fn bt_skb_alloc(size: usize, gfp: u32) -> *mut sk_buff;
    fn skb_put(skb: *mut sk_buff, len: usize) -> *mut c_void;
    fn skb_put_data(skb: *mut sk_buff, data: *const c_void, len: usize);
    fn skb_pull_data(skb: *mut sk_buff, len: usize) -> *mut c_void;
    fn hci_recv_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int;
    fn hci_skb_pkt_type(skb: *mut sk_buff) -> *mut u8;
    fn hci_opcode_ogf(opcode: u16) -> u16;
    fn hci_opcode_ocf(opcode: u16) -> u16;
}

// These types, constants, and handler declarations originate in the included
// kernel Bluetooth headers and are resolved by the containing translation.
#[allow(non_camel_case_types)]
type u8_ = u8;
#[allow(non_camel_case_types)]
type u16_ = u16;

pub unsafe fn hci_drv_cmd_status(
    hdev: *mut hci_dev,
    cmd: u16,
    status: u8,
) -> c_int {
    let hdr: *mut hci_drv_ev_hdr;
    let ev: *mut hci_drv_ev_cmd_status;
    let skb: *mut sk_buff;

    skb = bt_skb_alloc(
        core::mem::size_of::<hci_drv_ev_hdr>()
            + core::mem::size_of::<hci_drv_ev_cmd_status>(),
        GFP_KERNEL,
    );
    if skb.is_null() {
        return -ENOMEM;
    }

    hdr = skb_put(skb, core::mem::size_of::<hci_drv_ev_hdr>())
        as *mut hci_drv_ev_hdr;
    (*hdr).opcode = __cpu_to_le16(HCI_DRV_EV_CMD_STATUS);
    (*hdr).len = __cpu_to_le16(core::mem::size_of::<hci_drv_ev_cmd_status>() as u16);

    ev = skb_put(skb, core::mem::size_of::<hci_drv_ev_cmd_status>())
        as *mut hci_drv_ev_cmd_status;
    (*ev).opcode = __cpu_to_le16(cmd);
    (*ev).status = status;

    *hci_skb_pkt_type(skb) = HCI_DRV_PKT;

    hci_recv_frame(hdev, skb)
}

pub unsafe fn hci_drv_cmd_complete(
    hdev: *mut hci_dev,
    cmd: u16,
    status: u8,
    rp: *mut c_void,
    rp_len: usize,
) -> c_int {
    let hdr: *mut hci_drv_ev_hdr;
    let ev: *mut hci_drv_ev_cmd_complete;
    let skb: *mut sk_buff;

    skb = bt_skb_alloc(
        core::mem::size_of::<hci_drv_ev_hdr>()
            + core::mem::size_of::<hci_drv_ev_cmd_complete>()
            + rp_len,
        GFP_KERNEL,
    );
    if skb.is_null() {
        return -ENOMEM;
    }

    hdr = skb_put(skb, core::mem::size_of::<hci_drv_ev_hdr>())
        as *mut hci_drv_ev_hdr;
    (*hdr).opcode = __cpu_to_le16(HCI_DRV_EV_CMD_COMPLETE);
    (*hdr).len = __cpu_to_le16(
        (core::mem::size_of::<hci_drv_ev_cmd_complete>() + rp_len) as u16,
    );

    ev = skb_put(skb, core::mem::size_of::<hci_drv_ev_cmd_complete>())
        as *mut hci_drv_ev_cmd_complete;
    (*ev).opcode = __cpu_to_le16(cmd);
    (*ev).status = status;

    skb_put_data(skb, rp as *const c_void, rp_len);

    *hci_skb_pkt_type(skb) = HCI_DRV_PKT;

    hci_recv_frame(hdev, skb)
}

pub unsafe fn hci_drv_process_cmd(
    hdev: *mut hci_dev,
    skb: *mut sk_buff,
) -> c_int {
    let hdr: *mut hci_drv_cmd_hdr;
    let mut handler: *const hci_drv_handler = core::ptr::null();
    let mut opcode: u16;
    let mut len: u16;
    let mut ogf: u16;
    let mut ocf: u16;

    hdr = skb_pull_data(skb, core::mem::size_of::<hci_drv_cmd_hdr>())
        as *mut hci_drv_cmd_hdr;
    if hdr.is_null() {
        return -EILSEQ;
    }

    opcode = __le16_to_cpu((*hdr).opcode);
    len = __le16_to_cpu((*hdr).len);
    if len as usize != (*skb).len {
        return -EILSEQ;
    }

    ogf = hci_opcode_ogf(opcode);
    ocf = hci_opcode_ocf(opcode);

    if (*hdev).hci_drv.is_null() {
        return hci_drv_cmd_status(hdev, opcode, HCI_DRV_STATUS_UNKNOWN_COMMAND);
    }

    if ogf != HCI_DRV_OGF_DRIVER_SPECIFIC {
        if opcode < (*(*hdev).hci_drv).common_handler_count {
            handler = (*(*hdev).hci_drv).common_handlers.add(opcode as usize);
        }
    } else if ocf < (*(*hdev).hci_drv).specific_handler_count {
        handler = (*(*hdev).hci_drv).specific_handlers.add(ocf as usize);
    }

    if handler.is_null() || (*handler).func.is_none() {
        return hci_drv_cmd_status(hdev, opcode, HCI_DRV_STATUS_UNKNOWN_COMMAND);
    }

    if len as usize != (*handler).data_len {
        return hci_drv_cmd_status(hdev, opcode, HCI_DRV_STATUS_INVALID_PARAMETERS);
    }

    ((*handler).func.unwrap())(hdev, (*skb).data, len as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
