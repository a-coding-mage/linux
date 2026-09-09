/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Renesas RZ/V2H(P) Interrupt Control Unit (ICU)
 *
 * Copyright (C) 2025 Renesas Electronics Corporation.
 */

// Dependency supplied by the platform-device headers in the surrounding tree.
pub use crate::linux::platform_device::platform_device;

pub const RZV2H_ICU_DMAC_REQ_NO_DEFAULT: u16 = 0x3ff;
pub const RZV2H_ICU_DMAC_ACK_NO_DEFAULT: u16 = 0x7f;

// CONFIG_RENESAS_RZV2H_ICU selects the externally implemented declarations.
#[cfg(CONFIG_RENESAS_RZV2H_ICU)]
extern "C" {
    pub fn rzv2h_icu_register_dma_req(
        icu_dev: *mut platform_device,
        dmac_index: u8,
        dmac_channel: u8,
        req_no: u16,
    );

    pub fn rzv2h_icu_register_dma_ack(
        icu_dev: *mut platform_device,
        dmac_index: u8,
        dmac_channel: u8,
        ack_no: u16,
    );
}

#[cfg(not(CONFIG_RENESAS_RZV2H_ICU))]
#[inline]
pub unsafe fn rzv2h_icu_register_dma_req(
    _icu_dev: *mut platform_device,
    _dmac_index: u8,
    _dmac_channel: u8,
    _req_no: u16,
) {
}

#[cfg(not(CONFIG_RENESAS_RZV2H_ICU))]
#[inline]
pub unsafe fn rzv2h_icu_register_dma_ack(
    _icu_dev: *mut platform_device,
    _dmac_index: u8,
    _dmac_channel: u8,
    _ack_no: u16,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
