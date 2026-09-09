/* SPDX-License-Identifier: GPL-2.0 */

pub const CPUID_ID: u32 = 0;
pub const CPUID_CACHETYPE: u32 = 1;
pub const CPUID_TCM: u32 = 2;
pub const CPUID_TLBTYPE: u32 = 3;
pub const CPUID_MPUIR: u32 = 4;
pub const CPUID_MPIDR: u32 = 5;
pub const CPUID_REVIDR: u32 = 6;

#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_PFR0: u32 = 0x40;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_PFR1: u32 = 0x44;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_DFR0: u32 = 0x48;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_AFR0: u32 = 0x4c;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_MMFR0: u32 = 0x50;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_MMFR1: u32 = 0x54;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_MMFR2: u32 = 0x58;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_MMFR3: u32 = 0x5c;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_ISAR0: u32 = 0x60;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_ISAR1: u32 = 0x64;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_ISAR2: u32 = 0x68;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_ISAR3: u32 = 0x6c;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_ISAR4: u32 = 0x70;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_ISAR5: u32 = 0x74;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_ISAR6: u32 = 0x7c;
#[cfg(feature = "CONFIG_CPU_V7M")]
pub const CPUID_EXT_PFR2: u32 = 0x90;

// On non-V7M builds these constants are CP15 operand strings used by inline assembly.
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_PFR0: &str = "c1, 0";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_PFR1: &str = "c1, 1";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_DFR0: &str = "c1, 2";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_AFR0: &str = "c1, 3";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_MMFR0: &str = "c1, 4";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_MMFR1: &str = "c1, 5";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_MMFR2: &str = "c1, 6";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_MMFR3: &str = "c1, 7";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_ISAR0: &str = "c2, 0";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_ISAR1: &str = "c2, 1";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_ISAR2: &str = "c2, 2";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_ISAR3: &str = "c2, 3";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_ISAR4: &str = "c2, 4";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_ISAR5: &str = "c2, 5";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_ISAR6: &str = "c2, 7";
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const CPUID_EXT_PFR2: &str = "c3, 4";

pub const MPIDR_SMP_BITMASK: u32 = 0x3 << 30;
pub const MPIDR_SMP_VALUE: u32 = 0x2 << 30;
pub const MPIDR_MT_BITMASK: u32 = 0x1 << 24;
pub const MPIDR_HWID_BITMASK: u32 = 0xFFFFFF;
pub const MPIDR_INVALID: u32 = !MPIDR_HWID_BITMASK;
pub const MPIDR_LEVEL_BITS: u32 = 8;
pub const MPIDR_LEVEL_MASK: u32 = (1 << MPIDR_LEVEL_BITS) - 1;
#[inline]
pub const fn MPIDR_LEVEL_SHIFT(level: u32) -> u32 { MPIDR_LEVEL_BITS * level }
#[inline]
pub const fn MPIDR_AFFINITY_LEVEL(mpidr: u32, level: u32) -> u32 {
    (mpidr >> (MPIDR_LEVEL_BITS * level)) & MPIDR_LEVEL_MASK
}

pub const ARM_CPU_IMP_ARM: u32 = 0x41;
pub const ARM_CPU_IMP_BRCM: u32 = 0x42;
pub const ARM_CPU_IMP_DEC: u32 = 0x44;
pub const ARM_CPU_IMP_INTEL: u32 = 0x69;
pub const ARM_CPU_PART_ARM1136: u32 = 0x4100b360;
pub const ARM_CPU_PART_ARM1156: u32 = 0x4100b560;
pub const ARM_CPU_PART_ARM1176: u32 = 0x4100b760;
pub const ARM_CPU_PART_ARM11MPCORE: u32 = 0x4100b020;
pub const ARM_CPU_PART_CORTEX_A8: u32 = 0x4100c080;
pub const ARM_CPU_PART_CORTEX_A9: u32 = 0x4100c090;
pub const ARM_CPU_PART_CORTEX_A5: u32 = 0x4100c050;
pub const ARM_CPU_PART_CORTEX_A7: u32 = 0x4100c070;
pub const ARM_CPU_PART_CORTEX_A12: u32 = 0x4100c0d0;
pub const ARM_CPU_PART_CORTEX_A17: u32 = 0x4100c0e0;
pub const ARM_CPU_PART_CORTEX_A15: u32 = 0x4100c0f0;
pub const ARM_CPU_PART_CORTEX_A53: u32 = 0x4100d030;
pub const ARM_CPU_PART_CORTEX_A57: u32 = 0x4100d070;
pub const ARM_CPU_PART_CORTEX_A72: u32 = 0x4100d080;
pub const ARM_CPU_PART_CORTEX_A73: u32 = 0x4100d090;
pub const ARM_CPU_PART_CORTEX_A75: u32 = 0x4100d0a0;
pub const ARM_CPU_PART_MASK: u32 = 0xff00fff0;
pub const ARM_CPU_PART_BRAHMA_B15: u32 = 0x420000f0;
pub const ARM_CPU_PART_BRAHMA_B53: u32 = 0x42001000;
pub const ARM_CPU_PART_SA1100: u32 = 0x4400a110;
pub const ARM_CPU_PART_SA1110: u32 = 0x6900b110;
pub const ARM_CPU_REV_SA1110_A0: u32 = 0;
pub const ARM_CPU_REV_SA1110_B0: u32 = 4;
pub const ARM_CPU_REV_SA1110_B1: u32 = 5;
pub const ARM_CPU_REV_SA1110_B2: u32 = 6;
pub const ARM_CPU_REV_SA1110_B4: u32 = 8;
pub const ARM_CPU_XSCALE_ARCH_MASK: u32 = 0xe000;
pub const ARM_CPU_XSCALE_ARCH_V1: u32 = 0x2000;
pub const ARM_CPU_XSCALE_ARCH_V2: u32 = 0x4000;
pub const ARM_CPU_XSCALE_ARCH_V3: u32 = 0x6000;
pub const ARM_CPU_PART_SCORPION: u32 = 0x510002d0;

extern "C" {
    pub static mut processor_id: u32;
    pub fn lookup_processor(midr: u32) -> *mut proc_info_list;
}

#[repr(C)]
pub struct proc_info_list {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_CPU_CP15")]
#[inline]
pub unsafe fn read_cpuid(reg: u32) -> u32 {
    let mut val: u32;
    core::arch::asm!("mrc p15, 0, {0}, c0, c0, {1}", out(reg) val, const reg);
    val
}

#[cfg(feature = "CONFIG_CPU_CP15")]
#[inline]
pub unsafe fn read_cpuid_ext(_ext_reg: &str) -> u32 {
    // The C implementation interpolates the CP15 operand string into assembly.
    todo!("CP15 extended register operand")
}

#[cfg(all(not(feature = "CONFIG_CPU_CP15"), feature = "CONFIG_CPU_V7M"))]
#[inline]
pub unsafe fn read_cpuid(_reg: u32) -> u32 { 0 }

#[cfg(all(not(feature = "CONFIG_CPU_CP15"), feature = "CONFIG_CPU_V7M"))]
#[inline]
pub unsafe fn read_cpuid_ext(offset: u32) -> u32 {
    extern "C" { fn readl(addr: usize) -> u32; }
    extern "C" { static BASEADDR_V7M_SCB: usize; }
    readl(BASEADDR_V7M_SCB + offset as usize)
}

#[cfg(all(not(feature = "CONFIG_CPU_CP15"), not(feature = "CONFIG_CPU_V7M")))]
#[inline]
pub unsafe fn read_cpuid(_reg: u32) -> u32 { 0 }
#[cfg(all(not(feature = "CONFIG_CPU_CP15"), not(feature = "CONFIG_CPU_V7M")))]
#[inline]
pub unsafe fn read_cpuid_ext(reg: u32) -> u32 { read_cpuid(reg) }

#[cfg(feature = "CONFIG_CPU_CP15")]
#[inline]
pub unsafe fn read_cpuid_id() -> u32 { read_cpuid(CPUID_ID) }
#[cfg(feature = "CONFIG_CPU_CP15")]
#[inline]
pub unsafe fn read_cpuid_cachetype() -> u32 { read_cpuid(CPUID_CACHETYPE) }
#[cfg(feature = "CONFIG_CPU_CP15")]
#[inline]
pub unsafe fn read_cpuid_mputype() -> u32 { read_cpuid(CPUID_MPUIR) }

#[cfg(all(not(feature = "CONFIG_CPU_CP15"), feature = "CONFIG_CPU_V7M"))]
#[inline]
pub unsafe fn read_cpuid_id() -> u32 { read_cpuid_ext(0xD00) }
#[cfg(all(not(feature = "CONFIG_CPU_CP15"), feature = "CONFIG_CPU_V7M"))]
#[inline]
pub unsafe fn read_cpuid_cachetype() -> u32 { read_cpuid_ext(0xD10) }
#[cfg(all(not(feature = "CONFIG_CPU_CP15"), feature = "CONFIG_CPU_V7M"))]
#[inline]
pub unsafe fn read_cpuid_mputype() -> u32 { read_cpuid_ext(0xD90) }

#[cfg(all(not(feature = "CONFIG_CPU_CP15"), not(feature = "CONFIG_CPU_V7M")))]
#[inline]
pub unsafe fn read_cpuid_id() -> u32 { processor_id }

#[inline]
pub unsafe fn read_cpuid_implementor() -> u32 { (read_cpuid_id() & 0xFF000000) >> 24 }
#[inline]
pub unsafe fn read_cpuid_revision() -> u32 { read_cpuid_id() & 0x0000000f }
#[inline]
pub unsafe fn read_cpuid_part() -> u32 { read_cpuid_id() & ARM_CPU_PART_MASK }
#[inline]
pub unsafe fn read_cpuid_part_number() -> u32 { read_cpuid_id() & 0xFFF0 }
#[inline]
pub unsafe fn xscale_cpu_arch_version() -> u32 { read_cpuid_id() & ARM_CPU_XSCALE_ARCH_MASK }
#[inline]
pub unsafe fn read_cpuid_tcmstatus() -> u32 { read_cpuid(CPUID_TCM) }
#[inline]
pub unsafe fn read_cpuid_mpidr() -> u32 { read_cpuid(CPUID_MPIDR) }

#[inline]
pub unsafe fn cpu_is_sa1100() -> bool { read_cpuid_part() == ARM_CPU_PART_SA1100 }
#[inline]
pub unsafe fn cpu_is_sa1110() -> bool { read_cpuid_part() == ARM_CPU_PART_SA1110 }

#[cfg(not(feature = "CONFIG_CPU_XSC3"))]
#[inline]
pub fn cpu_is_xsc3() -> i32 { 0 }
#[cfg(feature = "CONFIG_CPU_XSC3")]
#[inline]
pub unsafe fn cpu_is_xsc3() -> i32 {
    let id = read_cpuid_id() & 0xffffe000;
    if id == 0x69056000 || id == 0x56056000 { 1 } else { 0 }
}

#[cfg(all(not(feature = "CONFIG_CPU_XSCALE"), not(feature = "CONFIG_CPU_XSC3"), not(feature = "CONFIG_CPU_MOHAWK")))]
#[inline]
pub fn cpu_is_xscale_family() -> i32 { 0 }
#[cfg(any(feature = "CONFIG_CPU_XSCALE", feature = "CONFIG_CPU_XSC3", feature = "CONFIG_CPU_MOHAWK"))]
#[inline]
pub unsafe fn cpu_is_xscale_family() -> i32 {
    let id = read_cpuid_id() & 0xffffe000;
    match id {
        0x69052000 | 0x69054000 | 0x69056000 | 0x56056000 | 0x56158000 => 1,
        _ => 0,
    }
}

#[cfg(any(feature = "CONFIG_CPU_PJ4", feature = "CONFIG_CPU_PJ4B"))]
#[inline]
pub unsafe fn cpu_is_pj4() -> i32 {
    if read_cpuid_id() & 0xff0fff00 == 0x560f5800 { 1 } else { 0 }
}
#[cfg(not(any(feature = "CONFIG_CPU_PJ4", feature = "CONFIG_CPU_PJ4B")))]
#[inline]
pub fn cpu_is_pj4() -> i32 { 0 }

#[inline]
pub fn cpuid_feature_extract_field(features: u32, field: i32) -> i32 {
    let mut feature = ((features >> field) & 15) as i32;
    if feature > 7 { feature -= 16; }
    feature
}

#[inline]
pub unsafe fn cpuid_feature_extract(reg: u32, field: i32) -> i32 {
    cpuid_feature_extract_field(read_cpuid_ext(reg), field)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
