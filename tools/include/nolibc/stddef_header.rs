/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Stddef definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C dependency: #include "nolibc.h" */

/* C header guard: _NOLIBC_STDDEF_H */

/* C dependency: #include "stdint.h" */

/* note: may already be defined */
/* C conditional: #ifndef NULL */
pub const NULL: *mut core::ffi::c_void = core::ptr::null_mut();
/* C conditional: #endif */

/* C conditional: #ifndef offsetof */
#[macro_export]
macro_rules! offsetof {
    ($TYPE:ty, $FIELD:tt) => {
        core::mem::offset_of!($TYPE, $FIELD)
    };
}
/* C conditional: #endif */

/* end C header guard: _NOLIBC_STDDEF_H */
