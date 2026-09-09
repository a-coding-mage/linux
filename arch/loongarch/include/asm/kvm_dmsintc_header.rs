/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2025 Loongson Technology Corporation Limited
 */

// Dependency supplied by the surrounding kernel translation:
// #include <linux/kvm_types.h>

#[repr(C)]
pub struct loongarch_dmsintc {
    pub kvm: *mut kvm,
    pub msg_addr_base: u64,
    pub msg_addr_size: u64,
    pub cpu_mask: u32,
}

#[repr(C)]
pub struct dmsintc_state {
    pub vector_map: [atomic64_t; 4],
}

unsafe extern "C" {
    pub fn kvm_loongarch_register_dmsintc_device() -> i32;
    pub fn dmsintc_inject_irq(vcpu: *mut kvm_vcpu);
    pub fn dmsintc_set_irq(kvm: *mut kvm, addr: u64, data: i32, level: i32) -> i32;
    pub fn dmsintc_deliver_msi_to_vcpu(
        kvm: *mut kvm,
        vcpu: *mut kvm_vcpu,
        vector: u32,
        level: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
