/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// linux/bitops.h, linux/serial_core.h, linux/sh_dma.h

/*
 * Generic header for SuperH (H)SCI(F) (used by sh/sh64 and related parts)
 */

/* Serial Control Register (@ = not supported by all parts) */
pub const SCSCR_TIE: u32 = 1u32 << 7; /* Transmit Interrupt Enable */
pub const SCSCR_RIE: u32 = 1u32 << 6; /* Receive Interrupt Enable */
pub const SCSCR_TE: u32 = 1u32 << 5; /* Transmit Enable */
pub const SCSCR_RE: u32 = 1u32 << 4; /* Receive Enable */
pub const SCSCR_REIE: u32 = 1u32 << 3; /* Receive Error Interrupt Enable @ */
pub const SCSCR_TOIE: u32 = 1u32 << 2; /* Timeout Interrupt Enable @ */
pub const SCSCR_CKE1: u32 = 1u32 << 1; /* Clock Enable 1 */
pub const SCSCR_CKE0: u32 = 1u32 << 0; /* Clock Enable 0 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SciRegType {
    SCIx_PROBE_REGTYPE,
    SCIx_SCI_REGTYPE,
    SCIx_IRDA_REGTYPE,
    SCIx_SCIFA_REGTYPE,
    SCIx_SCIFB_REGTYPE,
    SCIx_SH2_SCIF_FIFODATA_REGTYPE,
    SCIx_SH3_SCIF_REGTYPE,
    SCIx_SH4_SCIF_REGTYPE,
    SCIx_SH4_SCIF_BRG_REGTYPE,
    SCIx_SH4_SCIF_NO_SCSPTR_REGTYPE,
    SCIx_SH4_SCIF_FIFODATA_REGTYPE,
    SCIx_SH7705_SCIF_REGTYPE,
    SCIx_HSCIF_REGTYPE,
    SCIx_RZ_SCIFA_REGTYPE,
    SCIx_RZV2H_SCIF_REGTYPE,
    SCIx_NR_REGTYPES,
}

#[repr(C)]
pub struct PlatSciPortOps {
    pub init_pins: Option<unsafe extern "C" fn(*mut uart_port, u32)>,
}

/*
 * Platform device specific platform_data struct
 */
#[repr(C)]
pub struct PlatSciPort {
    pub r#type: u32, /* SCI / SCIF / IRDA / HSCIF */

    pub sampling_rate: u32,
    pub scscr: u32, /* SCSCR initialization */

    /*
     * Platform overrides if necessary, defaults otherwise.
     */
    pub regtype: u8,

    pub ops: *mut PlatSciPortOps,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
