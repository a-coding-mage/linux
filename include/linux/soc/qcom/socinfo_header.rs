/* SPDX-License-Identifier: GPL-2.0 */

// The following declarations correspond to the Linux kernel types and macros
// supplied by the surrounding translation unit.

/*
 * SMEM item id, used to acquire handles to respective
 * SMEM region.
 */
pub const SMEM_HW_SW_BUILD_ID: u32 = 137;

pub const SMEM_SOCINFO_BUILD_ID_LENGTH: usize = 32;
pub const SMEM_SOCINFO_CHIP_ID_LENGTH: usize = 32;

/*
 * SoC version type with major number in the upper 16 bits and minor
 * number in the lower 16 bits.
 */
#[inline]
pub const fn SOCINFO_MAJOR(ver: u32) -> u32 {
    (ver >> 16) & 0xffff
}

#[inline]
pub const fn SOCINFO_MINOR(ver: u32) -> u32 {
    ver & 0xffff
}

#[inline]
pub const fn SOCINFO_VERSION(maj: u32, min: u32) -> u32 {
    ((maj & 0xffff) << 16) | (min & 0xffff)
}

/* Socinfo SMEM item structure */
#[repr(C)]
pub struct socinfo {
    pub fmt: __le32,
    pub id: __le32,
    pub ver: __le32,
    pub build_id: [i8; SMEM_SOCINFO_BUILD_ID_LENGTH],
    /* Version 2 */
    pub raw_id: __le32,
    pub raw_ver: __le32,
    /* Version 3 */
    pub hw_plat: __le32,
    /* Version 4 */
    pub plat_ver: __le32,
    /* Version 5 */
    pub accessory_chip: __le32,
    /* Version 6 */
    pub hw_plat_subtype: __le32,
    /* Version 7 */
    pub pmic_model: __le32,
    pub pmic_die_rev: __le32,
    /* Version 8 */
    pub pmic_model_1: __le32,
    pub pmic_die_rev_1: __le32,
    pub pmic_model_2: __le32,
    pub pmic_die_rev_2: __le32,
    /* Version 9 */
    pub foundry_id: __le32,
    /* Version 10 */
    pub serial_num: __le32,
    /* Version 11 */
    pub num_pmics: __le32,
    pub pmic_array_offset: __le32,
    /* Version 12 */
    pub chip_family: __le32,
    pub raw_device_family: __le32,
    pub raw_device_num: __le32,
    /* Version 13 */
    pub nproduct_id: __le32,
    pub chip_id: [i8; SMEM_SOCINFO_CHIP_ID_LENGTH],
    /* Version 14 */
    pub num_clusters: __le32,
    pub ncluster_array_offset: __le32,
    pub num_subset_parts: __le32,
    pub nsubset_parts_array_offset: __le32,
    /* Version 15 */
    pub nmodem_supported: __le32,
    /* Version 16 */
    pub feature_code: __le32,
    pub pcode: __le32,
    pub npartnamemap_offset: __le32,
    pub nnum_partname_mapping: __le32,
    /* Version 17 */
    pub oem_variant: __le32,
    /* Version 18 */
    pub num_kvps: __le32,
    pub kvps_offset: __le32,
    /* Version 19 */
    pub num_func_clusters: __le32,
    pub boot_cluster: __le32,
    pub boot_core: __le32,
    /* Version 20 */
    pub raw_package_type: __le32,
    /* Version 21, 22, 23 */
    pub reserve1: [__le32; 4],
}

/* Internal feature codes */
#[repr(i32)]
pub enum qcom_socinfo_feature_code {
    /* External feature codes */
    SOCINFO_FC_UNKNOWN = 0x0,
    SOCINFO_FC_AA,
    SOCINFO_FC_AB,
    SOCINFO_FC_AC,
    SOCINFO_FC_AD,
    SOCINFO_FC_AE,
    SOCINFO_FC_AF,
    SOCINFO_FC_AG,
    SOCINFO_FC_AH,
}

/* Internal feature codes */
/* Valid values: 0 <= n <= 0xf */
#[inline]
pub const fn SOCINFO_FC_Yn(n: u32) -> u32 {
    0xf1 + n
}

pub const SOCINFO_FC_INT_MAX: u32 = SOCINFO_FC_Yn(0xf);

/* Product codes */
pub const SOCINFO_PC_UNKNOWN: u32 = 0;

#[inline]
pub const fn SOCINFO_PCn(n: u32) -> u32 {
    n + 1
}

pub const SOCINFO_PC_RESERVE: u32 = (1u32 << 31) - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
