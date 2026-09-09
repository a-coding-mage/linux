/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: linux/device.h, linux/cpu.h, linux/topology.h,
// linux/nodemask.h, linux/percpu.h, asm/ibt.h, and asm/cpuid/leaf_types.h.

#[cfg(not(feature = "CONFIG_SMP"))]
// C macro: cpu_physical_id(cpu) expands to boot_cpu_physical_apicid.
pub const fn cpu_physical_id(_cpu: usize) -> u32 {
    boot_cpu_physical_apicid
}

#[cfg(not(feature = "CONFIG_SMP"))]
unsafe extern "C" {
    static boot_cpu_physical_apicid: u32;
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe extern "C" {
    pub fn soft_restart_cpu();
}

unsafe extern "C" {
    pub fn ap_init_aperfmperf();

    pub fn mwait_usable(c: *const cpuinfo_x86) -> ::core::ffi::c_int;

    pub fn x86_family(sig: u32) -> u32;
    pub fn x86_model(sig: u32) -> u32;
    pub fn x86_stepping(sig: u32) -> u32;

    pub fn cpuid_family(l: *const leaf_0x1_0) -> u32;
    pub fn cpuid_model(l: *const leaf_0x1_0) -> u32;
}

#[cfg(feature = "CONFIG_X86_BUS_LOCK_DETECT")]
unsafe extern "C" {
    pub fn sld_setup(c: *mut cpuinfo_x86);
    pub fn handle_user_split_lock(regs: *mut pt_regs, error_code: isize) -> bool;
    pub fn handle_guest_split_lock(ip: usize) -> bool;
    pub fn handle_bus_lock(regs: *mut pt_regs);
    pub fn split_lock_init();
    pub fn bus_lock_init();
}

#[cfg(not(feature = "CONFIG_X86_BUS_LOCK_DETECT"))]
#[inline]
pub unsafe fn sld_setup(_c: *mut cpuinfo_x86) {}

#[cfg(not(feature = "CONFIG_X86_BUS_LOCK_DETECT"))]
#[inline]
pub unsafe fn handle_user_split_lock(_regs: *mut pt_regs, _error_code: isize) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_X86_BUS_LOCK_DETECT"))]
#[inline]
pub unsafe fn handle_guest_split_lock(_ip: usize) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_X86_BUS_LOCK_DETECT"))]
#[inline]
pub unsafe fn handle_bus_lock(_regs: *mut pt_regs) {}

#[cfg(not(feature = "CONFIG_X86_BUS_LOCK_DETECT"))]
#[inline]
pub unsafe fn split_lock_init() {}

#[cfg(not(feature = "CONFIG_X86_BUS_LOCK_DETECT"))]
#[inline]
pub unsafe fn bus_lock_init() {}

#[cfg(feature = "CONFIG_IA32_FEAT_CTL")]
unsafe extern "C" {
    pub fn init_ia32_feat_ctl(c: *mut cpuinfo_x86);
}

#[cfg(not(feature = "CONFIG_IA32_FEAT_CTL"))]
#[inline]
pub unsafe fn init_ia32_feat_ctl(_c: *mut cpuinfo_x86) {}

unsafe extern "C" {
    // C attributes: __noendbr.
    pub fn cet_disable();
}

#[repr(C)]
pub struct cpu_signature {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn intel_collect_cpu_info(sig: *mut cpu_signature);
    pub fn x86_read_arch_cap_msr() -> u64;
    pub fn intel_find_matching_signature(mc: *mut ::core::ffi::c_void,
                                         sig: *mut cpu_signature) -> bool;
    pub fn intel_microcode_sanity_check(mc: *mut ::core::ffi::c_void,
                                        print_err: bool,
                                        hdr_type: ::core::ffi::c_int) -> ::core::ffi::c_int;

    pub static mut cpus_stop_mask: cpumask;
}

// External types supplied by the included Linux and x86 headers.
pub type cpuinfo_x86 = ::core::ffi::c_void;
pub type leaf_0x1_0 = ::core::ffi::c_void;
pub type pt_regs = ::core::ffi::c_void;
pub type cpumask = ::core::ffi::c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
