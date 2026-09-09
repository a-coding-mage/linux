/* SPDX-License-Identifier: GPL-2.0 */

// Original header guard: __BCM63XX_FLASH_H

#[repr(C)]
pub enum Bcm63xxFlashType {
    BCM63XX_FLASH_TYPE_PARALLEL,
    BCM63XX_FLASH_TYPE_SERIAL,
    BCM63XX_FLASH_TYPE_NAND,
}

// C declaration: int __init bcm63xx_flash_register(void);
// The __init annotation is a build-time kernel attribute with no direct
// file-local Rust equivalent.
unsafe extern "C" {
    pub fn bcm63xx_flash_register() -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
