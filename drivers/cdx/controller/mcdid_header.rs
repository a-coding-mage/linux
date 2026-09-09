/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2008-2013 Solarflare Communications Inc.
 * Copyright (C) 2022-2025, Advanced Micro Devices, Inc.
 */

// C dependencies supplied by the surrounding kernel/CDX translation.

#[cfg(debug_assertions)]
macro_rules! CDX_WARN_ON_ONCE_PARANOID {
    ($x:expr) => { WARN_ON_ONCE($x) };
}

#[cfg(debug_assertions)]
macro_rules! CDX_WARN_ON_PARANOID {
    ($x:expr) => { WARN_ON($x) };
}

#[cfg(not(debug_assertions))]
macro_rules! CDX_WARN_ON_ONCE_PARANOID {
    ($x:expr) => {{ let _ = &$x; }};
}

#[cfg(not(debug_assertions))]
macro_rules! CDX_WARN_ON_PARANOID {
    ($x:expr) => {{ let _ = &$x; }};
}

pub const MCDI_BUF_LEN: usize = 8 + MCDI_CTL_SDU_LEN_MAX;

#[inline]
pub unsafe fn cdx_mcdi_if(cdx: *mut cdx_mcdi) -> *mut cdx_mcdi_iface {
    if (*cdx).mcdi.is_null() {
        core::ptr::null_mut()
    } else {
        &mut (*(*cdx).mcdi).iface
    }
}

extern "C" {
    pub fn cdx_mcdi_rpc_async(
        cdx: *mut cdx_mcdi,
        cmd: core::ffi::c_uint,
        inbuf: *const cdx_dword,
        inlen: usize,
        complete: cdx_mcdi_async_completer,
        cookie: core::ffi::c_ulong,
    ) -> core::ffi::c_int;

    pub fn cdx_mcdi_wait_for_quiescence(
        cdx: *mut cdx_mcdi,
        timeout_jiffies: core::ffi::c_uint,
    ) -> core::ffi::c_int;
}

/*
 * We expect that 16- and 32-bit fields in MCDI requests and responses
 * are appropriately aligned, but 64-bit fields are only
 * 32-bit-aligned.
 */

// Rust has no stable token-pasting operator. Callers provide the corresponding
// field-length constant as the third argument when using this direct equivalent.
macro_rules! MCDI_BYTE {
    ($buf:expr, $field:expr, $len:expr) => {{
        const _: () = assert!($len == 1);
        unsafe { *MCDI_PTR($buf, $field) }
    }};
}

macro_rules! MCDI_WORD {
    ($buf:expr, $field:expr, $len:expr) => {{
        const _: () = assert!($len == 2);
        unsafe { u16::from_le(*(MCDI_PTR($buf, $field) as *const u16)) }
    }};
}

macro_rules! MCDI_POPULATE_DWORD_1 {
    ($buf:expr, $field:expr, $name1:expr, $value1:expr) => {{
        unsafe { CDX_POPULATE_DWORD_1(*_MCDI_DWORD($buf, $field), $name1, $value1) }
    }};
}

macro_rules! MCDI_SET_QWORD {
    ($buf:expr, $field:expr, $value:expr) => {{
        unsafe {
            CDX_POPULATE_DWORD_1(_MCDI_DWORD($buf, $field)[0], CDX_DWORD, ($value as u32));
            CDX_POPULATE_DWORD_1(
                _MCDI_DWORD($buf, $field)[1],
                CDX_DWORD,
                (($value as u64) >> 32),
            );
        }
    }};
}

macro_rules! MCDI_QWORD {
    ($buf:expr, $field:expr) => {{
        unsafe {
            (CDX_DWORD_FIELD(_MCDI_DWORD($buf, $field)[0], CDX_DWORD) as u64)
                | ((CDX_DWORD_FIELD(_MCDI_DWORD($buf, $field)[1], CDX_DWORD) as u64) << 32)
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
