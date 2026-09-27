/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    Copyright IBM Corp. 1999, 2023
 */

//! Translation of the s390 translation-exception identification (TEID) header.

/// C `union teid`.
///
/// The C member `val` contains the complete value.  The alternate C member is
/// a set of implementation-defined C bit-fields; Rust has no native bit-field
/// syntax, so the same representation is exposed through `bits` and the
/// masks/shifts below.
#[repr(C)]
pub union teid {
    pub val: kernel::ffi::c_ulong,
    pub bits: teid_bits,
}

/// Raw representation of the bit-field view of [`teid`].
#[repr(C)]
#[derive(Copy, Clone)]
pub struct teid_bits {
    pub raw: kernel::ffi::c_ulong,
}

impl teid_bits {
    pub const ADDR_MASK: kernel::ffi::c_ulong = ((1 as kernel::ffi::c_ulong) << 52) - 1;
    pub const FSI_SHIFT: u32 = 52;
    pub const FSI_MASK: kernel::ffi::c_ulong = (0x3 as kernel::ffi::c_ulong) << Self::FSI_SHIFT;
    pub const B56_MASK: kernel::ffi::c_ulong = (1 as kernel::ffi::c_ulong) << 56;
    pub const B60_MASK: kernel::ffi::c_ulong = (1 as kernel::ffi::c_ulong) << 60;
    pub const B61_MASK: kernel::ffi::c_ulong = (1 as kernel::ffi::c_ulong) << 61;
    pub const AS_SHIFT: u32 = 62;
    pub const AS_MASK: kernel::ffi::c_ulong = (0x3 as kernel::ffi::c_ulong) << Self::AS_SHIFT;

    #[inline]
    pub const fn addr(self) -> kernel::ffi::c_ulong {
        self.raw & Self::ADDR_MASK
    }

    #[inline]
    pub const fn fsi(self) -> kernel::ffi::c_ulong {
        (self.raw & Self::FSI_MASK) >> Self::FSI_SHIFT
    }

    #[inline]
    pub const fn b56(self) -> bool {
        self.raw & Self::B56_MASK != 0
    }

    #[inline]
    pub const fn b60(self) -> bool {
        self.raw & Self::B60_MASK != 0
    }

    #[inline]
    pub const fn b61(self) -> bool {
        self.raw & Self::B61_MASK != 0
    }

    #[inline]
    pub const fn as_(self) -> kernel::ffi::c_ulong {
        (self.raw & Self::AS_MASK) >> Self::AS_SHIFT
    }
}

pub const TEID_FSI_UNKNOWN: kernel::ffi::c_int = 0;
pub const TEID_FSI_STORE: kernel::ffi::c_int = 1;
pub const TEID_FSI_FETCH: kernel::ffi::c_int = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
