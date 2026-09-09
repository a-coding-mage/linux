/* SPDX-License-Identifier: GPL-2.0-only */

// CONFIG_X86 selects the memory-encryption physical-address conversion in the
// original header; these symbols are supplied by the surrounding translation.
#[cfg(feature = "CONFIG_X86")]
macro_rules! __psp_pa {
    ($x:expr) => {
        __sme_pa($x)
    };
}

#[cfg(not(feature = "CONFIG_X86"))]
macro_rules! __psp_pa {
    ($x:expr) => {
        __pa($x)
    };
}

/*
 * Fields and bits used by most PSP mailboxes
 *
 * Note: Some mailboxes (such as SEV) have extra bits or different meanings
 * and should include an appropriate local definition in their source file.
 */
pub const PSP_CMDRESP_STS: u32 = 0x0000_ffff;
pub const PSP_TEE_STS_RING_BUSY: u32 = 0x0000_000d; // Ring already initialized
pub const PSP_CMDRESP_CMD: u32 = 0x00ff_0000;
pub const PSP_CMDRESP_RESERVED: u32 = 0x3f00_0000;
pub const PSP_CMDRESP_RECOVERY: u32 = 1u32 << 30;
pub const PSP_CMDRESP_RESP: u32 = 1u32 << 31;

pub const PSP_DRBL_MSG: u32 = PSP_CMDRESP_CMD;
pub const PSP_DRBL_RING: u32 = 1u32 << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
