/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency: declarations from <asm/reboot.h> are supplied externally.

pub type CpuEmergencyVirtCb = unsafe extern "C" fn();

// The following declarations are enabled when CONFIG_KVM_X86 is enabled.
#[cfg(feature = "CONFIG_KVM_X86")]
extern "C" {
    pub static mut virt_rebooting: bool;

    pub fn x86_virt_init();

    pub fn x86_virt_get_ref(feat: i32) -> i32;
    pub fn x86_virt_put_ref(feat: i32);

    pub fn x86_virt_emergency_disable_virtualization_cpu() -> i32;

    pub fn x86_virt_register_emergency_callback(callback: *mut CpuEmergencyVirtCb);
    pub fn x86_virt_unregister_emergency_callback(callback: *mut CpuEmergencyVirtCb);
}

// Fallback declarations when CONFIG_KVM_X86 is disabled.
#[cfg(not(feature = "CONFIG_KVM_X86"))]
#[inline(always)]
pub fn x86_virt_init() {}

#[cfg(not(feature = "CONFIG_KVM_X86"))]
#[inline]
pub fn x86_virt_emergency_disable_virtualization_cpu() -> i32 {
    -ENOENT
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
