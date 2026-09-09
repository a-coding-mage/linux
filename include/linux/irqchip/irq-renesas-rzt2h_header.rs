/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Renesas RZ/T2H Interrupt Control Unit (ICU)
 *
 * Copyright (C) 2025 Renesas Electronics Corporation.
 */

// Dependency supplied by the corresponding platform-device bindings.

pub const RZT2H_ICU_DMAC_REQ_NO_DEFAULT: u16 = 0x3ff;

// CONFIG_RENESAS_RZT2H_ICU selects the external implementation at build time.
#[cfg(CONFIG_RENESAS_RZT2H_ICU)]
unsafe extern "C" {
    pub fn rzt2h_icu_register_dma_req(
        icu_dev: *mut platform_device,
        dmac_index: u8,
        dmac_channel: u8,
        req_no: u16,
    );
}

#[cfg(not(CONFIG_RENESAS_RZT2H_ICU))]
#[inline]
pub unsafe fn rzt2h_icu_register_dma_req(
    _icu_dev: *mut platform_device,
    _dmac_index: u8,
    _dmac_channel: u8,
    _req_no: u16,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
