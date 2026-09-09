/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Broadcom BCM963xx SoC board nvram data structure.
 *
 * The nvram structure varies in size depending on the SoC board version. Use
 * the appropriate minimum BCM963XX_NVRAM_*_SIZE define for the information
 * you need instead of core::mem::size_of::<struct bcm963xx_nvram>() as this
 * may change.
 *
 * C header dependencies retained as external Rust dependencies:
 * linux/crc32.h, linux/if_ether.h, linux/sizes.h, linux/types.h
 */

pub const BCM963XX_NVRAM_V4_SIZE: usize = 300;
pub const BCM963XX_NVRAM_V5_SIZE: usize = 1 * SZ_1K;

pub const BCM963XX_DEFAULT_PSI_SIZE: u32 = 64;

pub const SZ_1K: usize = 1024;
pub const ETH_ALEN: usize = 6;

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum bcm963xx_nvram_nand_part {
    BCM963XX_NVRAM_NAND_PART_BOOT = 0,
    BCM963XX_NVRAM_NAND_PART_ROOTFS_1,
    BCM963XX_NVRAM_NAND_PART_ROOTFS_2,
    BCM963XX_NVRAM_NAND_PART_DATA,
    BCM963XX_NVRAM_NAND_PART_BBT,
}

pub const __BCM963XX_NVRAM_NAND_NR_PARTS: usize = 5;

#[repr(C)]
pub struct bcm963xx_nvram {
    pub version: u32,
    pub bootline: [core::ffi::c_char; 256],
    pub name: [core::ffi::c_char; 16],
    pub main_tp_number: u32,
    pub psi_size: u32,
    pub mac_addr_count: u32,
    pub mac_addr_base: [u8; ETH_ALEN],
    pub __reserved1: [u8; 2],
    pub checksum_v4: u32,
    pub __reserved2: [u8; 292],
    pub nand_part_offset: [u32; __BCM963XX_NVRAM_NAND_NR_PARTS],
    pub nand_part_size: [u32; __BCM963XX_NVRAM_NAND_NR_PARTS],
    pub __reserved3: [u8; 388],
    pub checksum_v5: u32,
}

#[inline]
pub unsafe fn bcm963xx_nvram_nand_part_offset(
    nvram: *const bcm963xx_nvram,
    part: bcm963xx_nvram_nand_part,
) -> u64 {
    (*nvram).nand_part_offset[part as usize] as u64 * SZ_1K as u64
}

#[macro_export]
macro_rules! BCM963XX_NVRAM_NAND_PART_OFFSET {
    ($nvram:expr, $part:ident) => {
        $crate::bcm963xx_nvram_nand_part_offset(
            $nvram,
            $crate::bcm963xx_nvram_nand_part::BCM963XX_NVRAM_NAND_PART_$part,
        )
    };
}

#[inline]
pub unsafe fn bcm963xx_nvram_nand_part_size(
    nvram: *const bcm963xx_nvram,
    part: bcm963xx_nvram_nand_part,
) -> u64 {
    (*nvram).nand_part_size[part as usize] as u64 * SZ_1K as u64
}

#[macro_export]
macro_rules! BCM963XX_NVRAM_NAND_PART_SIZE {
    ($nvram:expr, $part:ident) => {
        $crate::bcm963xx_nvram_nand_part_size(
            $nvram,
            $crate::bcm963xx_nvram_nand_part::BCM963XX_NVRAM_NAND_PART_$part,
        )
    };
}

/*
 * bcm963xx_nvram_checksum - Verify nvram checksum
 *
 * @nvram: pointer to full size nvram data structure
 * @expected_out: optional pointer to store expected checksum value
 * @actual_out: optional pointer to store actual checksum value
 *
 * Return: 0 if the checksum is valid, otherwise -EINVAL
 */
extern "C" {
    pub fn crc32_le(crc: u32, p: *const core::ffi::c_void, len: usize) -> u32;
}

pub const EINVAL: i32 = 22;

#[inline]
pub unsafe fn bcm963xx_nvram_checksum(
    nvram: *const bcm963xx_nvram,
    expected_out: *mut u32,
    actual_out: *mut u32,
) -> i32 {
    let zero: u32 = 0;
    let expected: u32;
    let len: usize;

    if (*nvram).version <= 4 {
        expected = (*nvram).checksum_v4;
        len = BCM963XX_NVRAM_V4_SIZE;
    } else {
        expected = (*nvram).checksum_v5;
        len = BCM963XX_NVRAM_V5_SIZE;
    }

    /* Calculate the CRC32 of the nvram with the checksum field set to 0. */
    let mut actual = crc32_le(!0, nvram.cast(), len - core::mem::size_of::<u32>());
    actual = crc32_le(actual, (&zero as *const u32).cast(), core::mem::size_of::<u32>());

    if !expected_out.is_null() {
        *expected_out = expected;
    }

    if !actual_out.is_null() {
        *actual_out = actual;
    }

    if expected == actual { 0 } else { -EINVAL }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
