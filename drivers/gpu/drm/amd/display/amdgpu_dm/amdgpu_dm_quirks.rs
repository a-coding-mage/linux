// SPDX-License-Identifier: MIT
/*
 * Copyright 2025 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct drm_device;

#[repr(C)]
pub struct amdgpu_display_manager {
    pub ddev: *mut drm_device,
    pub aux_hpd_discon_quirk: bool,
    pub edp0_on_dp1_quirk: bool,
}

#[repr(C)]
pub struct pci_dev {
    pub vendor: u16,
    pub device: u16,
    pub subsystem_vendor: u16,
    pub subsystem_device: u16,
    pub revision: u8,
}

#[repr(C)]
pub struct dmi_system_id {
    pub callback: Option<unsafe extern "C" fn(*const dmi_system_id) -> c_int>,
    pub matches: [dmi_match; 2],
}

#[repr(C)]
pub struct dmi_match {
    pub slot: c_int,
    pub substr: *const c_char,
}

extern "C" {
    fn dmi_check_system(list: *const dmi_system_id) -> c_int;
    fn drm_info(dev: *mut drm_device, fmt: *const c_char, ...);
}

#[repr(C)]
struct amdgpu_dm_quirks {
    aux_hpd_discon: bool,
    support_edp0_on_dp1: bool,
}

static mut quirk_entries: amdgpu_dm_quirks = amdgpu_dm_quirks {
    aux_hpd_discon: false,
    support_edp0_on_dp1: false,
};

unsafe extern "C" fn edp0_on_dp1_callback(_id: *const dmi_system_id) -> c_int {
    quirk_entries.support_edp0_on_dp1 = true;
    0
}

unsafe extern "C" fn aux_hpd_discon_callback(_id: *const dmi_system_id) -> c_int {
    quirk_entries.aux_hpd_discon = true;
    0
}

const DMI_SYS_VENDOR: c_int = 0;
const DMI_PRODUCT_NAME: c_int = 1;

macro_rules! dmi_match {
    ($slot:expr, $s:literal) => {
        dmi_match { slot: $slot, substr: concat!($s, "\0").as_ptr() as *const c_char }
    };
}

static dmi_quirk_table: [dmi_system_id; 16] = [
    dmi_system_id { callback: Some(aux_hpd_discon_callback), matches: [dmi_match!(DMI_SYS_VENDOR, "Dell Inc."), dmi_match!(DMI_PRODUCT_NAME, "Precision 3660")] },
    dmi_system_id { callback: Some(aux_hpd_discon_callback), matches: [dmi_match!(DMI_SYS_VENDOR, "Dell Inc."), dmi_match!(DMI_PRODUCT_NAME, "Precision 3260")] },
    dmi_system_id { callback: Some(aux_hpd_discon_callback), matches: [dmi_match!(DMI_SYS_VENDOR, "Dell Inc."), dmi_match!(DMI_PRODUCT_NAME, "Precision 3460")] },
    dmi_system_id { callback: Some(aux_hpd_discon_callback), matches: [dmi_match!(DMI_SYS_VENDOR, "Dell Inc."), dmi_match!(DMI_PRODUCT_NAME, "OptiPlex Tower Plus 7010")] },
    dmi_system_id { callback: Some(aux_hpd_discon_callback), matches: [dmi_match!(DMI_SYS_VENDOR, "Dell Inc."), dmi_match!(DMI_PRODUCT_NAME, "OptiPlex Tower 7010")] },
    dmi_system_id { callback: Some(aux_hpd_discon_callback), matches: [dmi_match!(DMI_SYS_VENDOR, "Dell Inc."), dmi_match!(DMI_PRODUCT_NAME, "OptiPlex SFF Plus 7010")] },
    dmi_system_id { callback: Some(aux_hpd_discon_callback), matches: [dmi_match!(DMI_SYS_VENDOR, "Dell Inc."), dmi_match!(DMI_PRODUCT_NAME, "OptiPlex SFF 7010")] },
    dmi_system_id { callback: Some(aux_hpd_discon_callback), matches: [dmi_match!(DMI_SYS_VENDOR, "Dell Inc."), dmi_match!(DMI_PRODUCT_NAME, "OptiPlex Micro Plus 7010")] },
    dmi_system_id { callback: Some(aux_hpd_discon_callback), matches: [dmi_match!(DMI_SYS_VENDOR, "Dell Inc."), dmi_match!(DMI_PRODUCT_NAME, "OptiPlex Micro 7010")] },
    dmi_system_id { callback: Some(edp0_on_dp1_callback), matches: [dmi_match!(DMI_SYS_VENDOR, "HP"), dmi_match!(DMI_PRODUCT_NAME, "HP Elite mt645 G8 Mobile Thin Client")] },
    dmi_system_id { callback: Some(edp0_on_dp1_callback), matches: [dmi_match!(DMI_SYS_VENDOR, "HP"), dmi_match!(DMI_PRODUCT_NAME, "HP EliteBook 645 14 inch G11 Notebook PC")] },
    dmi_system_id { callback: Some(edp0_on_dp1_callback), matches: [dmi_match!(DMI_SYS_VENDOR, "HP"), dmi_match!(DMI_PRODUCT_NAME, "HP EliteBook 665 16 inch G11 Notebook PC")] },
    dmi_system_id { callback: Some(edp0_on_dp1_callback), matches: [dmi_match!(DMI_SYS_VENDOR, "HP"), dmi_match!(DMI_PRODUCT_NAME, "HP ProBook 445 14 inch G11 Notebook PC")] },
    dmi_system_id { callback: Some(edp0_on_dp1_callback), matches: [dmi_match!(DMI_SYS_VENDOR, "HP"), dmi_match!(DMI_PRODUCT_NAME, "HP ProBook 465 16 inch G11 Notebook PC")] },
    dmi_system_id { callback: None, matches: [dmi_match!(0, ""), dmi_match!(0, "")] },
    dmi_system_id { callback: None, matches: [dmi_match!(0, ""), dmi_match!(0, "")] },
];

pub unsafe fn retrieve_dmi_info(dm: *mut amdgpu_display_manager) {
    let dev = (*dm).ddev;
    (*dm).aux_hpd_discon_quirk = false;
    (*dm).edp0_on_dp1_quirk = false;
    if dmi_check_system(dmi_quirk_table.as_ptr()) == 0 { return; }
    if quirk_entries.aux_hpd_discon {
        (*dm).aux_hpd_discon_quirk = true;
        drm_info(dev, b"aux_hpd_discon_quirk attached\0".as_ptr() as *const c_char);
    }
    if quirk_entries.support_edp0_on_dp1 {
        (*dm).edp0_on_dp1_quirk = true;
        drm_info(dev, b"support_edp0_on_dp1 attached\0".as_ptr() as *const c_char);
    }
}

#[repr(C)]
struct amdgpu_stutter_quirk { chip_vendor: u16, chip_device: u16, subsys_vendor: u16, subsys_device: u16, revision: u8 }

static amdgpu_stutter_quirk_list: [amdgpu_stutter_quirk; 2] = [
    amdgpu_stutter_quirk { chip_vendor: 0x1002, chip_device: 0x15dd, subsys_vendor: 0x1002, subsys_device: 0x15dd, revision: 0xc8 },
    amdgpu_stutter_quirk { chip_vendor: 0, chip_device: 0, subsys_vendor: 0, subsys_device: 0, revision: 0 },
];

pub unsafe fn dm_should_disable_stutter(pdev: *const pci_dev) -> bool {
    let mut p = amdgpu_stutter_quirk_list.as_ptr();
    while !p.is_null() && (*p).chip_device != 0 {
        if (*pdev).vendor == (*p).chip_vendor && (*pdev).device == (*p).chip_device &&
           (*pdev).subsystem_vendor == (*p).subsys_vendor && (*pdev).subsystem_device == (*p).subsys_device &&
           (*pdev).revision == (*p).revision { return true; }
        p = p.add(1);
    }
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
