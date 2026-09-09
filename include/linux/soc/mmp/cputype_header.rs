/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <asm/cputype.h> when CONFIG_ARM or CONFIG_ARM64 is
// enabled; the corresponding external symbols are supplied by dependencies.

/*
 *  CPU   Stepping   CPU_ID      CHIP_ID
 *
 * PXA168    S0    0x56158400   0x0000C910
 * PXA168    A0    0x56158400   0x00A0A168
 * PXA910    Y1    0x56158400   0x00F2C920
 * PXA910    A0    0x56158400   0x00F2C910
 * PXA910    A1    0x56158400   0x00A0C910
 * PXA920    Y0    0x56158400   0x00F2C920
 * PXA920    A0    0x56158400   0x00A0C920
 * PXA920    A1    0x56158400   0x00A1C920
 * MMP2      Z0    0x560f5811   0x00F00410
 * MMP2      Z1    0x560f5811   0x00E00410
 * MMP2      A0    0x560f5811   0x00A0A610
 * MMP3      A0    0x562f5842   0x00A02128
 * MMP3      B0    0x562f5842   0x00B02128
 */

extern "C" {
    pub static mut mmp_chip_id: u32;
    pub fn read_cpuid_id() -> u32;
}

#[cfg(CONFIG_MACH_MMP2_DT)]
#[inline]
pub unsafe fn cpu_is_mmp2() -> i32 {
    (((read_cpuid_id() >> 8) & 0xff) == 0x58
        && ((mmp_chip_id & 0xfff) == 0x410 || (mmp_chip_id & 0xfff) == 0x610)) as i32
}

#[cfg(not(CONFIG_MACH_MMP2_DT))]
#[inline]
pub const fn cpu_is_mmp2() -> i32 {
    0
}

#[cfg(CONFIG_MACH_MMP3_DT)]
#[inline]
pub unsafe fn cpu_is_mmp3() -> i32 {
    (((read_cpuid_id() >> 8) & 0xff) == 0x58 && (mmp_chip_id & 0xffff) == 0x2128) as i32
}

#[cfg(CONFIG_MACH_MMP3_DT)]
#[inline]
pub unsafe fn cpu_is_mmp3_a0() -> i32 {
    (cpu_is_mmp3() != 0 && (mmp_chip_id & 0x00ff0000) == 0x00a00000) as i32
}

#[cfg(CONFIG_MACH_MMP3_DT)]
#[inline]
pub unsafe fn cpu_is_mmp3_b0() -> i32 {
    (cpu_is_mmp3() != 0 && (mmp_chip_id & 0x00ff0000) == 0x00b00000) as i32
}

#[cfg(not(CONFIG_MACH_MMP3_DT))]
#[inline]
pub const fn cpu_is_mmp3() -> i32 {
    0
}

#[cfg(not(CONFIG_MACH_MMP3_DT))]
#[inline]
pub const fn cpu_is_mmp3_a0() -> i32 {
    0
}

#[cfg(not(CONFIG_MACH_MMP3_DT))]
#[inline]
pub const fn cpu_is_mmp3_b0() -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
