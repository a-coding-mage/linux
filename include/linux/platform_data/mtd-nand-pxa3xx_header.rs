/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/mtd/mtd.h and linux/mtd/partitions.h

/*
 * Current pxa3xx_nand controller has two chip select which both be workable but
 * historically all platforms remaining on platform data used only one. Switch
 * to device tree if you need more.
 */
#[repr(C)]
pub struct pxa3xx_nand_platform_data {
    /* Keep OBM/bootloader NFC timing configuration */
    pub keep_config: bool,
    /* Use a flash-based bad block table */
    pub flash_bbt: bool,
    /* Requested ECC strength and ECC step size */
    pub ecc_strength: i32,
    pub ecc_step_size: i32,
    /* Partitions */
    pub parts: *const mtd_partition,
    pub nr_parts: u32,
}

// Supplied by linux/mtd/partitions.h.

unsafe extern "C" {
    pub fn pxa3xx_set_nand_info(info: *mut pxa3xx_nand_platform_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
