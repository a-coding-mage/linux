/* SPDX-License-Identifier: GPL-2.0 */
/************************************************************************
File  : Clock H/w specific Information

Author: Pankaj Dev <pankaj.dev@st.com>

Copyright (C) 2014 STMicroelectronics
************************************************************************/

// Dependency intent from the original header: supplied by other kernel code.
extern "C" {
    static mut clkgen_a9_lock: spinlock_t;
}

#[repr(C)]
pub struct clkgen_field {
    pub offset: ::core::ffi::c_uint,
    pub mask: ::core::ffi::c_uint,
    pub shift: ::core::ffi::c_uint,
}

#[inline]
pub unsafe fn clkgen_read(
    base: *mut ::core::ffi::c_void,
    field: *mut clkgen_field,
) -> ::core::ffi::c_ulong {
    (readl((base as *mut u8).add((*field).offset as usize) as *mut ::core::ffi::c_void)
        >> (*field).shift)
        & (*field).mask as ::core::ffi::c_ulong
}

#[inline]
pub unsafe fn clkgen_write(
    base: *mut ::core::ffi::c_void,
    field: *mut clkgen_field,
    val: ::core::ffi::c_ulong,
) {
    writel(
        (readl((base as *mut u8).add((*field).offset as usize) as *mut ::core::ffi::c_void)
            & !((*field).mask as ::core::ffi::c_ulong << (*field).shift))
            | (val << (*field).shift),
        (base as *mut u8).add((*field).offset as usize) as *mut ::core::ffi::c_void,
    );
}

// Dependency intent from the original header: readl and writel are supplied by other code.
extern "C" {
    fn readl(addr: *mut ::core::ffi::c_void) -> ::core::ffi::c_ulong;
    fn writel(value: ::core::ffi::c_ulong, addr: *mut ::core::ffi::c_void);
}

#[macro_export]
macro_rules! CLKGEN_FIELD {
    ($offset:expr, $mask:expr, $shift:expr) => {
        $crate::clkgen_field {
            offset: $offset,
            mask: $mask,
            shift: $shift,
        }
    };
}

#[macro_export]
macro_rules! CLKGEN_READ {
    ($pll:expr, $field:ident) => {
        $crate::clkgen_read($pll.regs_base, &mut $pll.data.$field)
    };
}

#[macro_export]
macro_rules! CLKGEN_WRITE {
    ($pll:expr, $field:ident, $val:expr) => {
        $crate::clkgen_write($pll.regs_base, &mut $pll.data.$field, $val)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
