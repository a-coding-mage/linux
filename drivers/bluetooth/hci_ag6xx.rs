// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Bluetooth HCI UART driver for Intel/AG6xx devices
 *
 * Copyright (C) 2016 Intel Corporation
 */

#[repr(C)]
pub struct ag6xx_data {
    pub rx_skb: *mut sk_buff,
    pub txq: sk_buff_head,
}

#[repr(C, packed)]
pub struct pbn_entry {
    pub addr: __le32,
    pub plen: __le32,
    pub data: [__u8; 0],
}

unsafe fn ag6xx_open(hu: *mut hci_uart) -> c_int {
    let ag6xx: *mut ag6xx_data = kzalloc_obj::<ag6xx_data>();

    BT_DBG!("hu %p", hu);

    if ag6xx.is_null() {
        return -ENOMEM;
    }

    skb_queue_head_init(&mut (*ag6xx).txq);

    (*hu).priv_ = ag6xx as *mut c_void;
    0
}

unsafe fn ag6xx_close(hu: *mut hci_uart) -> c_int {
    let ag6xx = (*hu).priv_ as *mut ag6xx_data;

    BT_DBG!("hu %p", hu);

    skb_queue_purge(&mut (*ag6xx).txq);
    kfree_skb((*ag6xx).rx_skb);
    kfree(ag6xx as *mut c_void);

    (*hu).priv_ = core::ptr::null_mut();
    0
}

unsafe fn ag6xx_flush(hu: *mut hci_uart) -> c_int {
    let ag6xx = (*hu).priv_ as *mut ag6xx_data;

    BT_DBG!("hu %p", hu);

    skb_queue_purge(&mut (*ag6xx).txq);
    0
}

unsafe fn ag6xx_dequeue(hu: *mut hci_uart) -> *mut sk_buff {
    let ag6xx = (*hu).priv_ as *mut ag6xx_data;
    let skb = skb_dequeue(&mut (*ag6xx).txq);
    if skb.is_null() {
        return skb;
    }

    // Prepend skb with frame type
    core::ptr::copy_nonoverlapping(
        &bt_cb(skb).pkt_type as *const _ as *const u8,
        skb_push(skb, 1),
        1,
    );
    skb
}

unsafe fn ag6xx_enqueue(hu: *mut hci_uart, skb: *mut sk_buff) -> c_int {
    let ag6xx = (*hu).priv_ as *mut ag6xx_data;
    skb_queue_tail(&mut (*ag6xx).txq, skb);
    0
}

static ag6xx_recv_pkts: [h4_recv_pkt; 3] = [
    h4_recv_pkt { typ: H4_RECV_ACL, recv: hci_recv_frame },
    h4_recv_pkt { typ: H4_RECV_SCO, recv: hci_recv_frame },
    h4_recv_pkt { typ: H4_RECV_EVENT, recv: hci_recv_frame },
];

unsafe fn ag6xx_recv(hu: *mut hci_uart, data: *const c_void, count: c_int) -> c_int {
    let ag6xx = (*hu).priv_ as *mut ag6xx_data;

    if !test_bit(HCI_UART_REGISTERED, &(*hu).flags) {
        return -EUNATCH;
    }

    (*ag6xx).rx_skb = h4_recv_buf(
        hu,
        (*ag6xx).rx_skb,
        data,
        count,
        ag6xx_recv_pkts.as_ptr(),
        ag6xx_recv_pkts.len(),
    );
    if IS_ERR((*ag6xx).rx_skb) {
        let err = PTR_ERR((*ag6xx).rx_skb);
        bt_dev_err!((*hu).hdev, "Frame reassembly failed ({})", err);
        (*ag6xx).rx_skb = core::ptr::null_mut();
        return err;
    }
    count
}

unsafe fn intel_mem_write(hdev: *mut hci_dev, mut addr: u32, mut plen: u32, mut data: *const c_void) -> c_int {
    // Can write a maximum of 247 bytes per HCI command.
    // HCI cmd Header (3), Intel mem write header (6), data (247).
    while plen > 0 {
        let mut cmd_param = [0u8; 253];
        let fragment_len: u8 = if plen > 247 { 247 } else { plen as u8 };
        let leaddr = cpu_to_le32(addr);

        core::ptr::copy_nonoverlapping(&leaddr as *const _ as *const u8, cmd_param.as_mut_ptr(), 4);
        cmd_param[4] = 0;
        cmd_param[5] = fragment_len;
        core::ptr::copy_nonoverlapping(data as *const u8, cmd_param.as_mut_ptr().add(6), fragment_len as usize);

        let skb = __hci_cmd_sync(hdev, 0xfc8e, fragment_len as u16 + 6, cmd_param.as_ptr(), HCI_INIT_TIMEOUT);
        if IS_ERR(skb) {
            return PTR_ERR(skb);
        }
        kfree_skb(skb);

        plen -= fragment_len as u32;
        data = (data as *const u8).add(fragment_len as usize) as *const c_void;
        addr += fragment_len as u32;
    }
    0
}

unsafe fn ag6xx_setup(hu: *mut hci_uart) -> c_int {
    let hdev = (*hu).hdev;
    let mut skb: *mut sk_buff;
    let mut ver: intel_version = core::mem::zeroed();
    let mut fw: *const firmware;
    let mut fw_ptr: *const u8;
    let mut fwname = [0i8; 64];
    let mut patched = false;
    let mut err: c_int;

    (*hdev).set_diag = Some(btintel_set_diag);
    (*hdev).set_bdaddr = Some(btintel_set_bdaddr);

    err = btintel_enter_mfg(hdev); if err != 0 { return err; }
    err = btintel_read_version(hdev, &mut ver); if err != 0 { return err; }
    btintel_version_info(hdev, &ver);
    if ver.hw_platform != 0x37 { bt_dev_err!(hdev, "Unsupported Intel hardware platform: 0x%X", ver.hw_platform); return -EINVAL; }
    if ver.hw_variant != 0x0a { bt_dev_err!(hdev, "Unsupported Intel hardware variant: 0x%x", ver.hw_variant); return -EINVAL; }
    snprintf(fwname.as_mut_ptr(), fwname.len(), c"intel/ibt-hw-%x.%x.bddata", ver.hw_platform, ver.hw_variant);

    err = request_firmware(&mut fw, fwname.as_ptr(), &(*hdev).dev);
    if err < 0 { bt_dev_err!(hdev, "Failed to open Intel bddata file: %s (%d)", fwname.as_ptr(), err); } else {
        bt_dev_info!(hdev, "Applying bddata (%s)", fwname.as_ptr());
        skb = __hci_cmd_sync_ev(hdev, 0xfc2f, (*fw).size, (*fw).data, HCI_EV_CMD_STATUS, HCI_CMD_TIMEOUT);
        if IS_ERR(skb) { bt_dev_err!(hdev, "Applying bddata failed (%ld)", PTR_ERR(skb)); release_firmware(fw); return PTR_ERR(skb); }
        kfree_skb(skb); release_firmware(fw);
    }

    if ver.fw_patch_num != 0 { bt_dev_info!(hdev, "Device is already patched. patch num: %02x", ver.fw_patch_num); patched = true; } else {
        snprintf(fwname.as_mut_ptr(), fwname.len(), c"intel/ibt-hw-%x.%x.%x-fw-%x.%x.%x.%x.%x.pbn", ver.hw_platform, ver.hw_variant, ver.hw_revision, ver.fw_variant, ver.fw_revision, ver.fw_build_num, ver.fw_build_ww, ver.fw_build_yy);
        err = request_firmware(&mut fw, fwname.as_ptr(), &(*hdev).dev);
        if err >= 0 {
            fw_ptr = (*fw).data;
            bt_dev_info!(hdev, "Patching firmware file (%s)", fwname.as_ptr());
            while (*fw).size > fw_ptr.offset_from((*fw).data) as usize {
                let pbn = fw_ptr as *mut pbn_entry;
                if (*pbn).addr == 0xffffffff { bt_dev_info!(hdev, "Patching complete"); patched = true; break; }
                let addr = le32_to_cpu((*pbn).addr); let patch_len = le32_to_cpu((*pbn).plen);
                if (*fw).data.add((*fw).size) <= (*pbn).data.as_ptr().add(patch_len as usize) { bt_dev_info!(hdev, "Invalid patch len (%d)", patch_len); break; }
                bt_dev_info!(hdev, "Patching %td/%zu", fw_ptr.offset_from((*fw).data), (*fw).size);
                err = intel_mem_write(hdev, addr, patch_len, (*pbn).data.as_ptr() as *const c_void);
                if err != 0 { bt_dev_err!(hdev, "Patching failed"); break; }
                fw_ptr = (*pbn).data.as_ptr().add(patch_len as usize);
            }
            release_firmware(fw);
        }
    }

    err = btintel_exit_mfg(hdev, true, patched); if err != 0 { return err; }
    btintel_set_event_mask_mfg(hdev, false);
    btintel_check_bdaddr(hdev);
    0
}

static ag6xx_proto: hci_uart_proto = hci_uart_proto {
    id: HCI_UART_AG6XX, name: c"AG6XX", manufacturer: 2,
    open: Some(ag6xx_open), close: Some(ag6xx_close), flush: Some(ag6xx_flush), setup: Some(ag6xx_setup), recv: Some(ag6xx_recv), enqueue: Some(ag6xx_enqueue), dequeue: Some(ag6xx_dequeue),
};

pub unsafe fn ag6xx_init() -> c_int { hci_uart_register_proto(&ag6xx_proto) }
pub unsafe fn ag6xx_deinit() -> c_int { hci_uart_unregister_proto(&ag6xx_proto) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
