// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 Intel Corporation
 */

// Dependencies supplied by the surrounding Bluetooth implementation.

#[repr(C, packed)]
pub struct aosp_rp_le_get_vendor_capa {
    // v0.95: 15 octets
    pub status: u8,
    pub max_advt_instances: u8,
    pub offloaded_resolution_of_private_address: u8,
    pub total_scan_results_storage: u16,
    pub max_irk_list_sz: u8,
    pub filtering_support: u8,
    pub max_filter: u8,
    pub activity_energy_info_support: u8,
    pub version_supported: u16,
    pub total_num_of_advt_tracked: u16,
    pub extended_scan_support: u8,
    pub debug_logging_supported: u8,
    // v0.96: 16 octets
    pub le_address_generation_offloading_support: u8,
    // v0.98: 21 octets
    pub a2dp_source_offload_capability_mask: u32,
    pub bluetooth_quality_report_support: u8,
    // v1.00: 25 octets
    pub dynamic_audio_buffer_support: u32,
}

pub const VENDOR_CAPA_BASE_SIZE: usize = 15;
pub const VENDOR_CAPA_0_98_SIZE: usize = 21;

pub unsafe fn aosp_do_open(hdev: *mut hci_dev) {
    let mut skb: *mut sk_buff;
    let rp: *mut aosp_rp_le_get_vendor_capa;
    let version_supported: u16;

    if !(*hdev).aosp_capable {
        return;
    }

    bt_dev_dbg(hdev, "Initialize AOSP extension");

    // LE Get Vendor Capabilities Command
    skb = __hci_cmd_sync(hdev, hci_opcode_pack(0x3f, 0x153), 0, core::ptr::null_mut(), HCI_CMD_TIMEOUT);
    if is_err_or_null(skb) {
        if skb.is_null() {
            skb = err_ptr(-EIO);
        }
        bt_dev_err(hdev, "AOSP get vendor capabilities (%ld)", ptr_err(skb));
        return;
    }

    // A basic length check
    if (*skb).len < VENDOR_CAPA_BASE_SIZE {
        bt_dev_err(hdev, "AOSP capabilities length %d too short", (*skb).len);
        kfree_skb(skb);
        return;
    }

    rp = (*skb).data as *mut aosp_rp_le_get_vendor_capa;
    version_supported = u16::from_le((*rp).version_supported);
    // AOSP displays the version number like v0.98, v1.00, etc.
    bt_dev_info(hdev, "AOSP extensions version v%u.%02u", version_supported >> 8, version_supported & 0xff);

    // Do not support very old versions.
    if version_supported < 95 {
        bt_dev_warn(hdev, "AOSP capabilities version %u too old", version_supported);
        kfree_skb(skb);
        return;
    }

    if version_supported < 98 {
        bt_dev_warn(hdev, "AOSP quality report is not supported");
        kfree_skb(skb);
        return;
    }

    if (*skb).len < VENDOR_CAPA_0_98_SIZE {
        bt_dev_err(hdev, "AOSP capabilities length %d too short", (*skb).len);
        kfree_skb(skb);
        return;
    }

    // The bluetooth_quality_report_support is defined at version v0.98.
    if (*rp).bluetooth_quality_report_support != 0 {
        (*hdev).aosp_quality_report = true;
        bt_dev_info(hdev, "AOSP quality report is supported");
    }

    kfree_skb(skb);
}

pub unsafe fn aosp_do_close(hdev: *mut hci_dev) {
    if !(*hdev).aosp_capable {
        return;
    }
    bt_dev_dbg(hdev, "Cleanup of AOSP extension");
}

pub const BQR_OPCODE: u16 = hci_opcode_pack(0x3f, 0x015e);
pub const REPORT_ACTION_ADD: u8 = 0x00;
pub const REPORT_ACTION_DELETE: u8 = 0x01;
pub const REPORT_ACTION_CLEAR: u8 = 0x02;
pub const QUALITY_MONITORING: u32 = 1 << 0;
pub const APPRAOCHING_LSTO: u32 = 1 << 1;
pub const A2DP_AUDIO_CHOPPY: u32 = 1 << 2;
pub const SCO_VOICE_CHOPPY: u32 = 1 << 3;
pub const DEFAULT_BQR_EVENT_MASK: u32 = QUALITY_MONITORING | APPRAOCHING_LSTO | A2DP_AUDIO_CHOPPY | SCO_VOICE_CHOPPY;
pub const DEFALUT_REPORT_INTERVAL_MS: u16 = 5000;

#[repr(C, packed)]
pub struct aosp_bqr_cp {
    pub report_action: u8,
    pub event_mask: u32,
    pub min_report_interval: u16,
}

unsafe fn enable_quality_report(hdev: *mut hci_dev) -> i32 {
    let mut cp = aosp_bqr_cp { report_action: REPORT_ACTION_ADD, event_mask: DEFAULT_BQR_EVENT_MASK, min_report_interval: DEFALUT_REPORT_INTERVAL_MS };
    let mut skb = __hci_cmd_sync(hdev, BQR_OPCODE, core::mem::size_of::<aosp_bqr_cp>(), &mut cp as *mut _ as *mut core::ffi::c_void, HCI_CMD_TIMEOUT);
    if is_err_or_null(skb) {
        if skb.is_null() { skb = err_ptr(-EIO); }
        bt_dev_err(hdev, "Enabling Android BQR failed (%ld)", ptr_err(skb));
        return ptr_err(skb);
    }
    kfree_skb(skb);
    0
}

unsafe fn disable_quality_report(hdev: *mut hci_dev) -> i32 {
    let mut cp = aosp_bqr_cp { report_action: 0, event_mask: 0, min_report_interval: 0 };
    cp.report_action = REPORT_ACTION_CLEAR;
    let mut skb = __hci_cmd_sync(hdev, BQR_OPCODE, core::mem::size_of::<aosp_bqr_cp>(), &mut cp as *mut _ as *mut core::ffi::c_void, HCI_CMD_TIMEOUT);
    if is_err_or_null(skb) {
        if skb.is_null() { skb = err_ptr(-EIO); }
        bt_dev_err(hdev, "Disabling Android BQR failed (%ld)", ptr_err(skb));
        return ptr_err(skb);
    }
    kfree_skb(skb);
    0
}

pub unsafe fn aosp_has_quality_report(hdev: *mut hci_dev) -> bool { (*hdev).aosp_quality_report }

pub unsafe fn aosp_set_quality_report(hdev: *mut hci_dev, enable: bool) -> i32 {
    if !aosp_has_quality_report(hdev) { return -EOPNOTSUPP; }
    bt_dev_dbg(hdev, "quality report enable %d", enable as i32);
    if enable { enable_quality_report(hdev) } else { disable_quality_report(hdev) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
