/* SPDX-License-Identifier: GPL-2.0 */

/* MPUIR layout */
pub const MPUIR_nU: u32 = 1;
pub const MPUIR_DREGION: u32 = 8;
pub const MPUIR_IREGION: u32 = 16;
pub const MPUIR_DREGION_SZMASK: u32 = 0xFF << MPUIR_DREGION;
pub const MPUIR_IREGION_SZMASK: u32 = 0xFF << MPUIR_IREGION;

/* ID_MMFR0 data relevant to MPU */
pub const MMFR0_PMSA: u32 = 0xF << 4;
pub const MMFR0_PMSAv7: u32 = 3 << 4;
pub const MMFR0_PMSAv8: u32 = 4 << 4;

/* MPU D/I Size Register fields */
pub const PMSAv7_RSR_SZ: u32 = 1;
pub const PMSAv7_RSR_EN: u32 = 0;
pub const PMSAv7_RSR_SD: u32 = 8;

/* Number of subregions (SD) */
pub const PMSAv7_NR_SUBREGS: u32 = 8;
pub const PMSAv7_MIN_SUBREG_SIZE: u32 = 256;

/* The D/I RSR value for an enabled region spanning the whole of memory */
pub const PMSAv7_RSR_ALL_MEM: u32 = 63;

/* Individual bits in the DR/IR ACR */
pub const PMSAv7_ACR_XN: u32 = 1 << 12;
pub const PMSAv7_ACR_SHARED: u32 = 1 << 2;

/* C, B and TEX[2:0] bits only have semantic meanings when grouped */
pub const PMSAv7_RGN_CACHEABLE: u32 = 0xB;
pub const PMSAv7_RGN_SHARED_CACHEABLE: u32 = PMSAv7_RGN_CACHEABLE | PMSAv7_ACR_SHARED;
pub const PMSAv7_RGN_STRONGLY_ORDERED: u32 = 0;

/* Main region should only be shared for SMP */
#[cfg(feature = "CONFIG_SMP")]
pub const PMSAv7_RGN_NORMAL: u32 = PMSAv7_RGN_CACHEABLE | PMSAv7_ACR_SHARED;
#[cfg(not(feature = "CONFIG_SMP"))]
pub const PMSAv7_RGN_NORMAL: u32 = PMSAv7_RGN_CACHEABLE;

/* Access permission bits of ACR (only define those that we use) */
pub const PMSAv7_AP_PL1RO_PL0NA: u32 = 0x5 << 8;
pub const PMSAv7_AP_PL1RW_PL0RW: u32 = 0x3 << 8;
pub const PMSAv7_AP_PL1RW_PL0R0: u32 = 0x2 << 8;
pub const PMSAv7_AP_PL1RW_PL0NA: u32 = 0x1 << 8;

pub const PMSAv8_BAR_XN: u32 = 1;

pub const PMSAv8_LAR_EN: u32 = 1;
pub const PMSAv8_LAR_IDX: const fn(u32) -> u32 = |n| (n & 0x7) << 1;

pub const PMSAv8_AP_PL1RW_PL0NA: u32 = 0 << 1;
pub const PMSAv8_AP_PL1RW_PL0RW: u32 = 1 << 1;
pub const PMSAv8_AP_PL1RO_PL0RO: u32 = 3 << 1;

#[cfg(feature = "CONFIG_SMP")]
pub const PMSAv8_RGN_SHARED: u32 = 3 << 3; // inner sharable
#[cfg(not(feature = "CONFIG_SMP"))]
pub const PMSAv8_RGN_SHARED: u32 = 0 << 3;

pub const PMSAv8_RGN_DEVICE_nGnRnE: u32 = 0;
pub const PMSAv8_RGN_NORMAL: u32 = 1;

pub const PMSAv8_MAIR: const fn(u32, u32) -> u32 = |attr, mt| attr << (mt * 8);

#[cfg(feature = "CONFIG_CPU_V7M")]
pub const PMSAv8_MINALIGN: u32 = 32;
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
pub const PMSAv8_MINALIGN: u32 = 64;

/* For minimal static MPU region configurations */
pub const PMSAv7_PROBE_REGION: u32 = 0;
pub const PMSAv7_BG_REGION: u32 = 1;
pub const PMSAv7_RAM_REGION: u32 = 2;
pub const PMSAv7_ROM_REGION: u32 = 3;

/* Fixed for PMSAv8 only */
pub const PMSAv8_XIP_REGION: u32 = 0;
pub const PMSAv8_KERNEL_REGION: u32 = 1;

/* Maximum number of regions Linux is interested in */
pub const MPU_MAX_REGIONS: usize = 16;

pub const PMSAv7_DATA_SIDE: u32 = 0;
pub const PMSAv7_INSTR_SIDE: u32 = 1;

#[repr(C)]
pub union mpu_rgn__bindgen_ty_1 {
    pub drbar: u32,   /* PMSAv7 */
    pub prbar: u32,   /* PMSAv8 */
}

#[repr(C)]
pub union mpu_rgn__bindgen_ty_2 {
    pub drsr: u32,   /* PMSAv7 */
    pub prlar: u32,  /* PMSAv8 */
}

#[repr(C)]
pub union mpu_rgn__bindgen_ty_3 {
    pub dracr: u32,  /* PMSAv7 */
    pub unused: u32, /* not used in PMSAv8 */
}

#[repr(C)]
pub struct mpu_rgn {
    /* Assume same attributes for d/i-side */
    pub _bindgen_1: mpu_rgn__bindgen_ty_1,
    pub _bindgen_2: mpu_rgn__bindgen_ty_2,
    pub _bindgen_3: mpu_rgn__bindgen_ty_3,
}

#[repr(C)]
pub struct mpu_rgn_info {
    pub used: u32,
    pub rgns: [mpu_rgn; MPU_MAX_REGIONS],
}

extern "C" {
    pub static mut mpu_rgn_info: mpu_rgn_info;
}

#[cfg(feature = "CONFIG_ARM_MPU")]
extern "C" {
    pub fn pmsav7_adjust_lowmem_bounds();
    pub fn pmsav8_adjust_lowmem_bounds();
    pub fn pmsav7_setup();
    pub fn pmsav8_setup();
}

#[cfg(not(feature = "CONFIG_ARM_MPU"))]
#[inline]
pub fn pmsav7_adjust_lowmem_bounds() {}
#[cfg(not(feature = "CONFIG_ARM_MPU"))]
#[inline]
pub fn pmsav8_adjust_lowmem_bounds() {}
#[cfg(not(feature = "CONFIG_ARM_MPU"))]
#[inline]
pub fn pmsav7_setup() {}
#[cfg(not(feature = "CONFIG_ARM_MPU"))]
#[inline]
pub fn pmsav8_setup() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
