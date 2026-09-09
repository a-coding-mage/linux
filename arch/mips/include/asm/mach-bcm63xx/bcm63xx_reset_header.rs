/* SPDX-License-Identifier: GPL-2.0 */
// C header guard: __BCM63XX_RESET_H

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum bcm63xx_core_reset {
    BCM63XX_RESET_SPI,
    BCM63XX_RESET_ENET,
    BCM63XX_RESET_USBH,
    BCM63XX_RESET_USBD,
    BCM63XX_RESET_SAR,
    BCM63XX_RESET_DSL,
    BCM63XX_RESET_EPHY,
    BCM63XX_RESET_ENETSW,
    BCM63XX_RESET_PCM,
    BCM63XX_RESET_MPI,
    BCM63XX_RESET_PCIE,
    BCM63XX_RESET_PCIE_EXT,
}

unsafe extern "C" {
    pub fn bcm63xx_core_set_reset(reset_core: bcm63xx_core_reset, reset: ::core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
