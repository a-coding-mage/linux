/* SPDX-License-Identifier: GPL-2.0-or-later */
/* ASN.1 decoder
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C dependencies:
// #include <linux/asn1.h>
// #include <linux/types.h>

#[repr(C)]
pub struct asn1_decoder {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn asn1_ber_decoder(
        decoder: *const asn1_decoder,
        context: *mut core::ffi::c_void,
        data: *const u8,
        datalen: usize,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
