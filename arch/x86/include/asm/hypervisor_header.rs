/*
 * Copyright (C) 2008, VMware, Inc.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or
 * NON INFRINGEMENT.  See the GNU General Public License for more
 * details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 */

/* x86 hypervisor types */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum x86_hypervisor_type {
    X86_HYPER_NATIVE = 0,
    X86_HYPER_VMWARE,
    X86_HYPER_MS_HYPERV,
    X86_HYPER_XEN_PV,
    X86_HYPER_XEN_HVM,
    X86_HYPER_KVM,
    X86_HYPER_JAILHOUSE,
    X86_HYPER_ACRN,
    X86_HYPER_BHYVE,
}

/* CONFIG_HYPERVISOR_GUEST-dependent declarations. */
/* Dependencies: asm/kvm_para.h, asm/x86_init.h, asm/xen/hypervisor.h */

#[repr(C)]
pub struct hypervisor_x86 {
    /* Hypervisor name */
    pub name: *const ::core::ffi::c_char,

    /* Detection routine */
    pub detect: Option<unsafe extern "C" fn() -> u32>,

    /* Hypervisor type */
    pub type_: x86_hypervisor_type,

    /* init time callbacks */
    pub init: x86_hyper_init,

    /* runtime callbacks */
    pub runtime: x86_hyper_runtime,

    /* ignore nopv parameter */
    pub ignore_nopv: bool,
}

extern "C" {
    pub static x86_hyper_vmware: hypervisor_x86;
    pub static x86_hyper_ms_hyperv: hypervisor_x86;
    pub static x86_hyper_xen_pv: hypervisor_x86;
    pub static x86_hyper_kvm: hypervisor_x86;
    pub static x86_hyper_jailhouse: hypervisor_x86;
    pub static x86_hyper_acrn: hypervisor_x86;
    pub static x86_hyper_bhyve: hypervisor_x86;
    pub static mut x86_hyper_xen_hvm: hypervisor_x86;

    pub static mut nopv: bool;
    pub static mut x86_hyper_type: x86_hypervisor_type;
    pub fn init_hypervisor_platform();
}

#[cfg(feature = "CONFIG_HYPERVISOR_GUEST")]
pub unsafe fn hypervisor_is_type(type_: x86_hypervisor_type) -> bool {
    x86_hyper_type == type_
}

/* When CONFIG_HYPERVISOR_GUEST is disabled, the C inline definitions are: */
#[cfg(not(feature = "CONFIG_HYPERVISOR_GUEST"))]
pub unsafe fn init_hypervisor_platform() {}

#[cfg(not(feature = "CONFIG_HYPERVISOR_GUEST"))]
pub fn hypervisor_is_type(type_: x86_hypervisor_type) -> bool {
    type_ == x86_hypervisor_type::X86_HYPER_NATIVE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
