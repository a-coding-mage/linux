/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ARM specific SMP header, this contains our implementation
 * details.
 */

// Dependencies supplied by the surrounding ARM kernel translation.

/// Return true if we are running on a SMP platform.
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub const fn is_smp() -> bool { false }

#[cfg(all(feature = "CONFIG_SMP", feature = "CONFIG_SMP_ON_UP"))]
#[inline]
pub unsafe fn is_smp() -> bool {
    smp_on_up != 0
}

#[cfg(all(feature = "CONFIG_SMP", not(feature = "CONFIG_SMP_ON_UP")))]
#[inline]
pub const fn is_smp() -> bool { true }

extern "C" {
    static mut smp_on_up: u32;
    static mut __cpu_logical_map: *mut u32;
    static mut mpidr_hash: mpidr_hash;
    fn platform_can_secondary_boot() -> i32;
    fn platform_can_cpu_hotplug() -> i32;
}

#[repr(C)]
pub struct cpuinfo_arm {
    pub cpuid: u32,
    // Remaining fields are supplied by the ARM cpuinfo definition.
}

extern "C" {
    fn per_cpu_cpu_data(cpu: i32) -> *mut cpuinfo_arm;
    fn read_cpuid_part() -> u32;
    fn read_cpuid_ext(reg: u32) -> u32;
}

pub const ARM_CPU_PART_MASK: u32 = 0xfff0_0000;
pub const CPUID_EXT_MMFR3: u32 = 0;

/// Return part id for a given cpu.
#[inline]
pub unsafe fn smp_cpuid_part(cpu: i32) -> u32 {
    let cpu_info = &*per_cpu_cpu_data(cpu);
    if is_smp() { cpu_info.cpuid & ARM_CPU_PART_MASK } else { read_cpuid_part() }
}

// All SMP configurations have the extended CPUID registers.
#[cfg(not(feature = "CONFIG_MMU"))]
#[inline]
pub const fn tlb_ops_need_broadcast() -> i32 { 0 }

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn tlb_ops_need_broadcast() -> i32 {
    if !is_smp() { return 0; }
    (((read_cpuid_ext(CPUID_EXT_MMFR3) >> 12) & 0xf) < 2) as i32
}

#[cfg(any(not(feature = "CONFIG_SMP"), feature = "LINUX_ARM_ARCH_GE_7"))]
#[inline]
pub const fn cache_ops_need_broadcast() -> i32 { 0 }

#[cfg(all(feature = "CONFIG_SMP", not(feature = "LINUX_ARM_ARCH_GE_7")))]
#[inline]
pub unsafe fn cache_ops_need_broadcast() -> i32 {
    if !is_smp() { return 0; }
    (((read_cpuid_ext(CPUID_EXT_MMFR3) >> 12) & 0xf) < 1) as i32
}

#[inline]
pub unsafe fn cpu_logical_map(cpu: usize) -> u32 { *__cpu_logical_map.add(cpu) }

/// Retrieve the logical CPU index corresponding to a given MPIDR[23:0].
#[inline]
pub unsafe fn get_logical_index(mpidr: u32, nr_cpu_ids: i32) -> i32 {
    let mut cpu = 0;
    while cpu < nr_cpu_ids {
        if cpu_logical_map(cpu as usize) == mpidr { return cpu; }
        cpu += 1;
    }
    -22 // -EINVAL
}

/* Assembly code relies on this structure's memory layout (arch/arm/kernel/sleep.S). */
#[repr(C)]
pub struct mpidr_hash {
    pub mask: u32,
    pub shift_aff: [u32; 3],
    pub bits: u32,
}

#[inline]
pub unsafe fn mpidr_hash_size() -> u32 { 1u32 << mpidr_hash.bits }

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
extern "C" { fn platform_can_hotplug_cpu(cpu: u32) -> i32; }

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub const fn platform_can_hotplug_cpu(_cpu: u32) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
