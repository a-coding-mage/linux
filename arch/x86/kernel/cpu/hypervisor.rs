/*
 * Common hypervisor code
 *
 * Copyright (C) 2008, VMware, Inc.
 * Author : Alok N Kataria <akataria@vmware.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or NON INFRINGEMENT.
 */

// Kernel headers and build-time configuration are supplied by the surrounding crate.

#[cfg(CONFIG_XEN_PV)]
static X86_HYPERVISORS_XEN_PV: &hypervisor_x86 = &x86_hyper_xen_pv;
#[cfg(CONFIG_XEN)]
static X86_HYPERVISORS_XEN_HVM: &hypervisor_x86 = &x86_hyper_xen_hvm;
static X86_HYPERVISORS_VMWARE: &hypervisor_x86 = &x86_hyper_vmware;
static X86_HYPERVISORS_MS_HYPERV: &hypervisor_x86 = &x86_hyper_ms_hyperv;
#[cfg(CONFIG_KVM_GUEST)]
static X86_HYPERVISORS_KVM: &hypervisor_x86 = &x86_hyper_kvm;
#[cfg(CONFIG_JAILHOUSE_GUEST)]
static X86_HYPERVISORS_JAILHOUSE: &hypervisor_x86 = &x86_hyper_jailhouse;
#[cfg(CONFIG_ACRN_GUEST)]
static X86_HYPERVISORS_ACRN: &hypervisor_x86 = &x86_hyper_acrn;
#[cfg(CONFIG_BHYVE_GUEST)]
static X86_HYPERVISORS_BHYVE: &hypervisor_x86 = &x86_hyper_bhyve;

static HYPERVISORS: &[&hypervisor_x86] = &[
    #[cfg(CONFIG_XEN_PV)]
    X86_HYPERVISORS_XEN_PV,
    #[cfg(CONFIG_XEN)]
    X86_HYPERVISORS_XEN_HVM,
    X86_HYPERVISORS_VMWARE,
    X86_HYPERVISORS_MS_HYPERV,
    #[cfg(CONFIG_KVM_GUEST)]
    X86_HYPERVISORS_KVM,
    #[cfg(CONFIG_JAILHOUSE_GUEST)]
    X86_HYPERVISORS_JAILHOUSE,
    #[cfg(CONFIG_ACRN_GUEST)]
    X86_HYPERVISORS_ACRN,
    #[cfg(CONFIG_BHYVE_GUEST)]
    X86_HYPERVISORS_BHYVE,
];

static mut x86_hyper_type: x86_hypervisor_type = x86_hypervisor_type::X86_HYPERVISOR_TYPE_NONE;

#[no_mangle]
pub static mut nopv: bool = false;

unsafe fn parse_nopv(_arg: *mut u8) -> i32 {
    nopv = true;
    0
}

// early_param("nopv", parse_nopv);

#[inline]
unsafe fn detect_hypervisor_vendor() -> *const hypervisor_x86 {
    let mut h: *const hypervisor_x86 = core::ptr::null();
    let mut max_pri: u32 = 0;

    for p in HYPERVISORS.iter() {
        if nopv && !(*p).ignore_nopv {
            continue;
        }

        let pri = ((*p).detect)();
        if pri > max_pri {
            max_pri = pri;
            h = *p as *const hypervisor_x86;
        }
    }

    if !h.is_null() {
        pr_info!("Hypervisor detected: {}\n", (*h).name);
    }

    h
}

unsafe fn copy_array(src: *const core::ffi::c_void, target: *mut core::ffi::c_void, size: u32) {
    let n = size as usize / core::mem::size_of::<*const core::ffi::c_void>();
    let from = src as *const *const core::ffi::c_void;
    let to = target as *mut *const core::ffi::c_void;

    for i in 0..n {
        if !(*from.add(i)).is_null() {
            *to.add(i) = *from.add(i);
        }
    }
}

pub unsafe fn init_hypervisor_platform() {
    let h = detect_hypervisor_vendor();

    if h.is_null() {
        return;
    }

    copy_array(
        &(*h).init as *const _ as *const core::ffi::c_void,
        &mut x86_init.hyper as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&(*h).init) as u32,
    );
    copy_array(
        &(*h).runtime as *const _ as *const core::ffi::c_void,
        &mut x86_platform.hyper as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&(*h).runtime) as u32,
    );

    x86_hyper_type = (*h).type_;
    (x86_init.hyper.init_platform)();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
