// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2015 - 2021 Intel Corporation */

// Dependencies supplied by the surrounding kernel/Rust translation.

const ADF_PF_WAIT_RESTARTING_COMPLETE_DELAY: u32 = 100;
const ADF_VF_SHUTDOWN_RETRY: i32 = 100;

extern "C" {
    fn pci_num_vf(dev: *mut pci_dev) -> i32;
    fn accel_to_pci_dev(accel_dev: *mut adf_accel_dev) -> *mut pci_dev;
    fn adf_send_pf2vf_msg(
        accel_dev: *mut adf_accel_dev,
        vf: i32,
        msg: pfvf_message,
    ) -> i32;
    fn msleep(msecs: u32);
    fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct adf_accel_vf_info {
    pub init: bool,
    pub vf_compat_ver: u8,
    pub restarting: bool,
}

#[repr(C)]
pub struct adf_hw_device_data {
    pub extended_dc_capabilities: u32,
    pub accel_capabilities_mask: u32,
    pub ring_to_svc_map: u32,
}

#[repr(C)]
pub struct adf_accel_dev_pf {
    pub vf_info: *mut adf_accel_vf_info,
}

#[repr(C)]
pub struct adf_accel_dev {
    pub pf: adf_accel_dev_pf,
    pub hw_device: *mut adf_hw_device_data,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct pfvf_message {
    pub type_: u32,
}

#[repr(C)]
pub struct capabilities_v2_hdr {
    pub version: u8,
    pub payload_size: u8,
}

#[repr(C)]
pub struct capabilities_v2 {
    pub ext_dc_caps: u32,
    pub capabilities: u32,
    pub hdr: capabilities_v2_hdr,
}

#[repr(C)]
pub struct ring_to_svc_map_v1_hdr {
    pub version: u8,
    pub payload_size: u8,
}

#[repr(C)]
pub struct ring_to_svc_map_v1 {
    pub map: u32,
    pub hdr: ring_to_svc_map_v1_hdr,
}

extern "C" {
    static ADF_PF2VF_MSGTYPE_RESTARTING: u32;
    static ADF_PF2VF_MSGTYPE_RESTARTED: u32;
    static ADF_PF2VF_MSGTYPE_FATAL_ERROR: u32;
    static ADF_PFVF_COMPAT_FALLBACK: u8;
    static ADF_PFVF_CAPABILITIES_V2_VERSION: u8;
    static ADF_PFVF_RING_TO_SVC_VERSION: u8;
}

pub unsafe fn adf_pf2vf_notify_restarting(accel_dev: *mut adf_accel_dev) {
    let num_vfs = pci_num_vf(accel_to_pci_dev(accel_dev));
    let msg = pfvf_message { type_: ADF_PF2VF_MSGTYPE_RESTARTING };
    let mut vf = (*accel_dev).pf.vf_info;

    let mut i = 0;
    while i < num_vfs {
        (*vf).restarting = (*vf).init && (*vf).vf_compat_ver >= ADF_PFVF_COMPAT_FALLBACK;
        if (*vf).init && adf_send_pf2vf_msg(accel_dev, i, msg) != 0 {
            // dev_err(&GET_DEV(accel_dev), "Failed to send restarting msg to VF%d\n", i);
        }
        vf = vf.add(1);
        i += 1;
    }
}

pub unsafe fn adf_pf2vf_wait_for_restarting_complete(accel_dev: *mut adf_accel_dev) {
    let num_vfs = pci_num_vf(accel_to_pci_dev(accel_dev));
    let mut retries = ADF_VF_SHUTDOWN_RETRY;
    let mut vf_running;

    loop {
        vf_running = false;
        let mut vf = (*accel_dev).pf.vf_info;
        let mut i = 0;
        while i < num_vfs {
            if (*vf).restarting {
                vf_running = true;
            }
            vf = vf.add(1);
            i += 1;
        }
        if !vf_running {
            break;
        }
        msleep(ADF_PF_WAIT_RESTARTING_COMPLETE_DELAY);
        retries -= 1;
        if retries == 0 {
            break;
        }
    }
    // dev_warn(&GET_DEV(accel_dev), "Some VFs are still running\n");
}

pub unsafe fn adf_pf2vf_notify_restarted(accel_dev: *mut adf_accel_dev) {
    let num_vfs = pci_num_vf(accel_to_pci_dev(accel_dev));
    let msg = pfvf_message { type_: ADF_PF2VF_MSGTYPE_RESTARTED };
    let mut vf = (*accel_dev).pf.vf_info;
    let mut i = 0;
    while i < num_vfs {
        if (*vf).init && (*vf).vf_compat_ver >= ADF_PFVF_COMPAT_FALLBACK
            && adf_send_pf2vf_msg(accel_dev, i, msg) != 0
        {
            // dev_err(&GET_DEV(accel_dev), "Failed to send restarted msg to VF%d\n", i);
        }
        vf = vf.add(1);
        i += 1;
    }
}

pub unsafe fn adf_pf2vf_notify_fatal_error(accel_dev: *mut adf_accel_dev) {
    let num_vfs = pci_num_vf(accel_to_pci_dev(accel_dev));
    let msg = pfvf_message { type_: ADF_PF2VF_MSGTYPE_FATAL_ERROR };
    let mut vf = (*accel_dev).pf.vf_info;
    let mut i = 0;
    while i < num_vfs {
        if (*vf).init && (*vf).vf_compat_ver >= ADF_PFVF_COMPAT_FALLBACK
            && adf_send_pf2vf_msg(accel_dev, i, msg) != 0
        {
            // dev_err(&GET_DEV(accel_dev), "Failed to send fatal error msg to VF%d\n", i);
        }
        vf = vf.add(1);
        i += 1;
    }
}

pub unsafe fn adf_pf_capabilities_msg_provider(
    accel_dev: *mut adf_accel_dev,
    buffer: *mut u8,
    _compat: u8,
) -> i32 {
    let hw_data = (*accel_dev).hw_device;
    let caps_msg = capabilities_v2 {
        ext_dc_caps: (*hw_data).extended_dc_capabilities,
        capabilities: (*hw_data).accel_capabilities_mask,
        hdr: capabilities_v2_hdr {
            version: ADF_PFVF_CAPABILITIES_V2_VERSION,
            payload_size: core::mem::size_of::<capabilities_v2>() as u8,
        },
    };
    memcpy(buffer, &caps_msg as *const _ as *const u8, core::mem::size_of::<capabilities_v2>());
    0
}

pub unsafe fn adf_pf_ring_to_svc_msg_provider(
    accel_dev: *mut adf_accel_dev,
    buffer: *mut u8,
    _compat: u8,
) -> i32 {
    let rts_map_msg = ring_to_svc_map_v1 {
        map: (*(*accel_dev).hw_device).ring_to_svc_map,
        hdr: ring_to_svc_map_v1_hdr {
            version: ADF_PFVF_RING_TO_SVC_VERSION,
            payload_size: core::mem::size_of::<ring_to_svc_map_v1>() as u8,
        },
    };
    memcpy(buffer, &rts_map_msg as *const _ as *const u8, core::mem::size_of::<ring_to_svc_map_v1>());
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
