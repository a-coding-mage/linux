/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  S390 version
 *
 *  Derived from "include/asm-i386/types.h"
 */

/* C header guard: _UAPI_S390_TYPES_H */
/* Dependency: <asm-generic/int-ll64.h> */

/* The declarations below are excluded when compiling as an assembler source. */

pub type addr_t = core::ffi::c_ulong;
pub type saddr_t = core::ffi::c_long;

#[repr(C)]
pub struct __vector128_high_low {
    pub high: __u64,
    pub low: __u64,
}

#[repr(C)]
pub union __vector128_union {
    pub high_low: __vector128_high_low,
    pub u: [__u32; 4],
}

#[repr(C, packed(4))]
pub struct __vector128 {
    pub value: __vector128_union,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
