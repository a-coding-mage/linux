/*
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

/*
 * Device bus NAND private data
 *
 * The C header's externally supplied `struct mtd_partition`, `u32`, and
 * `u8` types are referenced here as future dependencies.
 */
#[repr(C)]
pub struct orion_nand_data {
    pub parts: *mut mtd_partition,
    pub nr_parts: u32,
    pub ale: u8,        /* address line number connected to ALE */
    pub cle: u8,        /* address line number connected to CLE */
    pub width: u8,      /* buswidth */
    pub chip_delay: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
