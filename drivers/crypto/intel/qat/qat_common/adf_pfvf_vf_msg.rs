// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2015 - 2021 Intel Corporation */

// Dependencies are supplied by the surrounding kernel translation.

/**
 * adf_vf2pf_notify_init() - send init msg to PF
 * @accel_dev: Pointer to acceleration VF device.
 */
pub unsafe fn adf_vf2pf_notify_init(accel_dev: *mut adf_accel_dev) -> i32 {
    let msg = pfvf_message { type_: ADF_VF2PF_MSGTYPE_INIT, ..core::mem::zeroed() };

    if adf_send_vf2pf_msg(accel_dev, msg) != 0 {
        dev_err!(GET_DEV(accel_dev), "Failed to send Init event to PF\n");
        return -EFAULT;
    }
    set_bit(ADF_STATUS_PF_RUNNING, &mut (*accel_dev).status);
    0
}

pub unsafe fn adf_vf2pf_notify_shutdown(accel_dev: *mut adf_accel_dev) {
    let msg = pfvf_message { type_: ADF_VF2PF_MSGTYPE_SHUTDOWN, ..core::mem::zeroed() };

    if test_bit(ADF_STATUS_PF_RUNNING, &(*accel_dev).status) != 0
        && adf_send_vf2pf_msg(accel_dev, msg) != 0
    {
        dev_err!(GET_DEV(accel_dev), "Failed to send Shutdown event to PF\n");
    }
}

pub unsafe fn adf_vf2pf_notify_restart_complete(accel_dev: *mut adf_accel_dev) {
    let msg = pfvf_message {
        type_: ADF_VF2PF_MSGTYPE_RESTARTING_COMPLETE,
        ..core::mem::zeroed()
    };

    /* Check compatibility version */
    if (*accel_dev).vf.pf_compat_ver < ADF_PFVF_COMPAT_FALLBACK {
        return;
    }
    if adf_send_vf2pf_msg(accel_dev, msg) != 0 {
        dev_err!(GET_DEV(accel_dev), "Failed to send Restarting complete event to PF\n");
    }
}

pub unsafe fn adf_vf2pf_request_version(accel_dev: *mut adf_accel_dev) -> i32 {
    let mut pf_version: u8;
    let compat: i32;
    let mut resp: pfvf_message = core::mem::zeroed();
    let msg = pfvf_message {
        type_: ADF_VF2PF_MSGTYPE_COMPAT_VER_REQ,
        data: ADF_PFVF_COMPAT_THIS_VERSION,
        ..core::mem::zeroed()
    };

    let ret = adf_send_vf2pf_req(accel_dev, msg, &mut resp);
    if ret != 0 {
        dev_err!(GET_DEV(accel_dev), "Failed to send Compatibility Version Request.\n");
        return ret;
    }

    pf_version = field_get(ADF_PF2VF_VERSION_RESP_VERS_MASK, resp.data) as u8;
    compat = field_get(ADF_PF2VF_VERSION_RESP_RESULT_MASK, resp.data) as i32;
    match compat {
        ADF_PF2VF_VF_COMPATIBLE | ADF_PF2VF_VF_COMPAT_UNKNOWN => {},
        ADF_PF2VF_VF_INCOMPATIBLE => {
            dev_err!(GET_DEV(accel_dev), "PF (vers %d) and VF (vers %d) are not compatible\n",
                     pf_version, ADF_PFVF_COMPAT_THIS_VERSION);
            return -EINVAL;
        }
        _ => {
            dev_err!(GET_DEV(accel_dev), "Invalid response from PF; assume not compatible\n");
            return -EINVAL;
        }
    }
    (*accel_dev).vf.pf_compat_ver = pf_version;
    0
}

pub unsafe fn adf_vf2pf_get_capabilities(accel_dev: *mut adf_accel_dev) -> i32 {
    let hw_data = (*accel_dev).hw_device;
    let mut cap_msg: capabilities_v3 = core::mem::zeroed();
    let mut len = core::mem::size_of::<capabilities_v3>();

    if (*accel_dev).vf.pf_compat_ver < ADF_PFVF_COMPAT_CAPABILITIES { return 0; }
    if adf_send_vf2pf_blkmsg_req(accel_dev, ADF_VF2PF_BLKMSG_REQ_CAP_SUMMARY,
                                  &mut cap_msg as *mut _ as *mut u8, &mut len) != 0 {
        dev_err!(GET_DEV(accel_dev), "QAT: Failed to get block message response\n");
        return -EFAULT;
    }
    match cap_msg.hdr.version {
        _ => {
            if len >= core::mem::size_of::<capabilities_v3>() { (*hw_data).clock_frequency = cap_msg.frequency; }
            else { dev_info!(GET_DEV(accel_dev), "Could not get frequency"); }
            if len >= core::mem::size_of::<capabilities_v2>() { (*hw_data).accel_capabilities_mask = cap_msg.capabilities; }
            else { dev_info!(GET_DEV(accel_dev), "Could not get capabilities"); }
            if len >= core::mem::size_of::<capabilities_v1>() { (*hw_data).extended_dc_capabilities = cap_msg.ext_dc_caps; }
            else { dev_err!(GET_DEV(accel_dev), "Capabilities message truncated to %d bytes\n", len); return -EFAULT; }
        }
    }
    0
}

pub unsafe fn adf_vf2pf_get_ring_to_svc(accel_dev: *mut adf_accel_dev) -> i32 {
    let mut rts_map_msg: ring_to_svc_map_v1 = core::mem::zeroed();
    let mut len = core::mem::size_of::<ring_to_svc_map_v1>();
    if (*accel_dev).vf.pf_compat_ver < ADF_PFVF_COMPAT_RING_TO_SVC_MAP { return 0; }
    if adf_send_vf2pf_blkmsg_req(accel_dev, ADF_VF2PF_BLKMSG_REQ_RING_SVC_MAP,
                                  &mut rts_map_msg as *mut _ as *mut u8, &mut len) != 0 {
        dev_err!(GET_DEV(accel_dev), "QAT: Failed to get block message response\n"); return -EFAULT;
    }
    if len < core::mem::size_of::<ring_to_svc_map_v1>() {
        dev_err!(GET_DEV(accel_dev), "RING_TO_SVC message truncated to %d bytes\n", len); return -EFAULT;
    }
    (*accel_dev).hw_device.ring_to_svc_map = rts_map_msg.map;
    0
}

#[inline]
unsafe fn field_get(mask: u32, value: u32) -> u32 {
    (value & mask) >> mask.trailing_zeros()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
