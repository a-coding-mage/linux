/* SPDX-License-Identifier: GPL-2.0 */
/*
 *   Data definitions for channel report processing
 *    Copyright IBM Corp. 2000, 2009
 *    Author(s): Ingo Adlung <adlung@de.ibm.com>,
 *		 Martin Schwidefsky <schwidefsky@de.ibm.com>,
 *		 Cornelia Huck <cornelia.huck@de.ibm.com>,
 */

/*
 * Channel Report Word
 *
 * C bit-fields are represented by their packed 32-bit storage word.  The
 * masks and shifts preserve the source layout and integer intent.
 */
#[repr(C, packed)]
pub struct crw {
    pub bits: u32,
}

impl crw {
    pub const RES1_SHIFT: u32 = 0;
    pub const SLCT_SHIFT: u32 = 1;
    pub const OFLW_SHIFT: u32 = 2;
    pub const CHN_SHIFT: u32 = 3;
    pub const RSC_SHIFT: u32 = 4;
    pub const ANC_SHIFT: u32 = 8;
    pub const RES2_SHIFT: u32 = 9;
    pub const ERC_SHIFT: u32 = 10;
    pub const RSID_SHIFT: u32 = 16;

    pub const RES1_MASK: u32 = 0x1;
    pub const SLCT_MASK: u32 = 0x1;
    pub const OFLW_MASK: u32 = 0x1;
    pub const CHN_MASK: u32 = 0x1;
    pub const RSC_MASK: u32 = 0xf;
    pub const ANC_MASK: u32 = 0x1;
    pub const RES2_MASK: u32 = 0x1;
    pub const ERC_MASK: u32 = 0x3f;
    pub const RSID_MASK: u32 = 0xffff;
}

pub type crw_handler_t = Option<unsafe extern "C" fn(*mut crw, *mut crw, ::core::ffi::c_int)>;

extern "C" {
    pub fn crw_register_handler(rsc: ::core::ffi::c_int, handler: crw_handler_t) -> ::core::ffi::c_int;
    pub fn crw_unregister_handler(rsc: ::core::ffi::c_int);
    pub fn crw_handle_channel_report();
    pub fn crw_wait_for_channel_report();
}

pub const NR_RSCS: u32 = 16;

pub const CRW_RSC_MONITOR: u32 = 0x2; /* monitoring facility */
pub const CRW_RSC_SCH: u32 = 0x3; /* subchannel */
pub const CRW_RSC_CPATH: u32 = 0x4; /* channel path */
pub const CRW_RSC_CONFIG: u32 = 0x9; /* configuration-alert facility */
pub const CRW_RSC_CSS: u32 = 0xB; /* channel subsystem */

pub const CRW_ERC_EVENT: u32 = 0x00; /* event information pending */
pub const CRW_ERC_AVAIL: u32 = 0x01; /* available */
pub const CRW_ERC_INIT: u32 = 0x02; /* initialized */
pub const CRW_ERC_TERROR: u32 = 0x03; /* temporary error */
pub const CRW_ERC_IPARM: u32 = 0x04; /* installed parm initialized */
pub const CRW_ERC_TERM: u32 = 0x05; /* terminal */
pub const CRW_ERC_PERRN: u32 = 0x06; /* perm. error, fac. not init */
pub const CRW_ERC_PERRI: u32 = 0x07; /* perm. error, facility init */
pub const CRW_ERC_PMOD: u32 = 0x08; /* installed parameters modified */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
