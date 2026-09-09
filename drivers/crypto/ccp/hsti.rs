// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Secure Processor device driver, security attributes
 *
 * Copyright (C) 2023-2024 Advanced Micro Devices, Inc.
 *
 * Author: Mario Limonciello <mario.limonciello@amd.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const PSP_CAPABILITY_PSP_SECURITY_OFFSET: u32 = 8;

#[repr(C, packed)]
pub struct hsti_request {
    pub header: psp_req_buffer_hdr,
    pub hsti: u32,
}

#[repr(C)]
pub struct psp_req_buffer_hdr {
    pub payload_size: usize,
    pub status: i32,
}

#[repr(C)]
pub struct psp_capability {
    pub security_reporting: u32,
    pub raw: u32,
    pub fused_part: i32,
    pub boot_integrity: i32,
    pub debug_lock_on: i32,
    pub tsme_status: i32,
    pub anti_rollback_status: i32,
    pub rpmc_production_enabled: i32,
    pub rpmc_spirom_available: i32,
    pub hsp_tpm_available: i32,
    pub rom_armor_enforced: i32,
}

#[repr(C)]
pub struct psp_device {
    pub dev: *mut device,
    pub capability: psp_capability,
}

#[repr(C)]
pub struct sp_device {
    pub psp_data: *mut psp_device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn kobj_to_dev(kobj: *mut kobject) -> *mut device;
    fn psp_send_platform_access_msg(cmd: u32, req: *mut psp_request) -> i32;
    fn cc_platform_has(attr: u32) -> bool;
    fn dev_notice(dev: *mut device, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
}

#[repr(C)]
pub struct psp_request {
    _private: [u8; 0],
}

const PSP_CMD_HSTI_QUERY: u32 = 0;
const CC_ATTR_HOST_MEM_ENCRYPT: u32 = 0;
const PSP_FEATURE_HSTI: u32 = 0;

unsafe fn psp_feature(psp: *mut psp_device, feature: u32) -> bool {
    // PSP_FEATURE(psp, HSTI), supplied by the surrounding kernel translation.
    ((*psp).capability.raw & feature) != 0
}

macro_rules! security_attribute_show {
    ($name:ident, $show:ident) => {
        pub unsafe extern "C" fn $show(
            d: *mut device,
            _attr: *mut device_attribute,
            buf: *mut u8,
        ) -> isize {
            let sp = dev_get_drvdata(d) as *mut sp_device;
            let psp = (*sp).psp_data;
            // sysfs_emit(buf, "%d\\n", psp->capability.name)
            sysfs_emit_i32(buf, (*psp).capability.$name)
        }
    };
}

extern "C" {
    fn sysfs_emit_i32(buf: *mut u8, value: i32) -> isize;
}

security_attribute_show!(fused_part, fused_part_show);
security_attribute_show!(boot_integrity, boot_integrity_show);
security_attribute_show!(debug_lock_on, debug_lock_on_show);
security_attribute_show!(tsme_status, tsme_status_show);
security_attribute_show!(anti_rollback_status, anti_rollback_status_show);
security_attribute_show!(rpmc_production_enabled, rpmc_production_enabled_show);
security_attribute_show!(rpmc_spirom_available, rpmc_spirom_available_show);
security_attribute_show!(hsp_tpm_available, hsp_tpm_available_show);
security_attribute_show!(rom_armor_enforced, rom_armor_enforced_show);

// DEVICE_ATTR_RO declarations and the attribute array are supplied by the
// kernel sysfs compatibility layer in the final translation unit.
pub static mut psp_security_attrs: [*mut attribute; 10] = [
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(),
];

pub unsafe extern "C" fn psp_security_is_visible(
    kobj: *mut kobject,
    _attr: *mut attribute,
    _idx: i32,
) -> u32 {
    let dev = kobj_to_dev(kobj);
    let sp = dev_get_drvdata(dev) as *mut sp_device;
    let psp = (*sp).psp_data;

    if !psp.is_null() && (*psp).capability.security_reporting != 0 {
        return 0o444;
    }
    0
}

#[repr(C)]
pub struct attribute_group {
    pub attrs: *mut *mut attribute,
    pub is_visible: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, i32) -> u32>,
}

pub static mut psp_security_attr_group: attribute_group = attribute_group {
    attrs: psp_security_attrs.as_mut_ptr(),
    is_visible: Some(psp_security_is_visible),
};

unsafe fn psp_populate_hsti(psp: *mut psp_device) -> i32 {
    let req = kzalloc_obj::<hsti_request>();
    if req.is_null() {
        return -12;
    }

    (*req).header.payload_size = core::mem::size_of::<hsti_request>();

    let mut ret = psp_send_platform_access_msg(PSP_CMD_HSTI_QUERY, req as *mut psp_request);
    if ret != 0 {
        kfree(req as *mut core::ffi::c_void);
        return ret;
    }

    if (*req).header.status != 0 {
        dev_dbg((*psp).dev, b"failed to populate HSTI state: %d\0".as_ptr(), (*req).header.status);
        ret = -22;
    } else {
        (*psp).capability.security_reporting = 1;
        (*psp).capability.raw |= (*req).hsti << PSP_CAPABILITY_PSP_SECURITY_OFFSET;
    }

    kfree(req as *mut core::ffi::c_void);
    ret
}

pub unsafe extern "C" fn psp_init_hsti(psp: *mut psp_device) -> i32 {
    if psp_feature(psp, PSP_FEATURE_HSTI) {
        let ret = psp_populate_hsti(psp);
        if ret != 0 {
            return ret;
        }
    }

    if (*psp).capability.security_reporting == 0 {
        return 0;
    }

    if (*psp).capability.tsme_status != 0 {
        if cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT) {
            dev_notice((*psp).dev, b"psp: Both TSME and SME are active, SME is unnecessary when TSME is active.\n\0".as_ptr());
        } else {
            dev_notice((*psp).dev, b"psp: TSME enabled\n\0".as_ptr());
        }
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
