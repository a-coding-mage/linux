/* SPDX-License-Identifier: GPL-2.0 */
/*
 *   Machine check handler definitions
 *
 *    Copyright IBM Corp. 2000, 2009
 *    Author(s): Ingo Adlung <adlung@de.ibm.com>,
 *		 Martin Schwidefsky <schwidefsky@de.ibm.com>,
 *		 Cornelia Huck <cornelia.huck@de.ibm.com>,
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// not implemented here.

pub const MCIC_SUBCLASS_MASK: u64 = (1u64 << 63) | (1u64 << 62) | (1u64 << 61)
    | (1u64 << 59) | (1u64 << 58) | (1u64 << 56) | (1u64 << 55)
    | (1u64 << 54) | (1u64 << 53) | (1u64 << 52) | (1u64 << 47)
    | (1u64 << 46) | (1u64 << 45) | (1u64 << 44);
pub const MCCK_CODE_SYSTEM_DAMAGE: u64 = 1u64 << 63;
pub const MCCK_CODE_EXT_DAMAGE: u64 = 1u64 << (63 - 5);
pub const MCCK_CODE_CP: u64 = 1u64 << (63 - 9);
pub const MCCK_CODE_CK: u64 = 1u64 << (63 - 11);
pub const MCCK_CODE_STG_ERROR: u64 = 1u64 << (63 - 16);
pub const MCCK_CODE_STG_KEY_ERROR: u64 = 1u64 << (63 - 18);
pub const MCCK_CODE_STG_DEGRAD: u64 = 1u64 << (63 - 19);
pub const MCCK_CODE_PSW_MWP_VALID: u64 = 1u64 << (63 - 20);
pub const MCCK_CODE_PSW_IA_VALID: u64 = 1u64 << (63 - 23);
pub const MCCK_CODE_STG_FAIL_ADDR: u64 = 1u64 << (63 - 24);
pub const MCCK_CODE_CR_VALID: u64 = 1u64 << (63 - 29);
pub const MCCK_CODE_GS_VALID: u64 = 1u64 << (63 - 36);
pub const MCCK_CODE_FC_VALID: u64 = 1u64 << (63 - 43);
pub const MCCK_CODE_CPU_TIMER_VALID: u64 = 1u64 << (63 - 46);

pub const MCCK_CODE_NO_GUEST: u64 = MCCK_CODE_CP | MCCK_CODE_EXT_DAMAGE | MCCK_CODE_CK;

// C bit-fields are represented by their containing 64-bit word.  The named
// fields occupy the documented single-bit positions, from most to least
// significant; unnamed fields are reserved.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MciBits {
    pub raw: u64,
}

#[repr(C)]
pub union mci {
    pub val: usize,
    pub bits: MciBits,
}

pub const MCESA_ORIGIN_MASK: usize = !0x3ffusize;
pub const MCESA_LC_MASK: usize = 0xfusize;
pub const MCESA_MIN_SIZE: usize = 1024;
pub const MCESA_MAX_SIZE: usize = 2048;

#[repr(C)]
pub struct mcesa {
    pub vector_save_area: [u8; 1024],
    pub guarded_storage_save_area: [u8; 32],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn nmi_alloc_mcesa_early(mcesad: *mut u64);
    pub fn nmi_alloc_mcesa(mcesad: *mut u64) -> i32;
    pub fn nmi_free_mcesa(mcesad: *mut u64);

    pub fn s390_handle_mcck();
    pub fn s390_do_machine_check(regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
