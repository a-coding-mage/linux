/* SPDX-License-Identifier: GPL-2.0 */

/* The C header defines this alias only when building the kernel. */
pub type kernel_ulong_t = usize;

/* PCMCIA */

pub const PCMCIA_DEV_ID_MATCH_MANF_ID: u16 = 0x0001;
pub const PCMCIA_DEV_ID_MATCH_CARD_ID: u16 = 0x0002;
pub const PCMCIA_DEV_ID_MATCH_FUNC_ID: u16 = 0x0004;
pub const PCMCIA_DEV_ID_MATCH_FUNCTION: u16 = 0x0008;
pub const PCMCIA_DEV_ID_MATCH_PROD_ID1: u16 = 0x0010;
pub const PCMCIA_DEV_ID_MATCH_PROD_ID2: u16 = 0x0020;
pub const PCMCIA_DEV_ID_MATCH_PROD_ID3: u16 = 0x0040;
pub const PCMCIA_DEV_ID_MATCH_PROD_ID4: u16 = 0x0080;
pub const PCMCIA_DEV_ID_MATCH_DEVICE_NO: u16 = 0x0100;
pub const PCMCIA_DEV_ID_MATCH_FAKE_CIS: u16 = 0x0200;
pub const PCMCIA_DEV_ID_MATCH_ANONYMOUS: u16 = 0x0400;

#[repr(C)]
pub struct pcmcia_device_id {
    pub match_flags: u16,

    pub manf_id: u16,
    pub card_id: u16,

    pub func_id: u8,

    /* for real multi-function devices */
    pub function: u8,

    /* for pseudo multi-function devices */
    pub device_no: u8,

    pub prod_id_hash: [u32; 4],

    /* not matched against in kernelspace */
    pub prod_id: [*const core::ffi::c_char; 4],

    /* not matched against */
    pub driver_info: kernel_ulong_t,
    pub cisfile: *mut core::ffi::c_char,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
