/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <asm/xen/hypervisor.h>

extern "C" {
    pub fn kvm_init_hyp_services();
    pub fn kvm_arm_hyp_service_available(func_id: u32) -> bool;
}

#[inline]
pub unsafe fn kvm_arch_init_hyp_services() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
