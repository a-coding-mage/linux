// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Extracted from cputable.c
 *
 *  Copyright (C) 2001 Ben. Herrenschmidt (benh@kernel.crashing.org)
 *
 *  Modifications for ppc64:
 *      Copyright (C) 2003 Dave Engebretsen <engebret@us.ibm.com>
 *  Copyright (C) 2005 Stephen Rothwell, IBM Corporation
 */

// Linux kernel dependencies supplied by other translation units.

#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub static mut powerpc_firmware_features: core::ffi::c_ulong = 0;

// EXPORT_SYMBOL_GPL(powerpc_firmware_features);

#[cfg(any(feature = "CONFIG_PPC_PSERIES", feature = "CONFIG_KVM_GUEST"))]
#[no_mangle]
pub static mut kvm_guest: crate::StaticKeyFalse = unsafe { core::mem::zeroed() };

// EXPORT_SYMBOL_GPL(kvm_guest);

#[cfg(any(feature = "CONFIG_PPC_PSERIES", feature = "CONFIG_KVM_GUEST"))]
#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[cfg(any(feature = "CONFIG_PPC_PSERIES", feature = "CONFIG_KVM_GUEST"))]
extern "C" {
    fn of_find_node_by_path(path: *const core::ffi::c_char) -> *mut DeviceNode;
    fn of_device_is_compatible(
        node: *const DeviceNode,
        compatible: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn of_node_put(node: *mut DeviceNode);
    fn static_branch_enable(key: *mut crate::StaticKeyFalse);
}

#[cfg(any(feature = "CONFIG_PPC_PSERIES", feature = "CONFIG_KVM_GUEST"))]
#[no_mangle]
pub unsafe extern "C" fn check_kvm_guest() -> core::ffi::c_int {
    let hyper_node: *mut DeviceNode;

    hyper_node = of_find_node_by_path(b"/hypervisor\0".as_ptr() as *const core::ffi::c_char);
    if hyper_node.is_null() {
        return 0;
    }

    if of_device_is_compatible(
        hyper_node,
        b"linux,kvm\0".as_ptr() as *const core::ffi::c_char,
    ) != 0
    {
        static_branch_enable(&raw mut kvm_guest);
    }

    of_node_put(hyper_node);
    0
}

// core_initcall(check_kvm_guest); // before kvm_guest_init()

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
