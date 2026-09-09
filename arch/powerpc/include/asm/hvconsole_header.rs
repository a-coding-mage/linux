/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * hvconsole.h
 * Copyright (C) 2004 Ryan S Arnold, IBM Corporation
 *
 * LPAR console support.
 */

/* Declarations are available only for kernel builds in the original header. */

/*
 * PSeries firmware will only send/recv up to 16 bytes of character data per
 * hcall.
 */
pub const MAX_VIO_PUT_CHARS: usize = 16;
pub const SIZE_VIO_GET_CHARS: usize = 16;

/*
 * Vio firmware always attempts to fetch MAX_VIO_GET_CHARS chars.  The 'count'
 * parm is included to conform to put_chars() function pointer template
 */
unsafe extern "C" {
    pub fn hvc_get_chars(vtermno: u32, buf: *mut u8, count: usize) -> isize;
    pub fn hvc_put_chars(vtermno: u32, buf: *const u8, count: usize) -> isize;

    /* Provided by HVC VIO */
    pub fn hvc_vio_init_early();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
